use crate::{
    //
    buffer::Buffer,
    code::keymap_cmd::KeyCmd,
    code::keymap_edit::KeyEdit,
    code::keymap_less::KeyLess,
    event::Event,
    Result,
};

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
    pub fn fold(&mut self, buf: &Buffer, evnt: Event) -> Result<Event> {
        match self {
            Keymap::Edit(km) => km.fold(buf, evnt),
            Keymap::Cmd(km) => km.fold(buf, evnt),
            Keymap::Less(km) => km.fold(buf, evnt),
            Keymap::None => unreachable!(),
        }
    }
}
