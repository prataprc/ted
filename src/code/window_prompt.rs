use crossterm::queue;
#[allow(unused_imports)]
use log::trace;
use regex::Regex;
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
    options: Vec<Regex>,
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
            options: Default::default(),
        };
        w.buffer.mode_insert();
        w
    }

    pub fn set_options(&mut self, options: Vec<Regex>) {
        self.options.extend(options.into_iter());
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
            cmp::min(good_col, wth.saturating_sub(1) as usize) as u16
        };
        Cursor::new(col, hgt - 1)
    }

    pub fn prompt_match(&self) -> Option<String> {
        let s = self.buffer.to_string();
        if s.len() > 0 && self.options.len() == 0 {
            return Some(s);
        }
        for re in self.options.iter() {
            if re.is_match(s.as_str()) {
                return Some(s);
            }
        }
        None
    }

    pub fn on_event(&mut self, _: &mut App, evnt: Event) -> Result<Event> {
        match evnt {
            Event::Esc => Ok(Event::Noop),
            evnt => self.buffer.on_event(evnt),
        }
    }

    pub fn on_refresh(&mut self, _: &mut App) -> Result<()> {
        let mut stdout = io::stdout();

        let (col, row_iter) = {
            let (col, _) = self.coord.to_origin_cursor();
            let (hgt, _) = self.coord.to_size();
            let start = hgt.saturating_sub(self.span_lines.len() as u16);
            (col, start..hgt)
        };
        for (row, line) in row_iter.zip(self.span_lines.iter_mut()) {
            line.set_cursor(Cursor { col, row });
            err_at!(Fatal, queue!(stdout, line))?;
        }
        err_at!(Fatal, queue!(stdout, span!(st: self.buffer.to_string())))?;
        Ok(())
    }
}
