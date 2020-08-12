use crossterm;

use std::{cmp, fmt, ops::Add, ops::RangeBounds, result};

pub use crate::window_less::WindowLess;
pub use crate::window_prompt::WindowPrompt;
pub use crate::window_status::WindowStatus;
pub use crate::window_suggt::WindowSuggest;

use crate::{
    buffer::{self, Buffer},
    colors::ColorScheme,
    event::{self, Event, DP},
    term::Spanline,
    text, Result,
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

    fn to_cursor(&self) -> Option<Cursor>;

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
    /// this buffer. If cursor is None, use the buffer's current cursor,
    /// else use the supplied cursor as cursor position.
    fn to_xy_cursor(&self, cursor: Option<usize>) -> buffer::Cursor;

    /// Return an iterator starting from line_idx. `dp` can either be
    /// [DP::Right] or [DP::Left] for either forward iteration or reverse
    /// iteration. In the forward direction, iteration will start from
    /// the cursor's current line. In reverse direction, iteration will start
    /// from the one before cursor's current line. Note that,
    /// `0 <= line_idx <= last_line_idx`.
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

    fn slice<R>(&self, char_range: R) -> String
    where
        R: RangeBounds<usize>;

    /// Return the character offset of first character for the requested
    /// `line_idx`. Note that, `0 <= line_idx < last_line_idx`.
    fn line_to_char(&self, line_idx: usize) -> usize;

    fn line(&self, line_idx: usize) -> String;

    /// Return the number of characters in the buffer.
    fn n_chars(&self) -> usize;

    /// Return the number of lines in the buffer.
    fn to_last_line_idx(&self) -> usize;

    /// Return the number of characters in line `line_idx`, starts from ZERO.
    fn len_line(&self, line_idx: usize) -> usize;
}

/// Render trait for window objects.
pub trait Render {
    type Buf;

    fn as_color_scheme(&self) -> &ColorScheme;

    fn to_span_line(&self, buf: &Self::Buf, a: usize, z: usize) -> Result<Spanline>;
}

/// Suggestion trait for tab completion.
pub trait Suggestion {
    fn on_complete(&self, cursor: Cursor) -> Vec<String>;
}

// Terminal coordinates, describes the four corners of a window.
// Origin is at (1, 1).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

pub struct JumpList {
    zero: Option<Jump>,
    older: Vec<Jump>,
    inner: Vec<Jump>,
    rowbits: Vec<u64>,
}

impl Default for JumpList {
    fn default() -> JumpList {
        JumpList {
            zero: None,
            older: Vec::default(),
            inner: Vec::default(),
            rowbits: Vec::default(),
        }
    }
}

impl JumpList {
    pub fn remember(&mut self, jmp: Jump) {
        let fix_row: bool = match self.zero.take() {
            Some(zero) if zero.row == jmp.row => {
                self.older.extend(self.inner.drain(..).rev());
                false
            }
            Some(zero) => {
                self.older.push(zero);
                self.older.extend(self.inner.drain(..).rev());
                true
            }
            None => true,
        };

        assert_eq!(self.inner.len(), 0);

        if fix_row && self.get_rowbit(jmp.row) {
            self.jumps_for_row(jmp.row).into_iter().rev().for_each(|i| {
                self.older.remove(i);
            })
        }
        self.set_rowbit(jmp.row);
        self.older.push(jmp);
        self.zero = None;
    }

    pub fn older(&mut self, n: usize, jmp: Option<Jump>) {
        match (self.zero.clone(), jmp) {
            (None, Some(jmp)) => {
                if self.get_rowbit(jmp.row) {
                    self.jumps_for_row(jmp.row).into_iter().rev().for_each(|i| {
                        self.older.remove(i);
                    })
                }
                self.set_rowbit(jmp.row);
                self.older.push(jmp);
            }
            _ => (),
        }
        // if user already moving around the jump-list, ignore `jmp`.

        match self.older.len() {
            0 => (),
            _ if n == 0 => (),
            _ => {
                self.zero.take().map(|zero| self.inner.push(zero));
                let off = {
                    let n = cmp::min(n, self.older.len()).saturating_sub(1);
                    let mut iter = self.older.iter().enumerate().rev().skip(n);
                    iter.next().map(|(i, _)| i).unwrap_or(0)
                };
                let mut iter = self.older.drain(off..);
                self.zero = iter.next();
                self.inner.extend(iter.rev());
            }
        }
    }

    pub fn newer(&mut self, n: usize) {
        match self.inner.len() {
            0 => (),
            _ if n == 0 => (),
            _ => {
                self.zero.take().map(|zero| self.older.push(zero));
                let off = {
                    let n = cmp::min(n, self.inner.len()).saturating_sub(1);
                    let mut iter = self.inner.iter().enumerate().rev().skip(n);
                    iter.next().map(|(i, _)| i).unwrap_or(0)
                };
                let mut iter = self.inner.drain(off..);
                self.zero = iter.next();
                self.older.extend(iter.rev());
            }
        }
    }

    pub fn update(&mut self, edit: &event::Edit) {
        let row = self.zero.clone().map(|zero| zero.row);
        self.zero = match (self.zero.take(), row) {
            (Some(jmp), Some(row)) => match jmp.update(&edit) {
                zero @ Some(_) => zero,
                None => {
                    self.del_rowbit(row);
                    None
                }
            },
            _ => None,
        };

        let (mut older, mut rows) = (vec![], vec![]);
        for jmp in self.older.drain(..) {
            let row = jmp.row;
            match jmp.update(&edit) {
                Some(jmp) => older.push(jmp),
                None => rows.push(row),
            }
        }
        rows.into_iter().for_each(|row| self.del_rowbit(row));
        self.older = older;

        let (mut inner, mut rows) = (vec![], vec![]);
        for jmp in self.inner.drain(..) {
            let row = jmp.row;
            match jmp.update(&edit) {
                Some(jmp) => inner.push(jmp),
                None => rows.push(row),
            }
        }
        rows.into_iter().for_each(|row| self.del_rowbit(row));
        self.inner = inner;
    }

    fn jumps_for_row(&self, row: usize) -> Vec<usize> {
        let mut offs = vec![];
        for (i, jmp) in self.older.iter().enumerate() {
            if jmp.row == row {
                offs.push(i)
            }
        }
        offs
    }

    fn get_rowbit(&mut self, row: usize) -> bool {
        let (wo, bo) = (row / 64, row % 64);
        match self.rowbits.len() {
            n if wo < n => (self.rowbits[wo] & (1 << bo)) > 0,
            _ => false,
        }
    }

    fn set_rowbit(&mut self, row: usize) {
        if self.rowbits.len() < (row / 64) + 1 {
            self.rowbits.resize((row / 64) + 1, u64::default());
        }
        let (wo, bo) = (row / 64, row % 64);
        self.rowbits[wo] = self.rowbits[wo] | (1 << bo);
    }

    fn del_rowbit(&mut self, row: usize) {
        let (wo, bo) = (row / 64, row % 64);
        match self.rowbits.len() {
            n if wo < n => {
                self.rowbits[wo] = self.rowbits[wo] & (!(1 << bo));
            }
            _ => (),
        }
    }
}

#[derive(Clone)]
pub struct Jump {
    buf_id: String,
    cursor: usize,
    col: usize,
    row: usize,
}

impl Jump {
    pub fn new(cursor: usize, buf: &Buffer) -> Self {
        let bc_xy = buf.to_xy_cursor(Some(cursor));
        Jump {
            buf_id: buf.to_id(),
            cursor,
            col: bc_xy.col,
            row: bc_xy.row,
        }
    }

    fn update(mut self, edit: &event::Edit) -> Option<Self> {
        use crate::event::Edit;

        match edit {
            Edit::Ins { cursor, txt } if self.cursor >= *cursor => {
                self.cursor += text::width(txt.chars());
                Some(self)
            }
            Edit::Del { cursor, txt } if self.cursor >= *cursor => {
                let n = text::width(txt.chars());
                match (*cursor..(*cursor + n)).contains(&self.cursor) {
                    false => {
                        self.cursor -= n;
                        Some(self)
                    }
                    true => None,
                }
            }
            Edit::Chg { cursor, oldt, newt } => {
                let n = text::width(oldt.chars());
                let m = text::width(newt.chars());
                match (*cursor..(*cursor + n)).contains(&self.cursor) {
                    false => {
                        self.cursor = self.cursor - n + m;
                        Some(self)
                    }
                    true => None,
                }
            }
            _ => Some(self),
        }
    }
}
