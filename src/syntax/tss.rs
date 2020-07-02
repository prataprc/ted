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
    fn tree_sitter_tss() -> ts::Language;
}

pub struct Tss {
    parser: ts::Parser,
    tree: ts::Tree,
    atmt: Automata,
}

impl Tss {
    pub fn new(buf: &Buffer, scheme: &ColorScheme) -> Result<Tss> {
        let lang = unsafe { tree_sitter_tss() };
        let (parser, tree) = syntax::new_parser(&buf.to_string(), lang)?;
        let atmt = Automata::from_str("tss", tss::TSS, scheme)?;

        debug!("{}", atmt);

        Ok(Tss { parser, tree, atmt })
    }
}

impl Tss {
    #[inline]
    pub fn to_language(&self) -> Option<ts::Language> {
        Some(unsafe { tree_sitter_tss() })
    }

    pub fn on_edit(&mut self, buf: &Buffer, evnt: Event) -> Result<Event> {
        match buf.to_mode() {
            "insert" => self.on_i_event(buf, evnt),
            "normal" => self.on_n_event(buf, evnt),
            _ => err_at!(Fatal, msg: format!("unreachable")),
        }
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

impl Tss {
    fn on_n_event(&mut self, _: &Buffer, evnt: Event) -> Result<Event> {
        use crate::event::Code;

        Ok(match evnt {
            Event::Noop => Event::Noop,
            Event::Code(Code::StatusCursor) => todo!(),
            evnt => evnt,
        })
    }

    fn on_i_event(&mut self, _: &Buffer, evnt: Event) -> Result<Event> {
        Ok(evnt)
    }
}
