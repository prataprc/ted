use crossterm::queue;
use lazy_static::lazy_static;
#[allow(unused_imports)]
use log::trace;
use regex::Regex;

use std::{cmp, convert::TryInto, fmt, result};

use crate::{
    buffer::Buffer,
    code,
    colors::{ColorScheme, Highlight},
    event::Event,
    term::{Span, Spanline, Style},
    text,
    window::{Coord, Cursor, Window},
    Error, Result,
};

lazy_static! {
    static ref RE_ERROR: Regex = Regex::new(r"(?i)error").unwrap();
}

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

impl<'a> From<(&'a code::Code, Coord, Vec<String>)> for WindowPrompt {
    fn from((app, coord, lines): (&'a code::Code, Coord, Vec<String>)) -> Self {
        let scheme = app.to_color_scheme(None);
        let style = Self::to_style(&lines, &scheme);
        let span_lines: Vec<Spanline> = {
            let iter = lines.into_iter().map(|l| {
                let span: Span = l.into();
                let spl: Spanline = span.using(style.clone()).into();
                spl
            });
            iter.collect()
        };
        let mut w = WindowPrompt {
            coord,
            span_lines,
            buffer: Buffer::empty(),
            options: Vec::default(),
        };
        w.buffer.mode_insert();
        w
    }
}

impl WindowPrompt {
    pub fn set_options(&mut self, options: Vec<Regex>) {
        self.options.extend(options.into_iter());
    }

    fn to_style(lines: &[String], scheme: &ColorScheme) -> Style {
        match lines.iter().any(|l| RE_ERROR.find(l.as_str()).is_some()) {
            true => scheme.to_style(Highlight::Error),
            false => scheme.to_style(Highlight::Canvas),
        }
    }
}

impl WindowPrompt {
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
}

impl Window for WindowPrompt {
    type App = code::Code;

    #[inline]
    fn to_name(&self) -> String {
        "window-prompt".to_string()
    }

    #[inline]
    fn to_coord(&self) -> Coord {
        self.coord
    }

    #[inline]
    fn to_cursor(&self) -> Option<Cursor> {
        let col: u16 = match self.span_lines.last() {
            Some(line) => {
                let n: u16 = {
                    let n = text::width(self.buffer.to_string().chars());
                    n.try_into().unwrap()
                };
                let m: u16 = line.to_width().try_into().unwrap();
                cmp::min(curz!(self.coord.col) + n + m, curz!(self.coord.wth))
            }
            None => 0,
        };
        Some(Cursor::new(col, curz!(self.coord.row) + self.coord.hgt))
    }

    #[inline]
    fn config_line_number(&self) -> bool {
        false
    }

    #[inline]
    fn config_scroll_offset(&self) -> u16 {
        0
    }

    fn on_event(&mut self, _: &mut code::Code, evnt: Event) -> Result<Event> {
        match evnt {
            Event::Esc => Ok(Event::Noop),
            evnt => self.buffer.on_event(evnt),
        }
    }

    fn on_refresh(&mut self, _: &mut code::Code) -> Result<()> {
        let col = curz!(self.coord.col);
        let till = curz!(self.coord.row) + self.coord.hgt;
        let rows = {
            let n: u16 = err_at!(FailConvert, self.span_lines.len().try_into())?;
            (till - n)..till
        };
        for (row, line) in rows.zip(self.span_lines.iter_mut()) {
            line.set_cursor(Cursor { col, row });
            err_at!(Fatal, termqu!(line))?;
        }
        {
            let input: Span = self.buffer.to_string().into();
            err_at!(Fatal, termqu!(input))?
        }
        Ok(())
    }
}
