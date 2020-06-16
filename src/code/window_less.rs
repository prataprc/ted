#[allow(unused_imports)]
use log::trace;
use unicode_width::UnicodeWidthChar;

use std::{convert::TryInto, fmt, mem, result};

use crate::{
    buffer::Buffer,
    code::{keymap::Keymap, Code},
    event::Event,
    location::Location,
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

impl WindowLess {
    #[inline]
    pub fn new(coord: Coord, content: &str) -> WindowLess {
        let loc = Location::new_ted("win-less");
        let mut w = WindowLess {
            coord,
            status_line: Default::default(),
            keymap: Default::default(),
            buffer: Buffer::from_reader(content.as_bytes(), loc).unwrap(),
        };
        w.buffer.mode_normal();
        w
    }
}

impl Window for WindowLess {
    type App = Code;

    #[inline]
    fn to_cursor(&self) -> Cursor {
        let (hgt, _) = self.coord.to_size();
        let col: usize = self
            .status_line
            .chars()
            .map(|ch| ch.width().unwrap_or(0))
            .sum();
        Cursor::new(col.try_into().unwrap(), hgt - 1)
    }

    fn on_event(&mut self, _: &mut Code, evnt: Event) -> Result<Event> {
        match evnt {
            Event::Esc => Ok(Event::Noop),
            evnt => {
                let mut km = mem::replace(&mut self.keymap, Default::default());
                let evnt = km.fold(&self.buffer, evnt)?;
                let evnt = self.buffer.on_event(evnt)?;
                self.keymap = km;
                Ok(evnt)
            }
        }
    }

    fn on_refresh(&mut self, _: &mut Code) -> Result<()> {
        //use crate::Error;
        //use crossterm::queue;
        //use std::io::{self, Write};

        //let mut stdout = io::stdout();

        //let (col, row_iter) = {
        //    let (col, _) = self.coord.to_origin_cursor();
        //    let (hgt, _) = self.coord.to_size();
        //    let start = hgt.saturating_sub(self.span_lines.len() as u16);
        //    (col, start..hgt)
        //};
        //for (row, line) in row_iter.zip(self.span_lines.iter_mut()) {
        //    line.set_cursor(Cursor { col, row });
        //    err_at!(Fatal, queue!(stdout, line))?;
        //}
        //err_at!(Fatal, queue!(stdout, span!(st: self.buffer.to_string())))?;
        Ok(())
    }
}
