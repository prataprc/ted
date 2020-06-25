#[allow(unused_imports)]
use log::trace;
use tree_sitter as ts;

use std::{cmp, fmt, iter::FromIterator, result};

use crate::{
    buffer::Buffer,
    colors::{ColorScheme, Highlight},
    event::DP,
    term::Style,
    term::{Span, Spanline},
    tss::{Automata, Token},
    window::WinBuffer,
    Error, Result,
};

/// Syntax highlighting using tree-sitter and ted-style-sheet automata.
pub fn highlight(
    buf: &Buffer,
    scheme: &ColorScheme,
    tree: &ts::Tree,
    atmt: &mut Automata,
    from: usize,
    till: usize,
) -> Result<Spanline> {
    let canvas = scheme.to_style(Highlight::Canvas);
    let root = tree.root_node();
    let mut syns = {
        let (depth, sibling) = (0, 0);
        let tok = Token::from_node(buf, &root, depth, sibling);
        trace!("{}", tok);
        match atmt.shift_in(&tok)? {
            Some(style) => vec![SyntSpan {
                depth: tok.depth,
                a: tok.a,
                z: tok.z,
                style,
            }],
            None => vec![],
        }
    };

    // trace!("highlight {}..{} syns:{:?}", from, till, syns);

    let depth = 1;
    syns.extend(do_highlight(
        buf, scheme, tree, atmt, root, depth, from, till,
    )?);
    syns.sort();
    trace!("sorted syns:{:?}", syns);

    let mut hl_spans = HlSpans::new(canvas, from, till);
    for syn in syns.into_iter() {
        hl_spans.push(syn)?
    }

    trace!("Hlspans {}", hl_spans);

    hl_spans.into_span_line(buf)
}

fn do_highlight(
    buf: &Buffer,
    scheme: &ColorScheme,
    tree: &ts::Tree,
    atmt: &mut Automata,
    node: ts::Node,
    depth: usize, // 0 is root level
    from: usize,  // character offset to highlight, inclusive
    till: usize,  // character offset to highlight, exclusive
) -> Result<Vec<SyntSpan>> {
    let mut syns = vec![];
    let mut tc = node.walk();

    let children: Vec<ts::Node> = node.children(&mut tc).collect();
    let mut toks = Vec::with_capacity(16);
    for (sibling, child) in children.iter().enumerate() {
        let tok = Token::from_node(buf, child, depth, sibling);
        let overlap = tok.is_overlap(from, till);
        // trace!("{} overlap:{}", tok, overlap);
        if overlap {
            match atmt.shift_in(&tok)? {
                Some(style) => syns.push(SyntSpan {
                    depth: tok.depth,
                    a: tok.a,
                    z: tok.z,
                    style,
                }),
                None => (),
            }
        }
        toks.push((tok, overlap))
    }

    // trace!("do-highlight {}..{} syns:{:?}", from, till, syns);

    for (sibling, child) in children.into_iter().enumerate() {
        if toks[sibling].1 {
            syns.extend({
                let depth = depth + 1;
                do_highlight(buf, scheme, tree, atmt, child, depth, from, till)?
            });
        }
    }

    Ok(syns)
}

// list of matching spans, sort it and convert them into spanline.
struct HlSpans {
    from: usize,
    till: usize,
    canvas: Style, // default style
    syns: Vec<SyntSpan>,
}

impl fmt::Display for HlSpans {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        let ss: Vec<String> = {
            let iter = self.syns.iter().map(|syn| syn.to_string());
            iter.collect()
        };
        write!(f, "{{{}}}", ss.join(","))
    }
}

impl HlSpans {
    fn new(canvas: Style, from: usize, till: usize) -> HlSpans {
        HlSpans {
            from,
            till,
            canvas,
            syns: Default::default(),
        }
    }

    fn clip_span(&self, mut syn: SyntSpan) -> SyntSpan {
        if self.from > syn.a {
            syn.a = self.from
        }
        if self.till < syn.z {
            syn.z = self.till
        }
        syn
    }

    fn push(&mut self, mut syn: SyntSpan) -> Result<()> {
        syn = self.clip_span(syn);
        match self.syns.len() {
            0 if self.from < syn.a => {
                // There is an un-matched span.
                self.syns.push(SyntSpan {
                    depth: 0,
                    a: self.from,
                    z: syn.a,
                    style: self.canvas.clone(),
                });
                self.syns.push(syn);
            }
            0 => self.syns.push(syn),
            _ => {
                trace!("syn: {} syns: {:?}", syn, self.syns);
                let SyntSpan { depth, a, z, style } = self.syns.pop().unwrap();
                assert!(a <= syn.a, "{}..{} {}", a, z, syn);
                if z < syn.a {
                    // there is a gap between two syntax-span.
                    // |..old..| <gap> |..new..|
                    self.syns.push(SyntSpan { depth, a, z, style });
                    self.syns.push(SyntSpan {
                        depth: 0,
                        a: z,
                        z: syn.a,
                        style: self.canvas.clone(),
                    });
                    self.syns.push(syn);
                } else if z == syn.a {
                    // perfect alignment.
                    // |..old..|..new..|
                    self.syns.push(SyntSpan { depth, a, z, style });
                    self.syns.push(syn)
                } else if a <= syn.a && syn.z <= z && syn.depth > depth {
                    // fully contained
                    // |..old..|..new..|..old..|
                    self.syns.push(SyntSpan {
                        depth,
                        a,
                        z: syn.a,
                        style: style.clone(),
                    });
                    self.syns.push(syn.clone());
                    self.syns.push(SyntSpan {
                        depth,
                        a: syn.z,
                        z,
                        style,
                    });
                } else if a <= syn.a && syn.z <= z {
                    // fully contained
                    // |..old..|
                    self.syns.push(SyntSpan {
                        depth,
                        a,
                        z,
                        style: style.clone(),
                    });
                } else if syn.depth >= depth {
                    // overlap, incoming span is more specific
                    self.syns.push(SyntSpan {
                        depth,
                        a,
                        z: syn.a,
                        style,
                    });
                    self.syns.push(syn);
                } else {
                    // overlap, existing span is more specific.
                    self.syns.push(SyntSpan { depth, a, z, style });
                    syn.a = z;
                    self.syns.push(syn)
                }
            }
        }

        Ok(())
    }

    fn into_span_line(&mut self, buf: &Buffer) -> Result<Spanline> {
        match self.syns.pop() {
            Some(SyntSpan { depth, a, z, style }) if z < self.till => {
                self.syns.push(SyntSpan { depth, a, z, style });
                self.syns.push({
                    let style = self.canvas.clone();
                    SyntSpan {
                        depth: 0,
                        a: z,
                        z: self.till,
                        style,
                    }
                });
            }
            Some(SyntSpan { depth, a, z, style }) if z == self.till => {
                self.syns.push(SyntSpan { depth, a, z, style });
            }
            Some(SyntSpan { z, .. }) => {
                err_at!(Fatal, msg: format!("misaligned {} {}", z, self.till))?
            }
            None => {
                self.syns.push({
                    let style = self.canvas.clone();
                    SyntSpan {
                        depth: 0,
                        a: self.from,
                        z: self.till,
                        style,
                    }
                });
            }
        }

        let mut spans: Vec<Span> = vec![];
        for syn in self.syns.iter() {
            spans.push(syn.into_span(buf)?);
        }
        Ok(spans.into_iter().collect())
    }
}

// matching syntax span with tss-automata, represents a single span/style.
#[derive(Clone)]
struct SyntSpan {
    depth: usize,
    a: usize, // character position, inclusive
    z: usize, // character position, exclusive
    style: Style,
}

impl fmt::Display for SyntSpan {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "[{},{}..{}]", self.depth, self.a, self.z)
    }
}

impl fmt::Debug for SyntSpan {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "[{},{}..{}]", self.depth, self.a, self.z)
    }
}

impl Eq for SyntSpan {}

impl PartialEq for SyntSpan {
    fn eq(&self, other: &Self) -> bool {
        self.a.eq(&other.a) && self.z.eq(&other.z)
    }
}

impl PartialOrd for SyntSpan {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        match self.a.partial_cmp(&other.a) {
            Some(cmp::Ordering::Equal) => {
                let (m, n) = (self.z - self.a, other.z - other.a);
                match m.partial_cmp(&n) {
                    Some(cmp::Ordering::Equal) => {
                        //
                        self.depth.partial_cmp(&other.depth)
                    }
                    cval => cval,
                }
            }
            cval => cval,
        }
    }
}

// a. lower value of start_char_idx sort before.
// b. if start_char_idx is equal sort by length.
// c. if length is equal sort by depth, more deeper (specific) span sort after.
impl Ord for SyntSpan {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        match self.a.cmp(&other.a) {
            cmp::Ordering::Equal => {
                let (m, n) = (self.z - self.a, other.z - other.a);
                match m.cmp(&n) {
                    cmp::Ordering::Equal => self.depth.cmp(&other.depth),
                    cval => cval,
                }
            }
            cval => cval,
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
