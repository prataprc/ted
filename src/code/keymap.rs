use crate::{
    //
    buffer::Buffer,
    code::keymap_cmd::KeyCmd,
    code::keymap_edit::KeyEdit,
    code::keymap_less::KeyLess,
    event::Event,
    Result,
};

#[derive(Clone)]
pub enum Keymap {
    Edit(KeyEdit),
    Cmd(KeyCmd),
    Less(KeyLess),
}

impl Default for Keymap {
    fn default() -> Keymap {
        Keymap::Edit(Default::default())
    }
}

impl Keymap {
    pub fn new_edit() -> Keymap {
        Default::default()
    }

    pub fn new_cmd() -> Keymap {
        Keymap::Cmd(Default::default())
    }

    pub fn new_less() -> Keymap {
        Keymap::Less(Default::default())
    }
}

impl Keymap {
    pub fn fold(&mut self, buf: &Buffer, evnt: Event) -> Result<Event> {
        match self {
            Keymap::Edit(km) => km.fold(buf, evnt),
            Keymap::Cmd(km) => km.fold(buf, evnt),
            Keymap::Less(km) => km.fold(buf, evnt),
        }
    }
}
