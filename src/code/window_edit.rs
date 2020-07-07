#[allow(unused_imports)]
use log::{debug, trace};

use std::{fmt, result};

use crate::{
    app::Application,
    buffer::{self, Buffer},
    code::{self, keymap::Keymap},
    colors::ColorScheme,
    event::{self, Event},
    syntax::{self, Syntax},
    term::Spanline,
    window::{Coord, Cursor, Render, WinBuffer, Window},
    Error, Result,
};

pub struct WindowEdit {
    coord: Coord,
    cursor: Cursor,
    obc_xy: buffer::Cursor,
    curr_buf_id: String,
    altn_buf_id: Option<String>,
    syn: syntax::Type,
    scheme: ColorScheme,
    keymap: Keymap,
    // configuration.
    scroll_off: u16,
    line_number: bool,
}

impl fmt::Display for WindowEdit {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(
            f,
            "WindowEdit<{}@{} {}>",
            self.cursor, self.coord, self.obc_xy,
        )
    }
}

impl WindowEdit {
    #[inline]
    pub fn new(coord: Coord, buf: &Buffer, app: &code::Code) -> WindowEdit {
        use crate::view::{NoWrap, Wrap};

        let cursor = if app.config.wrap {
            Wrap::initial_cursor(app.config.line_number)
        } else {
            NoWrap::initial_cursor(app.config.line_number)
        };

        let scheme = app.to_color_scheme(None);
        let we = WindowEdit {
            coord,
            cursor,
            obc_xy: (0, 0).into(),
            curr_buf_id: buf.to_id(),
            altn_buf_id: None,
            syn: syntax::detect(buf, &scheme).unwrap(),
            scheme,
            keymap: Keymap::new_edit(),
            // configuration
            scroll_off: app.as_ref().scroll_off,
            line_number: app.as_ref().line_number,
        };
        debug!("{} {} {}", we, we.scroll_off, we.line_number);
        we
    }

    pub fn set_buffer(&mut self, buf: &Buffer) -> &mut Self {
        self.altn_buf_id = Some(self.curr_buf_id.clone());
        self.curr_buf_id = buf.to_id();
        self
    }

    pub fn flip_buffer(&mut self) -> &mut Self {
        match self.altn_buf_id.take() {
            Some(altn_buf_id) => {
                let curr_buf_id = self.curr_buf_id.clone();
                self.curr_buf_id = altn_buf_id;
                self.altn_buf_id = Some(curr_buf_id);
            }
            None => (),
        }
        self
    }
}

impl WindowEdit {
    #[inline]
    pub fn to_buffer_id(&self) -> String {
        self.curr_buf_id.clone()
    }

    #[inline]
    pub fn to_text_type(&self) -> String {
        self.syn.as_name().to_string()
    }
}

impl Window for WindowEdit {
    type App = code::Code;

    #[inline]
    fn to_name(&self) -> String {
        "window-edit".to_string()
    }

    #[inline]
    fn to_coord(&self) -> Coord {
        self.coord
    }

    #[inline]
    fn to_cursor(&self) -> Cursor {
        self.coord.to_top_left() + self.cursor
    }

    #[inline]
    fn config_line_number(&self) -> bool {
        self.line_number
    }

    #[inline]
    fn config_scroll_offset(&self) -> u16 {
        self.scroll_off
    }

    fn on_event(&mut self, app: &mut code::Code, evnt: Event) -> Result<Event> {
        use crate::pubsub::Notify;

        let evnt = match app.take_buffer(&self.curr_buf_id) {
            Some(mut buf) => {
                let mut evnt = self.keymap.fold(app, &mut buf, evnt)?;
                evnt = buf.on_event(evnt)?;
                // after handling the event for buffer, handle for its file-type.
                evnt = self.syn.on_edit(&buf, evnt)?;
                app.add_buffer(buf);
                Ok(evnt)
            }
            None => Ok(evnt),
        }?;

        match evnt {
            Event::Code(event::Code::StatusCursor) => {
                let msg = vec![self.syn.to_status_cursor()?];
                app.notify("code", Notify::Status(msg))?;
                Ok(Event::Noop)
            }
            evnt => Ok(evnt),
        }
    }

    fn on_refresh(&mut self, app: &mut code::Code) -> Result<()> {
        use crate::view::{NoWrap, Wrap};

        let err = Error::Invalid(format!("buffer {}", self.curr_buf_id));
        self.cursor = if app.as_ref().wrap {
            let v: Wrap = (&*self, self.obc_xy).into();
            let buf = err_at!(app.as_buffer(&self.curr_buf_id).ok_or(err))?;
            v.render(buf, self)?
        } else {
            let v: NoWrap = (&*self, self.obc_xy).into();
            let buf = err_at!(app.as_buffer(&self.curr_buf_id).ok_or(err))?;
            v.render(buf, self)?
        };
        self.obc_xy = {
            let err = Error::Invalid(format!("buffer {}", self.curr_buf_id));
            err_at!(app.as_buffer(&self.curr_buf_id).ok_or(err))?.to_xy_cursor()
        };

        Ok(())
    }
}

impl Render for WindowEdit {
    #[inline]
    fn as_color_scheme(&self) -> &ColorScheme {
        &self.scheme
    }

    fn to_span_line(&self, buf: &Buffer, a: usize, z: usize) -> Result<Spanline> {
        self.syn.to_span_line(buf, a, z)
    }
}
