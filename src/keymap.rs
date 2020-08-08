use crate::{buffer::Buffer, event::Event, Error, Result};

pub use crate::keymap_cmd::KeyCmd;
pub use crate::keymap_edit::KeyEdit;
pub use crate::keymap_less::KeyLess;

trait Keymapper {
    fn fold(&mut self, buf: &Buffer, evnt: Event) -> Result<Event>;
}

#[derive(Clone)]
pub enum Keymap {
    Edit(KeyEdit),
    Cmd(KeyCmd),
    Less(KeyLess),
    None,
}

impl Default for Keymap {
    fn default() -> Keymap {
        Keymap::None
    }
}

impl Keymap {
    pub fn new_edit() -> Keymap {
        Keymap::Edit(KeyEdit::default())
    }

    pub fn new_cmd() -> Keymap {
        Keymap::Cmd(KeyCmd::default())
    }

    pub fn new_less() -> Keymap {
        Keymap::Less(KeyLess::default())
    }
}

impl Keymap {
    pub fn fold(&mut self, buf: &Buffer, evnt: Event) -> Result<Event> {
        match self {
            Keymap::Edit(km) => km.fold(buf, evnt),
            Keymap::Cmd(km) => km.fold(buf, evnt),
            Keymap::Less(km) => km.fold(buf, evnt),
            Keymap::None => err_at!(Fatal, msg: format!("keymap is none")),
        }
    }

    pub fn to_event_prefix(&self) -> Event {
        match self {
            Keymap::Edit(km) => km.to_event_prefix(),
            Keymap::Cmd(km) => km.to_event_prefix(),
            Keymap::Less(km) => km.to_event_prefix(),
            Keymap::None => Event::Noop,
        }
    }
}
