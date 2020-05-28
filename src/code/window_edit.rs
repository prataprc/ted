use log::trace;

use std::{fmt, result};

use crate::{
    buffer::{self, Buffer},
    code::{config::Config, ftype::FType, App},
    event::Event,
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
        write!(
            f,
            "WindowEdit<{:?} {}@{} {}>",
            self.buffer_id, self.cursor, self.coord, self.obc_xy,
        )
    }
}

impl WindowEdit {
    #[inline]
    pub fn new(coord: Coord, buf: &Buffer, config: &Config) -> WindowEdit {
        use crate::code::view::{NoWrap, Wrap};

        let cursor = if config.wrap {
            Wrap::initial_cursor(config.line_number)
        } else {
            NoWrap::initial_cursor(config.line_number)
        };
        let we = WindowEdit {
            coord,
            cursor,
            obc_xy: (0, 0).into(),
            buffer_id: buf.to_id(),
            ftype: Default::default(),
        };

        trace!("{}", we);
        we
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

    #[inline]
    pub fn set_buffer(&mut self, buf: &Buffer) {
        self.buffer_id = buf.to_id();
    }

    pub fn on_event(&mut self, app: &mut App, evnt: Event) -> Result<Event> {
        match app.take_buffer(&self.buffer_id) {
            Some(mut buf) => {
                let evnt = match self.ftype.on_event(app, &mut buf, evnt)? {
                    Event::Noop => Event::Noop,
                    evnt => buf.on_event(evnt)?,
                };
                app.add_buffer(buf);
                Ok(evnt)
            }
            None => Ok(evnt),
        }
    }

    pub fn on_refresh(&mut self, app: &mut App) -> Result<()> {
        use crate::code::view::{NoWrap, Wrap};

        self.cursor = if app.as_ref().wrap {
            let v = {
                let (coord, cursor) = (self.coord, self.cursor);
                let mut v = Wrap::new("edit", coord, cursor, self.obc_xy);
                v.set_scroll_off(app.as_ref().scroll_off);
                v.set_line_number(app.as_ref().line_number);
                v
            };
            let buf = app.as_buffer(&self.buffer_id);
            v.render(buf, app.as_color_scheme())?
        } else {
            let v = {
                let (coord, cursor) = (self.coord, self.cursor);
                let mut v = NoWrap::new("edit", coord, cursor, self.obc_xy);
                v.set_scroll_off(app.as_ref().scroll_off);
                v.set_line_number(app.as_ref().line_number);
                v
            };
            let buf = app.as_buffer(&self.buffer_id);
            v.render(buf, app.as_color_scheme())?
        };

        Ok(())
    }
}
