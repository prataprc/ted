use std::{fmt, result};

use crate::{
    buffer::{self, Buffer},
    code::App,
    event::Event,
    window::{Coord, Cursor},
    Result,
};

#[derive(Clone)]
pub struct WindowLine {
    name: String,
    coord: Coord,
    cursor: Cursor,
    obc_xy: buffer::Cursor,
    buffer: Buffer,
}

impl Default for WindowLine {
    fn default() -> WindowLine {
        WindowLine {
            name: Default::default(),
            coord: Default::default(),
            cursor: Default::default(),
            obc_xy: Default::default(),
            buffer: Buffer::empty(),
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
            buffer: Buffer::empty(),
        }
    }
}

impl WindowLine {
    #[inline]
    pub fn to_cursor(&self) -> Cursor {
        self.coord.to_top_left() + self.cursor
    }

    pub fn on_event(&mut self, _app: &mut App, evnt: Event) -> Result<Event> {
        match evnt {
            Event::Esc => Ok(Event::Esc),
            evnt => self.buffer.on_event(evnt),
        }
    }

    pub fn on_refresh(&mut self, app: &mut App) -> Result<()> {
        use crate::code::view::NoWrap;

        self.cursor = {
            let v = {
                let mut v = NoWrap::new(self.coord, self.cursor, self.obc_xy);
                v.set_scroll_off(app.as_ref().scroll_off);
                v.set_line_number(app.as_ref().line_number);
                v
            };
            v.render(&self.buffer)?
        };
        Ok(())
    }
}
