use crate::{event::Event, ftype_text, window::Context, Result};

#[derive(Clone)]
pub struct FType {
    p: FT,
    fallback: FT,
}

impl Default for FType {
    fn default() -> FType {
        FType {
            p: Default::default(),
            fallback: Default::default(),
        }
    }
}

impl FType {
    pub fn new(p: FT, fallback: FT) -> FType {
        FType { p, fallback }
    }

    pub fn on_event(&mut self, c: &mut Context, evnt: Event) -> Result<Event> {
        match self.p.on_event(c, evnt)? {
            Event::Noop => Ok(Event::Noop),
            evnt => self.fallback.on_event(c, evnt),
        }
    }

    pub fn to_type_name(&self) -> String {
        self.p.to_type_name()
    }
}

#[derive(Clone)]
pub enum FT {
    Text(ftype_text::Text),
}

impl Default for FT {
    fn default() -> FT {
        FT::Text(Default::default())
    }
}

impl FT {
    fn on_event(&mut self, c: &mut Context, evnt: Event) -> Result<Event> {
        match self {
            FT::Text(t) => t.on_event(c, evnt),
        }
    }

    fn to_type_name(&self) -> String {
        match self {
            FT::Text(t) => t.to_type_name(),
        }
    }
}
