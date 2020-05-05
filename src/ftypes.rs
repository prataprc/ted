use crate::{event::Event, ftype_txt_en, window::Context, Result};

#[derive(Clone)]
pub enum FType {
    Text(ftype_txt_en::Text),
}

impl Default for FType {
    fn default() -> FType {
        FType::Text(Default::default())
    }
}

impl FType {
    pub fn on_event(&mut self, c: &mut Context, evnt: Event) -> Result<Event> {
        match self {
            FType::Text(t) => t.on_event(c, evnt),
        }
    }

    pub fn to_type_name(&self) -> String {
        match self {
            FType::Text(t) => t.to_type_name(),
        }
    }
}
