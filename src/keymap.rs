use crate::keymap_ted::Ted;
use crate::window::Context;

pub enum Keymap {
    Ted(Ted),
}

impl Default for Keymap {
    fn default() -> Keymap {
        Ted(Default::default())
    }
}

impl Keymap {
    fn fold(&mut self, c: &mut Context, evnt: Event) -> Event {
        match self {
            Ted(km) => km.fold(c),
        }
    }
}
