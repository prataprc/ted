use std::{fmt, result};

use crate::{
    buffer::{self, Buffer},
    code::view,
    event::{Event, Ted},
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
    pub fn to_buffer_id(&self) -> String {
        self.buffer_id.clone()
    }

    #[inline]
    pub fn to_cursor(&self) -> Cursor {
        self.coord.to_top_left() + self.cursor
    }

    pub fn on_event(&mut self, app: &mut App, evnt: Event) -> Result<Event> {
        match evnt {
            Event::Td(Ted::NewBuffer) => {
                let buffer_id = {
                    let buffer = Buffer::empty();
                    let buffer_id = buffer.to_id();
                    app.add_buffer(buffer);
                    buffer_id
                };
                self.buffer_id = buffer_id;
                Ok(Event::Noop)
            }
            mut evnt => {
                let buf = app.as_mut_buffer(&self.buffer_id);
                buf.on_event(app, evnt)
            }
        }
    }

    pub fn on_refresh(&mut self, app: &mut App) -> Result<()> {
        self.cursor = if app.as_ref().wrap {
            let v = view::Wrap::new(self.coord, self.cursor, self.obc_xy);
            let buf = app.as_buffer(&self.buffer_id);
            v.render(&s, buf)?
        } else {
            let v = view::NoWrap::new(self.coord, self.cursor, self.obc_xy);
            let buf = app.as_buffer(&self.buffer_id);
            v.render(&s, buf)?
        };

        Ok(())
    }
}
