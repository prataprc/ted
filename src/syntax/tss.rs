#[allow(unused_imports)]
use log::{debug, error, trace};
use tree_sitter as ts;

use crate::{
    buffer::{self, Buffer},
    colors::ColorScheme,
    event::Event,
    syntax::{self, Syntax},
    term::{Span, Spanline},
    tss::{self, Automata},
    Error, Result,
};

extern "C" {
    fn tree_sitter_tss() -> ts::Language;
}

pub struct Tss {
    parser: ts::Parser,
    tree: Option<ts::Tree>,
    atmt: Automata,
    scheme: ColorScheme,
}

impl Tss {
    pub fn new(s: &str, scheme: &ColorScheme) -> Result<Tss> {
        let lang = unsafe { tree_sitter_tss() };
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
        let atmt = {
            let atmt = Automata::from_str("tss", tss::TSS, scheme)?;
            debug!("{}", atmt);
            atmt
        };
        Ok(Tss {
            parser,
            tree,
            atmt,
            scheme: scheme.clone(),
        })
    }
}

impl Syntax for Tss {
    #[inline]
    fn to_language(&self) -> Option<ts::Language> {
        Some(unsafe { tree_sitter_tss() })
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
        match self.tree.as_ref() {
            Some(tree) => {
                let mut atmt = self.atmt.clone();
                syntax::highlight(b, &self.scheme, tree, &mut atmt, a, z)
            }
            None => buffer::to_span_line(b, a, z),
        }
    }

    fn to_status_cursor(&self) -> Result<Span> {
        Ok(format!("").into())
    }
}