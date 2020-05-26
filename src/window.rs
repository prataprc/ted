use crossterm::{
    event::{self as ct_event, Event as TermEvent},
    style::{self, Attribute, Color, StyledContent},
    Command,
};

use std::{fmt, iter::FromIterator, ops::Add, result};

use crate::{color_scheme::Style, Error, Result};

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

#[derive(Clone)]
pub enum Notify {
    Status(Vec<Span>),
    None,
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
}

// Span object to render on screen.
#[derive(Clone)]
pub struct Span {
    pub content: StyledContent<String>,
    pub cursor: Option<Cursor>,
}

impl From<StyledContent<String>> for Span {
    fn from(content: StyledContent<String>) -> Span {
        Span {
            content,
            cursor: None,
        }
    }
}

impl From<String> for Span {
    fn from(text: String) -> Span {
        Span {
            content: style::style(text),
            cursor: None,
        }
    }
}

impl Span {
    pub fn on(mut self, color: Color) -> Self {
        self.content = self.content.on(color);
        self
    }

    pub fn with(mut self, color: Color) -> Self {
        self.content = self.content.with(color);
        self
    }

    pub fn attribute(mut self, attr: Attribute) -> Self {
        self.content = self.content.attribute(attr);
        self
    }

    pub fn using(mut self, style: Style) -> Self {
        let mut content = self.content.clone().on(style.bg).with(style.fg);
        for attr in style.attrs.iter() {
            content = content.attribute(attr.clone());
        }
        self.content = content;
        self
    }

    pub fn set_cursor(&mut self, cursor: Cursor) -> &mut Self {
        self.cursor = Some(cursor);
        self
    }
}

impl Command for Span {
    type AnsiType = String;

    fn ansi_code(&self) -> Self::AnsiType {
        use crossterm::cursor::MoveTo;

        let mut s = match &self.cursor {
            Some(Cursor { col, row }) => MoveTo(*col, *row).to_string(),
            None => Default::default(),
        };
        s.push_str(&self.content.to_string());
        s
    }
}

// Spanline object to render on screen.
#[derive(Clone)]
pub struct Spanline {
    spans: Vec<Span>,
    cursor: Option<Cursor>,
}

impl FromIterator<Span> for Spanline {
    fn from_iter<T>(iter: T) -> Spanline
    where
        T: IntoIterator<Item = Span>,
    {
        let spans: Vec<Span> = iter.into_iter().collect();
        Spanline {
            cursor: None,
            spans,
        }
    }
}

impl Default for Spanline {
    fn default() -> Spanline {
        Spanline {
            spans: Default::default(),
            cursor: None,
        }
    }
}

impl Spanline {
    pub fn set_cursor(&mut self, cursor: Cursor) -> &mut Self {
        self.cursor = Some(cursor);
        self
    }

    pub fn add_span(&mut self, span: Span) -> &mut Self {
        self.spans.push(span);
        self
    }

    pub fn is_empty(&self) -> bool {
        self.spans.len() == 0
    }
}

impl Command for Spanline {
    type AnsiType = String;

    fn ansi_code(&self) -> Self::AnsiType {
        use crossterm::cursor::MoveTo;

        let mut s = match &self.cursor {
            Some(Cursor { col, row }) => MoveTo(*col, *row).to_string(),
            None => Default::default(),
        };
        for span in self.spans.clone().into_iter() {
            s.push_str(&span.ansi_code());
        }
        s
    }
}

pub fn wait_ch(ch: Option<char>) -> Result<()> {
    loop {
        let tevnt: TermEvent = err_at!(Fatal, ct_event::read())?;
        match ch {
            Some(ch) => match tevnt {
                TermEvent::Key(ct_event::KeyEvent {
                    code: ct_event::KeyCode::Char(c),
                    modifiers: _,
                }) if ch == c => break,
                _ => (),
            },
            None => break,
        }
    }
    Ok(())
}
