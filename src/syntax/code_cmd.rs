#[allow(unused_imports)]
use log::{debug, trace};
use tree_sitter as ts;

use crate::{
    buffer::Buffer,
    colors::ColorScheme,
    event::Event,
    syntax,
    term::Spanline,
    tss::{self, Automata},
    Error, Result,
};

extern "C" {
    fn tree_sitter_code_cmd() -> ts::Language;
}

pub struct CodeCmd {
    parser: ts::Parser,
    tree: ts::Tree,
    atmt: Automata,
}

impl CodeCmd {
    pub fn new(s: &str, scheme: &ColorScheme) -> Result<CodeCmd> {
        let lang = unsafe { tree_sitter_code_cmd() };
        let (parser, tree) = syntax::new_parser(s, lang)?;
        let atmt = Automata::from_str("code_cmd", tss::CODE_CMD, scheme)?;

        debug!("{}", atmt);

        Ok(CodeCmd { parser, tree, atmt })
    }
}

impl CodeCmd {
    #[inline]
    pub fn to_language(&self) -> Option<ts::Language> {
        Some(unsafe { tree_sitter_code_cmd() })
    }

    pub fn on_edit(&mut self, buf: &Buffer, evnt: Event) -> Result<Event> {
        todo!()
    }

    pub fn to_span_line(
        &self,
        buf: &Buffer,
        scheme: &ColorScheme,
        from: usize,
        to: usize,
    ) -> Result<Spanline> {
        let mut atmt = self.atmt.clone();
        syntax::highlight(buf, scheme, &self.tree, &mut atmt, from, to)
    }
}
