use crossterm::{
    cursor as term_cursor,
    style::{self, Attribute, Color},
    Command,
};

use std::{fmt, mem, ops::Add, result};

use crate::{
    buffer::Buffer, window_edit::WindowEdit, window_file::WindowFile, window_prompt::WindowPrompt,
    Config, Event, Result,
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

// Application state
pub struct State {
    config: Config,
    buffers: Vec<Buffer>,
    window: Window,
}

impl Default for State {
    fn default() -> State {
        let coord: Coord = Default::default();
        State {
            buffers: Default::default(),
            config: Default::default(),
            window: Window::WF(WindowFile::new(coord)),
        }
    }
}

impl AsRef<Config> for State {
    fn as_ref(&self) -> &Config {
        &self.config
    }
}

impl State {
    pub fn new(config: Config, window: Window) -> State {
        State {
            buffers: Default::default(),
            config,
            window: window,
        }
    }

    pub fn take_buffer(&mut self, id: &str) -> Option<Buffer> {
        let i = {
            let mut iter = self.buffers.iter().enumerate();
            loop {
                match iter.next() {
                    Some((i, b)) if b.to_id() == id => break Some(i),
                    None => break None,
                    _ => (),
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

    pub fn to_window_cursor(&self) -> Cursor {
        self.window.to_cursor()
    }
}

impl State {
    pub fn on_event(&mut self, mut evnt: Event) -> Result<Event> {
        let mut window = mem::replace(&mut self.window, Default::default());
        evnt = window.on_event(self, evnt)?;
        self.window = window;
        Ok(evnt)
    }

    pub fn on_refresh(&mut self) -> Result<()> {
        let mut window = mem::replace(&mut self.window, Default::default());
        window.on_refresh(self)?;
        self.window = window;
        Ok(())
    }
}

pub enum Window {
    WF(WindowFile),
    WE(WindowEdit),
    WP(WindowPrompt),
    None,
}

impl Default for Window {
    fn default() -> Window {
        Window::None
    }
}

impl Window {
    fn on_event(&mut self, s: &mut State, evnt: Event) -> Result<Event> {
        match self {
            Window::WF(w) => w.on_event(s, evnt),
            Window::WE(w) => w.on_event(s, evnt),
            Window::WP(w) => w.on_event(s, evnt),
            Window::None => Ok(evnt),
        }
    }

    fn on_refresh(&mut self, s: &mut State) -> Result<()> {
        match self {
            Window::WF(w) => w.on_refresh(s),
            Window::WE(w) => w.on_refresh(s),
            Window::WP(w) => w.on_refresh(s),
            Window::None => Ok(()),
        }
    }

    fn to_cursor(&self) -> Cursor {
        match self {
            Window::WF(w) => w.to_cursor(),
            Window::WE(w) => w.to_cursor(),
            Window::WP(w) => w.to_cursor(),
            Window::None => Default::default(),
        }
    }
}

pub struct Context<'a> {
    pub state: &'a mut State,
    pub buffer: Buffer,
}

impl<'a> AsRef<Buffer> for Context<'a> {
    fn as_ref(&self) -> &Buffer {
        &self.buffer
    }
}

impl<'a> AsRef<State> for Context<'a> {
    fn as_ref(&self) -> &State {
        &self.state
    }
}

impl<'a> AsMut<Buffer> for Context<'a> {
    fn as_mut(&mut self) -> &mut Buffer {
        &mut self.buffer
    }
}

impl<'a> AsMut<State> for Context<'a> {
    fn as_mut(&mut self) -> &mut State {
        &mut self.state
    }
}

impl<'a> Context<'a> {
    pub fn new(s: &'a mut State, buffer: Buffer) -> Context<'a> {
        Context { state: s, buffer }
    }

    #[inline]
    pub fn as_buffer(&self) -> &Buffer {
        &self.buffer
    }

    #[inline]
    pub fn as_mut_buffer(&mut self) -> &mut Buffer {
        &mut self.buffer
    }
}

impl<'a> Context<'a> {
    #[inline]
    pub fn to_event_prefix(&self) -> Event {
        self.as_buffer().to_event_prefix()
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
