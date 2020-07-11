use crossterm;

use std::{fmt, ops::Add, result};

use crate::{
    buffer::{self},
    colors::ColorScheme,
    event::Event,
    event::DP,
    term::Spanline,
    Result,
};

#[macro_export]
macro_rules! cursor {
    ($col:expr, $row:expr) => {
        Cursor {
            col: $col,
            row: $row,
        }
    };
}

/// Window trait for all screen areas defined by ted applications.
pub trait Window {
    type App;

    fn to_name(&self) -> String;

    fn to_coord(&self) -> Coord;

    fn to_cursor(&self) -> Cursor;

    fn config_line_number(&self) -> bool;

    fn config_scroll_offset(&self) -> u16;

    fn on_event(&mut self, app: &mut Self::App, evnt: Event) -> Result<Event>;

    fn on_refresh(&mut self, app: &mut Self::App) -> Result<()>;
}

/// This is a simple abstraction trait for [buffer::Buffer]. Gives an idea
/// on window's api dependency with `Buffer`.
pub trait WinBuffer {
    /// Return the cursor position, as character index.
    fn to_char_cursor(&self) -> usize;

    /// Return the cursor position, as (col, row) starting from (0,), within
    /// this buffer.
    fn to_xy_cursor(&self) -> buffer::Cursor;

    /// Return an iterator starting from line_idx. `dp` can either be
    /// [DP::Right] or [DP::Left] for either forward iteration or reverse
    /// iteration. In the forward direction, iteration will start from
    /// the cursor's current line. In reverse direction, iteration will start
    /// from the one before cursor's current line. Note that,
    /// `0 <= line_idx < n_lines`.
    fn lines_at<'a>(
        &'a self,
        line_idx: usize,
        dp: DP,
    ) -> Result<Box<dyn Iterator<Item = String> + 'a>>;

    /// Return an iterator starting from char_idx. `dp` can either be
    /// [DP::Right] or [DP::Left] for either forward iteration or reverse
    /// iteration. In the forward direction, iteration will start from
    /// the cursor position. In reverse direction, iteration will start
    /// from the one before cursor position. Note that,
    /// `0 <= char_idx < n_chars`.
    fn chars_at<'a>(
        &'a self,
        char_idx: usize,
        dp: DP,
    ) -> Result<Box<dyn Iterator<Item = char> + 'a>>;

    /// Return the character offset of first character for the requested
    /// `line_idx`. Note that, `0 <= line_idx < n_lines`.
    fn line_to_char(&self, line_idx: usize) -> usize;

    fn line(&self, line_idx: usize) -> String;

    /// Return the number of characters in the buffer.
    fn n_chars(&self) -> usize;

    /// Return the number of lines in the buffer.
    fn n_lines(&self) -> usize;

    /// Return the number of characters in line `line_idx`, starts from ZERO.
    fn len_line(&self, line_idx: usize) -> usize;
}

/// Render trait for window objects.
pub trait Render {
    type Buf;

    fn as_color_scheme(&self) -> &ColorScheme;

    fn to_span_line(&self, buf: &Self::Buf, a: usize, z: usize) -> Result<Spanline>;
}

// Terminal coordinates, describes the four corners of a window.
// Origin is at (1, 1).
#[derive(Clone, Copy, Debug)]
pub struct Coord {
    pub col: u16,
    pub row: u16,
    pub hgt: u16,
    pub wth: u16,
}

impl Default for Coord {
    fn default() -> Coord {
        Coord {
            col: 1,
            row: 1,
            hgt: 0,
            wth: 0,
        }
    }
}

impl Coord {
    /// Create a new viewport for window.
    pub fn new(col: u16, row: u16, hgt: u16, wth: u16) -> Coord {
        Coord { col, row, hgt, wth }
    }

    /// Move the window viewport by `col_off` and `row_off`.
    #[inline]
    pub fn move_by(mut self, col_off: i16, row_off: i16) -> Self {
        self.col = ((self.col as i16) + col_off) as u16;
        self.row = ((self.row as i16) + row_off) as u16;
        self
    }

    /// Resize the window viewport by `height` and `width`.
    #[inline]
    pub fn resize_to(mut self, height: u16, width: u16) -> Self {
        self.hgt = height;
        self.wth = width;
        self
    }

    /// Return the origin point, top-left of the viewport. Position starts
    /// from (1, 1).
    #[inline]
    pub fn to_origin(&self) -> (u16, u16) {
        (self.col, self.row)
    }

    /// Return the origin point in cursor parlance, position starts from
    /// (0, 0)
    #[inline]
    pub fn to_origin_cursor(&self) -> (u16, u16) {
        (self.col.saturating_sub(1), self.row.saturating_sub(1))
    }

    /// Return the origin point as window [Cursor] object.
    #[inline]
    pub fn to_top_left(&self) -> Cursor {
        let (col, row) = self.to_origin_cursor();
        cursor!(col, row)
    }

    /// Return the height and width of the viewport. Height and width counting
    /// starts from 1, similar to len().
    #[inline]
    pub fn to_size(&self) -> (u16, u16) {
        (self.hgt, self.wth)
    }
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(
            f,
            "Coord<{},{},{},{}>",
            self.col, self.row, self.hgt, self.wth
        )
    }
}

// Cursor within the Window object, starts from (0, 0)
#[derive(Clone, Copy, Default, Debug, Eq, PartialEq)]
pub struct Cursor {
    pub col: u16,
    pub row: u16,
}

impl From<(u16, u16)> for Cursor {
    fn from((col, row): (u16, u16)) -> Cursor {
        Cursor { col, row }
    }
}

impl fmt::Display for Cursor {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "Cursor<{},{}>", self.col, self.row)
    }
}

impl Add for Cursor {
    type Output = Cursor;

    fn add(self, rhs: Cursor) -> Cursor {
        cursor!(self.col + rhs.col, self.row + rhs.row)
    }
}

impl From<Cursor> for crossterm::cursor::MoveTo {
    fn from(cursor: Cursor) -> crossterm::cursor::MoveTo {
        let Cursor { col, row } = cursor;
        crossterm::cursor::MoveTo(col, row)
    }
}

impl Cursor {
    pub fn new(col: u16, row: u16) -> Cursor {
        Cursor { col, row }
    }

    pub fn move_by(mut self, col: i16, row: i16) -> Self {
        self.col = ((self.col as i16) + col) as u16;
        self.row = ((self.row as i16) + row) as u16;
        self
    }

    pub fn discount_nu(mut self, nu_wth: u16) -> Self {
        self.col -= nu_wth;
        self
    }

    pub fn account_nu(mut self, nu_wth: u16) -> Self {
        self.col += nu_wth;
        self
    }
}
