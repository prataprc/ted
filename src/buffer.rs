use lazy_static::lazy_static;
use log::trace;
use ropey::{self, Rope};

use std::{
    cell::{self, RefCell},
    cmp,
    convert::TryFrom,
    ffi, fmt, io,
    iter::FromIterator,
    rc::{self, Rc},
    result,
    sync::Mutex,
};

use crate::{
    config::Config,
    event::Event,
    {err_at, Error, Result},
};

// TODO: review for saturating_add and saturating_sub in all modules.

const NEW_LINE_CHAR: char = '\n';

#[macro_export]
macro_rules! parse_n {
    ($xs:expr) => {
        err_at!(
            FailConvert,
            String::from_iter($xs.drain(..)).parse::<usize>()
        )
    };
}

// Buffer mode.
#[derive(Clone)]
pub enum Mode {
    Normal,
    Insert,
}

impl TryFrom<String> for Mode {
    type Error = Error;

    fn try_from(s: String) -> Result<Mode> {
        match s.as_str() {
            "normal" => Ok(Mode::Normal),
            "insert" => Ok(Mode::Insert),
            mode => err_at!(FailConvert, msg: format!("invalid mode {}", mode)),
        }
    }
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        match self {
            Mode::Normal => write!(f, "normal"),
            Mode::Insert => write!(f, "insert"),
        }
    }
}

// all bits and pieces of content is managed by buffer.
#[derive(Clone)]
pub struct Buffer {
    mode: Mode,

    location: Location,
    config: Config,
    read_only: bool,

    partial_evnt: Option<Event>,
    find_char: Option<Event>,
    change: Rc<RefCell<Change>>,
}

impl Default for Buffer {
    fn default() -> Buffer {
        Buffer {
            mode: Mode::Normal,

            location: Default::default(),
            config: Default::default(),
            read_only: false,

            partial_evnt: None,
            find_char: None,
            change: Default::default(),
        }
    }
}

impl Buffer {
    pub fn from_reader<R>(data: R, config: Config) -> Result<Buffer>
    where
        R: io::Read,
    {
        let buf = err_at!(FailBuffer, Rope::from_reader(data))?;
        // trace!("first {:p}", &buf);
        Ok(Buffer {
            mode: Mode::Normal,

            location: Default::default(),
            config,
            read_only: false,

            partial_evnt: None,
            find_char: None,
            change: Change::start(buf),
        })
    }

    pub fn empty(config: Config) -> Result<Buffer> {
        let buf = vec![];
        Self::from_reader(buf.as_slice(), config)
    }

    pub fn set_read_only(&mut self, read_only: bool) -> &mut Self {
        self.read_only = read_only;
        self
    }

    pub fn set_location(&mut self, loc: Location) -> &mut Self {
        self.location = loc;
        self
    }

    pub fn set_cursor(&mut self, cursor: usize) -> &mut Self {
        self.as_mut_change().set_cursor(cursor);
        self
    }

    pub fn as_change(&self) -> cell::Ref<Change> {
        self.change.as_ref().borrow()
    }

    pub fn as_mut_change(&mut self) -> cell::RefMut<Change> {
        self.change.as_ref().borrow_mut()
    }
}

impl Buffer {
    pub fn to_string(&self) -> String {
        self.as_change().as_ref().to_string()
    }

    pub fn to_location(&self) -> Location {
        self.location.clone()
    }

    pub fn to_id(&self) -> String {
        match &self.location {
            Location::Anonymous(s) => s.clone(),
            Location::Disk(s) => s.to_str().unwrap().to_string(),
        }
    }
}

impl Buffer {
    pub fn to_cursor(&self) -> usize {
        self.as_change().to_cursor()
    }

    pub fn to_xy_cursor(&self) -> (usize, usize) {
        self.as_change().to_xy_cursor()
    }

    pub fn handle_event(&mut self, evnt: Event) -> Result<Option<Event>> {
        match self.mode {
            Mode::Normal => self.handle_normal_event(evnt),
            Mode::Insert => self.handle_insert_event(evnt),
        }
    }

    fn handle_normal_prefix(&mut self, evnt: Event) -> Result<Option<Event>> {
        use Event::GotoPercent;
        use Event::{Backspace, Char, GotoCol, Left, PartialN, Right};
        use Event::{Down, DownA, FChar, GotoRowA, PartialG, TChar, Up, UpA};

        let m = evnt.to_modifiers();
        let (pe, e) = match self.partial_evnt.take() {
            None if m.is_empty() => match evnt {
                Char(ch, _) if '0' <= ch && ch <= '9' => (
                    //
                    Some(PartialN(vec![ch])),
                    None,
                ),
                evnt => (None, Some(evnt)),
            },
            Some(FChar(n, _, d)) if m.is_empty() => match evnt {
                Char(ch, _) => (None, Some(FChar(n, Some(ch), d))),
                _ => (None, None),
            },
            Some(TChar(n, _, d)) if m.is_empty() => match evnt {
                Char(ch, _) => (None, Some(TChar(n, Some(ch), d))),
                _ => (None, None),
            },
            Some(PartialN(mut xs)) if m.is_empty() => match evnt {
                Backspace(n) => {
                    let n = parse_n!(xs)?.saturating_add(n);
                    (None, Some(Left(n, false)))
                }
                Char(ch, _) if '0' <= ch && ch <= '9' => {
                    xs.push(ch);
                    (Some(PartialN(xs)), None)
                }
                Char('h', _) => (None, Some(Left(parse_n!(xs)?, true))),
                Char('l', _) => (None, Some(Right(parse_n!(xs)?, true))),
                Char('k', _) => (None, Some(Up(parse_n!(xs)?))),
                Char('j', _) => (None, Some(Down(parse_n!(xs)?))),
                Char('-', _) => (None, Some(UpA(parse_n!(xs)?))),
                Char('+', _) => (None, Some(DownA(parse_n!(xs)?))),
                Char(' ', _) => (None, Some(Right(parse_n!(xs)?, false))),
                Char('|', _) => (None, Some(GotoCol(parse_n!(xs)?))),
                Char('f', _) => (Some(FChar(parse_n!(xs)?, None, true)), None),
                Char('F', _) => (Some(FChar(parse_n!(xs)?, None, false)), None),
                Char('t', _) => (Some(TChar(parse_n!(xs)?, None, true)), None),
                Char('T', _) => (Some(TChar(parse_n!(xs)?, None, false)), None),
                Char(';', _) if self.find_char.is_some() => {
                    let m = parse_n!(xs)?;
                    let e = match self.find_char.clone().unwrap() {
                        FChar(_, None, _) => None,
                        FChar(_, Some(ch), d) => Some(FChar(m, Some(ch), d)),
                        TChar(_, None, _) => None,
                        TChar(_, Some(ch), d) => Some(FChar(m, Some(ch), d)),
                        _ => err_at!(Fatal, msg: format!("unreachable"))?,
                    };
                    (None, e)
                }
                Char(';', _) => (None, None),
                Char(',', _) if self.find_char.is_some() => {
                    let m = parse_n!(xs)?;
                    let e = match self.find_char.clone().unwrap() {
                        FChar(_, None, _) => None,
                        FChar(_, Some(ch), d) => Some(FChar(m, Some(ch), !d)),
                        TChar(_, None, _) => None,
                        TChar(_, Some(ch), d) => Some(FChar(m, Some(ch), !d)),
                        _ => err_at!(Fatal, msg: format!("unreachable"))?,
                    };
                    (None, e)
                }
                Char(',', _) => (None, None),
                Char('G', _) => (None, Some(GotoRowA(parse_n!(xs)?))),
                Char('g', _) => (Some(PartialG(parse_n!(xs)?)), None),
                Char('%', _) => (Some(GotoPercent(parse_n!(xs)?)), None),
                evnt @ Char('0', _) => (None, Some(evnt)),
                evnt @ Char('^', _) => (None, Some(evnt)),
                evnt => (Some(PartialN(xs)), Some(evnt)),
            },
            Some(PartialG(n)) if m.is_empty() => match evnt {
                Char('g', _) => (None, Some(GotoRowA(n))),
                _ => (None, Some(evnt)),
            },
            pe => (pe, Some(evnt)),
        };

        self.partial_evnt = pe;
        Ok(e)
    }

    fn handle_normal_event(&mut self, mut evnt: Event) -> Result<Option<Event>> {
        use Event::{Backspace, Char, FChar, GotoCol, Insert, Left, Right, TChar};
        use Event::{Down, DownA, GotoPercent, GotoRowA, Up, UpA};

        evnt = match self.handle_normal_prefix(evnt)? {
            Some(evnt) => evnt,
            None => return Ok(None),
        };

        let m = evnt.to_modifiers();
        match evnt {
            Char('i', _) if m.is_empty() => {
                self.mode = Mode::Insert;
                Ok(None)
            }
            Insert if m.is_empty() => {
                self.mode = Mode::Insert;
                Ok(None)
            }
            Left(n, lbnd) if m.is_empty() => {
                self.as_mut_change().move_left(n, lbnd);
                Ok(None)
            }
            Right(n, lbnd) if m.is_empty() => {
                self.as_mut_change().move_right(n, lbnd);
                Ok(None)
            }
            Backspace(n) if m.is_empty() => {
                self.as_mut_change().move_left(n, true /*line_bound*/);
                Ok(None)
            }
            GotoCol(n) if m.is_empty() => {
                self.as_mut_change().goto_column(n);
                Ok(None)
            }
            FChar(n, Some(ch), d) if d => {
                self.as_mut_change().next_char_n(n, ch, false /*till*/);
                Ok(None)
            }
            FChar(n, Some(ch), _) => {
                self.as_mut_change().prev_char_n(n, ch, false /*till*/);
                Ok(None)
            }
            FChar(_, _, _) => Ok(None),
            TChar(n, Some(ch), d) if d => {
                self.as_mut_change().next_char_n(n, ch, true /*till*/);
                Ok(None)
            }
            TChar(n, Some(ch), _) => {
                self.as_mut_change().prev_char_n(n, ch, true /*till*/);
                Ok(None)
            }
            TChar(_, _, _) => Ok(None),
            Up(n) => {
                self.as_mut_change().move_up(n);
                Ok(None)
            }
            Down(n) => {
                self.as_mut_change().move_down(n);
                Ok(None)
            }
            UpA(n) => {
                if self.as_mut_change().move_up(n) {
                    self.as_mut_change().home_a();
                }
                Ok(None)
            }
            DownA(n) => {
                if self.as_mut_change().move_down(n) {
                    self.as_mut_change().home_a();
                }
                Ok(None)
            }
            GotoRowA(n) => {
                if self.as_mut_change().goto_row(n) {
                    self.as_mut_change().home_a();
                }
                Ok(None)
            }
            GotoPercent(n) if n <= 100 => {
                self.as_mut_change().goto_percentage(n);
                Ok(None)
            }
            GotoPercent(_) => Ok(None),
            Char('h', _) if m.is_empty() => {
                self.as_mut_change().move_left(1, true /*line_bound*/);
                Ok(None)
            }
            Char('l', _) if m.is_empty() => {
                self.as_mut_change().move_right(1, true /*line_bound*/);
                Ok(None)
            }
            Char('j', _) if m.is_empty() => {
                self.as_mut_change().move_up(1);
                Ok(None)
            }
            Char('k', _) if m.is_empty() => {
                self.as_mut_change().move_down(1);
                Ok(None)
            }
            Char(' ', _) if m.is_empty() => {
                self.as_mut_change().move_right(1, true /*line_bound*/);
                Ok(None)
            }
            Char('0', _) if m.is_empty() => {
                self.as_mut_change().home();
                Ok(None)
            }
            Char('^', _) if m.is_empty() => {
                self.as_mut_change().home_a();
                Ok(None)
            }
            Char('|', _) if m.is_empty() => {
                self.as_mut_change().goto_column(1);
                Ok(None)
            }
            evnt => Ok(Some(evnt)),
        }
    }

    fn handle_insert_event(&mut self, evnt: Event) -> Result<Option<Event>> {
        use Event::{BackTab, Backspace, Char, Delete, Down, End, Enter};
        use Event::{Esc, Home, Insert, Left, Noop, PageDown, PageUp};
        use Event::{Right, Tab, Up, F};

        match evnt.clone() {
            Char(ch, _) => {
                self.change = Change::to_next_change(&mut self.change);
                self.as_mut_change().insert_char(ch);
                Ok(None)
            }
            Backspace(n) => {
                self.change = Change::to_next_change(&mut self.change);
                self.as_mut_change().backspace(n);
                Ok(None)
            }
            Enter => {
                self.change = Change::to_next_change(&mut self.change);
                self.as_mut_change().insert_char(NEW_LINE_CHAR);
                Ok(None)
            }
            Tab => {
                self.change = Change::to_next_change(&mut self.change);
                self.as_mut_change().insert_char('\t');
                Ok(None)
            }
            Delete => {
                self.change = Change::to_next_change(&mut self.change);
                self.as_mut_change().remove_at();
                Ok(None)
            }
            Left(n, lbnd) => {
                self.as_mut_change().move_left(n, lbnd);
                Ok(None)
            }
            Right(n, lbnd) => {
                self.as_mut_change().move_right(n, lbnd);
                Ok(None)
            }
            Up(n) => {
                self.as_mut_change().move_up(n);
                Ok(None)
            }
            Down(n) => {
                self.as_mut_change().move_down(n);
                Ok(None)
            }
            Home => {
                self.as_mut_change().home();
                Ok(None)
            }
            End => {
                self.as_mut_change().end();
                Ok(None)
            }
            Esc => {
                self.mode = Mode::Normal;
                Ok(None)
            }
            F(_, _) => Ok(Some(evnt)),
            BackTab | Insert | PageUp | PageDown | Noop => Ok(Some(evnt)),
            _ => todo!(),
        }
    }
}

#[derive(Clone)]
pub struct Change {
    buf: Rope,
    parent: Option<rc::Weak<RefCell<Change>>>,
    children: Vec<Rc<RefCell<Change>>>,
    cursor: usize,
}

impl Default for Change {
    fn default() -> Change {
        let bytes: Vec<u8> = vec![];

        Change {
            buf: Rope::from_reader(bytes.as_slice()).unwrap(),
            parent: None,
            children: Default::default(),
            cursor: 0,
        }
    }
}

impl From<Rope> for Change {
    fn from(buf: Rope) -> Change {
        Change {
            buf,
            parent: None,
            children: Default::default(),
            cursor: 0,
        }
    }
}

impl AsRef<Rope> for Change {
    fn as_ref(&self) -> &Rope {
        &self.buf
    }
}

impl AsMut<Rope> for Change {
    fn as_mut(&mut self) -> &mut Rope {
        &mut self.buf
    }
}

impl Change {
    fn start(buf: Rope) -> Rc<RefCell<Change>> {
        Rc::new(RefCell::new(Change {
            buf,
            parent: None,
            children: Default::default(),
            cursor: 0,
        }))
    }

    fn to_next_change(prev: &mut Rc<RefCell<Change>>) -> Rc<RefCell<Change>> {
        let next = Rc::new(RefCell::new(Change {
            buf: prev.borrow().as_ref().clone(),
            parent: None,
            children: Default::default(),
            cursor: prev.borrow().cursor,
        }));

        next.borrow_mut().children.push(Rc::clone(prev));
        prev.borrow_mut().parent = Some(Rc::downgrade(&next));

        next
    }

    pub fn to_cursor(&self) -> usize {
        self.cursor
    }

    pub fn to_xy_cursor(&self) -> (usize, usize) {
        let row_at = self.buf.char_to_line(self.cursor);
        let col_at = self.cursor - self.buf.line_to_char(row_at);
        (col_at, row_at)
    }
}

impl Change {
    fn set_cursor(&mut self, cursor: usize) -> &mut Self {
        self.cursor = cursor;
        self
    }

    fn insert_char(&mut self, ch: char) {
        // trace!("insert char {} {:p}", ch, &self.buf);
        self.buf.insert_char(self.cursor, ch);
        self.cursor += 1;
    }

    fn backspace(&mut self, n: usize) {
        if self.cursor > 0 {
            let cursor = self.cursor.saturating_sub(n);
            self.buf.remove(cursor..self.cursor);
        }
    }

    fn remove_at(&mut self) {
        if self.cursor < self.buf.len_chars() {
            self.buf.remove(self.cursor..=self.cursor);
        }
    }
}

impl Change {
    fn goto_column(&mut self, n: usize) {
        let cs: Vec<char> = self.buf.chars_at(self.cursor).take(n).collect();
        self.cursor += cs.len();
    }

    fn goto_row(&mut self, n: usize) -> bool {
        let line_idx = self.buf.char_to_line(self.cursor);
        match (n, self.buf.len_lines()) {
            (_, 0) => false,
            (n, _) if n < line_idx => {
                self.move_up(line_idx - n);
                true
            }
            (n, n_lines) if n < n_lines => {
                self.move_down(n - line_idx);
                true
            }
            _ => false,
        }
    }

    fn goto_percentage(&mut self, n: usize) -> bool {
        assert!(n <= 100);

        let line_idx = self.buf.char_to_line(self.cursor);
        match (n, self.buf.len_lines()) {
            (_, 0) => false,
            (n, mut n_lines) => {
                n_lines -= 1;
                let n = (((n_lines as f64) * (n as f64)) / (100 as f64)) as usize;
                if n < line_idx {
                    self.move_up(line_idx - n)
                } else {
                    self.move_down(n - line_idx)
                }
            }
        }
    }

    fn next_char_n(&mut self, n: usize, ch: char, till: bool) {
        let cs: Vec<(usize, char)> = {
            let iter = self.buf.chars_at(self.cursor).enumerate();
            iter.take_while(|(_, c)| *c != NEW_LINE_CHAR).collect()
        };
        let mut occurences: Vec<usize> = cs
            .into_iter()
            .filter_map(|(i, c)| if_else!(c == ch, Some(i), None))
            .take(n)
            .collect();
        let off = match occurences.pop() {
            Some(off) if till => off.saturating_sub(1),
            Some(off) => off,
            None => 0,
        };
        let cursor = self.cursor.saturating_add(off);
        let ln = self.buf.len_chars();
        self.cursor = if_else!(cursor < ln, cursor, ln);
    }

    fn prev_char_n(&mut self, n: usize, ch: char, till: bool) {
        let cs: Vec<(usize, char)> = {
            let iter = ReverseIter {
                iter: self.buf.chars_at(self.cursor),
            };
            iter.enumerate()
                .take_while(|(_, c)| *c != NEW_LINE_CHAR)
                .collect()
        };
        let mut occurences: Vec<usize> = cs
            .into_iter()
            .filter_map(|(i, c)| if_else!(c == ch, Some(i), None))
            .take(n)
            .collect();
        let off = match occurences.pop() {
            Some(off) if till => off.saturating_sub(1),
            Some(off) => off,
            None => 0,
        };
        self.cursor = self.cursor.saturating_sub(off);
    }

    fn move_left(&mut self, n: usize, line_bound: bool) {
        self.cursor = if line_bound {
            let line_idx = self.buf.char_to_line(self.cursor);
            let new_cursor = self.cursor.saturating_sub(n);
            if_else!(new_cursor > line_idx, new_cursor, line_idx)
        } else {
            self.cursor.saturating_sub(n)
        };
    }

    fn move_right(&mut self, n: usize, line_bound: bool) {
        let iter = self.buf.chars_at(self.cursor).take(n);
        let cs: Vec<char> = if line_bound {
            iter.take_while(|ch| *ch != NEW_LINE_CHAR).collect()
        } else {
            iter.collect()
        };
        self.cursor += cs.len()
    }

    fn move_up(&mut self, n: usize) -> bool {
        match self.buf.char_to_line(self.cursor) {
            0 => false,
            line_idx => {
                let line_idx = line_idx.saturating_sub(n);
                self.cursor = {
                    let col = cmp::min(
                        self.buf.line(line_idx).len_chars().saturating_sub(1),
                        self.to_col(),
                    );
                    bounded_num_op!(
                        self.buf.line_to_char(line_idx) + col,
                        self.buf.len_chars().saturating_sub(1)
                    )
                };
                true
            }
        }
    }

    fn move_down(&mut self, n: usize) -> bool {
        let col = self.to_col();

        match (self.buf.char_to_line(self.cursor), self.buf.len_lines()) {
            (_, 0) => false,
            (line_idx, n_lines) if line_idx == n_lines => false,
            (line_idx, n_lines) => {
                let line_idx = bounded_num_op!(
                    //
                    line_idx.saturating_add(n),
                    n_lines - 1
                );
                self.cursor = {
                    let col = cmp::min(
                        //
                        self.buf.line(line_idx).len_chars().saturating_sub(1),
                        col,
                    );
                    bounded_num_op!(
                        self.buf.line_to_char(line_idx) + col,
                        self.buf.len_chars().saturating_sub(1)
                    )
                };
                true
            }
        }
    }

    fn home(&mut self) {
        self.cursor = self.buf.line_to_char(self.buf.char_to_line(self.cursor));
    }

    fn home_a(&mut self) {
        self.home();
        let n = self
            .buf
            .chars_at(self.cursor)
            .take_while(|ch| ch.is_whitespace())
            .fold(0, |n, _| n + 1);
        self.cursor = bounded_num_op!(self.cursor + n, self.buf.len_chars());
    }

    fn end(&mut self) {
        let mut iter = self.buf.chars();
        for ch in iter.next() {
            if ch == NEW_LINE_CHAR {
                break;
            }
            self.cursor += 1;
        }
    }
}

impl Change {
    pub fn lines_at(&self, line_idx: usize) -> ropey::iter::Lines {
        self.buf.lines_at(line_idx)
    }

    fn to_col(&self) -> usize {
        let a_char = self.buf.line_to_char(self.buf.char_to_line(self.cursor));
        self.cursor - a_char
    }
}

// Location of buffer's content, typically a persistent medium.
#[derive(Clone)]
pub enum Location {
    Anonymous(String),
    Disk(ffi::OsString),
}

lazy_static! {
    static ref ANONYMOUS_COUNT: Mutex<usize> = Mutex::new(0);
}

impl Location {
    fn new_anonymous() -> Location {
        let mut count = ANONYMOUS_COUNT.lock().unwrap();
        *count = *count + 1;
        Location::Anonymous(format!("anonymous-{}", count))
    }

    fn new_disk(loc: &ffi::OsStr) -> Location {
        Location::Disk(loc.to_os_string())
    }
}

impl Default for Location {
    fn default() -> Location {
        Location::new_anonymous()
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        match self {
            Location::Anonymous(s) => write!(f, "{}", s),
            Location::Disk(s) => {
                let s = s.clone().into_string().unwrap();
                write!(f, "{}", s)
            }
        }
    }
}

struct ReverseIter<I, T>
where
    I: Iterator<Item = T>,
{
    iter: I,
}

impl<'a> Iterator for ReverseIter<ropey::iter::Chars<'a>, char> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        self.iter.prev()
    }
}
