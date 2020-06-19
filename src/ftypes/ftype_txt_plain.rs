use tree_sitter as ts;

use crate::{
    buffer::Buffer, colors::ColorScheme, event::Event, term::Spanline, window::Page, Result,
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
