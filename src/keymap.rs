use crate::{event::Event, keymap_code::KeyCode, state::Context, Result};

#[derive(Clone)]
pub enum Keymap {
    Code(KeyCode),
}

impl Default for Keymap {
    fn default() -> Keymap {
        Keymap::Code(Default::default())
    }
}

impl Keymap {
    pub fn fold(&mut self, c: &mut Context, evnt: Event) -> Result<Event> {
        match self {
            Keymap::Code(km) => km.fold(c, evnt),
        }
    }
}
