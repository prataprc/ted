use tree_sitter as ts;

use std::{fmt, ops::Add, result};

use crate::{
    buffer::{self, Buffer},
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

#[macro_export]
macro_rules! span {
    (fg:$fg:expr, bg:$bg:expr, st:$text:expr) => {{
        let span: Span = $text.into();
        span.with($fg).on($bg)
    }};
    (fg:$fg:expr, bg:$bg:expr, ($col:expr, $row:expr), st:$text:expr) => {{
        let mut span: Span = $text.into();
        span.set_cursor(Cursor { col: $col, row: $row });
        span.with($fg).on($bg)
    }};
    (($col:expr, $row:expr), st:$text:expr) => {{
        let mut span: Span = $text.into();
        span.set_cursor(Cursor { col: $col, row: $row });
        span
    }};
    (st:$text:expr) => {{
        let span: Span = $text.into();
        span
    }};
    (fg:$fg:expr, bg:$bg:expr, $($s:expr),*) => {{
        let span: Span = format!($($s),*).into();
        span.with($fg).on($bg)
    }};
    (fg:$fg:expr, bg:$bg:expr, ($col:expr, $row:expr), $($s:expr),*) => {{
        let mut span: Span = format!($($s),*).into();
        span.set_cursor(Cursor { col: $col, row: $row });
        span.with($fg).on($bg)
    }};
    (($col:expr, $row:expr), $($s:expr),*) => {{
        let mut span: Span = format!($($s),*).into();
        span.set_cursor(Cursor { col: $col, row: $row });
        span
    }};
    ($($s:expr),*) => {{
        let span: Span = format!($($s),*).into();
        span
    }};
}

pub trait Window {
    type App;

    fn to_cursor(&self) -> Cursor;

    fn on_event(&mut self, app: &mut Self::App, evnt: Event) -> Result<Event>;

    fn on_refresh(&mut self, app: &mut Self::App) -> Result<()>;
}

pub trait WinBuffer<'a> {
    type IterLine: Iterator<Item = &'a str>;
    type IterChar: Iterator<Item = char>;

    /// Return the cursor position, as (col, row) starting from (0,), within
    /// this buffer.
    fn to_xy_cursor(&self) -> buffer::Cursor;

    /// Return an iterator starting from line_idx. `dp` can either be
    /// [DP::Right] or [DP::Left] for either forward iteration or reverse
    /// iteration. In the forward direction, iteration will start from
    /// the cursor's current line. In reverse direction, iteration will start
    /// from the one before cursor's current line. Note that,
    /// `0 <= line_idx < n_lines`.
    fn lines_at(&'a self, line_idx: usize, dp: DP) -> Result<Self::IterLine>;

    /// Return an iterator starting from char_idx. `dp` can either be
    /// [DP::Right] or [DP::Left] for either forward iteration or reverse
    /// iteration. In the forward direction, iteration will start from
    /// the cursor position. In reverse direction, iteration will start
    /// from the one before cursor position. Note that,
    /// `0 <= char_idx < n_chars`.
    fn chars_at(&'a self, char_idx: usize, dp: DP) -> Result<Self::IterChar>;

    /// Return the character offset of first character for the requested
    /// `line_idx`. Note that, `0 <= line_idx < n_lines`.
    fn line_to_char(&self, line_idx: usize) -> usize;

    /// Return the line offset for requested `char_idx`, which must be a valid
    /// character offset within the buffer. [Buffer::to_cursor] is a `char_idx`.
    /// Note that, `0 <= char_idx < n_chars`.
    fn char_to_line(&self, char_idx: usize) -> usize;

    /// Return the number of characters in the buffer.
    fn n_chars(&self) -> usize;

    fn n_lines(&self) -> usize;

    fn len_line(&self, line_idx: usize) -> usize;

    /// Return whether the last character in buffer is NEWLINE.
    fn is_trailing_newline(&self) -> bool;
}

pub trait Render {
    fn to_span_line(&self, buf: &Buffer, from: usize, to: usize) -> Result<Spanline>;
}

pub trait Page {
    fn to_language(&self) -> Option<ts::Language>;

    fn to_name(&self) -> String;

    fn on_event(&mut self, buf: &mut Buffer, evnt: Event) -> Result<Event>;

    fn to_span_line(
        &self,
        buf: &Buffer,
        scheme: &ColorScheme,
        from: usize,
        to: usize,
    ) -> Option<Spanline>;
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
    pub fn to_origin_cursor(&self) -> (u16, u16) {
        (self.col.saturating_sub(1), self.row.saturating_sub(1))
    }

    #[inline]
    pub fn to_top_left(&self) -> Cursor {
        cursor!(self.col - 1, self.row - 1)
    }

    #[inline]
    pub fn to_trbl(&self, scroll_off: u16) -> (u16, u16, u16, u16) {
        let t = self.row + scroll_off;
        let r = self.col + self.wth - 1;
        let b = self.row + self.hgt - 1 - scroll_off;
        let l = self.col;
        (t, r, b, l)
    }

    #[inline]
    pub fn to_size(&self) -> (u16, u16) {
        (self.hgt, self.wth)
    }

    #[inline]
    pub fn empty_line(&self) -> Vec<char> {
        std::iter::repeat(' ').take(self.wth as usize).collect()
    }

    #[inline]
    pub fn to_cells(&self, n: usize) -> usize {
        let n_wth = n as u16;
        if (n_wth % self.wth) == 0 {
            n
        } else {
            (((n_wth / self.wth) * self.wth) + self.wth) as usize
        }
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

// Cursor within the Terminal/Window, starts from (0, 0)
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

impl Cursor {
    pub fn new(col: u16, row: u16) -> Cursor {
        Cursor { col, row }
    }

    pub fn next_cursors(self, coord: Coord) -> Vec<Cursor> {
        let mut cursors = Vec::with_capacity((coord.hgt * coord.wth) as usize);
        for r in 0..coord.hgt {
            for c in 0..coord.wth {
                cursors.push(Cursor { col: c, row: r })
            }
        }
        let n = (self.row * coord.hgt) + self.col;
        cursors.into_iter().skip(n as usize).collect()
    }

    pub fn prev_cursors(self, coord: Coord) -> Vec<Cursor> {
        let mut cursors = Vec::with_capacity((coord.hgt * coord.wth) as usize);
        for r in 0..coord.hgt {
            for c in 0..coord.wth {
                cursors.push(Cursor { col: c, row: r })
            }
        }
        let n = (self.row * coord.hgt) + self.col;
        cursors.into_iter().take(n as usize).rev().collect()
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
