#[allow(unused_imports)]
use log::trace;

use crate::{buffer::Buffer, event::Event, Result};

#[derive(Clone, Default)]
pub struct KeyPrompt;

impl KeyPrompt {
    pub fn fold(&mut self, _buf: &Buffer, evnt: Event) -> Result<Event> {
        Ok(evnt)
    }
}
