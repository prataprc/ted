use tree_sitter as ts;

use crate::{
    buffer::Buffer,
    color_scheme::ColorScheme,
    event::Event,
    ftypes,
    syntax::{Page, Syntax},
    tss::Automata,
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
    fn new(content: &str, scheme: &ColorScheme) -> Result<Toml> {
        let lang = unsafe { tree_sitter_toml() };
        let (parser, tree) = ftypes::new_parser(content, lang)?;
        let atmt = {
            let tss = include_str!("../../ts/toml.tss");
            Automata::from_str(tss, scheme)?
        };

        Ok(Toml { parser, tree, atmt })
    }

    fn set_style(&mut self, scheme: &ColorScheme) -> &mut Self {
        self
    }
}

impl Page for Toml {
    fn to_language(&self) -> Option<ts::Language> {
        Some(unsafe { tree_sitter_toml() })
    }

    fn to_name(&self) -> String {
        "toml".to_string()
    }

    fn on_event(&mut self, buf: &mut Buffer, evnt: Event) -> Result<Event> {
        match buf.to_mode() {
            "insert" => self.on_i_event(buf, evnt),
            "normal" => self.on_n_event(buf, evnt),
            _ => err_at!(Fatal, msg: format!("unreachable")),
        }
    }

    fn to_syntax<'a>(&'a self, buf: &'a Buffer, scheme: &'a ColorScheme) -> Result<Option<Syntax>> {
        Ok(Some(Syntax::new(
            buf,
            &self.tree,
            self.atmt.clone(),
            scheme,
        )))
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
