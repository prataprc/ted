use log::trace;

use std::{fmt, result};

use crate::{
    app::Application,
    buffer::{self, Buffer},
    code::{config::Config, keymap::Keymap, Code},
    colors::{ColorScheme, Highlight},
    event::Event,
    ftypes::PlainText,
    term::Spanline,
    window::{Coord, Cursor, Page, Render, WinBuffer, Window},
    Result,
};

pub struct WindowEdit {
    coord: Coord,
    cursor: Cursor,
    obc_xy: buffer::Cursor,
    buffer_id: String,
    page: Box<dyn Page>,
    scheme: ColorScheme,
    keymap: Keymap,
}

impl Default for WindowEdit {
    fn default() -> WindowEdit {
        WindowEdit {
            coord: Default::default(),
            cursor: Default::default(),
            obc_xy: (0, 0).into(),
            buffer_id: Default::default(),
            page: Box::new(PlainText),
            scheme: Default::default(),
            keymap: Default::default(),
        }
    }
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
            page: Box::new(PlainText),
            scheme: Default::default(),
            keymap: Keymap::new_edit(),
        };

        trace!("{}", we);
        we
    }

    pub fn set_page(&mut self, page: Box<dyn Page>) -> &mut Self {
        self.page = page;
        self
    }

    pub fn set_color_scheme(&mut self, scheme: ColorScheme) -> &mut Self {
        self.scheme = scheme;
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
        // self.ftype.to_type_name()
        todo!()
    }
}

impl Window for WindowEdit {
    type App = Code;

    #[inline]
    fn to_cursor(&self) -> Cursor {
        self.coord.to_top_left() + self.cursor
    }

    fn on_event(&mut self, app: &mut Code, evnt: Event) -> Result<Event> {
        use crate::window::Notify;

        let evnt = match app.take_buffer(&self.buffer_id) {
            Some(mut buf) => {
                let mut evnt = self.keymap.fold(&mut buf, evnt)?;
                evnt = buf.on_event(evnt)?;
                // after handling the event for buffer, handle for its file-type.
                evnt = self.page.on_event(&mut buf, evnt)?;
                app.add_buffer(buf);
                Ok(evnt)
            }
            None => Ok(evnt),
        }?;

        match evnt {
            Event::Notify(msg @ Notify::Status(_)) => {
                app.notify("code", msg)?;
                Ok(Event::Noop)
            }
            evnt => Ok(evnt),
        }
    }

    fn on_refresh(&mut self, app: &mut Code) -> Result<()> {
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
            v.render(buf, self, app.as_color_scheme())?
        } else {
            let v = {
                let (coord, cursor) = (self.coord, self.cursor);
                let mut v = NoWrap::new("edit", coord, cursor, self.obc_xy);
                v.set_scroll_off(app.as_ref().scroll_off);
                v.set_line_number(app.as_ref().line_number);
                v
            };
            let buf = app.as_buffer(&self.buffer_id);
            v.render(buf, self, app.as_color_scheme())?
        };
        self.obc_xy = app.as_buffer(&self.buffer_id).to_xy_cursor();

        Ok(())
    }
}

impl Render for WindowEdit {
    fn to_span_line(&self, buf: &Buffer, a: usize, z: usize) -> Result<Spanline> {
        match self.page.to_span_line(buf, &self.scheme, a, z) {
            Some(spl) => Ok(spl),
            None => {
                let spl = buffer::to_span_line(buf, a, z)?;
                Ok(spl.using(self.scheme.to_style(Highlight::Canvas)))
            }
        }
    }
}
