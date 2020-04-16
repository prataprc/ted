use crossterm::Command;
use log::trace;

use std::{
    convert::TryInto,
    fmt, io,
    iter::FromIterator,
    ops::{self, Bound, RangeBounds},
    result,
};

use crate::{buffer, Buffer, Config, Event, Result};

pub trait Window {
    fn new(coord: Coord, config: Config) -> Result<Self>;

    fn load<R>(&mut self, buffer: &Buffer) -> Result<Self> where R: io::Read;

    fn refresh(&mut self, buffer: &Buffer) -> Result<Render>;

    fn handle_event(&mut self, evnt: Event) -> Result<Option<Event>>;

    fn move_by(mut self, col_off: i16, row_off: i16) -> Self;

    fn resize_to(mut self, height: u16, width: u16) -> Self;
}

// Terminal coordinates, describes the four corners of a window.
// Origin is at (1, 1).
pub struct Coord {
    col: u16,
    row: u16,
    hgt: u16,
    wth: u16,
}

impl Default for Coord {
    fn default() -> Coord {
        Coord { col: 1, row: 1, hgt: 0, wth: 0 }
    }
}

impl Coord {
    pub fn new(col: u16, row: u16, hgt: u16, wth: u16) -> Coord {
        Coord { col, row, hgt, wth }
    }

    #[inline]
    pub fn move_by(mut self, col_off: i16, row_off: i16) -> Self {
        self.col = ((self.col as i16) + col_off) as u16;
        self.row = ((self.row as i16) + row_off) as u16;
        self
    }

    #[inline]
    pub fn resize_to(mut self, height: u16, width: u16) -> Self {
        self.hgt = height;
        self.wth = width;
        self
    }

    #[inline]
    pub fn to_origin(&self) -> (u16, u16) {
        (self.col, self.row)
    }

    #[inline]
    pub fn to_size(&self) -> (u16, u16) {
        (self.hgt, self.wth)
    }
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(
            f,
            "Coord<col:{} row:{} height:{} width:{}>",
            self.col, self.row, self.hgt, self.wth
        )
    }
}

// Cursor within the Terminal/Window, starts from (0, 0)
pub struct Cursor {
    col: u16
    row: u16
}

impl Cursor {
    fn new(col: u16, row: u16) -> Cursor {
        Cursor{ col, row }
    }
}

impl fmt::Display for Cursor {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "Cursor<col:{} row:{}>", self.col, self.row)
    }
}

// lines and cursor point point to render within the Terminal
pub struct Render {
    pub lines: Option<std::vec::IntoIter<(u16, u16, Span)>>,
    pub cursor: Option<Cursor>,
}

pub struct Span(String);

impl Command for Span {
    type AnsiType = String;

    fn ansi_code(&self) -> Self::AnsiType {
        self.0.clone()
    }
}

