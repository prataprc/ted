use crossterm::queue;
use unicode_width::UnicodeWidthChar;

use std::{
    cmp, fmt,
    io::{self, Write},
    result,
};

use crate::{
    buffer::Buffer,
    code::App,
    event::Event,
    window::{Coord, Cursor, Span, Spanline},
    Error, Result,
};

#[derive(Clone)]
pub struct WindowPrompt {
    coord: Coord,
    span_lines: Vec<Spanline>,

    buffer: Buffer,
}

impl fmt::Display for WindowPrompt {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "WindowPrompt<{}>", self.coord)
    }
}

impl WindowPrompt {
    #[inline]
    pub fn new(coord: Coord, lines: Vec<Spanline>) -> WindowPrompt {
        let mut w = WindowPrompt {
            coord,
            span_lines: lines,
            buffer: Buffer::empty(),
        };
        w.buffer.mode_insert().unwrap();
        w
    }
}

impl WindowPrompt {
    #[inline]
    pub fn to_cursor(&self) -> Cursor {
        let n = match self.span_lines.last() {
            Some(line) => line.to_width(),
            None => 0,
        };
        let m: usize = {
            let s = self.buffer.to_string();
            s.chars().filter_map(|ch| ch.width()).sum()
        };
        let (hgt, wth) = self.coord.to_size();
        let col = {
            let (col, _) = self.coord.to_origin_cursor();
            let good_col = (col as usize) + n + m;
            cmp::max(good_col, wth.saturating_sub(1) as usize) as u16
        };
        Cursor::new(col, hgt - 1)
    }

    pub fn on_event(&mut self, app: &mut App, evnt: Event) -> Result<Event> {
        match evnt {
            Event::Esc => Ok(Event::Noop),
            evnt => self.buffer.on_event(evnt),
        }
    }

    pub fn on_refresh(&mut self, _: &mut App) -> Result<()> {
        let mut stdout = io::stdout();

        let row_iter = {
            let (col, _) = self.coord.to_origin_cursor();
            let (hgt, wth) = self.coord.to_size();
            let row_start = hgt - 1 - (self.span_lines.len() as u16);
            (col..wth).zip(row_start..hgt)
        };
        for ((col, row), line) in row_iter.zip(self.span_lines.iter_mut()) {
            line.set_cursor(Cursor { col, row });
            err_at!(Fatal, queue!(stdout, line))?;
        }
        err_at!(Fatal, queue!(stdout, span!(st: self.buffer.to_string())))?;
        Ok(())
    }
}
