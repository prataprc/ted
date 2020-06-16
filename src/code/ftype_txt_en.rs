use tree_sitter as ts;

use crate::{
    buffer::Buffer,
    event::Event,
    ftype::{self, FileType},
    window::{Notify, Span},
    Error, Result,
};

extern "C" {
    fn tree_sitter_txt_en() -> ts::Language;
}

pub struct TextEn {
    parser: ts::Parser,
    tree: ts::Tree,
}

impl Default for TextEn {
    fn default() -> TextEn {
        TextEn::new("").unwrap()
    }
}

impl TextEn {
    fn new(content: &str) -> Result<TextEn> {
        let lang = unsafe { tree_sitter_txt_en() };
        let (parser, tree) = ftype::new_parser(content, lang)?;
        Ok(TextEn { parser, tree })
    }
}

impl FileType for TextEn {
    fn to_file_type_name(&self) -> String {
        "txt-en".to_string()
    }

    fn to_language(&self) -> Option<ts::Language> {
        Some(unsafe { tree_sitter_txt_en() })
    }

    fn on_event(&mut self, buf: &mut Buffer, evnt: Event) -> Result<Event> {
        match buf.to_mode() {
            "insert" => self.on_i_event(buf, evnt),
            "normal" => self.on_n_event(buf, evnt),
            _ => err_at!(Fatal, msg: format!("unreachable")),
        }
    }
}

impl TextEn {
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
        let (mut ws, mut ss, mut ls, mut ps) = (0, 0, 0, 0);
        let mut prev_kind: Option<&str> = None;
        let mut tc = self.tree.walk();
        for node in self.tree.root_node().children(&mut tc) {
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
        Ok(Event::Notify(Notify::Status(vec![span])))
    }
}
