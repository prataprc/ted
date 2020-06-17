use tree_sitter as ts;

use std::{cmp, iter::FromIterator};

use crate::{
    buffer::{self, Buffer},
    color_scheme::{ColorScheme, Highlight, Style},
    event::{Event, DP},
    tss::{Automata, Token},
    window::{Span, Spanline, WinBuffer},
    Result,
};

pub trait Page {
    fn to_language(&self) -> Option<ts::Language>;

    fn to_name(&self) -> String;

    fn on_event(&mut self, buf: &mut Buffer, evnt: Event) -> Result<Event>;

    fn to_syntax<'a>(
        &'a self,
        buf: &'a Buffer,
        scheme: &'a ColorScheme,
    ) -> Result<Option<Syntax<'a>>>;
}

pub struct Syntax<'a> {
    buf: &'a Buffer,
    tree: &'a ts::Tree,
    atmt: Automata,
    scheme: &'a ColorScheme,
}

impl<'a> Syntax<'a> {
    pub fn new(
        buf: &'a Buffer,
        tree: &'a ts::Tree,
        atmt: Automata,
        scheme: &'a ColorScheme,
    ) -> Syntax<'a> {
        Syntax {
            buf,
            tree,
            atmt,
            scheme,
        }
    }
}

impl<'a> WinBuffer<'a> for Syntax<'a> {
    type IterLine = buffer::IterLine<'a>;
    type IterChar = buffer::IterChar<'a>;

    fn to_xy_cursor(&self) -> buffer::Cursor {
        self.buf.to_xy_cursor()
    }

    fn lines_at(&'a self, line_idx: usize, dp: DP) -> Result<Self::IterLine> {
        self.buf.lines_at(line_idx, dp)
    }

    fn chars_at(&'a self, char_idx: usize, dp: DP) -> Result<Self::IterChar> {
        self.buf.chars_at(char_idx, dp)
    }

    fn line_to_char(&self, line_idx: usize) -> usize {
        self.buf.line_to_char(line_idx)
    }

    fn char_to_line(&self, char_idx: usize) -> usize {
        self.char_to_line(char_idx)
    }

    fn n_chars(&self) -> usize {
        self.buf.n_chars()
    }

    fn is_trailing_newline(&self) -> bool {
        self.is_trailing_newline()
    }

    fn to_span_line(&self, from: usize, to: usize, scheme: &ColorScheme) -> Result<Spanline> {
        todo!()
    }
}

impl<'a> Syntax<'a> {
    pub fn highlight(&mut self, from: usize, to: usize) -> Result<Spanline> {
        let canvas = self.scheme.to_style(Highlight::Canvas);
        let root = self.tree.root_node();
        let mut syns = {
            let (depth, sibling) = (0, 0);
            let tok = Token::from_node(self.buf, &root, depth, sibling);
            match self.atmt.shift_in(&tok)? {
                Some(style) => vec![SyntSpan {
                    depth: tok.depth,
                    a: tok.a,
                    z: tok.z,
                    style,
                }],
                None => vec![],
            }
        };

        syns.extend(self.do_highlight(root, 1 /*depth*/, from, to)?);
        syns.sort();

        let mut hl_spans = HlSpans::new(canvas);
        syns.into_iter().for_each(|syn| hl_spans.push(syn));

        hl_spans.into_span_line(self.buf)
    }

    fn do_highlight<'x>(
        &mut self,
        node: ts::Node<'x>,
        mut depth: usize, // 0 is root level
        from: usize,      // character offset to highlight, inclusive
        to: usize,        // character offset to highlight, exclusive
    ) -> Result<Vec<SyntSpan>> {
        let mut syns = vec![];
        let range = from..to;
        let mut tc = node.walk();

        for (sibling, child) in node.children(&mut tc).enumerate() {
            let tok = Token::from_node(self.buf, &child, depth, sibling);
            if range.contains(&tok.a) || range.contains(&tok.z) {
                match self.atmt.shift_in(&tok)? {
                    Some(style) => syns.push(SyntSpan {
                        depth: tok.depth,
                        a: tok.a,
                        z: tok.z,
                        style,
                    }),
                    None => (),
                }
            }
        }

        depth += 1;
        for (sibling, child) in node.children(&mut tc).enumerate() {
            let tok = Token::from_node(self.buf, &child, depth, sibling);
            if range.contains(&tok.a) || range.contains(&tok.z) {
                syns.extend(self.do_highlight(child, depth, from, to)?)
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
        self.a.eq(&other.a) && self.z.eq(&other.z)
    }
}

impl PartialOrd for SyntSpan {
    fn partial_cmp(&self, _other: &Self) -> Option<cmp::Ordering> {
        None
    }
}

impl Ord for SyntSpan {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        if self.a == other.a {
            (self.z - self.a).cmp(&(other.z - other.a))
        } else {
            self.a.cmp(&other.a)
        }
    }
}

impl SyntSpan {
    fn into_span(&self, buf: &Buffer) -> Result<Span> {
        let span: Span = {
            let iter = buf.chars_at(self.a, DP::Right)?.take(self.z - self.a);
            String::from_iter(iter).into()
        };
        Ok(span.using(self.style.clone()))
    }
}

struct HlSpans {
    canvas: Style, // default style
    syns: Vec<SyntSpan>,
}

impl HlSpans {
    fn new(canvas: Style) -> HlSpans {
        HlSpans {
            canvas,
            syns: Default::default(),
        }
    }

    fn push(&mut self, mut syn: SyntSpan) {
        match self.syns.len() {
            0 => self.syns.push(syn),
            _ => {
                let SyntSpan { depth, a, z, style } = self.syns.pop().unwrap();
                assert!(a <= syn.a);

                if z < syn.a {
                    self.syns.push(SyntSpan { depth, a, z, style });
                    self.syns.push(SyntSpan {
                        depth: 0,
                        a: z,
                        z: syn.a,
                        style: self.canvas.clone(),
                    });
                    self.syns.push(syn);
                } else if z == syn.a {
                    self.syns.push(SyntSpan { depth, a, z, style });
                    self.syns.push(syn)
                } else if syn.depth >= depth {
                    self.syns.push(SyntSpan {
                        depth,
                        a,
                        z: syn.a,
                        style,
                    });
                    self.syns.push(syn);
                } else {
                    syn.a = z;
                    self.syns.push(SyntSpan { depth, a, z, style });
                    self.syns.push(syn)
                }
            }
        }
    }

    fn into_span_line(&self, buf: &Buffer) -> Result<Spanline> {
        let mut spans: Vec<Span> = vec![];
        for syn in self.syns.iter() {
            spans.push(syn.into_span(buf)?);
        }
        Ok(Spanline::from_iter(spans.into_iter()))
    }
}
