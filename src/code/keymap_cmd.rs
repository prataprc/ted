#[allow(unused_imports)]
use log::trace;

use crate::{buffer::Buffer, code, event::Event, Result};

#[derive(Clone, Default)]
pub struct KeyCmd;

impl KeyCmd {
    pub fn fold(
        //
        &mut self,
        _: &code::Code,
        _: &Buffer,
        evnt: Event,
    ) -> Result<Event> {
        Ok(evnt)
    }

    pub fn to_event_prefix(&self) -> Event {
        Event::Noop
    }
}
