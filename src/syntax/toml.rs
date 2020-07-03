#[allow(unused_imports)]
use log::{debug, error, trace};
use tree_sitter as ts;

use crate::{
    buffer::Buffer,
    colors::ColorScheme,
    event::Event,
    syntax::{self, Syntax},
    term::{Span, Spanline},
    tss::{self, Automata},
    Error, Result,
};

extern "C" {
    fn tree_sitter_toml() -> ts::Language;
}

pub struct Toml {
    parser: ts::Parser,
    tree: ts::Tree,
    atmt: Automata,
}

impl Toml {
    pub fn new(s: &str, scheme: &ColorScheme) -> Result<Toml> {
        let lang = unsafe { tree_sitter_toml() };
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
            let atmt = Automata::from_str("toml", tss::TOML, scheme)?;
            debug!("{}", atmt);
            atmt
        };
        Ok(Toml { parser, tree, atmt })
    }
}

impl Syntax for Toml {
    #[inline]
    fn to_language(&self) -> Option<ts::Language> {
        Some(unsafe { tree_sitter_toml() })
    }

    fn on_edit(&mut self, buf: &Buffer, evnt: Event) -> Result<Event> {
        let new_evnt: Event = Default::default();
        for evnt in evnt.into_iter() {
            match evnt {
                Event::Edit(val) => match self.tree.take() {
                    Some(old_tree) => {
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
    }

    fn to_span_line(
        &self,
        buf: &Buffer,
        scheme: &ColorScheme,
        from: usize,
        to: usize,
    ) -> Result<Spanline> {
        let mut atmt = self.atmt.clone();
        syntax::highlight(buf, scheme, &self.tree, &mut atmt, from, to)
    }

    fn to_status_cursor(&self) -> Result<Span> {
        Ok(format!("").into())
    }
}
