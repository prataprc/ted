use log::error;
use tree_sitter as ts;

use std::{cmp, iter::FromIterator};

use crate::{
    buffer::Buffer,
    color_scheme::{ColorScheme, Highlight},
    event::DP,
    term::Style,
    term::{Span, Spanline},
    tss::{Automata, Token},
    window::WinBuffer,
    Result,
};

pub fn highlight(
    buf: &Buffer,
    scheme: &ColorScheme,
    tree: &ts::Tree,
    atmt: &mut Automata,
    from: usize,
    to: usize,
) -> Option<Spanline> {
    let canvas = scheme.to_style(Highlight::Canvas);
    let root = tree.root_node();
    let mut syns = {
        let (depth, sibling) = (0, 0);
        let tok = Token::from_node(buf, &root, depth, sibling);
        match atmt.shift_in(&tok) {
            Ok(Some(style)) => vec![SyntSpan {
                depth: tok.depth,
                a: tok.a,
                z: tok.z,
                style,
            }],
            Ok(None) => vec![],
            Err(err) => {
                error!("highlighting {}", err);
                return None;
            }
        }
    };

    let depth = 1;
    match do_highlight(buf, scheme, tree, atmt, root, depth, from, to) {
        Ok(ls) => syns.extend(ls),
        Err(err) => {
            error!("highlighting {}", err);
            return None;
        }
    }
    syns.sort();

    let mut hl_spans = HlSpans::new(canvas);
    syns.into_iter().for_each(|syn| hl_spans.push(syn));

    match hl_spans.into_span_line(buf) {
        Ok(spl) => Some(spl),
        Err(err) => {
            error!("highlighting {}", err);
            None
        }
    }
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
