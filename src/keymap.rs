use crate::{event::Event, keymap_ted::KeyTed, window::Context, Result};

#[derive(Clone)]
pub enum Keymap {
    Ted(KeyTed),
}

impl Default for Keymap {
    fn default() -> Keymap {
        Keymap::Ted(Default::default())
    }
}

impl Keymap {
    pub fn fold(
        //
        &mut self,
        c: &mut Context,
        evnt: Event,
    ) -> Result<(Event, Event)> {
        match self {
            Keymap::Ted(km) => km.fold(c, evnt),
        }
    }
}
