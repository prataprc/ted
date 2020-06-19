use tree_sitter as ts;

use crate::{
    buffer::Buffer,
    color_scheme::ColorScheme,
    event::Event,
    ftypes,
    term::{Span, Spanline},
    tss,
    window::{Notify, Page},
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
    fn new(content: &str) -> Result<Tss> {
        let lang = unsafe { tree_sitter_tss() };
        let (parser, tree) = ftypes::new_parser(content, lang)?;
        let atmt = Automata::from_str(tss::tss, scheme)?;
        Ok(Tss { parser, tree, atmt })
    }
}

impl Page for Tss {
    fn to_language(&self) -> Option<ts::Language> {
        Some(unsafe { tree_sitter_tss() })
    }

    fn to_name(&self) -> String {
        "tss".to_string()
    }

    fn on_event(&mut self, buf: &mut Buffer, evnt: Event) -> Result<Event> {
        match buf.to_mode() {
            "insert" => self.on_i_event(buf, evnt),
            "normal" => self.on_n_event(buf, evnt),
            _ => err_at!(Fatal, msg: format!("unreachable")),
        }
    }

    fn to_span_line(&self, _: &Buffer, _: &ColorScheme, _: usize, _: usize) -> Option<Spanline> {
        let mut atmt = self.atmt.clone();
        syntax::highlight(buf, scheme, &self.tree, &mut atmt, from, to)
    }
}

impl Tss {
    fn on_n_event(&mut self, _: &mut Buffer, evnt: Event) -> Result<Event> {
        use crate::event::Code;

        Ok(match evnt {
            Event::Noop => Event::Noop,
            Event::Code(Code::StatusCursor) => todo!(),
            evnt => evnt,
        })
    }

    fn on_i_event(&mut self, _buf: &mut Buffer, evnt: Event) -> Result<Event> {
        Ok(evnt)
    }
}
