//! Module manages all things related to terminal.

use crossterm::{execute, queue};
use log::trace;

use std::{
    fmt,
    io::{self, Write},
    result,
};

use crate::{window::Cursor, Error, Result};

/// Captures the screen and cleans up on exit.
pub struct Terminal {
    /// number of colums on the screen
    pub cols: u16,
    /// number of rows on the screen
    pub rows: u16,
}

impl From<(u16, u16)> for Terminal {
    fn from((cols, rows): (u16, u16)) -> Terminal {
        Terminal { cols, rows }
    }
}

impl Terminal {
    /// initialize the terminal.
    pub fn init() -> Result<Terminal> {
        use crossterm::cursor::Hide;
        use crossterm::event::EnableMouseCapture;
        use crossterm::terminal::{enable_raw_mode, size, EnterAlternateScreen};

        let tm: Terminal = err_at!(Fatal, size())?.into();

        err_at!(Fatal, enable_raw_mode())?;
        err_at!(
            Fatal,
            execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture, Hide)
        )?;
        trace!(
            "{} color_count:{}",
            tm,
            crossterm::style::available_color_count()
        );

        Ok(tm)
    }
}

impl fmt::Display for Terminal {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "Terminal<{},{}>", self.cols, self.rows)
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        use crossterm::cursor::Show;
        use crossterm::event::DisableMouseCapture;
        use crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};

        execute!(
            io::stdout(),
            LeaveAlternateScreen,
            DisableMouseCapture,
            Show
        )
        .unwrap();
        disable_raw_mode().unwrap();
    }
}

/// Flush the terminal with cursor position.
#[inline]
pub fn flush(cursor: Cursor) -> Result<()> {
    use crossterm::cursor::{MoveTo, Show};

    let mut stdout = io::stdout();
    let Cursor { col, row } = cursor;
    err_at!(Fatal, queue!(stdout, MoveTo(col, row), Show))?;
    err_at!(Fatal, stdout.flush())?;
    Ok(())
}

/// Hide the cursor, subsequently application shall buffer the changes.
#[inline]
pub fn hide_cursor() -> Result<()> {
    use crossterm::cursor::Hide;

    let mut stdout = io::stdout();
    err_at!(Fatal, queue!(stdout, Hide))?;
    Ok(())
}

pub enum Attribute {
    Reset,
    Bold,
    Dim,
    Underlined,
    Reverse,
    Hidden,
    NoBold,
    NormalIntensity,
    NoUnderline,
    NoBlink,
    NoReverse,
    NoHidden,
}

impl From<Attribute> for crossterm::style::Attribute {
    fn from(attr: Attribute) -> crossterm::style::Attribute {
        use Attribute::NormalIntensity;
        use Attribute::{Bold, Dim, Hidden, Reset, Reverse, Underlined};
        use Attribute::{NoBlink, NoBold, NoHidden, NoReverse, NoUnderline};

        match attr {
            Reset => crossterm::style::Attribute::Reset,
            Bold => crossterm::style::Attribute::Bold,
            Dim => crossterm::style::Attribute::Dim,
            Underlined => crossterm::style::Attribute::Underlined,
            Reverse => crossterm::style::Attribute::Reverse,
            Hidden => crossterm::style::Attribute::Hidden,
            NoBold => crossterm::style::Attribute::NoBold,
            NormalIntensity => crossterm::style::Attribute::NormalIntensity,
            NoUnderline => crossterm::style::Attribute::NoUnderline,
            NoBlink => crossterm::style::Attribute::NoBlink,
            NoReverse => crossterm::style::Attribute::NoReverse,
            NoHidden => crossterm::style::Attribute::NoHidden,
        }
    }
}

impl fmt::Display for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        use Attribute::NormalIntensity;
        use Attribute::{Bold, Dim, Hidden, Reset, Reverse, Underlined};
        use Attribute::{NoBlink, NoBold, NoHidden, NoReverse, NoUnderline};

        match self {
            Reset => write!(f, "reset"),
            Bold => write!(f, "bold"),
            Dim => write!(f, "dim"),
            Underlined => write!(f, "underlined"),
            Reverse => write!(f, "reverse"),
            Hidden => write!(f, "hidden"),
            NormalIntensity => write!(f, "normal-intensity"),
            NoBold => write!(f, "no-bold"),
            NoUnderline => write!(f, "no-underline"),
            NoBlink => write!(f, "no-blink"),
            NoReverse => write!(f, "no-reverse"),
            NoHidden => write!(f, "no-hidden"),
        }
    }
}

pub enum Color {
    Reset,
    Black,
    DarkGrey,
    Red,
    DarkRed,
    Green,
    DarkGreen,
    Yellow,
    DarkYellow,
    Blue,
    DarkBlue,
    Magenta,
    DarkMagenta,
    Cyan,
    DarkCyan,
    White,
    Grey,
    Rgb { r: u8, g: u8, b: u8 },
    AnsiValue(u8),
}

impl From<Color> for crossterm::style::Color {
    fn from(color: Color) -> crossterm::style::Color {
        use Color::Reset;
        use Color::{AnsiValue, DarkCyan, Grey, Rgb, White};
        use Color::{Black, DarkGreen, DarkGrey, DarkRed, Green, Red, Yellow};
        use Color::{Blue, Cyan, DarkBlue, DarkMagenta, DarkYellow, Magenta};

        match color {
            Reset => crossterm::style::Color::Reset,
            Black => crossterm::style::Color::Black,
            DarkGrey => crossterm::style::Color::DarkGreen,
            Red => crossterm::style::Color::Red,
            DarkRed => crossterm::style::Color::DarkRed,
            Green => crossterm::style::Color::Green,
            DarkGreen => crossterm::style::Color::DarkGreen,
            Yellow => crossterm::style::Color::Yellow,
            DarkYellow => crossterm::style::Color::DarkYellow,
            Blue => crossterm::style::Color::Blue,
            DarkBlue => crossterm::style::Color::DarkBlue,
            Magenta => crossterm::style::Color::Magenta,
            DarkMagenta => crossterm::style::Color::DarkMagenta,
            Cyan => crossterm::style::Color::Cyan,
            DarkCyan => crossterm::style::Color::DarkCyan,
            White => crossterm::style::Color::White,
            Grey => crossterm::style::Color::Grey,
            Rgb { r, g, b } => crossterm::style::Color::Rgb { r, g, b },
            AnsiValue(val) => crossterm::style::Color::AnsiValue(val),
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        use Color::Reset;
        use Color::{AnsiValue, DarkCyan, Grey, Rgb, White};
        use Color::{Black, DarkGreen, DarkGrey, DarkRed, Green, Red, Yellow};
        use Color::{Blue, Cyan, DarkBlue, DarkMagenta, DarkYellow, Magenta};

        match self {
            Reset => write!(f, "reset"),
            Black => write!(f, "black"),
            DarkGrey => write!(f, "dark-grey"),
            Red => write!(f, "red"),
            DarkRed => write!(f, "dark-red"),
            Green => write!(f, "green"),
            DarkGreen => write!(f, "dark-green"),
            Yellow => write!(f, "yellow"),
            DarkYellow => write!(f, "dark-yellow"),
            Blue => write!(f, "blue"),
            DarkBlue => write!(f, "dark-blue"),
            Magenta => write!(f, "magenta"),
            DarkMagenta => write!(f, "dark-magenta"),
            Cyan => write!(f, "cyan"),
            DarkCyan => write!(f, "dark-cyan"),
            White => write!(f, "white"),
            Grey => write!(f, "grey"),
            Rgb { r, g, b } => write!(f, "rgb<{},{},{}>", r, g, b),
            AnsiValue(val) => write!(f, "ansi-value<{}>", val),
        }
    }
}
