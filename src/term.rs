//! Module manages all things related to terminal.

use crossterm::{self, style::StyledContent, Command};
use lazy_static::lazy_static;
#[allow(unused_imports)]
use log::{debug, trace};
use unicode_width::UnicodeWidthChar;

use std::{
    convert::TryInto,
    fmt,
    io::{self, Write},
    iter::FromIterator,
    result,
    sync::Mutex,
};

use crate::{text, window::Cursor, Error, Result};

lazy_static! {
    pub(crate) static ref TERM: Mutex<Terminal> = {
        let tm = Terminal::init().unwrap();
        Mutex::new(tm)
    };
}

#[macro_export]
macro_rules! hidecr {
    () => {{
        use crossterm::{cursor::Hide, queue};
        use std::io::Write;
        use std::ops::DerefMut;

        let mut tm = err_at!(Fatal, crate::term::TERM.lock())?;
        err_at!(Fatal, queue!(&mut tm.deref_mut().buf, Hide))
    }};
}

#[macro_export]
macro_rules! curz {
    ($val:expr) => {
        $val.saturating_sub(1)
    };
}

#[macro_export]
macro_rules! termqu {
    ($arg0:expr $(, $args:expr)* $(,)?) => {{
        use crossterm::queue;
        use std::ops::DerefMut;
        use std::io::Write;

        let mut tm = err_at!(Fatal, crate::term::TERM.lock())?;
        err_at!(Fatal, queue!(&mut tm.deref_mut().buf, $arg0, $($args)*))
    }};
}

#[macro_export]
macro_rules! termex {
    ($cur:expr) => {{
        use crossterm::{
            cursor::{MoveTo, Show},
            execute,
        };
        use std::io::{self, Write};
        use std::ops::DerefMut;

        let mut tm = err_at!(Fatal, crate::term::TERM.lock())?;
        let buf = &mut tm.deref_mut().buf;

        let move_to: MoveTo = $cur.into();
        let res = err_at!(Fatal, execute!(buf, move_to, Show));

        let mut stdout = io::stdout();
        err_at!(IOError, stdout.write(buf))?;
        err_at!(IOError, stdout.flush())?;

        tm.buf.truncate(0);
        res
    }};
}

/// Captures the screen and cleans up on exit.
pub struct Terminal {
    /// number of colums on the screen
    pub cols: u16,
    /// number of rows on the screen
    pub rows: u16,
    /// queue-buffer
    pub buf: Vec<u8>,
}

impl From<(u16, u16)> for Terminal {
    fn from((cols, rows): (u16, u16)) -> Terminal {
        Terminal {
            cols,
            rows,
            buf: Default::default(),
        }
    }
}

impl Terminal {
    /// initialize the terminal.
    pub fn init() -> Result<Terminal> {
        use crossterm::terminal::{enable_raw_mode, size, EnterAlternateScreen};
        use crossterm::{cursor::Hide, event::EnableMouseCapture, execute};

        let tm: Terminal = err_at!(Fatal, size())?.into();

        err_at!(Fatal, enable_raw_mode())?;
        err_at!(
            Fatal,
            execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture, Hide)
        )?;
        debug!(
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
        use crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};
        use crossterm::{cursor::Show, event::DisableMouseCapture, execute};

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
    Dim,
    Italic,
    RapidBlink,
    SlowBlink,
    CrossedOut,
    Encircled,
    Framed,
    Reverse,
}

impl From<Attribute> for crossterm::style::Attribute {
    fn from(attr: Attribute) -> crossterm::style::Attribute {
        match attr {
            Attribute::Reset => crossterm::style::Attribute::Reset,
            Attribute::Bold => crossterm::style::Attribute::Bold,
            Attribute::Underlined => crossterm::style::Attribute::Underlined,
            Attribute::Dim => crossterm::style::Attribute::Dim,
            Attribute::Italic => crossterm::style::Attribute::Italic,
            Attribute::RapidBlink => crossterm::style::Attribute::RapidBlink,
            Attribute::SlowBlink => crossterm::style::Attribute::SlowBlink,
            Attribute::CrossedOut => crossterm::style::Attribute::CrossedOut,
            Attribute::Encircled => crossterm::style::Attribute::Encircled,
            Attribute::Framed => crossterm::style::Attribute::Framed,
            Attribute::Reverse => crossterm::style::Attribute::Reverse,
        }
    }
}

impl fmt::Display for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        match self {
            Attribute::Reset => write!(f, "reset"),
            Attribute::Bold => write!(f, "bold"),
            Attribute::Underlined => write!(f, "underlined"),
            Attribute::Dim => write!(f, "dim"),
            Attribute::Italic => write!(f, "italic"),
            Attribute::RapidBlink => write!(f, "rapid-blink"),
            Attribute::SlowBlink => write!(f, "slow-blink"),
            Attribute::CrossedOut => write!(f, "crossed-out"),
            Attribute::Encircled => write!(f, "encircled"),
            Attribute::Framed => write!(f, "framed"),
            Attribute::Reverse => write!(f, "reverse"),
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

        match self.clone() {
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
            Rgb { r, g, b } => {
                let rgb = (r as u32) << 16 | (g as u32) << 8 | (b as u32);
                write!(f, "#{:x}", rgb)
            }
            AnsiValue(val) => write!(f, "{:x}", val),
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
        match attrs.len() {
            0 => write!(f, "{},{}", self.bg, self.fg),
            _ => write!(f, "{},{},{}", self.bg, self.fg, attrs.join("|")),
        }
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
            let err = Invalid(String::new(), format!("bad style"));
            value.as_table().ok_or(err)?
        };

        let mut style: Style = Default::default();
        for (key, value) in table.iter() {
            let value = {
                let msg = format!("bad style key:{:?} value:{:?}", key, value);
                value.as_str().ok_or(Invalid(String::new(), msg))?
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
                    let r = err_at!(FailConvert, from_str_radix(&s[0..2], 16), color)?;
                    let g = err_at!(FailConvert, from_str_radix(&s[2..4], 16), color)?;
                    let b = err_at!(FailConvert, from_str_radix(&s[4..6], 16), color)?;
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
            match item.trim() {
                "bold" => attrs.push(Attribute::Bold),
                "underlined" | "underline" => attrs.push(Attribute::Underlined),
                "dim" => attrs.push(Attribute::Dim),
                "italic" => attrs.push(Attribute::Italic),
                "slowblink" | "slow-blink" | "slow_blink" => {
                    //
                    attrs.push(Attribute::SlowBlink)
                }
                "rapidblink" | "rapid-blink" | "rapid_blink" => {
                    //
                    attrs.push(Attribute::RapidBlink)
                }
                "crossedout" | "crossed-out" | "crossed_out" => {
                    //
                    attrs.push(Attribute::CrossedOut)
                }
                "framed" => attrs.push(Attribute::Framed),
                "encircled" => attrs.push(Attribute::Encircled),
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

impl fmt::Debug for Span {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "S({:?})", self.as_content())
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "S({})", self.as_content())
    }
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

    /// Return the span's content.
    #[inline]
    pub fn as_content(&self) -> &str {
        self.content.content()
    }

    /// Return the display-width for this span.
    #[inline]
    pub fn to_width(&self) -> usize {
        self.content
            .content()
            .chars()
            .filter_map(|ch| ch.width())
            .sum()
    }

    /// Return the display-width for this span as u16.
    #[inline]
    pub fn to_width_u16(&self) -> Result<u16> {
        err_at!(FailConvert, self.to_width().try_into())
    }

    fn trim_newline(mut self) -> (Span, usize) {
        let (content, n) = text::Format::trim_newline(self.as_content());
        let content = content.to_string();
        self.content = StyledContent::new(self.content.style().clone(), content);
        (self, n)
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

/// Spanline object to render on screen. They are always within the single
/// screen-line and can be padded to the right to cover the entire width of
/// the window's viewport. A spanline can be composed of one or more Span
/// values.
#[derive(Clone)]
pub struct Spanline {
    spans: Vec<Span>,
    cursor: Option<Cursor>,
}

impl From<String> for Spanline {
    fn from(s: String) -> Spanline {
        let span: Span = s.into();
        span.into()
    }
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
    /// Spanline may need to be padded with `n_pad` bytes to cover the width
    /// of the window's viewport.
    pub fn right_padding(&mut self, n_pad: u16, style: Style) {
        use std::iter::repeat;

        if n_pad > 0 {
            let n = n_pad as usize;
            let span: Span = String::from_iter(repeat(' ').take(n)).into();
            self.spans.push(span.using(style))
        }
    }
}

impl fmt::Debug for Spanline {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        let spans: Vec<String> = {
            let iter = self.spans.iter().map(|s| format!("{:?}", s));
            iter.collect()
        };
        write!(f, "L({:?})", spans.join(","))
    }
}

impl fmt::Display for Spanline {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        let spans: Vec<String> = {
            let iter = self.spans.iter().map(|s| format!("{:?}", s));
            iter.collect()
        };
        write!(f, "L({})", spans.join(","))
    }
}

impl Spanline {
    /// Set the cursor position for this span-line, this is optional.
    #[inline]
    pub fn set_cursor(&mut self, cursor: Cursor) -> &mut Self {
        self.cursor = Some(cursor);
        self
    }

    /// Add a new span value.
    #[inline]
    pub fn add_span(&mut self, span: Span) -> &mut Self {
        self.spans.push(span);
        self
    }

    /// Return whether the span-line is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.spans.len() == 0
    }

    /// Return the character-wise width of the span-line.
    #[inline]
    pub fn to_width(&self) -> usize {
        self.spans.iter().map(|span| span.to_width()).sum()
    }

    /// Apply `style` to the entire span-line, including all of its span areas.
    pub fn using(mut self, style: Style) -> Self {
        self.spans = {
            let iter = self.spans.drain(..);
            iter.map(|span| span.using(style.clone())).collect()
        };
        self
    }

    pub fn trim_newline(&mut self) -> usize {
        match self.spans.pop() {
            Some(span) => {
                let (span, n) = span.trim_newline();
                self.spans.push(span);
                n
            }
            None => 0,
        }
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
