use crossterm::{
    cursor,
    style::{self, Attribute, Color},
    Command,
};

use std::{fmt, result};

use crate::{Buffer, Config, Event, Result};

pub trait Window {
    fn to_origin(&self) -> (u16, u16);

    fn to_cursor(&self) -> Cursor;

    fn handle_event(&mut self, ctxt: &mut Context, evnt: Event) -> Result<Option<Event>>;

    fn refresh(&mut self, ctxt: &mut Context) -> Result<()>;
}

// Application context.
pub struct Context {
    pub buffers: Vec<Buffer>,
    pub config: Config,
}

impl Context {
    pub fn new(config: Config) -> Context {
        Context {
            buffers: Default::default(),
            config,
        }
    }

    pub fn as_mut_buffer(&mut self, id: &str) -> Option<&mut Buffer> {
        for b in self.buffers.iter_mut() {
            if b.to_id() == id {
                return Some(b);
            }
        }
        None
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
#[derive(Clone, Default, Copy, Debug)]
pub struct Cursor {
    pub col: u16,
    pub row: u16,
}

impl Cursor {
    pub fn new(col: u16, row: u16) -> Cursor {
        Cursor { col, row }
    }
}

impl From<(u16, u16)> for Cursor {
    fn from((col, row): (u16, u16)) -> Cursor {
        Cursor { col, row }
    }
}

impl fmt::Display for Cursor {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "Cursor<col:{} row:{}>", self.col, self.row)
    }
}

// Span object to render on screen.
pub struct Span {
    text: String,
    fg: Color,
    bg: Color,
    attr: Attribute,
    cursor: Cursor,
}

impl Span {
    // Refer to https://jonasjacek.github.io/colors
    const DEFAULT_BG: Color = Color::AnsiValue(0); // Black
    const DEFAULT_FG: Color = Color::AnsiValue(124); // Red3
    const DEFAULT_ATTR: Attribute = Attribute::Bold;

    pub fn new(text: String, cursor: Cursor) -> Span {
        Span {
            text,
            fg: Self::DEFAULT_FG,
            bg: Self::DEFAULT_BG,
            attr: Self::DEFAULT_ATTR,
            cursor,
        }
    }

    pub fn set_fg(&mut self, fg: Color) -> &mut Self {
        self.fg = fg;
        self
    }

    pub fn set_bg(&mut self, bg: Color) -> &mut Self {
        self.bg = bg;
        self
    }

    pub fn set_attr(&mut self, attr: Attribute) -> &mut Self {
        self.attr = attr;
        self
    }
}

impl Command for Span {
    type AnsiType = String;

    fn ansi_code(&self) -> Self::AnsiType {
        let mut s = cursor::MoveTo(self.cursor.col, self.cursor.row).to_string();
        s.push_str(
            &style::style(&self.text)
                .on(self.bg)
                .with(self.fg)
                .attribute(self.attr)
                .to_string(),
        );
        s
    }
}
