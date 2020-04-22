use lazy_static::lazy_static;
use log::trace;
use ropey::{self, Rope};

use std::{
    cell::{self, RefCell},
    cmp,
    convert::TryFrom,
    ffi, fmt, io,
    iter::FromIterator,
    ops::Bound,
    rc::{self, Rc},
    result,
    sync::Mutex,
};

use crate::{
    config::Config,
    event::Event,
    {err_at, Error, Result},
};

const NEW_LINE_CHAR: char = '\n';

#[macro_export]
macro_rules! parse_n {
    ($xs:expr) => {
        err_at!(
            FailConvert,
            String::from_iter($xs.drain(..)).parse::<usize>()
        )?
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
        use Event::{Backspace, Char, Left, PartialN, Right};

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
            Some(PartialN(mut xs)) if m.is_empty() => match evnt {
                Char(ch, _) if '0' <= ch && ch <= '9' => {
                    xs.push(ch);
                    (Some(PartialN(xs)), None)
                }
                Char('h', _) => (None, Some(Left(parse_n!(xs), true))),
                Backspace(n) => (None, Some(Left(parse_n!(xs) + n, false))),
                Char('l', _) => (None, Some(Right(parse_n!(xs), true))),
                Char(' ', _) => (None, Some(Right(parse_n!(xs), false))),
                evnt => (Some(PartialN(xs)), Some(evnt)),
            },
            pe => (pe, Some(evnt)),
        };

        self.partial_evnt = pe;
        Ok(e)
    }

    fn handle_normal_event(&mut self, mut evnt: Event) -> Result<Option<Event>> {
        use Event::{Backspace, Char, Insert, Left, Right};

        evnt = match self.handle_normal_prefix(evnt)? {
            Some(evnt) => evnt,
            None => return Ok(None),
        };

        let m = evnt.to_modifiers();
        match (evnt, m.is_empty()) {
            (Insert, true) => {
                self.mode = Mode::Insert;
                Ok(None)
            }
            (Left(n, lbnd), true) => {
                self.as_mut_change().move_left(n, lbnd);
                Ok(None)
            }
            (Right(n, lbnd), true) => {
                self.as_mut_change().move_right(n, lbnd);
                Ok(None)
            }
            (Backspace(n), true) => {
                self.as_mut_change().move_left(n, true /*line_bound*/);
                Ok(None)
            }
            (Char('i', _), true) => {
                self.mode = Mode::Insert;
                Ok(None)
            }
            (Char('h', _), true) => {
                self.as_mut_change().move_left(1, true /*line_bound*/);
                Ok(None)
            }
            (Char('l', _), true) => {
                self.as_mut_change().move_right(1, true /*line_bound*/);
                Ok(None)
            }
            (Char(' ', _), true) => {
                self.as_mut_change().move_right(1, true /*line_bound*/);
                Ok(None)
            }
            (Char('0', _), true) => {
                self.as_mut_change().home();
                Ok(None)
            }
            (Char('^', _), true) => {
                self.as_mut_change().home_non_blank();
                Ok(None)
            }
            (evnt, _) => Ok(Some(evnt)),
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

    fn move_up(&mut self, mut n: usize) {
        let col = self.to_col();

        let line_idx = {
            let mut line_idx = self.buf.char_to_line(self.cursor);
            let mut lines = self.to_lines();
            loop {
                match lines.prev() {
                    Some(_) if n > 0 => {
                        line_idx -= 1;
                        n -= 1;
                    }
                    Some(_) => break line_idx,
                    None => break line_idx,
                }
            }
        };
        let col = cmp::min(self.buf.line(line_idx).len_chars(), col);
        self.cursor = self.buf.line_to_char(line_idx) + col;
    }

    fn move_down(&mut self, mut n: usize) {
        let col = self.to_col();

        let line_idx = {
            let mut line_idx = self.buf.char_to_line(self.cursor);
            let mut lines = self.to_lines();
            loop {
                match lines.next() {
                    Some(_) if n > 0 => {
                        line_idx += 1;
                        n -= 1;
                    }
                    Some(_) => break line_idx,
                    None => break line_idx,
                }
            }
        };
        let col = cmp::min(self.buf.line(line_idx).len_chars(), col);
        self.cursor = self.buf.line_to_char(line_idx) + col;
    }

    fn home(&mut self) {
        self.cursor = self.buf.line_to_char(self.buf.char_to_line(self.cursor));
    }

    fn home_non_blank(&mut self) {
        self.home();
        let n = self
            .buf
            .chars_at(self.cursor)
            .take_while(|ch| ch.is_whitespace())
            .fold(0, |n, _| n + 1);
        self.cursor = self.cursor + n;
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
    fn to_lines(&self) -> ropey::iter::Lines {
        let line_idx = self.buf.char_to_line(self.cursor);
        self.buf.lines_at(line_idx)
    }

    pub fn lines_at(&self, line_idx: usize) -> ropey::iter::Lines {
        self.buf.lines_at(line_idx)
    }

    fn to_col(&self) -> usize {
        let a_char = self.buf.line_to_char(self.buf.char_to_line(self.cursor));
        self.cursor - a_char
    }
}

struct TabfixIter<'a> {
    change: cell::Ref<'a, Change>,
    from: Bound<usize>,
    to: Bound<usize>,
    tabstop: String,
}

impl<'a> Iterator for TabfixIter<'a> {
    type Item = (usize, String);

    fn next(&mut self) -> Option<Self::Item> {
        use std::ops::Bound::{Included, Unbounded};

        let r: &Rope = self.change.as_ref();
        let n_lines = r.len_lines();
        match (self.from, self.to) {
            (Included(from), Unbounded) if from < n_lines => {
                // TODO: can this replace be made in-place
                self.from = Included(from + 1);
                let l = r.line(from).to_string().replace('\t', &self.tabstop);
                Some((from + 1, l))
            }
            (Included(from), Included(to)) if from < n_lines && from <= to => {
                self.from = Included(from + 1);
                // TODO: can this replace be made in-place
                let l = r.line(from).to_string().replace('\t', &self.tabstop);
                Some((from + 1, l))
            }
            _ => None,
        }
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
