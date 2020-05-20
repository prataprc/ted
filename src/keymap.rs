use crate::{
    //
    buffer::Buffer,
    event::Event,
    keymap_code::KeyCode,
    state::State,
    Result,
};

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
    pub fn fold(&mut self, buf: &mut Buffer, s: &mut State, evnt: Event) -> Result<Event> {
        match self {
            Keymap::Code(km) => km.fold(buf, s, evnt),
        }
    }
}
