use log::trace;
use ropey::{self, Rope};
use unicode_width::UnicodeWidthChar;

use std::{
    cell::{self, RefCell},
    cmp, ffi, io,
    ops::{Bound, RangeBounds},
    rc::{self, Rc},
};

use crate::{
    config::Config,
    event::Event,
    {err_at, Error, Result},
};

const NEW_LINE_CHAR: char = '\n';

#[derive(Clone)]
pub enum State {
    Insert,
    Normal,
}

// all bits and pieces of content is managed by buffer.
#[derive(Clone)]
pub struct Buffer {
    file_loc: ffi::OsString,
    config: Config,

    state: State,
    change: Rc<RefCell<Change>>,
    cursor: usize, // cursor is char_idx into buffer, where next insert happens.
}

impl Default for Buffer {
    fn default() -> Buffer {
        Buffer {
            file_loc: Default::default(),
            config: Default::default(),

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
            file_loc: Default::default(),
            config,

            state: State::Normal,
            change: Change::start(buf),
            cursor: 0,
        })
    }

    pub fn set_file_loc(&mut self, file_loc: &ffi::OsStr) {
        self.file_loc = file_loc.to_os_string()
    }
}

impl Buffer {
    pub fn to_string(&self) -> String {
        self.as_change().as_ref().to_string()
    }

    pub fn to_file_loc(&self) -> ffi::OsString {
        self.file_loc.clone()
    }

    fn as_change(&self) -> cell::Ref<Change> {
        self.change.as_ref().borrow()
    }

    fn as_mut_change(&mut self) -> cell::RefMut<Change> {
        self.change.as_ref().borrow_mut()
    }

    pub fn iter_lines<'a>(
        &'a self,
        from: Bound<usize>,
        to: Bound<usize>,
    ) -> impl Iterator<Item = String> + 'a {
        TabfixIter {
            change: self.as_change(),
            from,
            to,
            tabstop: self.config.tabstop.clone(),
        }
    }

    pub fn change_lines<'a>(&'a self) -> impl Iterator<Item = String> + 'a {
        let (from, to) = self.as_change().change_at;
        self.iter_lines(from, to)
    }
}

impl Buffer {
    pub fn handle_event(&mut self, evnt: Event) -> Result<Res> {
        match self.state {
            State::Normal => self.handle_normal_event(evnt),
            State::Insert => self.handle_insert_event(evnt),
        }
    }

    fn handle_normal_event(&mut self, evnt: Event) -> Result<Res> {
        let (col_at, row_at) = self.update_cursor(self.cursor);
        Ok(Res::new(col_at, row_at, Some(evnt)))
    }

    fn handle_insert_event(&mut self, evnt: Event) -> Result<Res> {
        use Event::{BackTab, Backspace, Char, Delete, Down, End, Enter};
        use Event::{Esc, Home, Insert, Left, Noop, PageDown, PageUp};
        use Event::{Right, Tab, Up, F};

        let cursr = self.cursor;
        let line_idx = self.as_change().char_to_line(cursr);
        let start_idx = self.as_change().line_to_char(line_idx);

        let ((col_at, row_at), evnt) = match evnt.clone() {
            Char(ch, _) => {
                self.change = Change::to_next_change(&mut self.change);
                self.as_mut_change().insert_char(cursr, ch);
                (self.update_cursor(cursr + 1), None)
            }
            Backspace if cursr == 0 => ((0, 0), None),
            Backspace => {
                let new_cursor = cursr - 1;
                self.change = Change::to_next_change(&mut self.change);
                self.as_mut_change().remove(new_cursor..cursr);
                (self.update_cursor(new_cursor), None)
            }
            Enter => {
                self.change = Change::to_next_change(&mut self.change);
                self.as_mut_change().insert_char(cursr, NEW_LINE_CHAR);
                (self.update_cursor(cursr + 1), None)
            }
            Left if start_idx == cursr => (self.update_cursor(cursr), None),
            Left => (self.update_cursor(cursr - 1), None),
            Right => {
                if line_last_char(self.as_change().as_ref(), cursr) == cursr {
                    (self.update_cursor(cursr), None)
                } else {
                    (self.update_cursor(cursr + 1), None)
                }
            }
            Up if cursr == 0 => (self.update_cursor(cursr), None),
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
                    (None, _) => (self.update_cursor(cursr), None),
                    (Some(pline), Some(_)) => {
                        let row_at = line_idx - 1;
                        let col_at = cmp::min(
                            pline.chars().collect::<Vec<char>>().len() - 1,
                            cursr - start_idx,
                        );
                        let new_cursor = self.as_change().line_to_char(row_at);
                        (self.update_cursor(new_cursor + col_at), None)
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
                    (None, _) => (self.update_cursor(cursr), None),
                    (Some(_), None) => (self.update_cursor(cursr), None),
                    (Some(_), Some(nline)) => {
                        let row_at = line_idx + 1;
                        let n = nline.chars().collect::<Vec<char>>().len();
                        let col_at = if n > 0 {
                            cmp::min(n - 1, cursr - start_idx)
                        } else {
                            0
                        };
                        let new_cursor = self.as_change().line_to_char(row_at);
                        (self.update_cursor(new_cursor + col_at), None)
                    }
                }
            }
            Home => (self.update_cursor(start_idx), None),
            End => {
                let new_cursor = line_last_char(self.as_change().as_ref(), cursr);
                (self.update_cursor(new_cursor), None)
            }
            Tab => {
                self.change = Change::to_next_change(&mut self.change);
                self.as_mut_change().insert_char(cursr, '\t');
                (self.update_cursor(cursr + 1), None)
            }
            Delete => {
                if cursr < line_last_char(self.as_change().as_ref(), cursr) {
                    self.change = Change::to_next_change(&mut self.change);
                    self.as_mut_change().remove(cursr..(cursr + 1));
                }
                (self.update_cursor(cursr), None)
            }
            F(_, _) | BackTab | Insert | PageUp | PageDown | Noop | Esc => {
                (self.update_cursor(cursr), Some(evnt))
            }
        };

        Ok(Res::new(col_at, row_at, evnt))
    }

    fn update_cursor(&mut self, new_cursor: usize) -> (usize, usize) {
        let (col_at, row_at) = {
            let row_at = self.as_change().char_to_line(new_cursor);
            let col_at = new_cursor - self.as_change().line_to_char(row_at);
            match self.as_change().lines_at(row_at).next() {
                Some(line) => {
                    let a_col_at: usize = line
                        .to_string()
                        .chars()
                        .take(col_at)
                        .map(|ch| match ch {
                            '\t' => 4,
                            ch => ch.width().unwrap(),
                        })
                        .sum();
                    (a_col_at, row_at)
                }
                None => (col_at, row_at),
            }
        };

        self.cursor = new_cursor;
        (col_at, row_at)
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
    trace!("line_last_char {} {} {}", start_idx, chars.len(), n);
    start_idx + chars.len() - n
}

pub struct Res {
    pub col_at: usize, // starts from ZERO
    pub row_at: usize, // starts from ZERO
    pub evnt: Option<Event>,
}

impl Res {
    #[inline]
    fn new(col_at: usize, row_at: usize, evnt: Option<Event>) -> Res {
        Res {
            col_at,
            row_at,
            evnt,
        }
    }
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
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let r: &Rope = self.change.as_ref();
        let n_lines = r.len_lines();
        let (line, from) = match (self.from, self.to) {
            (Bound::Included(from), Bound::Unbounded) if from < n_lines => {
                (Some(r.line(from).to_string()), Bound::Included(from + 1))
            }
            (Bound::Included(from), Bound::Included(to)) if from <= to => {
                (Some(r.line(from).to_string()), Bound::Included(from + 1))
            }
            (from, _) => (None, from),
        };

        self.from = from;

        match line {
            // TODO: can this replace be made in-place
            Some(line) => Some(line.replace('\t', &self.tabstop)),
            None => None,
        }
    }
}
