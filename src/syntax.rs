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
    to: usize,
) -> Result<Spanline> {
    let canvas = scheme.to_style(Highlight::Canvas);
    let root = tree.root_node();
    let mut syns = {
        let (depth, sibling) = (0, 0);
        let tok = Token::from_node(buf, &root, depth, sibling);
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

    trace!("highlight {}..{} syns:{:?}", from, to, syns);

    let depth = 1;
    syns.extend(do_highlight(
        buf, scheme, tree, atmt, root, depth, from, to,
    )?);
    syns.sort();

    let mut hl_spans = HlSpans::new(canvas, from, to);
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
    to: usize,    // character offset to highlight, exclusive
) -> Result<Vec<SyntSpan>> {
    let mut syns = vec![];
    let range = from..to;
    let mut tc = node.walk();

    for (sibling, child) in node.children(&mut tc).enumerate() {
        let tok = Token::from_node(buf, &child, depth, sibling);
        if range.contains(&tok.a) || range.contains(&tok.z) {
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
    }

    trace!("do-highlight {}..{} syns:{:?}", from, to, syns);

    for (sibling, child) in node.children(&mut tc).enumerate() {
        let tok = Token::from_node(buf, &child, depth, sibling);
        if range.contains(&tok.a) || range.contains(&tok.z) {
            syns.extend({
                let depth = depth + 1;
                do_highlight(buf, scheme, tree, atmt, child, depth, from, to)?
            });
        }
    }

    Ok(syns)
}

// list of matching spans, sort it and convert them into spanline.
struct HlSpans {
    from: usize,
    to: usize,
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
    fn new(canvas: Style, from: usize, to: usize) -> HlSpans {
        HlSpans {
            from,
            to,
            canvas,
            syns: Default::default(),
        }
    }

    fn push(&mut self, mut syn: SyntSpan) -> Result<()> {
        match self.syns.len() {
            0 if self.from < syn.a => {
                self.syns.push(SyntSpan {
                    depth: 0,
                    a: self.from,
                    z: syn.a,
                    style: self.canvas.clone(),
                });
                self.syns.push(syn);
            }
            0 if self.from == syn.a => self.syns.push(syn),
            0 => err_at!(
                Fatal,
                msg: format!("misaligned span {} {}", self.from, syn.a)
            )?,
            _ => {
                let SyntSpan { depth, a, z, style } = self.syns.pop().unwrap();
                assert!(a <= syn.a);

                if z < syn.a {
                    // there is a gap between two syntax-span.
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
                    self.syns.push(SyntSpan { depth, a, z, style });
                    self.syns.push(syn)
                } else if z > syn.z {
                    // fully contained
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
            Some(SyntSpan { depth, a, z, style }) if z < self.to => {
                self.syns.push(SyntSpan { depth, a, z, style });
                self.syns.push({
                    let style = self.canvas.clone();
                    SyntSpan {
                        depth: 0,
                        a: z,
                        z: self.to,
                        style,
                    }
                });
            }
            Some(SyntSpan { depth, a, z, style }) if z == self.to => {
                self.syns.push(SyntSpan { depth, a, z, style });
            }
            Some(SyntSpan { z, .. }) => {
                err_at!(Fatal, msg: format!("misaligned {} {}", z, self.to))?
            }
            None => {
                self.syns.push({
                    let style = self.canvas.clone();
                    SyntSpan {
                        depth: 0,
                        a: self.from,
                        z: self.to,
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
        write!(f, "[{},{}-{}]", self.depth, self.a, self.z)
    }
}

impl fmt::Debug for SyntSpan {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "[{},{}-{}]", self.depth, self.a, self.z)
    }
}

impl Eq for SyntSpan {}

impl PartialEq for SyntSpan {
    fn eq(&self, other: &Self) -> bool {
        self.a.eq(&other.a) && self.z.eq(&other.z)
    }
}

impl PartialOrd for SyntSpan {
    fn partial_cmp(&self, _: &Self) -> Option<cmp::Ordering> {
        None
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
