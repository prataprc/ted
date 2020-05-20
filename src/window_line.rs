use std::{fmt, result};

use crate::{
    buffer::{self, Buffer},
    event::Event,
    state::State,
    view,
    window::{Coord, Cursor},
    Result,
};

#[derive(Clone)]
pub struct WindowLine {
    name: String,
    coord: Coord,
    cursor: Cursor,
    obc_xy: buffer::Cursor,
    buffer: Option<Buffer>,
}

impl Default for WindowLine {
    fn default() -> WindowLine {
        WindowLine {
            name: Default::default(),
            coord: Default::default(),
            cursor: Default::default(),
            obc_xy: Default::default(),
            buffer: Some(Buffer::empty()),
        }
    }
}

impl fmt::Display for WindowLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "WindowLine<{},{}>", self.name, self.coord)
    }
}

impl WindowLine {
    #[inline]
    pub fn new(name: &str, coord: Coord) -> WindowLine {
        WindowLine {
            name: name.to_string(),
            coord,
            cursor: cursor!(0, 0),
            obc_xy: (0, 0).into(),
            buffer: Some(Buffer::empty()),
        }
    }
}

impl WindowLine {
    #[inline]
    pub fn as_buffer<'a>(&self, s: &'a State) -> &'a Buffer {
        todo!()
    }

    #[inline]
    pub fn as_mut_buffer<'a>(&self, s: &'a mut State) -> &'a mut Buffer {
        todo!()
    }

    #[inline]
    pub fn to_cursor(&self) -> Cursor {
        self.coord.to_top_left() + self.cursor
    }

    pub fn on_event(&mut self, s: &mut State, evnt: Event) -> Result<Event> {
        match evnt {
            Event::Esc => Ok(Event::Esc),
            evnt => match &mut self.buffer {
                Some(buffer) => buffer.on_event(s, evnt),
                None => Ok(evnt),
            },
        }
    }

    pub fn on_refresh(&mut self, s: &mut State) -> Result<()> {
        self.cursor = match self.buffer.as_ref() {
            Some(buffer) => {
                let v = view::NoWrap::new(self.coord, self.cursor, self.obc_xy);
                v.render(s, buffer)?
            }
            None => self.cursor,
        };

        Ok(())
    }
}
