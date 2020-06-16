use crate::{buffer::Buffer, event::Event, ftype::FileType, Result};

#[derive(Default)]
pub struct PlainText;

impl FileType for PlainText {
    pub fn to_file_type_name(&self) -> String {
        "txt-plain"
    }

    pub fn to_language(&self) -> Option<ts::Language> {
        None
    }

    pub fn on_event(&mut self, _: &mut Buffer, evnt: Event) -> Result<Event> {
        Ok(evnt)
    }
}
