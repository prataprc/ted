use crate::{buffer::Buffer, event::Event, Result};

#[derive(Clone)]
pub struct Plugin {
    p: Inner,
    fallback: Inner,
}

impl Default for Plugin {
    fn default() -> Plugin {
        Plugin {
            p: Inner::Text(Default::default()),
            fallback: Inner::Text(Default::default()),
        }
    }
}

impl Plugin {
    fn on_event(&mut self, buf: &mut Buffer, evnt: Event) -> Result<Event> {
        match self.p.on_event(buf, evnt)? {
            Event::Noop => Ok(Event::Noop),
            evnt => self.fallback.on_event(buf, evnt),
        }
    }
}

#[derive(Clone)]
enum Inner {
    Text(Text),
}

impl Inner {
    fn on_event(&mut self, buf: &mut Buffer, evnt: Event) -> Result<Event> {
        match self {
            Inner::Text(p) => p.on_event(buf, evnt),
        }
    }
}

#[derive(Clone)]
struct Text;

impl Default for Text {
    fn default() -> Text {
        Text
    }
}

impl Text {
    fn on_event(&mut self, _buf: &mut Buffer, _evnt: Event) -> Result<Event> {
        todo!()
    }
}
