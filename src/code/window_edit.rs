#[allow(unused_imports)]
use log::{debug, trace};

use std::{fmt, result};

use crate::{
    app::Application,
    buffer::{self, Buffer},
    code::{config::Config, keymap::Keymap, Code},
    colors::ColorScheme,
    event::Event,
    syntax,
    term::Spanline,
    window::{Coord, Cursor, Render, WinBuffer, Window},
    Error, Result,
};

pub struct WindowEdit {
    coord: Coord,
    cursor: Cursor,
    obc_xy: buffer::Cursor,
    buffer_id: String,
    syn: syntax::Type,
    scheme: ColorScheme,
    keymap: Keymap,
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
    pub fn new(coord: Coord, config: Config, buf: &Buffer, scheme: &ColorScheme) -> WindowEdit {
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
            syn: syntax::detect(buf, scheme).unwrap(),
            scheme: scheme.clone(),
            keymap: Keymap::new_edit(),
        };
        debug!("{}", we);
        we
    }
}

impl WindowEdit {
    #[inline]
    pub fn to_buffer_id(&self) -> String {
        self.buffer_id.clone()
    }

    #[inline]
    pub fn to_text_type(&self) -> String {
        self.syn.to_name().to_string()
    }
}

impl Window for WindowEdit {
    type App = Code;

    #[inline]
    fn to_cursor(&self) -> Cursor {
        self.coord.to_top_left() + self.cursor
    }

    fn on_event(&mut self, app: &mut Code, evnt: Event) -> Result<Event> {
        use crate::pubsub::Notify;

        let evnt = match app.take_buffer(&self.buffer_id) {
            Some(mut buf) => {
                let mut evnt = self.keymap.fold(&mut buf, evnt)?;
                evnt = buf.on_event(evnt)?;
                // after handling the event for buffer, handle for its file-type.
                evnt = self.syn.on_edit(&buf, evnt)?;
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

        let err = Error::Invalid(format!("buffer {}", self.buffer_id));
        self.cursor = if app.as_ref().wrap {
            let v = {
                let (coord, cursor) = (self.coord, self.cursor);
                let mut v = Wrap::new("edit", coord, cursor, self.obc_xy);
                v.set_scroll_off(app.as_ref().scroll_off);
                v.set_line_number(app.as_ref().line_number);
                v
            };
            let buf = err_at!(app.as_buffer(&self.buffer_id).ok_or(err))?;
            v.render(buf, self, &self.scheme)?
        } else {
            let v = {
                let (coord, cursor) = (self.coord, self.cursor);
                let mut v = NoWrap::new("edit", coord, cursor, self.obc_xy);
                v.set_scroll_off(app.as_ref().scroll_off);
                v.set_line_number(app.as_ref().line_number);
                v
            };
            let buf = err_at!(app.as_buffer(&self.buffer_id).ok_or(err))?;
            v.render(buf, self, &self.scheme)?
        };
        self.obc_xy = {
            let err = Error::Invalid(format!("buffer {}", self.buffer_id));
            err_at!(app.as_buffer(&self.buffer_id).ok_or(err))?.to_xy_cursor()
        };

        Ok(())
    }
}

impl Render for WindowEdit {
    fn to_span_line(&self, buf: &Buffer, a: usize, z: usize) -> Result<Spanline> {
        self.syn.to_span_line(buf, &self.scheme, a, z)
    }
}
