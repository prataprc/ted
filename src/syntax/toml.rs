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
    tree: Option<ts::Tree>,
    atmt: Automata,
    scheme: ColorScheme,
}

impl Clone for Toml {
    fn clone(&self) -> Self {
        let lang = unsafe { tree_sitter_toml() };
        let parser = {
            let mut parser = ts::Parser::new();
            parser.set_language(lang).ok();
            parser
        };
        Toml {
            parser,
            tree: self.tree.clone(),
            atmt: self.atmt.clone(),
            scheme: self.scheme.clone(),
        }
    }
}

impl Toml {
    pub fn new(s: &str, scheme: ColorScheme) -> Result<Toml> {
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
            let atmt = Automata::from_str("toml", tss::TOML, &scheme)?;
            debug!("{}", atmt);
            atmt
        };
        Ok(Toml {
            parser,
            tree,
            atmt,
            scheme,
        })
    }
}

impl Syntax for Toml {
    #[inline]
    fn to_language(&self) -> Option<ts::Language> {
        Some(unsafe { tree_sitter_toml() })
    }

    fn on_edit(&mut self, buf: &Buffer, evnt: Event) -> Result<Event> {
        let mut new_evnt: Event = Event::default();
        for evnt in evnt.into_iter() {
            match evnt {
                Event::Edit(edit) => match self.tree.take() {
                    Some(mut old_tree) => {
                        old_tree.edit(&edit.into_ts_input(buf)?);
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

    fn to_span_line(&self, buf: &Buffer, a: usize, z: usize) -> Result<Spanline> {
        match self.tree.as_ref() {
            Some(tree) => {
                let mut atmt = self.atmt.clone();
                syntax::highlight(buf, &self.scheme, tree, &mut atmt, a, z)
            }
            None => buf.to_span_line(a, z),
        }
    }

    fn to_status_cursor(&self) -> Result<Span> {
        Ok(format!("").into())
    }
}
