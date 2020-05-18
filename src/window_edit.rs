use std::{fmt, result};

use crate::{
    buffer::{self, Buffer},
    event::{Event, Ted},
    state::Context,
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
    pub fn to_buffer_id(&self) -> String {
        self.buffer_id.clone()
    }

    #[inline]
    pub fn to_cursor(&self) -> Cursor {
        self.coord.to_top_left() + self.cursor
    }

    pub fn on_event(&mut self, c: &mut Context, evnt: Event) -> Result<Event> {
        match evnt {
            Event::Td(Ted::UseBuffer { buffer_id }) => {
                self.buffer_id = buffer_id;
                Ok(Event::Noop)
            }
            mut evnt => match c.take_buffer(&self.buffer_id) {
                Some(buffer) => {
                    c.buffer = Some(buffer);
                    evnt = Buffer::on_event(c, evnt)?;
                    let buffer = c.buffer.take().unwrap();
                    c.add_buffer(buffer);
                    Ok(evnt)
                }
                None => Ok(evnt),
            },
        }
    }

    pub fn on_refresh(&mut self, c: &mut Context) -> Result<()> {
        let state = c.as_state();
        self.cursor = if state.as_ref().wrap {
            let v = view::Wrap::new(self.coord, self.cursor, self.obc_xy);
            let buf = state.as_buffer(&self.buffer_id);
            v.render(&state, buf)?
        } else {
            let v = view::NoWrap::new(self.coord, self.cursor, self.obc_xy);
            let buf = state.as_buffer(&self.buffer_id);
            v.render(&state, buf)?
        };

        Ok(())
    }
}
