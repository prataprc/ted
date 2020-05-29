use crate::{buffer::Buffer, code::keymap_edit::KeyEdit, event::Event, Result};

#[derive(Clone)]
pub enum Keymap {
    Edit(KeyEdit),
}

impl Default for Keymap {
    fn default() -> Keymap {
        Keymap::Edit(Default::default())
    }
}

impl Keymap {
    pub fn fold(&mut self, buf: &Buffer, evnt: Event) -> Result<Event> {
        match self {
            Keymap::Edit(k) => k.fold(buf, evnt),
        }
    }
}
