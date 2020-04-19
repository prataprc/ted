use lazy_static::lazy_static;
use ropey::{self, Rope};
use unicode_width::UnicodeWidthChar;

use std::{
    cell::{self, RefCell},
    cmp, ffi, fmt, io,
    ops::{Bound, RangeBounds},
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

// Buffer state.
#[derive(Clone)]
pub enum State {
    Normal,
    Insert,
}

// all bits and pieces of content is managed by buffer.
#[derive(Clone)]
pub struct Buffer {
    location: Location,
    config: Config,
    read_only: bool,

    state: State,
    change: Rc<RefCell<Change>>,
    cursor: usize, // cursor is char_idx into buffer, where next insert happens.
}

impl Default for Buffer {
    fn default() -> Buffer {
        Buffer {
            location: Default::default(),
            config: Default::default(),
            read_only: false,

            state: State::Normal,
            change: Default::default(),
            cursor: 0,
        }
    }
}

impl Buffer {
    pub fn from_reader<R>(data: R, config: Config) -> Result<Buffer>
    where
        R: io::Read,
    {
        let buf = err_at!(FailBuffer, Rope::from_reader(data))?;
        Ok(Buffer {
            location: Default::default(),
            config,
            read_only: false,

            state: State::Normal,
            change: Change::start(buf),
            cursor: 0,
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
        self.cursor = cursor;
        self
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

    fn as_change(&self) -> cell::Ref<Change> {
        self.change.as_ref().borrow()
    }

    fn as_mut_change(&mut self) -> cell::RefMut<Change> {
        self.change.as_ref().borrow_mut()
    }

    pub fn to_lines<'a>(
        &'a self,
        from: Bound<usize>,
        to: Bound<usize>,
    ) -> impl Iterator<Item = (usize, String)> + 'a {
        TabfixIter {
            change: self.as_change(),
            from,
            to,
            tabstop: self.config.tabstop.clone(),
        }
    }

    pub fn to_changed_lines<'a>(
        //
        &'a self,
    ) -> impl Iterator<Item = (usize, String)> + 'a {
        let (from, to) = self.as_change().change_at;
        self.to_lines(from, to)
    }
}

impl Buffer {
    pub fn visual_cursor(&self) -> (usize, usize) {
        let tabstop = self.config.tabstop.len(); // TODO: account for unicode
        let row_at = self.as_change().char_to_line(self.cursor);
        let col_at = self.cursor - self.as_change().line_to_char(row_at);
        match self.as_change().lines_at(row_at).next() {
            Some(line) => {
                let a_col_at: usize = line
                    .to_string()
                    .chars()
                    .take(col_at)
                    .map(|ch| match ch {
                        '\t' => tabstop,
                        ch => ch.width().unwrap(),
                    })
                    .sum();
                (a_col_at, row_at)
            }
            None => (col_at, row_at),
        }
    }

    pub fn handle_event(&mut self, evnt: Event) -> Result<Option<Event>> {
        match self.state {
            State::Normal => self.handle_normal_event(evnt),
            State::Insert => self.handle_insert_event(evnt),
        }
    }

    fn handle_normal_event(&mut self, evnt: Event) -> Result<Option<Event>> {
        use Event::{Char, Insert};

        match evnt.clone() {
            Insert => {
                self.state = State::Insert;
                Ok(None)
            }
            Char('i', m) if m.is_empty() => {
                self.state = State::Insert;
                Ok(None)
            }
            _ => Ok(Some(evnt)),
        }
    }

    fn handle_insert_event(&mut self, evnt: Event) -> Result<Option<Event>> {
        use Event::{BackTab, Backspace, Char, Delete, Down, End, Enter};
        use Event::{Esc, Home, Insert, Left, Noop, PageDown, PageUp};
        use Event::{Right, Tab, Up, F};

        let cursr = self.cursor;
        let line_idx = self.as_change().char_to_line(cursr);
        let start_idx = self.as_change().line_to_char(line_idx);

        match evnt.clone() {
            Char(ch, _) => {
                self.change = Change::to_next_change(&mut self.change);
                self.as_mut_change().insert_char(cursr, ch);
                self.cursor = cursr + 1;
                Ok(None)
            }
            Backspace if cursr == 0 => Ok(None),
            Backspace => {
                let new_cursor = cursr - 1;
                self.change = Change::to_next_change(&mut self.change);
                self.as_mut_change().remove(new_cursor..cursr);
                self.cursor = new_cursor;
                Ok(None)
            }
            Enter => {
                self.change = Change::to_next_change(&mut self.change);
                self.as_mut_change().insert_char(cursr, NEW_LINE_CHAR);
                self.cursor = cursr + 1;
                Ok(None)
            }
            Left if start_idx == cursr => Ok(None),
            Left => {
                self.cursor = cursr - 1;
                Ok(None)
            }
            Right => {
                let new_cursor = {
                    let c = self.as_change();
                    line_last_char(c.as_ref(), cursr)
                };
                self.cursor = if new_cursor == cursr {
                    cursr
                } else {
                    cursr + 1
                };
                Ok(None)
            }
            Up if cursr == 0 => Ok(None),
            Up => {
                let (prev_line, cur_line) = {
                    let change = self.as_change();
                    let mut lines = change.lines_at(line_idx);
                    (
                        lines.prev().map(|x| x.to_string()),
                        lines.next().map(|x| x.to_string()),
                    )
                };
                match (prev_line, cur_line) {
                    (None, _) => Ok(None),
                    (Some(pline), Some(_)) => {
                        let row_at = line_idx - 1;
                        let col_at = cmp::min(
                            pline.chars().collect::<Vec<char>>().len() - 1,
                            cursr - start_idx,
                        );
                        let new_cursor = self.as_change().line_to_char(row_at);
                        self.cursor = new_cursor + col_at;
                        Ok(None)
                    }
                    _ => err_at!(Fatal, msg: format!("unreachable"))?,
                }
            }
            Down => {
                let (cur_line, next_line) = {
                    let change = self.as_change();
                    let mut lines = change.lines_at(line_idx);
                    (
                        lines.next().map(|x| x.to_string()),
                        lines.next().map(|x| x.to_string()),
                    )
                };
                match (cur_line, next_line) {
                    (None, _) | (Some(_), None) => Ok(None),
                    (Some(_), Some(nline)) => {
                        let row_at = line_idx + 1;
                        let n = nline.chars().collect::<Vec<char>>().len();
                        let col_at = if n > 0 {
                            cmp::min(n - 1, cursr - start_idx)
                        } else {
                            0
                        };
                        let new_cursor = self.as_change().line_to_char(row_at);
                        self.cursor = new_cursor + col_at;
                        Ok(None)
                    }
                }
            }
            Home => {
                self.cursor = start_idx;
                Ok(None)
            }
            End => {
                self.cursor = {
                    let c = self.as_change();
                    line_last_char(c.as_ref(), cursr)
                };
                Ok(None)
            }
            Tab => {
                self.change = Change::to_next_change(&mut self.change);
                self.as_mut_change().insert_char(cursr, '\t');
                self.cursor = cursr + 1;
                Ok(None)
            }
            Delete => {
                let new_cursor = {
                    let c = self.as_change();
                    line_last_char(c.as_ref(), cursr)
                };
                if cursr < new_cursor {
                    self.change = Change::to_next_change(&mut self.change);
                    self.as_mut_change().remove(cursr..(cursr + 1));
                }
                Ok(None)
            }
            Esc => {
                self.state = State::Normal;
                Ok(None)
            }
            F(_, _) | BackTab | Insert | PageUp | PageDown | Noop => {
                //
                Ok(Some(evnt))
            }
            _ => todo!(),
        }
    }
}

fn line_last_char(buf: &Rope, cursor: usize) -> usize {
    let line_idx = buf.char_to_line(cursor);
    let start_idx = buf.line_to_char(line_idx);
    let line = buf.line(line_idx);
    let chars: Vec<char> = line.chars().collect();
    let mut iter = chars.iter().rev();
    let n = match (iter.next(), iter.next()) {
        (Some('\n'), Some('\r')) => 2,
        (Some('\r'), Some('\n')) => 2,
        (Some('\n'), _) => 1,
        _ => 0,
    };
    start_idx + chars.len() - n
}

#[derive(Clone)]
struct Change {
    buf: Rope,
    change_at: (Bound<usize>, Bound<usize>),
    parent: Option<rc::Weak<RefCell<Change>>>,
    children: Vec<Rc<RefCell<Change>>>,
}

impl Default for Change {
    fn default() -> Change {
        let bytes: Vec<u8> = vec![];

        Change {
            buf: Rope::from_reader(bytes.as_slice()).unwrap(),
            change_at: (Bound::Unbounded, Bound::Unbounded),
            parent: None,
            children: Default::default(),
        }
    }
}

impl From<Rope> for Change {
    fn from(buf: Rope) -> Change {
        Change {
            buf,
            change_at: (Bound::Unbounded, Bound::Unbounded),
            parent: None,
            children: Default::default(),
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
            change_at: (Bound::Unbounded, Bound::Unbounded),
            parent: None,
            children: Default::default(),
        }))
    }

    fn to_next_change(prev: &mut Rc<RefCell<Change>>) -> Rc<RefCell<Change>> {
        let next = Rc::new(RefCell::new(Change {
            buf: prev.borrow().as_ref().clone(),
            change_at: (Bound::Unbounded, Bound::Unbounded),
            parent: None,
            children: Default::default(),
        }));

        next.borrow_mut().children.push(Rc::clone(prev));
        prev.borrow_mut().parent = Some(Rc::downgrade(&next));

        next
    }
}

impl Change {
    fn insert_char(&mut self, cursor: usize, ch: char) {
        let line_idx = if ch == NEW_LINE_CHAR {
            self.buf.char_to_line(cursor) + 1
        } else {
            self.buf.char_to_line(cursor)
        };
        self.change_at = (Bound::Included(line_idx), Bound::Included(line_idx));
        self.buf.insert_char(cursor, ch)
    }

    fn remove<R>(&mut self, char_range: R)
    where
        R: RangeBounds<usize>,
    {
        let line_idx = match char_range.start_bound() {
            Bound::Excluded(char_idx) => self.buf.char_to_line(*char_idx),
            Bound::Included(char_idx) => self.buf.char_to_line(*char_idx + 1),
            Bound::Unbounded => 0,
        };
        self.change_at = (Bound::Included(line_idx), Bound::Included(line_idx));
        self.buf.remove(char_range)
    }
}

impl Change {
    fn char_to_line(&self, cursor: usize) -> usize {
        self.buf.char_to_line(cursor)
    }

    fn line_to_char(&self, cursor: usize) -> usize {
        self.buf.char_to_line(cursor)
    }

    fn lines_at(&self, line_idx: usize) -> ropey::iter::Lines {
        self.buf.lines_at(line_idx)
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
