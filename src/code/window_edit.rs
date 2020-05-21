use std::{fmt, result};

use crate::{
    buffer::{self, Buffer},
    code::{ftype::FType, view, App},
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
    ftype: FType,
}

impl fmt::Display for WindowEdit {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "WindowEdit<{}>", self.coord)
    }
}

impl WindowEdit {
    #[inline]
    pub fn new(coord: Coord, buf: &Buffer) -> WindowEdit {
        WindowEdit {
            coord,
            cursor: cursor!(0, 0),
            obc_xy: (0, 0).into(),
            buffer_id: buf.to_id(),
            ftype: Default::default(),
        }
    }

    pub fn set_ftype(&mut self, ftype: FType) -> &mut Self {
        self.ftype = ftype;
        self
    }
}

impl WindowEdit {
    #[inline]
    pub fn to_buffer_id(&self) -> String {
        self.buffer_id.clone()
    }

    #[inline]
    pub fn to_file_type(&self) -> String {
        self.ftype.to_type_name()
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
            evnt => match app.take_buffer(&self.buffer_id) {
                Some(mut buf) => {
                    let evnt = match self.ftype.on_event(app, &mut buf, evnt)? {
                        Event::Noop => Event::Noop,
                        evnt => buf.on_event(evnt)?,
                    };
                    app.add_buffer(buf);
                    Ok(evnt)
                }
                None => Ok(evnt),
            },
        }
    }

    pub fn on_refresh(&mut self, app: &mut App) -> Result<()> {
        self.cursor = if app.as_ref().wrap {
            let v = view::Wrap::new(self.coord, self.cursor, self.obc_xy);
            let buf = app.as_buffer(&self.buffer_id);
            v.render(app, buf)?
        } else {
            let v = view::NoWrap::new(self.coord, self.cursor, self.obc_xy);
            let buf = app.as_buffer(&self.buffer_id);
            v.render(app, buf)?
        };

        Ok(())
    }
}
