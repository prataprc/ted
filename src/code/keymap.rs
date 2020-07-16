use crate::{
    buffer::Buffer,
    code,
    code::{keymap_cmd::KeyCmd, keymap_edit::KeyEdit, keymap_less::KeyLess},
    event::Event,
    Error, Result,
};

trait Keymapper {
    fn fold(&mut self, _: &code::Code, buf: &Buffer, evnt: Event) -> Result<Event>;
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
        Keymap::Edit(Default::default())
    }

    pub fn new_cmd() -> Keymap {
        Keymap::Cmd(Default::default())
    }

    pub fn new_less() -> Keymap {
        Keymap::Less(Default::default())
    }
}

impl Keymap {
    pub fn fold(&mut self, app: &code::Code, buf: &Buffer, evnt: Event) -> Result<Event> {
        match self {
            Keymap::Edit(km) => km.fold(app, buf, evnt),
            Keymap::Cmd(km) => km.fold(app, buf, evnt),
            Keymap::Less(km) => km.fold(app, buf, evnt),
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
