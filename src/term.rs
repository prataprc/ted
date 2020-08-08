//! Module manages all things related to terminal.

use crossterm::{self, Command};
use lazy_static::lazy_static;
#[allow(unused_imports)]
use log::{debug, trace, warn};

use std::{
    convert::{TryFrom, TryInto},
    fmt,
    io::{self, Write},
    iter::FromIterator,
    result,
    sync::Mutex,
};

use crate::{
    text,
    window::{Coord, Cursor},
    Error, Result,
};

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
macro_rules! termbg {
    ($bg:expr) => {{
        use crossterm::{queue, style::SetBackgroundColor};
        use std::io::Write;
        use std::ops::DerefMut;

        let bg = SetBackgroundColor($bg.into());

        let mut tm = err_at!(Fatal, crate::term::TERM.lock())?;
        err_at!(Fatal, queue!(&mut tm.deref_mut().buf, bg))
    }};
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
        #[allow(unused_imports)]
        use log::warn;
        use std::io::{self, Write};
        use std::ops::DerefMut;

        let mut tm = err_at!(Fatal, crate::term::TERM.lock())?;
        let buf = &mut tm.deref_mut().buf;

        let move_to: MoveTo = $cur.into();
        let res = err_at!(Fatal, execute!(buf, move_to, Show));

        let mut stdout = io::stdout();
        err_at!(IOError, stdout.write(buf))?;
        err_at!(IOError, stdout.flush())?;

        debug!("screen buffer {}", tm.buf.len());
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
            buf: Vec::default(),
        }
    }
}

impl Terminal {
    /// Initialize the terminal.
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

    #[inline]
    pub fn to_screen_coord(&self) -> Coord {
        Coord::new(1, 1, self.rows, self.cols)
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
#[derive(Clone, Eq, PartialEq)]
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
#[derive(Clone, Eq, PartialEq)]
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

impl<'a> TryFrom<&'a String> for Color {
    type Error = Error;

    fn try_from(color: &'a String) -> Result<Color> {
        use std::iter::repeat;
        let from_str_radix = u8::from_str_radix;

        let n = color.len();
        let color = match color.as_str() {
            "reset" => Color::Reset,
            "black" => Color::Black,
            "grey" => Color::Grey,
            "darkgrey" => Color::DarkGrey,
            "dark-grey" => Color::DarkGrey,
            "dark_grey" => Color::DarkGrey,
            "red" => Color::Red,
            "darkred" => Color::DarkRed,
            "dark-red" => Color::DarkRed,
            "dark_red" => Color::DarkRed,
            "green" => Color::Green,
            "darkgreen" => Color::DarkGreen,
            "dark-green" => Color::DarkGreen,
            "dark_green" => Color::DarkGreen,
            "yellow" => Color::Yellow,
            "darkyellow" => Color::DarkYellow,
            "dark-yellow" => Color::DarkYellow,
            "dark_yellow" => Color::DarkYellow,
            "blue" => Color::Blue,
            "darkblue" => Color::DarkBlue,
            "dark-blue" => Color::DarkBlue,
            "dark_blue" => Color::DarkBlue,
            "magenta" => Color::Magenta,
            "darkmagenta" => Color::DarkMagenta,
            "dark-magenta" => Color::DarkMagenta,
            "dark_magenta" => Color::DarkMagenta,
            "cyan" => Color::Cyan,
            "darkcyan" => Color::DarkCyan,
            "dark-cyan" => Color::DarkCyan,
            "dark_cyan" => Color::DarkCyan,
            "white" => Color::White,
            _ if n == 0 => Color::Rgb { r: 0, g: 0, b: 0 },
            color => match color.chars().next() {
                Some('#') if n == 1 => Color::Rgb { r: 0, g: 0, b: 0 },
                Some('#') => {
                    let p = {
                        let r = 6_usize.saturating_sub(n);
                        let iter = repeat('0').take(r);
                        String::from_iter(iter)
                    };
                    let s = p + &color[1..];
                    let r = {
                        let rc = from_str_radix(&s[0..2], 16);
                        err_at!(FailConvert, rc, color)?
                    };
                    let g = {
                        let rc = from_str_radix(&s[2..4], 16);
                        err_at!(FailConvert, rc, color)?
                    };
                    let b = {
                        let rc = from_str_radix(&s[4..6], 16);
                        err_at!(FailConvert, rc, color)?
                    };
                    Color::Rgb { r, g, b }
                }
                Some(_) => {
                    let rc = from_str_radix(color, 10);
                    match err_at!(FailConvert, rc) {
                        Ok(n) => Color::AnsiValue(n),
                        _ => {
                            let rc = from_str_radix(color, 16);
                            Color::AnsiValue(err_at!(FailConvert, rc)?)
                        }
                    }
                }
                None => {
                    let msg = format!("invalid color");
                    err_at!(FailConvert, msg: msg)?
                }
            },
        };
        Ok(color)
    }
}

impl TryFrom<i64> for Color {
    type Error = Error;

    fn try_from(color: i64) -> Result<Color> {
        let color: u8 = err_at!(
            Invalid,
            color.try_into(),
            format!("color value {} > 255", color)
        )?;
        Ok(Color::AnsiValue(color))
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
            AnsiValue(val) => write!(f, "0x{:x}", val),
        }
    }
}

/// Style describes the background-color, foreground-color
/// and display attributes.
#[derive(Clone)]
pub struct Style {
    pub bg: Option<Color>,
    pub fg: Option<Color>,
    pub attrs: Vec<Attribute>,
}

impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        let attrs: Vec<String> = {
            let iter = self.attrs.iter().map(|a| a.to_string());
            iter.collect()
        };
        let fg = self
            .fg
            .as_ref()
            .map(|c| c.to_string())
            .unwrap_or("none".to_string());
        let bg = self
            .bg
            .as_ref()
            .map(|c| c.to_string())
            .unwrap_or("none".to_string());
        let attrs = attrs.join("|");
        write!(f, "Style<fg:{},bg:{},attrs:{}>", fg, bg, attrs)
    }
}

impl Default for Style {
    fn default() -> Style {
        Style {
            fg: None,
            bg: None,
            attrs: Vec::default(),
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
    pub fn from_toml(value: &toml::Value) -> Result<Style> {
        use crate::Error::Invalid;

        let table = {
            let err = Invalid(String::new(), format!("bad style"));
            err_at!(value.as_table().ok_or(err))?
        };

        let mut style: Style = Style::default();
        for (key, val) in table.iter() {
            match key.as_str() {
                "on" | "bg" => style.bg = Some(Style::to_color(val)?),
                "with" | "fg" => style.fg = Some(Style::to_color(val)?),
                "attr" | "attribute" => {
                    style.attrs = match val {
                        toml::Value::String(val) => Style::to_attrs(val)?,
                        val => err_at!(Invalid, msg: format!("bad attr: {}", val))?,
                    }
                }
                _ => (),
            }
        }

        Ok(style)
    }

    pub fn set_fg(&mut self, color: Option<Color>) -> &mut Self {
        self.fg = color;
        self
    }

    pub fn set_bg(&mut self, color: Option<Color>) -> &mut Self {
        self.bg = color;
        self
    }

    pub fn add_attr(&mut self, attr: Attribute) -> &mut Self {
        self.attrs.push(attr);
        self
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
    pub fn to_color(color: &toml::Value) -> Result<Color> {
        let color: Color = match color {
            toml::Value::Integer(val) => (*val).try_into()?,
            toml::Value::String(color) => color.try_into()?,
            color => err_at!(Fatal, msg: format!("unexpected color {}", color))?,
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

        let mut attrs: Vec<Attribute> = Vec::default();
        for item in ss.into_iter() {
            match item.trim() {
                "bold" => attrs.push(Attribute::Bold),
                "underlined" | "underline" => attrs.push(Attribute::Underlined),
                "dim" => attrs.push(Attribute::Dim),
                "italic" => attrs.push(Attribute::Italic),
                "slowblink" | "slow-blink" | "slow_blink" | "blink" => {
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
#[derive(Clone, Eq, PartialEq)]
pub struct Span {
    pub content: String,
    pub fg: Option<Color>,
    pub bg: Option<Color>,
    pub attrs: Vec<Attribute>,
    pub cursor: Option<Cursor>,
}

impl fmt::Debug for Span {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "S({:?})", self.content)
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "S({})", self.content)
    }
}

impl From<String> for Span {
    fn from(text: String) -> Span {
        Span {
            content: text,
            fg: None,
            bg: None,
            attrs: Vec::default(),
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
        self.bg = Some(color);
        self
    }

    /// Set the foreground-color.
    pub fn with(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    /// Set the display attribute.
    pub fn attributes(mut self, attrs: Vec<Attribute>) -> Self {
        self.attrs = attrs;
        self
    }

    /// Set style for this span's content.
    pub fn using(mut self, style: Style) -> Self {
        self.fg = style.fg;
        self.bg = style.bg;
        self.attrs = style.attrs;
        self
    }

    /// Return the display-width for this span.
    #[inline]
    pub fn to_width(&self) -> usize {
        text::width(self.content.chars())
    }

    /// Return the display-width for this span as u16.
    #[inline]
    pub fn to_width_u16(&self) -> Result<u16> {
        err_at!(FailConvert, self.to_width().try_into())
    }

    pub fn is_empty(&self) -> bool {
        self.content.trim().len() == 0
    }

    fn trim_newline(mut self) -> (Span, usize) {
        let (content, n) = text::Format::trim_newline(&self.content);
        self.content = content.to_string();
        (self, n)
    }
}

impl Command for Span {
    type AnsiType = String;

    fn ansi_code(&self) -> Self::AnsiType {
        use crossterm::cursor::MoveTo;
        use crossterm::style::{SetAttribute, SetBackgroundColor, SetForegroundColor};

        let mut s = match &self.cursor {
            Some(Cursor { col, row }) => MoveTo(*col, *row).to_string(),
            None => String::default(),
        };
        match &self.fg {
            Some(fg) => {
                let cmd = SetForegroundColor(fg.clone().into());
                // warn!("fg {} {:?}", cmd.to_string().len(), self.content);
                s.push_str(&cmd.to_string())
            }
            None => (),
        };
        match &self.bg {
            Some(bg) => {
                let cmd = SetBackgroundColor(bg.clone().into());
                // warn!("bg {}", cmd.to_string().len());
                s.push_str(&cmd.to_string())
            }
            None => (),
        };
        for attr in self.attrs.iter() {
            let cmd = SetAttribute(attr.clone().into());
            // warn!("attr {}", cmd.to_string().len());
            s.push_str(&cmd.to_string())
        }
        s.push_str(&self.content);
        // warn!("span {}", s.len());
        s
    }
}

/// Spanline object to render on screen. They are always within the single
/// screen-line and can be padded to the right to cover the entire width of
/// the window's viewport. A spanline can be composed of one or more Span
/// values.
#[derive(Clone, Eq, PartialEq)]
pub struct Spanline {
    spans: Vec<Span>,
    cursor: Option<Cursor>,
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
            spans: Vec::default(),
            cursor: None,
        }
    }
}

impl Spanline {
    /// Spanline may need to be padded with `n_pad` bytes to cover the width
    /// of the window's viewport.
    pub fn right_padding(&mut self, n_pad: u16) -> &mut Self {
        use std::iter::repeat;

        if n_pad > 0 {
            let n = n_pad as usize;
            let span: Span = String::from_iter(repeat(' ').take(n)).into();
            self.spans.push(span)
        }
        self
    }

    pub fn optimize_spans(&mut self, canvas: Style) -> &mut Self {
        // carry forward previous background color to next span.
        let mut bg = canvas.bg.clone();
        for span in self.spans.iter_mut() {
            if span.bg == bg {
                span.bg = None
            } else {
                bg = span.bg.clone()
            }
            // when rendering blank chars, don't bother with foreground
            if span.is_empty() {
                span.fg = None;
            }
        }
        self
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
            None => String::default(),
        };
        for span in self.spans.clone().into_iter() {
            s.push_str(&span.ansi_code());
        }
        s
    }
}
