use crossterm::{
    style::{self, Attribute, Color, StyledContent},
    Command,
};

use std::{
    convert::{TryFrom, TryInto},
    fmt,
    iter::FromIterator,
    ops::Add,
    result,
    sync::mpsc,
};

use crate::{
    event::Event, state::Context, window_code::WindowCode, window_line::WindowLine,
    window_prompt::WindowPrompt, Error, Result,
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

pub fn new_window_line(typ: &str, mut coord: Coord) -> WindowLine {
    let (col, _) = coord.to_origin();
    let (hgt, wth) = coord.to_size();
    let row = match typ {
        "cmdline" => hgt.saturating_sub(2),
        "stsline" => hgt.saturating_sub(2),
        "tbcline" => hgt.saturating_sub(3),
        _ => unreachable!(),
    };
    coord = Coord::new(col, row, 1, wth);
    WindowLine::new("cmd-line", coord)
}

pub enum Message {
    Notify(Notify),
    None,
}

pub enum Notify {
    None,
}

#[derive(Clone)]
pub enum Window {
    Code(Box<WindowCode>),
    Prompt(Box<WindowPrompt>),
    None,
}

impl fmt::Display for Window {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        match self {
            Window::Code(_) => write!(f, "window-code"),
            Window::Prompt(_) => write!(f, "window-prompt"),
            Window::None => write!(f, "window-none"),
        }
    }
}

impl Eq for Window {}

impl PartialEq for Window {
    fn eq(&self, other: &Window) -> bool {
        use Window::{Code, Prompt};

        match (self, other) {
            (Code(_), Code(_)) => true,
            (Prompt(_), Prompt(_)) => true,
            (Window::None, Window::None) => true,
            _ => false,
        }
    }
}

impl TryFrom<Window> for Event {
    type Error = Error;

    fn try_from(w: Window) -> Result<Event> {
        match w {
            Window::Code(_) => Ok(Event::Noop),
            Window::Prompt(w) => (*w).try_into(),
            Window::None => Ok(Event::Noop),
        }
    }
}

impl Default for Window {
    fn default() -> Window {
        Window::None
    }
}

impl Window {
    pub fn to_cursor(&self) -> Cursor {
        match self {
            Window::Code(w) => w.to_cursor(),
            Window::Prompt(w) => w.to_cursor(),
            Window::None => Default::default(),
        }
    }

    pub fn on_event(&mut self, c: &mut Context, evnt: Event) -> Result<Event> {
        match self {
            Window::Code(w) => w.on_event(c, evnt),
            Window::Prompt(w) => w.on_event(c, evnt),
            Window::None => Ok(evnt),
        }
    }

    pub fn on_refresh(&mut self, c: &mut Context) -> Result<()> {
        match self {
            Window::Code(w) => w.on_refresh(c),
            Window::Prompt(w) => w.on_refresh(c),
            Window::None => Ok(()),
        }
    }
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
        (self.col - 1, self.row - 1)
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
    text: StyledContent<String>,
    cursor: Option<Cursor>,
}

impl From<StyledContent<String>> for Span {
    fn from(text: StyledContent<String>) -> Span {
        Span { text, cursor: None }
    }
}

impl From<String> for Span {
    fn from(text: String) -> Span {
        Span {
            text: style::style(text),
            cursor: None,
        }
    }
}

impl Span {
    pub fn on(mut self, color: Color) -> Self {
        self.text = self.text.on(color);
        self
    }

    pub fn with(mut self, color: Color) -> Self {
        self.text = self.text.with(color);
        self
    }

    pub fn attribute(mut self, attr: Attribute) -> Self {
        self.text = self.text.attribute(attr);
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
        s.push_str(&self.text.to_string());
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
