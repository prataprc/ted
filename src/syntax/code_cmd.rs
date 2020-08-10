#[allow(unused_imports)]
use log::{debug, error, trace};
use tree_sitter as ts;

use crate::{
    buffer::{self, Buffer},
    colors::ColorScheme,
    event::Event,
    syntax::Syntax,
    term::{Span, Spanline},
    Error, Result,
};

extern "C" {
    fn tree_sitter_code_cmd() -> ts::Language;
}

pub struct CodeCmd {
    parser: ts::Parser,
    tree: Option<ts::Tree>,
}

impl Clone for CodeCmd {
    fn clone(&self) -> Self {
        let lang = unsafe { tree_sitter_code_cmd() };
        let parser = {
            let mut parser = ts::Parser::new();
            parser.set_language(lang).ok();
            parser
        };
        CodeCmd {
            parser,
            tree: self.tree.clone(),
        }
    }
}

impl CodeCmd {
    pub fn new(s: &str, _: ColorScheme) -> Result<CodeCmd> {
        let lang = unsafe { tree_sitter_code_cmd() };
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
        Ok(CodeCmd { parser, tree })
    }

    pub fn to_command_name(&self) -> Option<String> {
        let root = self.tree.as_ref()?.root_node();
        match root.child(root.child_count().saturating_sub(1)) {
            Some(node) => match node.kind() {
                "cmd" => Some(node.child(0).as_ref()?.kind().to_string()),
                _ => None,
            },
            None => None,
        }
    }

    pub fn into_parse_tree(self) -> Option<ts::Tree> {
        self.tree
    }
}

impl Syntax for CodeCmd {
    #[inline]
    fn to_language(&self) -> Option<ts::Language> {
        Some(unsafe { tree_sitter_code_cmd() })
    }

    fn on_edit(&mut self, buf: &Buffer, evnt: Event) -> Result<Event> {
        let mut new_evnt: Event = Event::default();
        for evnt in evnt.into_iter() {
            match evnt {
                Event::Write(val) => match self.tree.take() {
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
        buffer::to_span_line(b, a, z)
    }

    fn to_status_cursor(&self) -> Result<Span> {
        Ok(format!("").into())
    }
}
