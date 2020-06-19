//! Module `buffer` implement editing and cursor movement commands
//! over text content.

use lazy_static::lazy_static;
#[allow(unused_imports)]
use log::trace;
use regex::Regex;
use ropey::{self, Rope};

use std::{
    borrow::Borrow,
    cell::{self, RefCell},
    cmp, fmt, io,
    iter::FromIterator,
    mem,
    ops::Bound,
    rc::{self, Rc},
    result,
    sync::Mutex,
};

use crate::{
    event::{Event, Mto, DP},
    location::Location,
    term::{Span, Spanline},
    window::WinBuffer,
    {err_at, Error, Result},
};

/// Newline character supported by this buffer implementation.
pub const NL: char = '\n';

/// Maximum number of lines supported by this buffer implementation.
pub const MAX_LINES: usize = 1_000_000_000;

lazy_static! {
    static ref BUFFER_NUM: Mutex<usize> = Mutex::new(0);
}

/// Cursor within the buffer, where the first row, first column
/// start from (0, 0).
#[derive(Clone, Copy, Default, Debug)]
pub struct Cursor {
    pub col: usize,
    pub row: usize,
}

impl From<(usize, usize)> for Cursor {
    fn from(t: (usize, usize)) -> Cursor {
        Cursor { col: t.0, row: t.1 }
    }
}

impl fmt::Display for Cursor {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "BC<{},{}>", self.col, self.row)
    }
}

impl Cursor {
    /// Compute the difference between two cursor points. If `O` is
    /// old-cursor and `N` is new-cursor then following should hold
    /// true.
    ///
    /// * D = O - N;
    /// * N = O + D;
    #[inline]
    pub fn diff(&self, new: &Self) -> (isize, isize) {
        let dcol = (new.col as isize) - (self.col as isize);
        let drow = (new.row as isize) - (self.row as isize);
        (dcol, drow)
    }
}

impl PartialEq for Cursor {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.col == other.col
    }
}

impl Eq for Cursor {}

impl PartialOrd for Cursor {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        if self.row == other.row {
            self.col.partial_cmp(&other.col)
        } else {
            self.row.partial_cmp(&other.row)
        }
    }
}

impl Ord for Cursor {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        if self.row == other.row {
            self.row.cmp(&other.row)
        } else {
            self.col.cmp(&other.col)
        }
    }
}

// all bits and pieces of content is managed by buffer.
pub struct Buffer {
    /// Source for this buffer, typically a file from local disk.
    pub location: Location,
    /// Mark this buffer read-only, in which case insert ops are not allowed.
    pub read_only: bool,

    // Globally counting buffer number.
    num: usize, // buffer number
    // Buffer states
    inner: Inner,

    // Last search command applied on this buffer.
    mto_pattern: Mto,
    // Last find character (within the line) command  applied on this buffer.
    mto_find_char: Mto,
    // Number of times to repeat an insert operation.
    insert_repeat: usize,
    // Collection of events applied during the last insert session.
    last_inserts: Vec<Event>,
}

#[derive(Clone)]
enum Inner {
    Insert(InsertBuffer),
    Normal(NormalBuffer),
}

impl Default for Inner {
    fn default() -> Inner {
        Inner::Normal(Default::default())
    }
}

/// Create and configure a text buffer.
impl Buffer {
    pub fn from_reader<R>(data: R, loc: Location) -> Result<Buffer>
    where
        R: io::Read,
    {
        let buf = err_at!(FailBuffer, Rope::from_reader(data))?;
        let mut num = BUFFER_NUM.lock().unwrap();
        *num = *num + 1;
        let b = Buffer {
            location: loc,
            read_only: false,

            num: *num,
            inner: Inner::Normal(NormalBuffer::new(buf)),

            mto_pattern: Default::default(),
            mto_find_char: Default::default(),
            insert_repeat: Default::default(),
            last_inserts: Default::default(),
        };

        Ok(b)
    }

    pub fn empty() -> Buffer {
        let buf = vec![];
        Self::from_reader(buf.as_slice(), Default::default()).unwrap()
    }

    pub fn set_cursor(&mut self, cursor: usize) -> &mut Self {
        match &mut self.inner {
            Inner::Normal(val) => {
                val.set_cursor(cursor);
            }
            Inner::Insert(val) => {
                val.set_cursor(cursor);
            }
        };
        self
    }

    pub fn set_location(&mut self, loc: Location) -> &mut Self {
        self.location = loc;
        self
    }

    pub fn set_read_only(&mut self, read_only: bool) -> &mut Self {
        self.read_only = read_only;
        self
    }
}

impl Buffer {
    #[inline]
    fn to_change(&self) -> cell::Ref<Change> {
        match &self.inner {
            Inner::Normal(val) => val.to_change(),
            Inner::Insert(val) => val.to_change(),
        }
    }

    fn to_mut_change(&mut self) -> cell::RefMut<Change> {
        match &mut self.inner {
            Inner::Insert(ib) => ib.to_mut_change(),
            Inner::Normal(nb) => nb.to_mut_change(),
        }
    }
}

impl Buffer {
    /// Return whether buffer is marked read-only.
    #[inline]
    pub fn is_read_only(&self) -> bool {
        self.read_only
    }

    /// Return whether buffer is marked as modified.
    #[inline]
    pub fn is_modified(&self) -> bool {
        let change = self.to_change();
        change.parent.is_some() || !change.children.is_empty()
    }

    /// Return current buffer state as string.
    #[inline]
    pub fn to_mode(&self) -> &'static str {
        match &self.inner {
            Inner::Insert(_) => "insert",
            Inner::Normal(_) => "normal",
        }
    }

    /// Return buffer id, constructed from its location string.
    #[inline]
    pub fn to_id(&self) -> String {
        match self.to_location() {
            Location::Memory(s) => s,
            Location::Disk(s) => s.to_str().unwrap().to_string(),
            Location::Ted(s) => s,
        }
    }

    /// Buffer number, handy for users to rotate between buffers.
    #[inline]
    pub fn to_num(&self) -> usize {
        self.num
    }

    /// Return buffer's location.
    #[inline]
    pub fn to_location(&self) -> Location {
        self.location.clone()
    }
}

impl Buffer {
    /// Return the underlying text, if buffer is really large this can be
    /// a costly operation.
    #[inline]
    pub fn to_string(&self) -> String {
        self.to_change().as_ref().to_string()
    }

    /// Return the cursor position, as character offset, within this buffer.
    #[inline]
    pub fn to_cursor(&self) -> usize {
        self.to_change().to_cursor()
    }

    /// Like `to_xy_cursor` method, but return only the column offset for the
    /// cursor-line.
    pub fn to_col(&self) -> usize {
        self.to_xy_cursor().col
    }

    /// Return the number of text lines in this buffer.
    #[inline]
    pub fn n_lines(&self) -> usize {
        let change = self.to_change();
        change.buf.len_lines()
    }

    /// For the line identified by `line_idx`, starting from 0, return the
    /// length of the line. Note that, `0 <= line_idx < n_lines`.
    #[inline]
    pub fn line_len(&self, line_idx: usize) -> usize {
        let change = self.to_change();
        change.buf.line(line_idx).len_chars()
    }

    /// Similar to `line_to_char` but for the current cursor line.
    #[inline]
    pub fn line_home(&self) -> usize {
        let change = self.to_change();
        change
            .buf
            .line_to_char(change.buf.char_to_line(self.to_cursor()))
    }

    #[inline]
    pub fn char_to_line(&self, char_idx: usize) -> usize {
        self.to_change().buf.char_to_line(char_idx)
    }

    /// Return the byte offset for requested `char_idx`, which must be a valid
    /// character offset within the buffer. [Buffer::to_cursor] is a `char_idx`.
    /// Note that, `0 <= char_idx < n_chars`.
    #[inline]
    pub fn char_to_byte(&self, char_idx: usize) -> usize {
        self.to_change().buf.char_to_byte(char_idx)
    }

    /// Return the character under the requested offset `char_idx`.
    /// Note that `0 <= char_idx < n_chars`.
    #[inline]
    pub fn char(&self, char_idx: usize) -> char {
        let change = self.to_change();
        change.buf.char(char_idx)
    }

    pub fn byte_to_char(&self, byte_idx: usize) -> usize {
        self.to_change().buf.byte_to_char(byte_idx)
    }
}

impl<'a> WinBuffer<'a> for Buffer {
    type IterLine = IterLine<'a>;
    type IterChar = IterChar<'a>;

    fn to_xy_cursor(&self) -> Cursor {
        self.to_change().to_xy_cursor()
    }

    fn lines_at(&'a self, line_idx: usize, dp: DP) -> Result<Self::IterLine> {
        let change = self.to_change();
        let iter = unsafe {
            let cref: &Change = change.borrow();
            (cref as *const Change)
                .as_ref()
                .unwrap()
                .buf
                .lines_at(line_idx)
        };

        match dp {
            DP::Right => Ok(IterLine {
                _change: change,
                iter,
                reverse: false,
            }),
            DP::Left => Ok(IterLine {
                _change: change,
                iter,
                reverse: true,
            }),
            _ => err_at!(Fatal, msg: format!("unreachable")),
        }
    }

    fn chars_at(&'a self, char_idx: usize, dp: DP) -> Result<Self::IterChar> {
        let change = self.to_change();
        let iter = unsafe {
            let cref: &Change = change.borrow();
            let r: &Rope = {
                let c = (cref as *const Change).as_ref().unwrap();
                c.as_ref()
            };
            r.chars_at(char_idx)
        };

        match dp {
            DP::Right => Ok(IterChar {
                _change: Some(change),
                iter,
                reverse: false,
            }),
            DP::Left => Ok(IterChar {
                _change: Some(change),
                iter,
                reverse: true,
            }),
            _ => err_at!(Fatal, msg: format!("unreachable")),
        }
    }

    fn line_to_char(&self, line_idx: usize) -> usize {
        self.to_change().buf.line_to_char(line_idx)
    }

    fn char_to_line(&self, char_idx: usize) -> usize {
        self.to_change().buf.char_to_line(char_idx)
    }

    fn n_chars(&self) -> usize {
        let change = &self.to_change();
        change.buf.len_chars()
    }

    fn n_lines(&self) -> usize {
        let change = &self.to_change();
        change.buf.len_lines()
    }

    fn len_line(&self, line_idx: usize) -> usize {
        let change = &self.to_change();
        change.buf.line(line_idx).len_chars()
    }

    fn is_trailing_newline(&self) -> bool {
        match self.n_chars() {
            0 => false,
            n => self.char(n - 1) == NL,
        }
    }
}

pub fn to_span_line(buf: &Buffer, a: usize, z: usize) -> Result<Spanline> {
    let span: Span = {
        let iter = buf.chars_at(a, DP::Right)?.take(z - a);
        String::from_iter(iter).into()
    };
    Ok(span.into())
}

impl Buffer {
    #[inline]
    pub fn skip_whitespace(&mut self, dp: DP) -> usize {
        self.to_mut_change().skip_whitespace(dp)
    }

    #[inline]
    pub fn skip_alphanumeric(&mut self, dp: DP) -> usize {
        self.to_mut_change().skip_alphanumeric(dp)
    }

    #[inline]
    pub fn skip_non_whitespace(&mut self, dp: DP) -> usize {
        self.to_mut_change().skip_non_whitespace(dp)
    }

    #[inline]
    pub fn cmd_insert_char(&mut self, ch: char) -> Result<()> {
        let change = match &mut self.inner {
            Inner::Normal(nb) => &mut nb.change,
            Inner::Insert(ib) => &mut ib.change,
        };

        *change = Change::to_next_change(change);
        self.to_mut_change().insert_char(ch)
    }

    #[inline]
    pub fn cmd_backspace(&mut self, n: usize) -> Result<()> {
        let change = match &mut self.inner {
            Inner::Normal(nb) => &mut nb.change,
            Inner::Insert(ib) => &mut ib.change,
        };

        *change = Change::to_next_change(change);
        self.to_mut_change().backspace(n)
    }

    #[inline]
    pub fn cmd_remove_at(&mut self, from: Bound<usize>, to: Bound<usize>) -> Result<()> {
        let change = match &mut self.inner {
            Inner::Normal(nb) => &mut nb.change,
            Inner::Insert(ib) => &mut ib.change,
        };

        *change = Change::to_next_change(change);
        self.to_mut_change().remove_at(from, to)
    }
}

impl Buffer {
    pub fn on_event(&mut self, evnt: Event) -> Result<Event> {
        let evnt = match self.to_mode() {
            "insert" => self.handle_i_event(evnt),
            "normal" => self.handle_n_event(evnt),
            _ => err_at!(Fatal, msg: format!("unreachable")),
        }?;

        Ok(evnt)
    }

    pub fn mode_normal(&mut self) {
        self.inner = match mem::replace(&mut self.inner, Default::default()) {
            Inner::Insert(ib) => Inner::Normal(ib.into()),
            inner @ Inner::Normal(_) => inner,
        };
    }

    pub fn mode_insert(&mut self) {
        self.inner = match mem::replace(&mut self.inner, Default::default()) {
            Inner::Normal(nb) => Inner::Insert(nb.into()),
            inner @ Inner::Insert(_) => inner,
        };
    }
}

impl Buffer {
    fn to_insert_n(evnt: Event) -> (Option<usize>, Event) {
        use crate::event::{Event::Md, Mod};

        match evnt {
            Md(Mod::Insert(n, dp)) => (Some(n), Md(Mod::Insert(n, dp))),
            Md(Mod::Append(n, dp)) => (Some(n), Md(Mod::Append(n, dp))),
            Md(Mod::Open(n, dp)) => (Some(n), Md(Mod::Open(n, dp))),
            _ => (None, evnt),
        }
    }

    fn ex_n_insert(&mut self, evnt: Event) -> Result<Event> {
        use crate::event::{Event::Md, Mod};

        let nr = mem::replace(&mut self.inner, Default::default());
        let (inner, evnt) = match nr {
            Inner::Normal(nb) => match evnt {
                Md(Mod::Insert(n, pos)) if n > 0 => {
                    self.insert_repeat = n - 1;
                    if pos == DP::Caret {
                        mto_home(self, DP::Caret)?;
                    }
                    (Inner::Insert(nb.into()), Event::Noop)
                }
                Md(Mod::Append(n, pos)) if n > 0 => {
                    self.insert_repeat = n - 1;
                    if pos == DP::End {
                        mto_end(self)?;
                    }
                    mto_right(self, 1, DP::Nobound)?;
                    (Inner::Insert(nb.into()), Event::Noop)
                }
                Md(Mod::Open(n, DP::Left)) if n > 0 => {
                    self.insert_repeat = n - 1;
                    mto_home(self, DP::Nope)?;
                    self.cmd_insert_char(NL)?;
                    mto_left(self, 1, DP::Nobound)?;
                    (Inner::Insert(nb.into()), Event::Noop)
                }
                Md(Mod::Open(n, DP::Right)) if n > 0 => {
                    self.insert_repeat = n - 1;
                    mto_end(self)?;
                    mto_right(self, 1, DP::Nobound)?;
                    self.cmd_insert_char(NL)?;
                    (Inner::Insert(nb.into()), Event::Noop)
                }
                _ => (Inner::Normal(nb), Event::Noop),
            },
            inner @ Inner::Insert(_) => (inner, evnt),
        };

        self.inner = inner;
        Ok(evnt)
    }

    fn handle_n_event(&mut self, evnt: Event) -> Result<Event> {
        use crate::event::Event::Mt;

        // switch to insert mode.
        let evnt = match Self::to_insert_n(evnt) {
            (Some(n), evnt) if n > 0 => {
                let evnt = self.ex_n_insert(evnt)?;
                return self.handle_i_event(evnt);
            }
            (_, evnt) => evnt,
        };

        let evnt = match evnt {
            Event::Noop => Event::Noop,
            // execute motion command.
            Mt(Mto::Left(n, dp)) => mto_left(self, n, dp)?,
            Mt(Mto::Right(n, dp)) => mto_right(self, n, dp)?,
            Mt(Mto::Up(n, dp)) => mto_up(self, n, dp)?,
            Mt(Mto::Down(n, dp)) => mto_down(self, n, dp)?,
            Mt(Mto::Col(n)) => mto_column(self, n)?,
            Mt(Mto::Home(dp)) => mto_home(self, dp)?,
            Mt(Mto::End) => mto_end(self)?,
            Mt(Mto::Row(n, dp)) => mto_row(self, n, dp)?,
            Mt(Mto::Percent(n)) => mto_percent(self, n)?,
            Mt(Mto::Cursor(n)) => mto_cursor(self, n)?,
            Mt(e @ Mto::CharF(_, _, _)) => {
                self.mto_find_char = e.clone();
                mto_char(self, e)?
            }
            Mt(e @ Mto::CharT(_, _, _)) => {
                self.mto_find_char = e.clone();
                mto_char(self, e)?
            }
            Mt(Mto::CharR(n, dir)) => {
                let e = self.mto_find_char.clone();
                mto_char(self, e.transform(n, dir)?)?
            }
            Mt(e @ Mto::Word(_, _, _)) => mto_words(self, e)?,
            Mt(e @ Mto::WWord(_, _, _)) => mto_wwords(self, e)?,
            Mt(e @ Mto::Sentence(_, _)) => mto_sentence(self, e)?,
            Mt(e @ Mto::Para(_, _)) => mto_para(self, e)?,
            Mt(e @ Mto::Bracket(_, _, _, _)) => mto_bracket(self, e)?,
            Mt(e @ Mto::Pattern(_, Some(_), _)) => {
                self.mto_pattern = e.clone();
                mto_pattern(self, e)?
            }
            Mt(Mto::PatternR(n, dir)) => {
                let e = self.mto_pattern.clone();
                mto_pattern(self, e.transform(n, dir)?)?
            }
            evnt => evnt,
        };

        Ok(evnt)
    }

    fn handle_i_event(&mut self, evnt: Event) -> Result<Event> {
        match evnt {
            Event::Noop => Ok(Event::Noop),
            evnt => {
                self.last_inserts.push(evnt.clone());
                self.ex_i_event(evnt)
            }
        }
    }

    fn ex_i_event(&mut self, evnt: Event) -> Result<Event> {
        use crate::event::Event::{Backspace, Char, Delete, Enter, Esc, Mt, Tab};

        let evnt = match evnt {
            // movement
            Mt(Mto::Left(n, dp)) => mto_left(self, n, dp)?,
            Mt(Mto::Right(n, dp)) => mto_right(self, n, dp)?,
            Mt(Mto::Up(n, dp)) => mto_up(self, n, dp)?,
            Mt(Mto::Down(n, dp)) => mto_down(self, n, dp)?,
            Mt(Mto::Home(dp)) => mto_home(self, dp)?,
            Mt(Mto::End) => mto_end(self)?,
            // Handle mode events.
            Esc => {
                self.repeat()?;
                mto_left(self, 1, DP::LineBound)?;
                self.mode_normal();
                Event::Noop
            }
            // on going insert
            Char(ch, _) => {
                self.cmd_insert_char(ch)?;
                Event::Noop
            }
            Backspace => {
                self.cmd_backspace(1)?;
                Event::Noop
            }
            Enter => {
                self.cmd_insert_char(NL)?;
                Event::Noop
            }
            Tab => {
                self.cmd_insert_char('\t')?;
                Event::Noop
            }
            Delete => {
                let from = Bound::Included(self.to_cursor());
                let to = from.clone();
                self.cmd_remove_at(from, to)?;
                Event::Noop
            }
            evnt => evnt,
        };

        Ok(evnt)
    }

    fn repeat(&mut self) -> Result<()> {
        use crate::event::Event::{Backspace, Char, Delete, Enter, Tab};
        let (last_inserts, insert_repeat) = {
            let evnts: Vec<Event> = self.last_inserts.drain(..).collect();
            let valid = evnts.iter().all(|evnt| match evnt {
                Char(_, _) | Enter | Tab | Backspace | Delete => true,
                _ => false,
            });
            if valid {
                (evnts, self.insert_repeat)
            } else {
                (vec![], self.insert_repeat)
            }
        };

        for _ in 0..insert_repeat {
            for evnt in last_inserts.iter() {
                self.ex_i_event(evnt.clone())?;
            }
        }

        self.insert_repeat = 0;
        self.last_inserts = last_inserts;
        Ok(())
    }
}

#[derive(Clone)]
struct NormalBuffer {
    change: Rc<RefCell<Change>>,
}

impl Default for NormalBuffer {
    fn default() -> NormalBuffer {
        NormalBuffer {
            change: Default::default(),
        }
    }
}

impl From<InsertBuffer> for NormalBuffer {
    fn from(ib: InsertBuffer) -> NormalBuffer {
        NormalBuffer { change: ib.change }
    }
}

impl NormalBuffer {
    fn new(buf: Rope) -> NormalBuffer {
        let mut nb: NormalBuffer = Default::default();
        nb.change = Change::start(buf);
        nb
    }

    fn set_cursor(&mut self, cursor: usize) -> &mut Self {
        self.to_mut_change().set_cursor(cursor);
        self
    }
}

impl NormalBuffer {
    fn to_change(&self) -> cell::Ref<Change> {
        self.change.as_ref().borrow()
    }

    fn to_mut_change(&mut self) -> cell::RefMut<Change> {
        self.change.as_ref().borrow_mut()
    }
}

#[derive(Clone)]
struct InsertBuffer {
    change: Rc<RefCell<Change>>,
}

impl From<NormalBuffer> for InsertBuffer {
    fn from(nb: NormalBuffer) -> InsertBuffer {
        InsertBuffer { change: nb.change }
    }
}

impl Default for InsertBuffer {
    fn default() -> InsertBuffer {
        InsertBuffer {
            change: Default::default(),
        }
    }
}

impl InsertBuffer {
    fn set_cursor(&mut self, cursor: usize) -> &mut Self {
        self.to_mut_change().set_cursor(cursor);
        self
    }
}

impl InsertBuffer {
    fn to_change(&self) -> cell::Ref<Change> {
        self.change.as_ref().borrow()
    }

    fn to_mut_change(&mut self) -> cell::RefMut<Change> {
        self.change.as_ref().borrow_mut()
    }
}

#[derive(Clone)]
struct Change {
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
        let next = {
            let prev_change: &Change = &prev.as_ref().borrow();
            Rc::new(RefCell::new(Change {
                buf: prev_change.as_ref().clone(),
                parent: None,
                children: Default::default(),
                cursor: prev_change.cursor,
            }))
        };

        next.borrow_mut().children.push(Rc::clone(prev));
        prev.borrow_mut().parent = Some(Rc::downgrade(&next));

        next
    }

    fn to_cursor(&self) -> usize {
        self.cursor
    }

    fn to_xy_cursor(&self) -> Cursor {
        let row_at = self.buf.char_to_line(self.cursor);
        let col_at = self.cursor - self.buf.line_to_char(row_at);
        (col_at, row_at).into()
    }
}

impl Change {
    fn set_cursor(&mut self, cursor: usize) -> &mut Self {
        self.cursor = cursor;
        self
    }

    fn insert_char(&mut self, ch: char) -> Result<()> {
        self.buf.insert_char(self.cursor, ch);
        self.cursor += 1;
        Ok(())
    }

    fn backspace(&mut self, n: usize) -> Result<()> {
        if self.cursor > 0 {
            let cursor = self.cursor.saturating_sub(n);
            self.buf.remove(cursor..self.cursor);
        }
        Ok(())
    }

    fn remove_at(&mut self, from: Bound<usize>, to: Bound<usize>) -> Result<()> {
        use std::ops::Bound::{Excluded, Included, Unbounded};

        let n = self.buf.len_chars();
        let from = match from {
            Included(from) => cmp::min(from, n.saturating_sub(1)),
            Excluded(from) => cmp::min(from.saturating_add(1), n),
            Unbounded => 0,
        };
        let to = match to {
            Included(to) => cmp::min(to.saturating_add(1), n),
            Excluded(to) => cmp::min(to, n),
            Unbounded => n,
        };
        if from < to {
            self.buf.remove(from..to);
        }
        Ok(())
    }
}

impl Change {
    fn skip_whitespace(&mut self, dp: DP) -> usize {
        let mut n = 0;
        let n = loop {
            match self.iter(dp).next() {
                Some(ch) if ch.is_whitespace() => n += 1,
                Some(_) => break n,
                None => break n,
            }
        };
        self.cursor = if_else!(dp == DP::Right, self.cursor + n, self.cursor - n);
        n
    }

    fn skip_non_whitespace(&mut self, dp: DP) -> usize {
        let mut n = 0;
        let n = loop {
            match self.iter(dp).next() {
                Some(ch) if ch.is_whitespace() => n += 1,
                Some(_) => break n,
                None => break n,
            }
        };
        self.cursor = if_else!(
            //
            dp == DP::Right,
            self.cursor + n,
            self.cursor - n
        );
        n
    }

    fn skip_alphanumeric(&mut self, dp: DP) -> usize {
        let mut n = 0;
        let n = loop {
            match self.iter(dp).next() {
                Some(ch) if ch.is_alphanumeric() => n += 1,
                Some(_) => break n,
                None => break n,
            }
        };
        self.cursor = if_else!(
            //
            dp == DP::Right,
            self.cursor + n,
            self.cursor - n
        );
        n
    }

    //fn fwd_match_group(&mut self) {
    //    self.cursor = {
    //        let mut iter = self.iter(true /*fwd*/).enumerate();
    //        let res = loop {
    //            match iter.next() {
    //                Some((i, '(')) => break Some((')', i + 1, true)),
    //                Some((i, ')')) => break Some(('(', i, false)),
    //                Some((i, '{')) => break Some(('}', i + 1, true)),
    //                Some((i, '}')) => break Some(('{', i, false)),
    //                Some((i, '<')) => break Some(('>', i + 1, true)),
    //                Some((i, '>')) => break Some(('<', i, false)),
    //                Some((i, '[')) => break Some(('[', i + 1, true)),
    //                Some((i, ']')) => break Some(('[', i, false)),
    //                Some((_, NL)) => break None,
    //                Some(_) => (),
    //                None => break None,
    //            };
    //        };
    //        if let Some((nch, noff, fwd)) = res {
    //            let cursor = self.cursor + noff;
    //            let mut iter = self.iter_at(fwd, cursor).enumerate();
    //            loop {
    //                match iter.next() {
    //                    Some((i, ch)) if ch == nch && fwd => {
    //                        break cursor + i;
    //                    }
    //                    Some((i, ch)) if ch == nch => {
    //                        break cursor - i - 1;
    //                    }
    //                    Some(_) => (),
    //                    None => break cursor,
    //                }
    //            }
    //        } else {
    //            self.cursor
    //        }
    //    };
    //}
}

impl Change {
    fn iter<'a>(&'a self, dp: DP) -> Box<dyn Iterator<Item = char> + 'a> {
        let chars = self.buf.chars_at(self.cursor);
        match dp {
            DP::Left => Box::new(IterChar {
                _change: None,
                iter: chars,
                reverse: true,
            }),
            DP::Right => Box::new(chars),
            _ => unreachable!(),
        }
    }
}

pub fn mto_left(buf: &mut Buffer, n: usize, dp: DP) -> Result<Event> {
    let mut cursor = buf.to_cursor();
    cursor = match dp {
        DP::LineBound => {
            let home = buf.line_home();
            let new_cursor = cursor.saturating_sub(n);
            Ok(if_else!(new_cursor > home, new_cursor, home))
        }
        DP::Nobound => Ok(cursor.saturating_sub(n)),
        _ => err_at!(Fatal, msg: format!("unreachable")),
    }?;

    buf.set_cursor(cursor);
    Ok(Event::Noop)
}

pub fn mto_right(buf: &mut Buffer, n: usize, dp: DP) -> Result<Event> {
    let mut cursor = buf.to_cursor();
    for ch in buf.chars_at(cursor, DP::Right)?.take(n) {
        match dp {
            DP::LineBound if ch == NL => break,
            DP::Nobound | DP::LineBound => (),
            _ => err_at!(Fatal, msg: format!("unreachable"))?,
        }
        cursor += 1
    }

    buf.set_cursor(cursor);
    Ok(Event::Noop)
}

pub fn mto_home(buf: &mut Buffer, pos: DP) -> Result<Event> {
    buf.set_cursor(buf.line_home());
    match pos {
        DP::Caret => {
            buf.skip_whitespace(DP::Right);
        }
        DP::Nope => (),
        _ => err_at!(Fatal, msg: format!("unreachable"))?,
    }
    Ok(Event::Noop)
}

pub fn mto_up(buf: &mut Buffer, n: usize, pos: DP) -> Result<Event> {
    let mut cursor = buf.to_cursor();
    match buf.char_to_line(cursor) {
        0 => Ok(Event::Noop),
        row => {
            let row = row.saturating_sub(n);
            cursor = {
                let col = {
                    let n_chars = buf.line_len(row);
                    cmp::min(n_chars.saturating_sub(2), buf.to_col())
                };
                buf.line_to_char(row) + col
            };
            buf.set_cursor(cursor);
            match pos {
                DP::Caret => mto_home(buf, DP::Caret),
                DP::Nope => Ok(Event::Noop),
                _ => {
                    err_at!(Fatal, msg: format!("unreachable"))?;
                    Ok(Event::Noop)
                }
            }
        }
    }
}

pub fn mto_down(buf: &mut Buffer, n: usize, pos: DP) -> Result<Event> {
    let row = buf.char_to_line(buf.to_cursor());
    match buf.n_lines() {
        0 => Ok(Event::Noop),
        n_rows if row == n_rows => Ok(Event::Noop),
        n_rows => {
            let row = limite!(row.saturating_add(n), n_rows);
            let cursor = {
                let n_chars = buf.line_len(row);
                let col = cmp::min(n_chars.saturating_sub(2), buf.to_col());
                buf.line_to_char(row) + col
            };
            buf.set_cursor(cursor);
            match pos {
                DP::Caret => mto_home(buf, DP::Caret),
                DP::Nope => Ok(Event::Noop),
                _ => {
                    err_at!(Fatal, msg: format!("unreachable"))?;
                    Ok(Event::Noop)
                }
            }
        }
    }
}

pub fn mto_column(buf: &mut Buffer, n: usize) -> Result<Event> {
    let n = {
        let m = {
            let cursor = buf.to_cursor();
            buf.line_len(buf.char_to_line(cursor)).saturating_sub(1)
        };
        cmp::min(m, n).saturating_sub(1)
    };
    buf.set_cursor(buf.line_home() + n);
    Ok(Event::Noop)
}

pub fn mto_row(buf: &mut Buffer, n: usize, pos: DP) -> Result<Event> {
    let row = buf.char_to_line(buf.to_cursor());
    let n = n.saturating_sub(1);
    match buf.n_lines() {
        0 => Ok(Event::Noop),
        n_rows if n == 0 => mto_down(buf, n_rows.saturating_sub(1), pos),
        _ if n < row => mto_up(buf, row - n, pos),
        n_rows if n <= n_rows => mto_down(buf, n - row, pos),
        n_rows => mto_down(buf, n_rows.saturating_sub(1), pos),
    }
}

pub fn mto_percent(buf: &mut Buffer, n: usize) -> Result<Event> {
    let row = buf.char_to_line(buf.to_cursor());
    match buf.n_lines() {
        0 => Ok(Event::Noop),
        mut n_rows if n < 100 => {
            n_rows = n_rows.saturating_sub(1);
            match (((n_rows as f64) * (n as f64)) / (100 as f64)) as usize {
                n if n < row => mto_up(buf, row - n, DP::Caret),
                n => mto_down(buf, n - row, DP::Caret),
            }
        }
        n_rows => mto_down(buf, n_rows.saturating_sub(1), DP::Caret),
    }
}

pub fn mto_cursor(buf: &mut Buffer, n: usize) -> Result<Event> {
    let cursor = buf.to_cursor();
    buf.set_cursor(limite!(cursor + n, buf.n_chars()));
    Ok(Event::Noop)
}

// TODO: create an option of having sticky cursor.
pub fn mto_end(buf: &mut Buffer) -> Result<Event> {
    let mut cursor = buf.to_cursor();
    {
        let mut iter = buf.chars_at(buf.to_cursor(), DP::Right)?;
        loop {
            match iter.next() {
                Some(NL) => break (),
                Some(_) => cursor += 1,
                None => break (),
            }
        }
    }
    buf.set_cursor(cursor);
    Ok(Event::Noop)
}

pub fn mto_char(buf: &mut Buffer, evnt: Mto) -> Result<Event> {
    let (mut n, ch, dp, pos) = match evnt {
        Mto::CharF(n, Some(ch), dp) => (n, ch, dp, DP::Find),
        Mto::CharT(n, Some(ch), dp) => (n, ch, dp, DP::Till),
        Mto::None => return Ok(Event::Noop),
        _ => unreachable!(),
    };

    let mut cursor = buf.to_cursor();
    let home = buf.line_home();
    cursor = match dp {
        DP::Right => {
            let mut iter = buf.chars_at(cursor, DP::Right)?.enumerate();
            loop {
                match iter.next() {
                    Some((_, NL)) => break cursor,
                    Some((i, c)) if c == ch && n == 0 && pos == DP::Find => {
                        break cursor.saturating_add(i);
                    }
                    Some((i, c)) if c == ch && n == 0 => {
                        break cursor.saturating_add(i.saturating_sub(1));
                    }
                    Some((_, c)) if c == ch => n -= 1,
                    _ => (),
                }
            }
        }
        DP::Left => {
            let mut iter = buf.chars_at(cursor, DP::Left)?.enumerate();
            loop {
                match iter.next() {
                    Some((_, NL)) => break cursor,
                    Some((i, c)) if c == ch && n == 0 && pos == DP::Find => {
                        break cursor.saturating_sub(i + 1);
                    }
                    Some((i, c)) if c == ch && n == 0 => {
                        break cursor.saturating_sub(i);
                    }
                    Some((_, c)) if c == ch => n -= 1,
                    _ => (),
                }
            }
        }
        _ => unreachable!(),
    };

    buf.set_cursor(if_else!(cursor > home, cursor, home));
    Ok(Event::Noop)
}

pub fn mto_words(buf: &mut Buffer, evnt: Mto) -> Result<Event> {
    match evnt {
        Mto::Word(n, DP::Left, pos) => {
            for _ in 0..n {
                let n = buf.skip_whitespace(DP::Left);
                match pos {
                    DP::End if n == 0 => {
                        buf.skip_alphanumeric(DP::Left);
                        mto_right(buf, 1, DP::Nobound)?;
                    }
                    DP::End => {
                        buf.skip_alphanumeric(DP::Left);
                        mto_right(buf, 1, DP::Nobound)?;
                    }
                    DP::Start if n == 0 => {
                        buf.skip_alphanumeric(DP::Left);
                        buf.skip_whitespace(DP::Left);
                    }
                    DP::Start => (),
                    _ => unreachable!(),
                }
            }
            Ok(Event::Noop)
        }
        Mto::Word(n, DP::Right, pos) => {
            for _ in 0..n {
                let n = buf.skip_whitespace(DP::Right);
                match pos {
                    DP::End if n == 0 => {
                        buf.skip_alphanumeric(DP::Right);
                        mto_left(buf, 1, DP::Nobound)?;
                    }
                    DP::End => {
                        buf.skip_alphanumeric(DP::Right);
                        mto_left(buf, 1, DP::Nobound)?;
                    }
                    DP::Start if n == 0 => {
                        buf.skip_alphanumeric(DP::Right);
                        buf.skip_whitespace(DP::Right);
                    }
                    DP::Start => (),
                    _ => unreachable!(),
                }
            }
            Ok(Event::Noop)
        }
        _ => err_at!(Fatal, msg: format!("unreachable")),
    }
}

pub fn mto_wwords(buf: &mut Buffer, evnt: Mto) -> Result<Event> {
    match evnt {
        Mto::WWord(n, DP::Left, pos) => {
            for _ in 0..n {
                let n = buf.skip_whitespace(DP::Left);
                match pos {
                    DP::Start if n == 0 => {
                        buf.skip_non_whitespace(DP::Left);
                        mto_right(buf, 1, DP::Nobound)?;
                    }
                    DP::Start => {
                        buf.skip_non_whitespace(DP::Left);
                        mto_right(buf, 1, DP::Nobound)?;
                    }
                    DP::End if n == 0 => {
                        buf.skip_non_whitespace(DP::Left);
                        buf.skip_whitespace(DP::Left);
                    }
                    DP::End => (),
                    _ => unreachable!(),
                }
            }
            Ok(Event::Noop)
        }
        Mto::WWord(n, DP::Right, pos) => {
            for _ in 0..n {
                let n = buf.skip_whitespace(DP::Right);
                match pos {
                    DP::End if n == 0 => {
                        buf.skip_non_whitespace(DP::Right);
                        mto_left(buf, 1, DP::Nobound)?;
                    }
                    DP::End => {
                        buf.skip_non_whitespace(DP::Right);
                        mto_left(buf, 1, DP::Nobound)?;
                    }
                    DP::Start if n == 0 => {
                        buf.skip_non_whitespace(DP::Right);
                        buf.skip_whitespace(DP::Right);
                    }
                    DP::Start => (),
                    _ => unreachable!(),
                }
            }
            Ok(Event::Noop)
        }
        _ => err_at!(Fatal, msg: format!("unreachable")),
    }
}

pub fn mto_sentence(buf: &mut Buffer, e: Mto) -> Result<Event> {
    let is_ws = |ch: char| ch.is_whitespace();

    let mut cursor = buf.to_cursor();
    let mut pch: Option<char> = None;
    cursor = match e {
        Mto::Sentence(mut n, DP::Left) => {
            let mut iter = buf.chars_at(cursor, DP::Left)?.enumerate();
            Ok(loop {
                pch = match (iter.next(), pch) {
                    (Some((i, '.')), Some(pch)) if is_ws(pch) => {
                        if n > 1 {
                            n -= 1;
                        } else {
                            break cursor.saturating_sub(i);
                        }
                        Some('.')
                    }
                    (Some((i, NL)), Some(NL)) => {
                        if n > 1 {
                            n -= 1;
                        } else {
                            break cursor.saturating_sub(i);
                        }
                        Some(NL)
                    }
                    (Some((_, ch)), _) => Some(ch),
                    (None, _) => break 0,
                };
            })
        }
        Mto::Sentence(mut n, DP::Right) => {
            let mut iter = buf.chars_at(cursor, DP::Right)?.enumerate();
            Ok(loop {
                pch = match (pch, iter.next()) {
                    (Some('.'), Some((i, ch))) if is_ws(ch) => {
                        if n > 1 {
                            n -= 1;
                        } else {
                            break cursor.saturating_add(i);
                        }
                        Some('.')
                    }
                    (Some(NL), Some((i, NL))) => {
                        if n > 1 {
                            n -= 1;
                        } else {
                            break cursor.saturating_add(i);
                        }
                        Some(NL)
                    }
                    (_, Some((_, ch))) => Some(ch),
                    (_, None) => {
                        break buf.n_chars().saturating_sub(1);
                    }
                };
            })
        }
        _ => err_at!(Fatal, msg: format!("unreachable")),
    }?;

    buf.set_cursor(cursor);
    buf.skip_whitespace(DP::Right);

    Ok(Event::Noop)
}

pub fn mto_para(buf: &mut Buffer, evnt: Mto) -> Result<Event> {
    let mut cursor = buf.to_cursor();
    let row = buf.char_to_line(cursor);
    cursor = match evnt {
        Mto::Para(mut n, DP::Left) => {
            let mut iter = buf.lines_at(row, DP::Left)?.enumerate();
            let cursor = loop {
                match iter.next() {
                    Some((i, line)) => match line.chars().next() {
                        Some(NL) if n == 0 => {
                            break buf.line_to_char(row - (i + 1));
                        }
                        Some(NL) => n -= 1,
                        Some(_) => (),
                        None => break buf.line_to_char(row - (i + 1)),
                    },
                    None => break 0,
                }
            };
            Ok(cursor)
        }
        Mto::Para(mut n, DP::Right) => {
            let mut iter = buf.lines_at(row, DP::Right)?.enumerate();
            let cursor = loop {
                match iter.next() {
                    Some((i, line)) => match line.chars().next() {
                        Some(NL) if n == 0 => {
                            break buf.line_to_char(row + i);
                        }
                        Some(NL) => n -= 1,
                        Some(_) => (),
                        None => break buf.line_to_char(row + i),
                    },
                    None => break buf.n_chars().saturating_sub(1),
                }
            };
            Ok(cursor)
        }
        _ => err_at!(Fatal, msg: format!("unreachable")),
    }?;

    buf.set_cursor(cursor);
    Ok(Event::Noop)
}

pub fn mto_bracket(buf: &mut Buffer, e: Mto) -> Result<Event> {
    let mut m = 0;
    let mut cursor = buf.to_cursor();
    match e {
        Mto::Bracket(mut n, yin, yan, DP::Left) => {
            let mut iter = buf.chars_at(cursor, DP::Left)?.enumerate();
            cursor -= loop {
                match iter.next() {
                    Some((_, ch)) if ch == yin && m > 0 => m -= 1,
                    Some((i, ch)) if ch == yin && n == 0 => break i + 1,
                    Some((_, ch)) if ch == yin => n -= 1,
                    Some((_, ch)) if ch == yan => m += 1,
                    Some(_) => (),
                    None => break 0,
                }
            };
        }
        Mto::Bracket(mut n, yin, yan, DP::Right) => {
            let mut iter = buf.chars_at(cursor, DP::Right)?.enumerate();
            cursor += {
                loop {
                    match iter.next() {
                        Some((_, ch)) if ch == yin && m > 0 => m -= 1,
                        Some((i, ch)) if ch == yin && n == 0 => break i,
                        Some((_, ch)) if ch == yin => n -= 1,
                        Some((_, ch)) if ch == yan => m += 1,
                        Some(_) => (),
                        None => break 0,
                    }
                }
            };
        }
        _ => err_at!(Fatal, msg: format!("unreachable"))?,
    }

    buf.set_cursor(cursor);
    Ok(Event::Noop)
}

pub fn mto_pattern(buf: &mut Buffer, evnt: Mto) -> Result<Event> {
    let (n, pattern, dp) = match evnt {
        Mto::Pattern(n, Some(pattern), dp) => Ok((n, pattern, dp)),
        _ => err_at!(Fatal, msg: format!("unreachable")),
    }?;

    let search = {
        let text = buf.to_string();
        Search::new(&pattern, &text, dp)?
    };
    let mut cursor = buf.to_cursor();
    let byte_off = buf.char_to_byte(cursor);

    let n = n.saturating_sub(1);
    cursor = match dp {
        DP::Left => {
            let item = search.rev(byte_off).skip(n).next();
            match item {
                Some((s, _)) => Ok(s),
                None => Ok(cursor),
            }
        }
        DP::Right => {
            let item = search.iter(byte_off).skip(n).next();
            match item {
                Some((s, _)) => Ok(s),
                None => Ok(cursor),
            }
        }
        _ => err_at!(Fatal, msg: format!("unreachable")),
    }?;

    buf.set_cursor(cursor);
    Ok(Event::Noop)
}

pub struct IterLine<'a> {
    _change: cell::Ref<'a, Change>, // holding a reference.
    iter: ropey::iter::Lines<'a>,
    reverse: bool,
}

impl<'a> Iterator for IterLine<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.reverse {
            self.iter.prev().map(|l| l.as_str().unwrap_or(""))
        } else {
            self.iter.next().map(|l| l.as_str().unwrap_or(""))
        }
    }
}

pub struct IterChar<'a> {
    _change: Option<cell::Ref<'a, Change>>, // holding a reference.
    iter: ropey::iter::Chars<'a>,
    reverse: bool,
}

impl<'a> Iterator for IterChar<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.reverse {
            self.iter.prev()
        } else {
            self.iter.next()
        }
    }
}

#[derive(Clone)]
struct Search {
    re: Regex,
    matches: Vec<(usize, usize)>, // byte (start, end)
    dp: DP,
}

impl Search {
    fn new(patt: &str, text: &str, dp: DP) -> Result<Search> {
        let re = err_at!(BadPattern, Regex::new(patt), format!("{}", patt))?;
        let matches = re.find_iter(text).map(|m| (m.start(), m.end())).collect();
        Ok(Search { re, matches, dp })
    }

    fn iter(&self, byte_off: usize) -> impl Iterator<Item = (usize, usize)> {
        match self.find(byte_off, &self.matches[..]) {
            Some(i) => {
                let mut ms = self.matches[i..].to_vec();
                ms.extend(&self.matches[..i]);
                ms.into_iter()
            }
            None => self.matches.clone().into_iter(),
        }
    }

    fn rev(&self, byte_off: usize) -> impl Iterator<Item = (usize, usize)> {
        match self.find(byte_off, &self.matches[..]) {
            Some(i) => {
                let mut ms = self.matches[i..].to_vec();
                ms.extend(&self.matches[..i]);
                ms.into_iter().rev()
            }
            None => self.matches.clone().into_iter().rev(),
        }
    }

    fn find(&self, byte_off: usize, rs: &[(usize, usize)]) -> Option<usize> {
        if rs.len() < 8
        /* TODO: no magic number */
        {
            let mut iter = rs
                .iter()
                .enumerate()
                .skip_while(|(_, (_, e))| *e < byte_off)
                .skip_while(|(_, (s, _))| byte_off >= *s);
            match iter.next() {
                None => None,
                Some((i, _)) => Some(i),
            }
        } else {
            let m = rs.len() / 2;
            match &rs[m] {
                (_, e) if *e < byte_off => match self.find(byte_off, &rs[m..]) {
                    None => None,
                    Some(i) => Some(m + i),
                },
                (s, _) if byte_off >= *s => match self.find(byte_off, &rs[m..]) {
                    None => None,
                    Some(i) => Some(m + i),
                },
                _ => self.find(byte_off, &rs[..m]),
            }
        }
    }
}
