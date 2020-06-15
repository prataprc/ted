use tree_sitter as ts;

use crate::{
    buffer::Buffer,
    code::App,
    event::Event,
    window::{Notify, Span},
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
        let parser = new_parser().unwrap();
        Text { parser, tree: None }
    }
}

impl Clone for Text {
    fn clone(&self) -> Text {
        let parser = new_parser().unwrap();
        Text { parser, tree: None }
    }
}

impl Text {
    pub fn to_type_name(&self) -> String {
        "txt".to_string()
    }

    pub fn on_event(&mut self, app: &mut App, buf: &mut Buffer, evnt: Event) -> Result<Event> {
        match buf.to_mode() {
            "insert" => self.on_i_event(app, buf, evnt),
            "normal" => self.on_n_event(app, buf, evnt),
            _ => err_at!(Fatal, msg: format!("unreachable")),
        }
    }
}

impl Text {
    fn on_n_event(&mut self, app: &mut App, buf: &mut Buffer, evnt: Event) -> Result<Event> {
        use crate::event::Code::StatusCursor;

        self.tree = match self.tree.take() {
            tree @ Some(_) => tree,
            None => self.parser.parse(&buf.to_string(), None),
        };

        let evnt = match evnt {
            Event::Noop => Event::Noop,
            Event::Code(StatusCursor) => self.to_status_cursor(app, evnt)?,
            evnt => evnt,
        };

        Ok(evnt)
    }

    fn on_i_event(&mut self, _app: &mut App, _buf: &mut Buffer, evnt: Event) -> Result<Event> {
        Ok(evnt)
    }

    fn to_status_cursor(&mut self, app: &mut App, _: Event) -> Result<Event> {
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
                app.notify("code", Notify::Status(vec![span]))?;
                Ok(Event::Noop)
            }
        }
    }
}

fn new_parser() -> Result<ts::Parser> {
    let mut p = ts::Parser::new();
    let language = unsafe { tree_sitter_txt_en() };
    err_at!(FailParse, p.set_language(language))?;
    Ok(p)
}
