#[allow(unused_imports)]
use log::{debug, error, trace};
use tree_sitter as ts;

use crate::{
    buffer::{self, Buffer},
    colors::{ColorScheme, Highlight},
    event::Event,
    syntax::Syntax,
    term::{Span, Spanline},
    Error, Result,
};

extern "C" {
    fn tree_sitter_txt_plain() -> ts::Language;
}

pub struct PlainText {
    parser: ts::Parser,
    tree: Option<ts::Tree>,
    scheme: ColorScheme,
}

impl PlainText {
    pub fn new(s: &str, scheme: &ColorScheme) -> Result<PlainText> {
        let lang = unsafe { tree_sitter_txt_plain() };
        let mut parser = {
            let mut parser = ts::Parser::new();
            err_at!(FailParse, parser.set_language(lang))?;
            parser
        };
        let tree = match parser.parse(s, None) {
            Some(tree) => {
                debug!("lang:{:?}\n{}", lang, tree.root_node().to_sexp());
                Some(tree)
            }
            None => {
                error!("tree sitter parse failed lang:{:?}", lang);
                None
            }
        };
        Ok(PlainText {
            parser,
            tree,
            scheme: scheme.clone(),
        })
    }
}

impl Syntax for PlainText {
    #[inline]
    fn to_language(&self) -> Option<ts::Language> {
        Some(unsafe { tree_sitter_txt_plain() })
    }

    fn on_edit(&mut self, buf: &Buffer, evnt: Event) -> Result<Event> {
        let mut new_evnt: Event = Default::default();
        for evnt in evnt.into_iter() {
            match evnt {
                Event::Edit(val) => match self.tree.take() {
                    Some(mut old_tree) => {
                        old_tree.edit(&val.into());
                        let s = buf.to_string();
                        self.tree = self.parser.parse(&s, Some(&old_tree));
                    }
                    None => {
                        self.tree = self.parser.parse(&buf.to_string(), None);
                    }
                },
                evnt => new_evnt.push(evnt),
            }
        }
        Ok(new_evnt)
    }

    fn to_span_line(&self, b: &Buffer, a: usize, z: usize) -> Result<Spanline> {
        let spl = buffer::to_span_line(b, a, z)?;
        Ok(spl.using(self.scheme.to_style(Highlight::Canvas)))
    }

    fn to_status_cursor(&self) -> Result<Span> {
        let nodes: Vec<ts::Node> = match self.tree.as_ref() {
            Some(tree) => {
                let mut tc = tree.walk();
                tree.root_node().children(&mut tc).collect()
            }
            None => vec![],
        };

        let (mut ws, mut ss, mut ls, mut ps) = (0, 0, 0, 0);
        let mut prev_kind: Option<&str> = None;
        for node in nodes.into_iter() {
            match (prev_kind, node.kind()) {
                (_, "word") | (_, "wword") => ws += 1,
                (_, "dot") => ss += 1,
                (Some("nl"), "nl") => {
                    ls += 1;
                    ps += 1;
                }
                (_, "nl") => ls += 1,
                _ => err_at!(Fatal, msg: format!("unreachable"))?,
            }
            prev_kind = Some(node.kind());
        }
        Ok(format!("{} {} {} {}", ws, ls, ss, ps).into())
    }
}
