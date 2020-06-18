use tree_sitter as ts;

use crate::{
    buffer::Buffer,
    color_scheme::ColorScheme,
    event::Event,
    window::{Page, Spanline},
    Result,
};

#[derive(Default, Clone)]
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

    fn to_span_line(&self, _: &Buffer, _: &ColorScheme, _: usize, _: usize) -> Option<Spanline> {
        None
    }
}
