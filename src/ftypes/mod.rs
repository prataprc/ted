use tree_sitter as ts;

use crate::{buffer::Buffer, event::Event, Error, Result};

mod ftype_txt_en;
mod ftype_txt_plain;

pub use ftype_txt_en::TextEn;
pub use ftype_txt_plain::PlainText;

pub trait FileType {
    fn to_language(&self) -> Option<ts::Language>;

    fn to_file_type_name(&self) -> String;

    fn on_event(&mut self, buf: &mut Buffer, evnt: Event) -> Result<Event>;
}

pub fn new_parser(content: &str, lang: ts::Language) -> Result<(ts::Parser, ts::Tree)> {
    let mut parser = {
        let mut parser = ts::Parser::new();
        err_at!(FailParse, parser.set_language(lang))?;
        parser
    };
    let tree = parser.parse(content, None).unwrap();
    Ok((parser, tree))
}
