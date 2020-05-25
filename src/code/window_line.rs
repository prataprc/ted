use std::{fmt, result};

use crate::{
    buffer::{self, Buffer},
    code::App,
    event::Event,
    location::Location,
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

impl fmt::Display for WindowLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "WindowLine<{},{}>", self.name, self.coord)
    }
}

impl WindowLine {
    #[inline]
    pub fn new(name: &str, coord: Coord) -> WindowLine {
        use crate::code::view::NoWrap;

        let line_number = false;
        let buf = {
            let loc = Location::new_ted(name);
            Buffer::from_reader(vec![].as_slice(), loc).unwrap()
        };
        WindowLine {
            name: name.to_string(),
            coord,
            cursor: NoWrap::initial_cursor(line_number),
            obc_xy: (0, 0).into(),
            buffer: buf,
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

    pub fn on_refresh(&mut self, _app: &mut App) -> Result<()> {
        use crate::code::view::NoWrap;

        self.cursor = {
            let v = {
                let mut v = NoWrap::new(self.coord, self.cursor, self.obc_xy);
                v.set_scroll_off(0).set_line_number(false);
                v
            };
            v.render(&self.buffer)?
        };
        Ok(())
    }
}
