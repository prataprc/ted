use tree_sitter as ts;

use crate::{
    event::Event,
    state::Context,
    window::{Span, Window},
    Error, Result,
};

extern "C" {
    fn tree_sitter_txt_en() -> ts::Language;
}

pub struct Text {
    parser: ts::Parser,
    tree: Option<ts::Tree>,
}

impl Default for Text {
    fn default() -> Text {
        let parser = {
            let mut p = ts::Parser::new();
            let language = unsafe { tree_sitter_txt_en() };
            p.set_language(language).unwrap();
            p
        };

        Text { parser, tree: None }
    }
}

impl Clone for Text {
    fn clone(&self) -> Text {
        let parser = {
            let mut p = ts::Parser::new();
            let language = unsafe { tree_sitter_txt_en() };
            p.set_language(language).unwrap();
            p
        };

        Text { parser, tree: None }
    }
}

impl Text {
    pub fn to_type_name(&self) -> String {
        "txt".to_string()
    }

    pub fn on_event(&mut self, c: &mut Context, evnt: Event) -> Result<Event> {
        match c.as_buffer().to_mode() {
            "insert" => self.on_i_event(c, evnt),
            "normal" => self.on_n_event(c, evnt),
            _ => err_at!(Fatal, msg: format!("unreachable")),
        }
    }
}

impl Text {
    fn on_n_event(&mut self, c: &mut Context, evnt: Event) -> Result<Event> {
        use crate::event::{Event::Td, Ted};

        self.tree = match self.tree.take() {
            tree @ Some(_) => tree,
            None => {
                let b = c.as_mut_buffer();
                self.parser.parse(&b.to_string(), None)
            }
        };

        let evnt = match evnt {
            Event::Noop => Event::Noop,
            Td(Ted::StatusCursor) => self.to_status_cursor(c, evnt)?,
            evnt => evnt,
        };

        Ok(evnt)
    }

    fn on_i_event(&mut self, _: &mut Context, evnt: Event) -> Result<Event> {
        Ok(evnt)
    }

    fn to_status_cursor(&mut self, c: &mut Context, _: Event) -> Result<Event> {
        use crate::window_code::Message;

        match &self.tree {
            None => Ok(Event::Noop),
            Some(tree) => {
                let (mut ws, mut ss, mut ls, mut ps) = (0, 0, 0, 0);
                let mut prev_kind: Option<&str> = None;
                let mut tc = tree.walk();
                for node in tree.root_node().children(&mut tc) {
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
                let span: Span = format!("{} {} {} {}", ws, ls, ss, ps).into();
                let w = match c.to_window() {
                    Window::Code(mut w) => {
                        w.post(c, Message::Status(span));
                        Window::Code(w)
                    }
                    w => w,
                };
                c.set_window(w);
                Ok(Event::Noop)
            }
        }
    }
}
