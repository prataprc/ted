use std::{fmt, result};

use crate::{
    buffer::{self, Buffer},
    event::Event,
    state::{Context, State},
    view,
    window::{Coord, Cursor},
    Result,
};

#[derive(Clone, Default)]
pub struct WindowLine {
    name: String,
    coord: Coord,
    cursor: Cursor,
    obc_xy: buffer::Cursor,
    buffer: Option<Buffer>,
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
    pub fn to_cursor(&self) -> Cursor {
        self.coord.to_top_left() + self.cursor
    }

    pub fn on_event(&mut self, c: &mut Context, evnt: Event) -> Result<Event> {
        let buffer = c.buffer.take();

        let (line_buffer, evnt) = match self.buffer.take() {
            Some(buffer) => {
                c.buffer = Some(buffer);
                let evnt = Buffer::on_event(c, evnt)?;
                (c.buffer.take(), evnt)
            }
            None => (None, evnt),
        };
        self.buffer = line_buffer;

        c.buffer = buffer;
        Ok(evnt)
    }

    pub fn on_refresh(&mut self, c: &mut Context) -> Result<()> {
        let state: &State = c.as_ref();
        self.cursor = match self.buffer.as_ref() {
            Some(buffer) => {
                let v = view::NoWrap::new(self.coord, self.cursor, self.obc_xy);
                v.render(state, buffer)?
            }
            None => self.cursor,
        };

        Ok(())
    }
}
