use std::{fmt, result};

use crate::{
    buffer::{self, Buffer},
    event::{Event, Ted},
    state::State,
    view,
    window::{Coord, Cursor},
    Result,
};

#[derive(Clone, Default)]
pub struct WindowEdit {
    coord: Coord,
    cursor: Cursor,
    obc_xy: buffer::Cursor,
    buffer_id: String,
}

impl fmt::Display for WindowEdit {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "WindowEdit<{}>", self.coord)
    }
}

impl WindowEdit {
    #[inline]
    pub fn new(coord: Coord) -> WindowEdit {
        WindowEdit {
            coord,
            cursor: cursor!(0, 0),
            obc_xy: (0, 0).into(),
            buffer_id: Default::default(),
        }
    }
}

impl WindowEdit {
    #[inline]
    pub fn as_buffer<'a>(&self, s: &'a State) -> &'a Buffer {
        s.as_buffer(&self.buffer_id)
    }

    #[inline]
    pub fn as_mut_buffer<'a>(&self, s: &'a mut State) -> &'a mut Buffer {
        s.as_mut_buffer(&self.buffer_id)
    }

    #[inline]
    pub fn to_buffer_id(&self) -> String {
        self.buffer_id.clone()
    }

    #[inline]
    pub fn to_cursor(&self) -> Cursor {
        self.coord.to_top_left() + self.cursor
    }

    pub fn on_event(&mut self, s: &mut State, evnt: Event) -> Result<Event> {
        match evnt {
            Event::Td(Ted::NewBuffer) => {
                let buffer_id = {
                    let buffer = Buffer::empty();
                    let buffer_id = buffer.to_id();
                    s.add_buffer(buffer);
                    buffer_id
                };
                self.buffer_id = buffer_id;
                Ok(Event::Noop)
            }
            mut evnt => match s.take_buffer(&self.buffer_id) {
                Some(buffer) => {
                    let evnt = buffer.on_event(s, evnt)?;
                    s.add_buffer(buffer);
                    Ok(evnt)
                }
                None => Ok(evnt),
            },
        }
    }

    pub fn on_refresh(&mut self, s: &mut State) -> Result<()> {
        self.cursor = if s.as_ref().wrap {
            let v = view::Wrap::new(self.coord, self.cursor, self.obc_xy);
            let buf = s.as_buffer(&self.buffer_id);
            v.render(&s, buf)?
        } else {
            let v = view::NoWrap::new(self.coord, self.cursor, self.obc_xy);
            let buf = s.as_buffer(&self.buffer_id);
            v.render(&s, buf)?
        };

        Ok(())
    }
}
