#[allow(unused_imports)]
use log::trace;

use crate::{buffer::Buffer, event::Event, Result};

#[derive(Clone, Default)]
pub struct KeyLess;

impl KeyLess {
    pub fn fold(&mut self, _buf: &Buffer, evnt: Event) -> Result<Event> {
        Ok(evnt)
    }
}
