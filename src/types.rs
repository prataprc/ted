use crate::window::Context;

#[derive(Clone)]
pub struct Type {
    p: Inner,
    fallback: Inner,
}

impl Default for Type {
    fn default() -> Type {
        Type {
            p: Inner::Text(Default::default()),
            fallback: Inner::Text(Default::default()),
        }
    }
}

impl Type {
    //
}

#[derive(Clone)]
enum Inner {
    Text(Text),
}

impl Inner {
    //
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
