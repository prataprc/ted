//! Module manages all things related to terminal.

use crossterm::{self, execute, queue, style::StyledContent, Command};
use log::trace;
use unicode_width::UnicodeWidthChar;

use std::{
    fmt,
    io::{self, Write},
    iter::FromIterator,
    result,
};

use crate::{window::Cursor, Error, Result};

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

/// Attribute details for terminal text.
#[derive(Clone)]
pub enum Attribute {
    Reset,
    Bold,
    Underlined,
    Reverse,
}

impl From<Attribute> for crossterm::style::Attribute {
    fn from(attr: Attribute) -> crossterm::style::Attribute {
        use Attribute::{Bold, Reset, Reverse, Underlined};

        match attr {
            Reset => crossterm::style::Attribute::Reset,
            Bold => crossterm::style::Attribute::Bold,
            Underlined => crossterm::style::Attribute::Underlined,
            Reverse => crossterm::style::Attribute::Reverse,
        }
    }
}

impl fmt::Display for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        use Attribute::{Bold, Reset, Reverse, Underlined};

        match self {
            Reset => write!(f, "reset"),
            Bold => write!(f, "bold"),
            Underlined => write!(f, "underlined"),
            Reverse => write!(f, "reverse"),
        }
    }
}

/// Color details for terminal text.
#[derive(Clone)]
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

/// Style describes the background-color, foreground-color
/// and display attributes.
#[derive(Clone)]
pub struct Style {
    pub bg: Color,
    pub fg: Color,
    pub attrs: Vec<Attribute>,
}

impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        let attrs: Vec<String> = {
            let iter = self.attrs.iter().map(|a| a.to_string());
            iter.collect()
        };
        write!(f, "fg:{},bg:{},attrs:{}", self.bg, self.fg, attrs.join("|"))
    }
}

impl Default for Style {
    fn default() -> Style {
        Style {
            fg: Color::White,
            bg: Color::Black,
            attrs: Default::default(),
        }
    }
}

impl Style {
    /// Create a style description from toml value. Eg: toml format.
    ///
    /// { on: <color>, with: <color>, attr: <attr> }
    ///
    /// * "on" and "bg" are treated as equivalent.
    /// * "with" and "fg" are treated as equivalent.
    /// * "attr" and "attribute" are treated as equivalent.
    /// * for <color> refer to [to_color] method for details.
    /// * for <attr> refer to [to_color] method for details.
    pub fn from_toml(value: &toml::Value, canvas: &Style) -> Result<Style> {
        use crate::Error::Invalid;

        let table = {
            let err = Invalid(format!("bad style"));
            value.as_table().ok_or(err)?
        };

        let mut style: Style = Default::default();
        for (key, value) in table.iter() {
            let value = {
                let msg = format!("bad style key:{:?} value:{:?}", key, value);
                value.as_str().ok_or(Invalid(msg))?
            };
            match key.as_str() {
                "on" | "bg" => style.bg = Style::to_color(value, canvas)?,
                "with" | "fg" => style.fg = Style::to_color(value, canvas)?,
                "attr" | "attribute" => style.attrs = Style::to_attrs(value)?,
                _ => (),
            }
        }

        Ok(style)
    }

    /// Can be one of the the following literal.
    ///
    /// * reset, black, grey, darkgrey, dark-grey, dark_grey,
    /// * red, darkred, dark-red, dark_red,
    /// * green, darkgreen, dark-green, dark_green,
    /// * yellow, darkyellow, dark-yellow, dark_yellow,
    /// * blue, darkblue, dark-blue, dark_blue,
    /// * magenta, darkmagenta, dark-magenta, dark_magenta,
    /// * cyan, darkcyan, dark-cyan, dark_cyan,
    /// * white
    /// * bg-canvas, use the canvas' background color,
    /// * fg-canvas, use the canvas' foreground color,
    pub fn to_color(color: &str, canvas: &Style) -> Result<Color> {
        use std::iter::repeat;
        let from_str_radix = u8::from_str_radix;

        let n = color.len();
        let color = match color {
            "reset" => Color::Reset,
            "black" => Color::Black,
            "grey" => Color::Grey,
            "darkgrey" | "dark-grey" | "dark_grey" => Color::DarkGrey,
            "red" => Color::Red,
            "darkred" | "dark-red" | "dark_red" => Color::DarkRed,
            "green" => Color::Green,
            "darkgreen" | "dark-green" | "dark_green" => Color::DarkGreen,
            "yellow" => Color::Yellow,
            "darkyellow" | "dark-yellow" | "dark_yellow" => Color::DarkYellow,
            "blue" => Color::Blue,
            "darkblue" | "dark-blue" | "dark_blue" => Color::DarkBlue,
            "magenta" => Color::Magenta,
            "darkmagenta" | "dark-magenta" | "dark_magenta" => Color::DarkMagenta,
            "cyan" => Color::Cyan,
            "darkcyan" | "dark-cyan" | "dark_cyan" => Color::DarkCyan,
            "white" => Color::White,
            "bg-canvas" => canvas.bg.clone(),
            "fg-canvas" => canvas.fg.clone(),
            _ if n == 0 => Color::Rgb { r: 0, g: 0, b: 0 },
            color => match color.chars().next() {
                Some('#') if n == 1 => Color::Rgb { r: 0, g: 0, b: 0 },
                Some('#') => {
                    let p = {
                        let iter = repeat('0').take(6_usize.saturating_sub(n));
                        String::from_iter(iter)
                    };
                    let s = p + &color[1..];
                    let r = err_at!(FailConvert, from_str_radix(&s[0..2], 16))?;
                    let g = err_at!(FailConvert, from_str_radix(&s[2..4], 16))?;
                    let b = err_at!(FailConvert, from_str_radix(&s[4..6], 16))?;
                    Color::Rgb { r, g, b }
                }
                Some(_) => match err_at!(FailConvert, from_str_radix(color, 10)) {
                    Ok(n) => Color::AnsiValue(n),
                    _ => {
                        let n = err_at!(FailConvert, from_str_radix(color, 16))?;
                        Color::AnsiValue(n)
                    }
                },
                None => err_at!(FailConvert, msg: format!("invalid color"))?,
            },
        };

        Ok(color)
    }

    /// Can be comma-separate or pipe-separated literals:
    ///
    /// * bold, underlined, underline, reverse.
    ///
    /// EG: `bold | underlined` (or) `bold, underlined`.
    pub fn to_attrs(attr: &str) -> Result<Vec<Attribute>> {
        let ss: Vec<&str> = if attr.contains(",") {
            attr.split(",").collect()
        } else if attr.contains("|") {
            attr.split("|").collect()
        } else {
            vec![attr]
        };

        let mut attrs: Vec<Attribute> = Default::default();
        for item in ss.into_iter() {
            match item {
                "bold" => attrs.push(Attribute::Bold),
                "underlined" => attrs.push(Attribute::Underlined),
                "underline" => attrs.push(Attribute::Underlined),
                "reverse" => attrs.push(Attribute::Reverse),
                _ => err_at!(Invalid, msg: format!("invalid attr {:?}", item))?,
            }
        }
        Ok(attrs)
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
            content: crossterm::style::style(text),
            cursor: None,
        }
    }
}

impl Span {
    /// Set the cursor position for this span, this is optional.
    pub fn set_cursor(&mut self, cursor: Cursor) -> &mut Self {
        self.cursor = Some(cursor);
        self
    }

    /// Set the background-color.
    pub fn on(mut self, color: Color) -> Self {
        self.content = self.content.on(color.into());
        self
    }

    /// Set the foreground-color.
    pub fn with(mut self, color: Color) -> Self {
        self.content = self.content.with(color.into());
        self
    }

    /// Set the display attribute.
    pub fn attribute(mut self, attr: Attribute) -> Self {
        self.content = self.content.attribute(attr.into());
        self
    }

    /// Set style for this span's content.
    pub fn using(mut self, style: Style) -> Self {
        let mut content = self
            .content
            .clone()
            .on(style.bg.into())
            .with(style.fg.into());
        for attr in style.attrs.iter() {
            content = content.attribute(attr.clone().into());
        }
        self.content = content;
        self
    }

    /// return the span's content.
    #[inline]
    pub fn to_content(&self) -> String {
        self.content.content().to_string()
    }

    /// return the display-width for this span.
    #[inline]
    pub fn to_width(&self) -> usize {
        self.content
            .content()
            .chars()
            .filter_map(|ch| ch.width())
            .sum()
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

impl From<Span> for Spanline {
    fn from(span: Span) -> Spanline {
        Spanline {
            cursor: None,
            spans: vec![span],
        }
    }
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
    pub fn right_padding(&mut self, n_pad: u16, style: Style) {
        use std::iter::repeat;

        if n_pad > 0 {
            let n = n_pad as usize;
            let span: Span = String::from_iter(repeat(' ').take(n)).into();
            self.spans.push(span.using(style))
        }
    }
}

impl Spanline {
    #[inline]
    pub fn set_cursor(&mut self, cursor: Cursor) -> &mut Self {
        self.cursor = Some(cursor);
        self
    }

    #[inline]
    pub fn add_span(&mut self, span: Span) -> &mut Self {
        self.spans.push(span);
        self
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.spans.len() == 0
    }

    #[inline]
    pub fn to_width(&self) -> usize {
        self.spans.iter().map(|span| span.to_width()).sum()
    }

    pub fn using(mut self, style: Style) -> Self {
        self.spans = {
            let iter = self.spans.drain(..);
            iter.map(|span| span.using(style.clone())).collect()
        };
        self
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
