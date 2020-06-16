use crate::{buffer::Buffer, event::Event, Result};

pub trait FileType {
    fn to_language(&self) -> Option<ts::Language>;

    fn to_file_type_name(&self) -> String;

    fn on_event(&mut self, buf: &mut Buffer, evnt: Event) -> Result<Event>;
}

pub fn new_parser(cont: &str, lang: ts::Language) -> Result<(ts::Parser, ts::Tree)> {
    let mut parser = {
        let mut p = ts::Parser::new();
        err_at!(FailParse, p.set_language(lang))?;
        Some(p)
    };
    let tree = parser.parse(content, None);
    Ok((parser, tree))
}
