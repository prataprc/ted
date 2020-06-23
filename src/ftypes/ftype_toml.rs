use log::trace;
use tree_sitter as ts;

use crate::{
    buffer::Buffer,
    colors::ColorScheme,
    event::Event,
    ftypes, syntax,
    term::Spanline,
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
    pub fn new(buf: &Buffer, scheme: &ColorScheme) -> Result<Toml> {
        let lang = unsafe { tree_sitter_toml() };
        let (parser, tree) = ftypes::new_parser(&buf.to_string(), lang)?;
        let atmt = Automata::from_str("toml", tss::TOML, scheme)?;

        trace!("{}", atmt);

        Ok(Toml { parser, tree, atmt })
    }

    #[inline]
    pub fn to_name() -> String {
        "toml".to_string()
    }
}

impl Toml {
    #[inline]
    pub fn to_language(&self) -> Option<ts::Language> {
        Some(unsafe { tree_sitter_toml() })
    }

    pub fn on_event(&mut self, buf: &mut Buffer, evnt: Event) -> Result<Event> {
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

impl Toml {
    fn on_n_event(&mut self, _: &mut Buffer, evnt: Event) -> Result<Event> {
        use crate::event::Code;

        Ok(match evnt {
            Event::Noop => Event::Noop,
            Event::Code(Code::StatusCursor) => self.to_status_cursor()?,
            evnt => evnt,
        })
    }

    fn on_i_event(&mut self, _buf: &mut Buffer, evnt: Event) -> Result<Event> {
        Ok(evnt)
    }

    fn to_status_cursor(&mut self) -> Result<Event> {
        todo!()
        //let (mut ws, mut ss, mut ls, mut ps) = (0, 0, 0, 0);
        //let mut prev_kind: Option<&str> = None;
        //let mut tc = self.tree.walk();
        //for node in self.tree.root_node().children(&mut tc) {
        //    match (prev_kind, node.kind()) {
        //        (_, "word") | (_, "wword") => ws += 1,
        //        (_, "dot") => ss += 1,
        //        (Some("nl"), "nl") => {
        //            ls += 1;
        //            ps += 1;
        //        }
        //        (_, "nl") => ls += 1,
        //        _ => err_at!(Fatal, msg: format!("unreachable"))?,
        //    }
        //    prev_kind = Some(node.kind());
        //}
        //let span: Span = format!("{} {} {} {}", ws, ls, ss, ps).into();
        //Ok(Event::Notify(Notify::Status(vec![span])))
    }
}
