#[allow(unused_imports)]
use log::trace;

use crate::{buffer::Buffer, code, event::Event, Error, Result};

#[derive(Clone, Default)]
pub struct KeyLess;

impl KeyLess {
    pub fn fold(&mut self, _: &code::Code, buf: &Buffer, evnt: Event) -> Result<Event> {
        match buf.to_mode() {
            "normal" => self.normal_fold(evnt),
            "insert" => unreachable!(),
            _ => err_at!(Fatal, msg: format!("unreachable")),
        }
    }

    pub fn to_event_prefix(&self) -> Event {
        Event::Noop
    }
}

impl KeyLess {
    fn normal_fold(&mut self, evnt: Event) -> Result<Event> {
        Ok(evnt)
    }
}
