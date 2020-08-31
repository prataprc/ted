use crossterm::{cursor as term_cursor, queue};
#[allow(unused_imports)]
use log::trace;

use std::{convert::TryInto, fmt, io, mem, result};

use crate::{
    buffer::{self, Buffer},
    code::{self, cmd},
    colors::ColorScheme,
    event::Event,
    keymap::Keymap,
    location::Location,
    term::Spanline,
    view,
    window::{Coord, Cursor, Render, WinBuffer, Window, WindowSuggest},
    Error, Result,
};

pub struct WindowCmd {
    coord: Coord,
    cursor: Cursor,
    obc_xy: buffer::Cursor,
    buf: Buffer,
    scheme: ColorScheme,
    keymap: Keymap,
    wsugg: WindowSuggest,
}

impl fmt::Display for WindowCmd {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(
            f,
            "WindowCmd<{}@{} {}>",
            self.cursor, self.coord, self.obc_xy
        )
    }
}

impl WindowCmd {
    pub fn new(coord: Coord, app: &code::Code) -> Result<WindowCmd> {
        let mut buf = {
            let read_only = false;
            let loc = Location::new_ted("code-cmd", io::empty(), read_only)?;
            Buffer::from_reader(loc)?
        };
        buf.set_insert_mode();
        buf.cud_char(None, ':').unwrap();

        let cursor = view::NoWrap::initial_cursor(false /*line_number*/);
        let obc_xy = (0, 0).into();
        Ok(WindowCmd {
            coord,
            cursor,
            obc_xy,
            buf,
            scheme: app.to_color_scheme(None),
            keymap: Keymap::new_cmd(),
            wsugg: app.to_wsugg(),
        })
    }
}

impl Window for WindowCmd {
    type App = code::Code;

    #[inline]
    fn to_name(&self) -> String {
        "window-cmd".to_string()
    }

    #[inline]
    fn to_coord(&self) -> Coord {
        self.coord
    }

    #[inline]
    fn to_cursor(&self) -> Option<Cursor> {
        Some(self.coord.to_top_left() + self.cursor)
    }

    #[inline]
    fn config_wrap(&self) -> bool {
        false
    }

    #[inline]
    fn config_line_number(&self) -> bool {
        false
    }

    #[inline]
    fn config_scroll_offset(&self) -> u16 {
        0
    }

    fn on_event(&mut self, app: &mut code::Code, mut evnt: Event) -> Result<Event> {
        use crate::code::cmd::Command;

        let mut buf = mem::replace(&mut self.buf, Buffer::empty());
        evnt = match self.keymap.fold(&mut buf, evnt)? {
            Event::N(n) => {
                let s = format!(".,.+{}", n.saturating_sub(1));
                buf.cud_str(Some(0), &s)?;
                Event::Noop
            }
            Event::Enter(_) => {
                let mut val: cmd::Cmd = {
                    let content = buf.to_string();
                    (content, self.scheme.clone()).try_into()?
                };
                let mut evnt = val.on_command(app)?;
                evnt.push(Event::Esc);
                evnt
            }
            evnt => buf.on_event(evnt)?,
        };
        self.buf = buf;
        Ok(evnt)
    }

    fn on_refresh(&mut self, _app: &mut code::Code) -> Result<()> {
        let (col, row) = self.coord.to_origin_cursor();
        err_at!(Fatal, termqu!(term_cursor::MoveTo(col, row)))?;

        let mut v: view::NoWrap = (&*self, self.obc_xy).try_into()?;
        v.shift_cursor(&self.buf)?;
        self.cursor = v.render(&self.buf, self, None)?;
        self.obc_xy = self.buf.to_xy_cursor(None);

        Ok(())
    }
}

impl Render for WindowCmd {
    type Buf = Buffer;

    #[inline]
    fn as_color_scheme(&self) -> &ColorScheme {
        &self.scheme
    }

    #[inline]
    fn to_span_line(&self, buf: &Self::Buf, a: usize, z: usize) -> Result<Spanline> {
        buf.to_span_line(a, z)
    }
}
