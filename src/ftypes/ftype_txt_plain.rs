use tree_sitter as ts;

use crate::{
    buffer::Buffer,
    color_scheme::ColorScheme,
    event::Event,
    syntax::{Page, Syntax},
    Result,
};

#[derive(Default)]
pub struct PlainText;

impl Page for PlainText {
    fn to_language(&self) -> Option<ts::Language> {
        None
    }

    fn to_name(&self) -> String {
        "txt-plain".to_string()
    }

    fn on_event(&mut self, _: &mut Buffer, evnt: Event) -> Result<Event> {
        Ok(evnt)
    }

    fn to_syntax<'a>(&'a self, buf: &'a Buffer, scheme: &'a ColorScheme) -> Result<Option<Syntax>> {
        Ok(None)
    }
}
