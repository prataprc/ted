use std::cmp;

use crate::{buffer::Buffer, color_scheme::ColorScheme, tss::Automata};

struct Syntax<'a, 'b, 'c, 'd> {
    buf: &'a Buffer,
    tree: &'b ts::Tree;
    atm: &'c mut Automata,
    scheme: &'d ColorScheme,
}

impl Syntax {
    fn new(buf: &Buffer, tree: &ts::Tree, atm: &mut Automata, scheme: &ColorScheme) -> Syntax {
        Syntax {
            buf,
            tree,
            atm,
            scheme,
        }
    }
}

impl Syntax {
    pub fn highlight(&mut self, from: usize, to: usize) -> Spanline {
        let root = tree.root_node();
        let (depth, sibling) = (0, 0);
        let mut tc = tree.walk();

        let mut syns = {
            let tok = Token::from_node(self.buf, &root, depth, sibling);
            match self.atm.shift_in(tok)? {
                Some(style) => vec![SyntSpan{
                    depth: tok.depth, a: tok.a, z: tok.z, style
                }],
                None => vec![],
            }
        };

        syns.extend(&self.do_highlight(root, depth + 1, from, to, &mut tc)?);
        syns.sort();

        let mut hl_spans = HlSpans::new(self.scheme.to_style("canvas"));
        syns.into_iter().for_each(|syn| hl_spans.push(syn));

        hl_spans.into_span_line()
    }

    fn do_highlight(
        &mut self,
        node: ts::Node,
        mut depth: usize, // 0 is root level
        from: usize,  // character offset to highlight, inclusive
        to: usize, // character offset to highlight, exclusive
        tc: &mut ts::TreeCursor,
    ) -> Result<Vec<SyntSpan>> {
        let mut syns = vec![];
        let range = (from..to);

        for (sibling, child) in node.children(tc).enumerate() {
            let tok = Token::from_node(self.buf, &child, depth, sibling);
            if range.contains(tok.a) || range.contains(tok.z) {
                match self.atm.shift_in(tok)? {
                    Some(style) => syns.push(SyntSpan{
                        depth: tok.depth, a: tok.a, z: tok.z, style
                    }),
                    None => (),
                }
            }
        }

        depth += 1;
        for child in node.children(tc) {
            let tok = Token::from_node(self.buf, &child, depth, sibling);
            if range.contains(tok.a) || range.contains(tok.z) {
                syns.extend(&self.do_highlight(child, depth, from, to, tc))
            }
        }

        Ok(syns)
    }
}

struct SyntSpan {
    depth: usize,
    a: usize, // character position, inclusive
    z: usize, // character position, exclusive
    style: Style,
}

impl Eq for SyntSpan {}

impl PartialEq for SyntSpan {
    fn eq(&self, other: &Self) -> bool {
        self.a.eq(other.a) && self.z.eq(other.z)
    }
}

impl PartialOrd for SyntSpan {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        None
    }
}

impl Ord for SyntSpan {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        if self.a == other.a {
            (self.z - self.a).cmp(other.z - other.a)
        } else {
            self.a.cmp(other.a)
        }
    }
}

impl SyntSpan {
    fn into_span(&self, buf: &Buffer) -> Span {
        let mut span: Span = {
            let iter = buf.chars_at(self.a).take(self.z - self.a);
            String::from_iter(iter).into()
        };
        span.using(self.style)
        span
    }
}

struct HlSpans {
    canvas: Style,
    syns: Vec<SyntSpan>
}

impl HlSpans {
    fn new(canvas: Style) -> HlSpans {
        HlSpans { canvas, syns: Default::default() }
    }

    fn push(&mut self, syn: SyntSpan) {
        match self.syns.len(), {
            (0, => self.syns.push(syn),
            _, => {
                SyntSpan { depth, a, z, style } = self.syns.pop().unwrap();
                assert!(a <= syn.a);
                if z < syn.a {
                    self.spans.push(SyntSpan { depth,  a, z, style });
                    self.spans.push(SyntSpan {
                        depth: 0, a: z, z: syn.a style: self.canvas.clone()
                    });
                    self.spans.push(syn);
                } else if z == syn.a {
                    self.spans.push(syn)
                } else syn.depth >= depth {
                    self.spans.push(SyntSpan { depth, a, z: syn.a, style });
                    self.spans.push(syn);
                }
            }
        }
    }

    fn into_span_line(&self) -> Spanline {
        let spans: Vec<Span> = {
            let iter = self.syns.iter().map(|syn| syn.into_span());
            iter.collect()
        };
        spans.into()
    }
}
