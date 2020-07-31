#[allow(unused_imports)]
use log::trace;

use std::{convert::TryInto, fmt, io, mem, result};

use crate::{
    buffer::Buffer,
    code::{self, keymap::Keymap},
    event::Event,
    location::Location,
    text,
    window::{Coord, Cursor, Window},
    Result,
};

pub struct WindowLess {
    coord: Coord,
    status_line: String,
    keymap: Keymap,
    buffer: Buffer,
}

impl fmt::Display for WindowLess {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "WindowLess<{}>", self.coord)
    }
}

impl<'a, 'b> From<(&'a code::Code, &'b str, Coord)> for WindowLess {
    fn from((_, content, coord): (&'a code::Code, &'b str, Coord)) -> Self {
        let loc = Location::new_ted("win-less", io::empty()).unwrap();
        let mut w = WindowLess {
            coord,
            status_line: String::default(),
            keymap: Keymap::new_less(),
            buffer: Buffer::from_reader(content.as_bytes(), loc).unwrap(),
        };
        w.buffer.mode_normal();
        w
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
    fn to_cursor(&self) -> Cursor {
        let col: usize = text::width(self.status_line.chars());
        Cursor::new(col.try_into().unwrap(), curz!(self.coord.hgt))
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
            Event::Esc => Ok(Event::Noop),
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
