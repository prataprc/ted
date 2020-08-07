#[allow(unused_imports)]
use log::{debug, trace};

use std::{cmp, convert::TryFrom, fmt, io, mem, result};

use crate::{
    buffer::Buffer,
    code::{self, keymap::Keymap},
    colors::ColorScheme,
    event::{self, Event},
    location::Location,
    syntax, view,
    window::{Coord, Cursor, Window},
    Error, Result,
};

pub struct WindowLess {
    coord: Coord,
    cursor: Cursor,
    buffer: Buffer,
    syn: syntax::Type,
    scheme: ColorScheme,
    keymap: Keymap,
    old_screen: Option<Vec<view::ScrLine>>,
    // configuration
    wrap: bool,
}

impl fmt::Display for WindowLess {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "WindowLess<{}>", self.coord)
    }
}

impl<'a> TryFrom<(&'a code::Code, event::Code, Coord)> for WindowLess {
    type Error = Error;

    fn try_from(arg: (&'a code::Code, event::Code, Coord)) -> Result<Self> {
        let (app, less, mut coord) = arg;
        let (name, content, wrap) = match less {
            event::Code::Less {
                name,
                hgt,
                content,
                wrap,
            } => {
                coord.hgt = cmp::min(hgt, coord.hgt);
                (name, content, wrap)
            }
            val => err_at!(Invalid, msg: format!("{} != WindowLess", val))?,
        };
        let loc = {
            let read_only = true;
            Location::new_ted(&name, io::empty(), read_only)?
        };
        let scheme = app.to_color_scheme(None);
        let buf = Buffer::from_reader(content.as_bytes(), loc)?;
        let syn = syntax::detect(&buf, &scheme)?;
        let mut w = WindowLess {
            coord,
            cursor: Cursor::default(),
            buffer: buf,
            syn,
            scheme,
            keymap: Keymap::new_edit(),
            old_screen: None,
            wrap,
        };
        debug!("{}", w);
        w.buffer.mode_normal();
        Ok(w)
    }
}

impl Window for WindowLess {
    type App = code::Code;

    #[inline]
    fn to_name(&self) -> String {
        "window-less".to_string()
    }

    #[inline]
    fn to_coord(&self) -> Coord {
        self.coord
    }

    #[inline]
    fn to_cursor(&self) -> Option<Cursor> {
        None
    }

    #[inline]
    fn config_line_number(&self) -> bool {
        false
    }

    #[inline]
    fn config_scroll_offset(&self) -> u16 {
        0
    }

    fn on_event(&mut self, app: &mut code::Code, evnt: Event) -> Result<Event> {
        match evnt {
            evnt @ Event::Esc => Ok(evnt),
            evnt => {
                let mut km = mem::replace(&mut self.keymap, Keymap::default());
                let evnt = km.fold(app, &self.buffer, evnt)?;
                let evnt = self.buffer.on_event(evnt)?;
                self.keymap = km;
                Ok(evnt)
            }
        }
    }

    fn on_refresh(&mut self, _: &mut code::Code) -> Result<()> {
        //use crate::Error;
        //use crossterm::queue;
        //use std::io::{self, Write};

        //let (col, row_iter) = {
        //    let (col, _) = self.coord.to_origin_cursor();
        //    let start = self.coord.hgt.saturating_sub(
        //        self.span_lines.len() as u16
        //    );
        //    (col, start..self.coord.hgt)
        //};
        //for (row, line) in row_iter.zip(self.span_lines.iter_mut()) {
        //    line.set_cursor(Cursor { col, row });
        //    err_at!(Fatal, termqu!(line))?;
        //}
        //let span: Span = self.buffer.to_string().into();
        //err_at!(Fatal, termqu!(span))?;
        Ok(())
    }
}
