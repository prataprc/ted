use crossterm::{cursor as term_cursor, queue};
#[allow(unused_imports)]
use log::trace;

use std::{fmt, iter::FromIterator, mem, result};

use crate::{
    buffer::{self, Buffer},
    code::{keymap::Keymap, Code},
    colors::Highlight,
    event::Event,
    syntax,
    location::Location,
    term::{Span, Spanline},
    window::{Coord, Cursor, Render, WinBuffer, Window},
    Error, Result,
};

pub struct WindowCmd {
    coord: Coord,
    cursor: Cursor,
    obc_xy: buffer::Cursor,
    buffer: Buffer,
    syn: syntax::Types,
    scheme: ColorScheme,
    keymap: Keymap,
}

impl fmt::Display for WindowCmd {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "WindowCmd<{}@{} {}>", self.cursor, self.coord, self.obc_xy)
    }
}

impl WindowCmd {
    #[inline]
    pub fn new(coord: Coord, scheme: &ColorScheme) -> WindowCmd {
        use crate::code::view::NoWrap;

        let buf = {
            let loc = Location::new_ted("code-cmd");
            let mut buf = Buffer::from_reader(vec![].as_slice(), loc).unwrap();
            buf.mode_insert();
            buf
        };
        let cursor = NoWrap::initial_cursor(false /*line_number*/);
        let obc_xy = (0, 0).into();
        WindowLine {
            coord,
            cursor,
            obc_xy,
            buffer: buf,
            syn: syntax::CodeCmd::new("".to_string(), scheme).unwrap(),
            scheme,
            keymap: Keymap::new_cmd(),
        }
    }
}

impl Window for WindowCmd {
    type App = Code;

    #[inline]
    fn to_cursor(&self) -> Cursor {
        self.coord.to_top_left() + self.cursor
    }

    fn on_event(&mut self, _app: &mut Code, mut evnt: Event) -> Result<Event> {
        use crate::event::Code;

        evnt = {
            let keymap = mem::replace(&mut self.keymap, Default::default());
            let evnt = keymap.fold(&mut self.buffer, evnt)?;
            self.keymap = keymap
            evnt
        };
        evnt = self.buffer.on_event(evnt)?;
    }

    fn on_refresh(&mut self, app: &mut Code) -> Result<()> {
        use crate::code::view::NoWrap;
        use std::iter::repeat;

        let scheme = app.as_color_scheme();

        let (col, row) = self.coord.to_origin_cursor();
        err_at!(Fatal, termqu!(term_cursor::MoveTo(col, row)))?;

        let mut inner = mem::replace(&mut self.inner, Default::default());
        match &mut inner {
            Inner::Cmd {
                buffer,
                obc_xy,
                cursor,
                ..
            } => {
                let (name, coord) = (&self.name, self.coord);
                let mut v = NoWrap::new(name, coord, *cursor, *obc_xy);
                v.set_scroll_off(0).set_line_number(false);
                *cursor = v.render(buffer, self, scheme)?;
                *obc_xy = buffer.to_xy_cursor();
            }
            Inner::Status { spans } => {
                for span in spans.iter() {
                    err_at!(Fatal, termqu!(span))?;
                }
                let padding = {
                    let n: usize = spans.iter().map(|span| span.to_width()).sum();
                    let iter = repeat(' ').take((self.coord.wth as usize) - n);
                    let padding: Span = String::from_iter(iter).into();
                    padding.using(scheme.to_style(Highlight::StatusLine))
                };
                err_at!(Fatal, termqu!(padding))?;
            }
            Inner::Tab { spans } => {
                for span in spans.iter() {
                    err_at!(Fatal, termqu!(span))?;
                }
                let padding = {
                    let n: usize = spans.iter().map(|span| span.to_width()).sum();
                    let iter = repeat(' ').take((self.coord.wth as usize) - n);
                    let padding: Span = String::from_iter(iter).into();
                    padding.using(scheme.to_style(Highlight::TabLine))
                };
                err_at!(Fatal, termqu!(padding))?;
            }
            Inner::None => (),
        };
        self.inner = inner;

        Ok(())
    }
}

impl Render for WindowLine {
    fn to_span_line(&self, buf: &Buffer, a: usize, z: usize) -> Result<Spanline> {
        buffer::to_span_line(buf, a, z)
    }
}
