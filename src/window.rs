use crossterm::{
    cursor as term_cursor,
    style::{self, Attribute, Color},
    Command,
};

use std::{fmt, ops::Add, result};

use crate::{Buffer, Config, Event, Result};

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
        let mut spn = Span::new(&$text);
        spn.set_fg($fg).set_bg($bg);
        spn
    }};
    (fg:$fg:expr, bg:$bg:expr, ($col:expr, $row:expr), st:$text:expr) => {{
        let mut spn = Span::new(&$text);
        spn.set_cursor(Cursor { col: $col, row: $row });
        spn.set_fg($fg).set_bg($bg);
        spn
    }};
    (($col:expr, $row:expr), st:$text:expr) => {{
        let mut spn = Span::new(&$text);
        spn.set_cursor(Cursor { col: $col, row: $row });
        spn
    }};
    (st:$text:expr) => {{
        Span::new(&$text)
    }};
    (fg:$fg:expr, bg:$bg:expr, $($s:expr),*) => {{
        let mut spn = Span::new(&format!($($s),*));
        spn.set_fg($fg).set_bg($bg);
        spn
    }};
    (fg:$fg:expr, bg:$bg:expr, ($col:expr, $row:expr), $($s:expr),*) => {{
        let mut spn = Span::new(&format!($($s),*));
        spn.set_cursor(Cursor { col: $col, row: $row });
        spn.set_fg($fg).set_bg($bg);
        spn
    }};
    (($col:expr, $row:expr), $($s:expr),*) => {{
        let mut spn = Span::new(&format!($($s),*));
        spn.set_cursor(Cursor { col: $col, row: $row });
        spn
    }};
    ($($s:expr),*) => {{
        Span::new(&format!($($s),*))
    }};
}

#[macro_export]
macro_rules! on_win_event {
    ($state:expr, $evnt:expr) => {{
        match $state.window.take() {
            Some(window) => {
                $state.event = $evnt;
                let s = window.on_event($state)?;
                s.window = Some(window);
            },
            None => (),
        };
    }}
}

#[macro_export]
macro_rules! on_win_refresh {
    ($state:expr) => {{
        match $state.window.take() {
            Some(window) => {
                let s = window.on_refresh($state)?;
                s.window = Some(window);
            },
            None => None,
        };
    }}
}

// Application state
pub struct State {
    buffers: Vec<Buffer>,
    config: Config,
    window: Option<WindowFile>,
    event: Event,
}

impl Default for State {
    fn default() -> State {
        let coord: Coord = Default::default(),
        State {
            buffers: Default::default(),
            config: Default::default(),
            window: WindowFile::new(coord),
            event: Default::default(),
        }
    }
}

impl State {
    pub fn new(config: Config, window: WindowFile) -> State {
        State {
            buffers: Default::default(),
            config,
            window,
            event: Default::default(),
        }
    }
}

impl State {
    pub fn as_buffer(&self, id: &str) -> &Buffer {
        for b in self.buffers.iter() {
            if b.to_id() == id {
                return b;
            }
        }
        unreachable!()
    }

    pub fn as_mut_buffer(&mut self, id: &str) -> &mut Buffer {
        for b in self.buffers.iter_mut() {
            if b.to_id() == id {
                return b;
            }
        }
        unreachable!()
    }

    pub fn take_buffer(&mut self, id: &str) -> Option<Buffer> {
        let i = {
            let iter = self.buffers.iter().enumerate();
            loop {
                match iter.next() {
                    Some((i, b)) if b.to_id() == id => break Some(i),
                    None => break None
                    _ => ()
                }
            }
        };
        match i {
            Some(i) => Some(self.buffers.remove(i)),
            None => None,
        }
    }

    pub fn add_buffer(&mut self, buffer: Buffer) {
        self.buffers.insert(0, buffer)
    }

    pub fn to_window_cursor(&self) -> Cursor {
        self.app.as_window().to_cursor()
    }
}

// Terminal coordinates, describes the four corners of a window.
// Origin is at (1, 1).
#[derive(Clone, Debug)]
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
#[derive(Clone, Default, Copy, Debug)]
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

// Span object to render on screen.
#[derive(Clone)]
pub struct Span {
    text: String,
    fg: Option<Color>,
    bg: Option<Color>,
    attr: Option<Attribute>,
    cursor: Option<Cursor>,
}

impl Span {
    pub fn new(text: &str) -> Span {
        Span {
            text: text.to_string(),
            fg: Default::default(),
            bg: Default::default(),
            attr: Default::default(),
            cursor: Default::default(),
        }
    }

    pub fn set_cursor(&mut self, cursor: Cursor) -> &mut Self {
        self.cursor = Some(cursor);
        self
    }

    pub fn set_fg(&mut self, fg: Color) -> &mut Self {
        self.fg = Some(fg);
        self
    }

    pub fn set_bg(&mut self, bg: Color) -> &mut Self {
        self.bg = Some(bg);
        self
    }

    pub fn set_attr(&mut self, attr: Attribute) -> &mut Self {
        self.attr = Some(attr);
        self
    }
}

impl Command for Span {
    type AnsiType = String;

    fn ansi_code(&self) -> Self::AnsiType {
        let mut s = match &self.cursor {
            Some(Cursor { col, row }) => {
                //
                term_cursor::MoveTo(*col, *row).to_string()
            }
            None => Default::default(),
        };
        s.push_str(&{
            let mut ss = style::style(&self.text);
            if let Some(bg) = &self.bg {
                ss = ss.on(*bg);
            }
            if let Some(fg) = &self.fg {
                ss = ss.with(*fg);
            }
            if let Some(attr) = &self.attr {
                ss = ss.attribute(*attr);
            }
            ss.to_string()
        });

        s
    }
}
