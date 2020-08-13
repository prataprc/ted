//! Module `buffer` implement editing and cursor movement commands
//! over text content.

use lazy_static::lazy_static;
#[allow(unused_imports)]
use log::{debug, trace};
use regex::Regex;
use ropey::{self, Rope};

use std::{
    borrow::Borrow,
    cell::{self, RefCell},
    cmp,
    convert::{TryFrom, TryInto},
    fmt, io,
    iter::FromIterator,
    mem,
    ops::RangeBounds,
    rc::{self, Rc},
    result,
    sync::Mutex,
    vec,
};

use crate::{
    event::{Event, Mto, DP},
    location::Location,
    mark,
    term::{Span, Spanline},
    text,
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

#[derive(Clone, Copy)]
enum StickyCol {
    Home,
    End,
    None,
}

impl Default for StickyCol {
    fn default() -> Self {
        StickyCol::None
    }
}

#[derive(Clone)]
enum TabState {
    Active(String),
    None,
}

impl Default for TabState {
    fn default() -> Self {
        TabState::None
    }
}

// all bits and pieces of content is managed by buffer.
#[derive(Clone)]
pub struct Buffer {
    /// Source for this buffer, typically a file from local disk.
    pub location: Location,
    /// Mark this buffer read-only, in which case insert ops are not allowed.
    pub read_only: bool,
    /// Text-format for this buffer.
    pub format: text::Format,

    // Globally counting buffer number.
    num: usize, // buffer number
    // Buffer states
    inner: Inner,
    // current tab-completion state
    tab_state: TabState,

    // mark-list [a-z]
    marks: mark::Marks,
    // sticky state for cursor column.
    sticky_col: StickyCol,
    // Last search command applied on this buffer.
    mto_pattern: Mto,
    // Last find character command (within the line) applied on this buffer.
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
        Inner::Normal(NormalBuffer::default())
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Buffer {
            location: Location::default(),
            read_only: bool::default(),
            format: text::Format::default(),

            num: usize::default(),
            inner: Inner::default(),
            tab_state: TabState::default(),

            marks: mark::new_marks(),
            sticky_col: StickyCol::default(),
            mto_pattern: Mto::default(),
            mto_find_char: Mto::default(),
            insert_repeat: usize::default(),
            last_inserts: Vec::default(),
        }
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
            format: text::Format::default(),

            num: *num,
            inner: Inner::Normal(NormalBuffer::new(buf)),
            tab_state: TabState::default(),

            marks: mark::new_marks(),
            sticky_col: StickyCol::default(),
            mto_pattern: Mto::default(),
            mto_find_char: Mto::default(),
            insert_repeat: usize::default(),
            last_inserts: Vec::default(),
        };

        Ok(b)
    }

    pub fn empty() -> Buffer {
        Self::from_reader(io::empty(), Location::default()).unwrap()
    }

    pub fn set_cursor(&mut self, cursor: usize) -> &mut Self {
        match &mut self.inner {
            Inner::Normal(val) => val.set_cursor(cursor),
            Inner::Insert(val) => val.set_cursor(cursor),
        };
        self
    }

    pub fn set_read_only(&mut self, read_only: bool) -> &mut Self {
        self.read_only = read_only;
        self
    }

    pub fn set_format(&mut self, format: text::Format) -> &mut Self {
        self.format = format;
        self
    }

    pub fn clear_sticky_col(&mut self) -> &mut Self {
        self.sticky_col = StickyCol::default();
        self
    }

    pub fn set_sticky_col(&mut self, pos: DP, at: &str) -> &mut Self {
        match (pos, at) {
            (DP::TextCol, _) => self.sticky_col = StickyCol::default(),
            (DP::None, _) => self.sticky_col = StickyCol::default(),
            (DP::StickyCol, "home") => self.sticky_col = StickyCol::Home,
            (DP::StickyCol, "end") => self.sticky_col = StickyCol::End,
            (pos, at) => panic!("invalid position: {} {}", pos, at),
        };
        self
    }
}

impl WinBuffer for Buffer {
    fn to_char_cursor(&self) -> usize {
        self.to_change().to_char_cursor()
    }

    fn to_xy_cursor(&self, cursor: Option<usize>) -> Cursor {
        self.to_change().to_xy_cursor(cursor)
    }

    fn lines_at<'a>(
        &'a self,
        line_idx: usize,
        dp: DP,
    ) -> Result<Box<dyn Iterator<Item = String> + 'a>> {
        let change = self.to_change();
        let line_idx = cmp::min(change.rope.len_lines(), line_idx);
        let iter = unsafe {
            let cref: &Change = change.borrow();
            let cref = (cref as *const Change).as_ref().unwrap();
            cref.rope.lines_at(line_idx)
        };

        match dp {
            DP::Right => Ok(Box::new(IterLine {
                _change: change,
                iter,
                reverse: false,
            })),
            DP::Left => Ok(Box::new(IterLine {
                _change: change,
                iter,
                reverse: true,
            })),
            _ => err_at!(Fatal, msg: format!("unreachable")),
        }
    }

    fn chars_at<'a>(
        &'a self,
        char_idx: usize,
        dp: DP,
    ) -> Result<Box<dyn Iterator<Item = char> + 'a>> {
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
            DP::Right => Ok(Box::new(IterChar {
                _change: Some(change),
                iter,
                reverse: false,
            })),
            DP::Left => Ok(Box::new(IterChar {
                _change: Some(change),
                iter,
                reverse: true,
            })),
            _ => err_at!(Fatal, msg: format!("unreachable")),
        }
    }

    #[inline]
    fn slice<R>(&self, char_range: R) -> String
    where
        R: RangeBounds<usize>,
    {
        self.to_change().rope.slice(char_range).to_string()
    }

    #[inline]
    fn line_to_char(&self, line_idx: usize) -> usize {
        self.to_change().rope.line_to_char(line_idx)
    }

    #[inline]
    fn line(&self, line_idx: usize) -> String {
        let change = self.to_change();
        change.rope.line(line_idx).to_string()
    }

    #[inline]
    fn n_chars(&self) -> usize {
        let change = &self.to_change();
        change.rope.len_chars()
    }

    fn to_last_line_idx(&self) -> usize {
        let n_lines = {
            let change = &self.to_change();
            change.rope.len_lines()
        };
        for line_idx in (0..n_lines).rev() {
            let home = self.line_to_char(line_idx);
            match self.n_chars() {
                ln if ln == home => continue,
                _ => return line_idx,
            }
        }
        0
    }

    #[inline]
    fn len_line(&self, line_idx: usize) -> usize {
        let change = &self.to_change();
        change.rope.line(line_idx).len_chars()
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
            Location::Memory { name, .. } => name.clone(),
            Location::Disk { path_file, .. } => match path_file.to_str() {
                Some(s) => s.to_string(),
                None => format!("{:?}", path_file),
            },
            Location::Ted { name, .. } => name.clone(),
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

    /// Return the underlying text, if buffer is really large this can be
    /// a costly operation.
    #[inline]
    pub fn to_string(&self) -> String {
        self.to_change().as_ref().to_string()
    }

    pub fn byte_to_char(&self, byte_idx: usize) -> usize {
        self.to_change().as_ref().byte_to_char(byte_idx)
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

    fn char_to_line(&self, char_idx: usize) -> usize {
        self.to_change().rope.char_to_line(char_idx)
    }

    #[inline]
    fn to_line_home(&self, cursor: Option<usize>) -> usize {
        let cursor = cursor.unwrap_or(self.to_char_cursor());
        {
            let change = self.to_change();
            let line_idx = change.rope.char_to_line(cursor);
            change.rope.line_to_char(line_idx)
        }
    }
}

impl Buffer {
    pub fn on_event(&mut self, evnt: Event) -> Result<Event> {
        let evnt = match self.to_mode() {
            "insert" => self.handle_i_event(evnt),
            "normal" => self.handle_n_event(evnt),
            mode => err_at!(Fatal, msg: format!("invalid buffer-mode {}", mode)),
        }?;

        Ok(evnt)
    }

    pub fn mode_normal(&mut self) {
        self.inner = match mem::replace(&mut self.inner, Inner::default()) {
            Inner::Insert(ib) => Inner::Normal(ib.into()),
            inner @ Inner::Normal(_) => inner,
        };
    }

    pub fn mode_insert(&mut self) {
        self.inner = match mem::replace(&mut self.inner, Inner::default()) {
            Inner::Normal(nb) => Inner::Insert(nb.into()),
            inner @ Inner::Insert(_) => inner,
        };
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
    pub fn cmd_insert(&mut self, char_idx: usize, text: &str) -> Result<()> {
        let change = match &mut self.inner {
            Inner::Normal(nb) => &mut nb.change,
            Inner::Insert(ib) => &mut ib.change,
        };

        *change = Change::to_next_change(change);
        self.to_mut_change().insert(char_idx, text)
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
    pub fn cmd_delete_over<R>(&mut self, range: R) -> Result<()>
    where
        R: RangeBounds<usize>,
    {
        let change = match &mut self.inner {
            Inner::Normal(nb) => &mut nb.change,
            Inner::Insert(ib) => &mut ib.change,
        };

        *change = Change::to_next_change(change);
        self.to_mut_change().remove_over(range)
    }
}

impl Buffer {
    fn handle_n_event(&mut self, evnt: Event) -> Result<Event> {
        // try switching to insert mode, if event is insert command.
        match Self::to_insert_n(evnt.clone()) {
            Some(0) | None => (),
            _ => {
                let evnt = self.ex_n_insert(evnt)?;
                return self.handle_i_event(evnt);
            }
        };

        debug!("buffer {}", evnt);
        let evnt = match evnt {
            // motion command - characterwise.
            Event::Mt(Mto::Left(n, dp)) => {
                let cursor = mto_left(self, n, dp)?;
                self.set_cursor(cursor).clear_sticky_col();
                Event::Noop
            }
            Event::Mt(Mto::Right(n, dp)) => {
                let cursor = mto_right(self, n, dp)?;
                self.set_cursor(cursor).clear_sticky_col();
                Event::Noop
            }
            Event::Mt(Mto::LineHome(dp)) => {
                let cursor = mto_line_home(self, dp)?;
                self.set_cursor(cursor).set_sticky_col(dp, "home");
                Event::Noop
            }
            Event::Mt(Mto::LineEnd(n, dp)) => {
                let cursor = mto_line_end(self, n, dp)?;
                self.set_cursor(cursor).set_sticky_col(dp, "end");
                Event::Noop
            }
            Event::Mt(Mto::LineMiddle(p, _)) if p <= 1 => {
                let cursor = mto_line_middle(self, 50)?;
                self.set_cursor(cursor).clear_sticky_col();
                Event::Noop
            }
            Event::Mt(Mto::LineMiddle(p, _)) => {
                let cursor = mto_line_middle(self, p)?;
                self.set_cursor(cursor).clear_sticky_col();
                Event::Noop
            }
            Event::Mt(Mto::Col(n)) => {
                let cursor = mto_column(self, n)?;
                self.set_cursor(cursor).clear_sticky_col();
                Event::Noop
            }
            Event::Mt(e @ Mto::CharF(_, _, _)) => {
                self.mto_find_char = e.clone();
                let cursor = mto_char(self, e)?;
                self.set_cursor(cursor).clear_sticky_col();
                Event::Noop
            }
            Event::Mt(e @ Mto::CharT(_, _, _)) => {
                self.mto_find_char = e.clone();
                let cursor = mto_char(self, e)?;
                self.set_cursor(cursor).clear_sticky_col();
                Event::Noop
            }
            Event::Mt(Mto::CharR(n, dir)) => {
                let e = self.mto_find_char.clone();
                let cursor = mto_char(self, e.dir_xor(n, dir)?)?;
                self.set_cursor(cursor).clear_sticky_col();
                Event::Noop
            }
            // motion command - linewise.
            Event::Mt(Mto::Up(n, dp)) => {
                let cursor = mto_up(self, n, dp)?;
                self.set_cursor(cursor);
                Event::Noop
            }
            Event::Mt(Mto::Down(n, dp)) => {
                let cursor = mto_down(self, n, dp)?;
                self.set_cursor(cursor);
                Event::Noop
            }
            Event::Mt(Mto::Row(n, dp)) => {
                let n = n.saturating_sub(1);
                let cursor = mto_row(self, n, dp)?;
                self.set_cursor(cursor);
                Event::Noop
            }
            Event::Mt(Mto::Percent(n, dp)) => {
                let cursor = mto_percent(self, n, dp)?;
                self.set_cursor(cursor);
                Event::Noop
            }
            Event::Mt(Mto::Cursor(n)) => {
                let cursor = mto_cursor(self, n)?;
                self.set_cursor(cursor);
                Event::Noop
            }
            // motion command - word/sentence/para wise
            Event::Mt(Mto::Word(n, DP::Left, pos)) => {
                let cursor = mto_words_left(self, n, pos)?;
                self.set_cursor(cursor).clear_sticky_col();
                Event::Noop
            }
            Event::Mt(Mto::Word(n, DP::Right, pos)) => {
                let cursor = mto_words_right(self, n, pos)?;
                self.set_cursor(cursor).clear_sticky_col();
                Event::Noop
            }
            Event::Mt(Mto::WWord(n, DP::Left, pos)) => {
                let cursor = mto_wwords_left(self, n, pos)?;
                self.set_cursor(cursor).clear_sticky_col();
                Event::Noop
            }
            Event::Mt(Mto::WWord(n, DP::Right, pos)) => {
                let cursor = mto_wwords_right(self, n, pos)?;
                self.set_cursor(cursor).clear_sticky_col();
                Event::Noop
            }
            Event::Mt(Mto::Sentence(n, DP::Left)) => {
                let cursor = mto_sentence_left(self, n)?;
                self.set_cursor(cursor).clear_sticky_col();
                Event::Noop
            }
            Event::Mt(Mto::Sentence(n, DP::Right)) => {
                let cursor = mto_sentence_right(self, n)?;
                self.set_cursor(cursor).clear_sticky_col();
                Event::Noop
            }
            Event::Mt(Mto::Para(n, DP::Left)) => {
                let cursor = mto_paras_left(self, n)?;
                self.set_cursor(cursor).clear_sticky_col();
                Event::Noop
            }
            Event::Mt(Mto::Para(n, DP::Right)) => {
                let cursor = mto_paras_right(self, n)?;
                self.set_cursor(cursor).clear_sticky_col();
                Event::Noop
            }
            // motion command, other motions.
            Event::Mt(Mto::MatchPair) => {
                let cursor = mto_match_pair(self)?;
                self.set_cursor(cursor).clear_sticky_col();
                Event::Noop
            }
            Event::Mt(Mto::UnmatchPair(n, ch, dir)) => {
                let cursor = mto_unmatch_pair(self, ch, n, dir)?;
                self.set_cursor(cursor).clear_sticky_col();
                Event::Noop
            }
            // motion command marks and jumps
            Event::Mt(Mto::Jump(typ, mindex)) => {
                let mrk = mark::get_mark(&self.marks, mindex);
                match mrk {
                    Some(mrk) if typ == '`' => {
                        let cursor = mrk.to_cursor();
                        self.set_cursor(cursor).clear_sticky_col();
                        Event::Noop
                    }
                    Some(mrk) if typ == '\'' => {
                        self.set_cursor(mrk.to_cursor());
                        let cursor = mto_line_home(self, DP::TextCol)?;
                        self.set_cursor(cursor).clear_sticky_col();
                        Event::Noop
                    }
                    _ => Event::Mt(Mto::Jump(typ, mindex)),
                }
            }
            Event::Mark(mrk) => match mrk.to_index() {
                'a'..='z' | '\'' | '`' => {
                    let mrk = mrk.into_mark(self);
                    mark::set_mark(&mut self.marks, mrk);
                    Event::Noop
                }
                'A'..='Z' => Event::Mark(mrk.into_mark(self)),
                _ => Event::Noop,
            },

            Event::Mt(e @ Mto::Bracket(_, _, _, _)) => mto_bracket(self, e)?,
            Event::Mt(e @ Mto::Pattern(_, Some(_), _)) => {
                self.mto_pattern = e.clone();
                mto_pattern(self, e)?
            }
            Event::Mt(Mto::PatternR(n, dir)) => {
                let e = self.mto_pattern.clone();
                mto_pattern(self, e.dir_xor(n, dir)?)?
            }
            evnt => evnt,
        };

        Ok(evnt)
    }

    fn to_insert_n(evnt: Event) -> Option<usize> {
        use crate::event::{Event::Md, Mod};

        match evnt {
            Md(Mod::Insert(n, _)) => Some(n),
            Md(Mod::Append(n, _)) => Some(n),
            Md(Mod::Open(n, _)) => Some(n),
            _ => None,
        }
    }

    fn ex_n_insert(&mut self, evnt: Event) -> Result<Event> {
        use crate::event::{Event::Md, Mod};

        let nr = mem::replace(&mut self.inner, Inner::default());
        let (inner, evnt) = match nr {
            Inner::Normal(nb) => match evnt {
                Md(Mod::Insert(n, pos)) if n > 0 => {
                    self.insert_repeat = n - 1;
                    if pos == DP::TextCol {
                        let cursor = mto_line_home(self, pos)?;
                        self.set_cursor(cursor).set_sticky_col(pos, "home");
                    }
                    (Inner::Insert(nb.into()), Event::Noop)
                }
                Md(Mod::Append(n, pos)) if n > 0 => {
                    self.insert_repeat = n - 1;
                    if pos == DP::End {
                        let cursor = mto_line_end(self, 1, pos)?;
                        self.set_cursor(cursor);
                    }
                    let cursor = mto_right(self, 1, DP::Nobound)?;
                    self.set_cursor(cursor).clear_sticky_col();
                    (Inner::Insert(nb.into()), Event::Noop)
                }
                Md(Mod::Open(n, DP::Left)) if n > 0 => {
                    self.insert_repeat = n - 1;
                    {
                        let cursor = mto_line_home(self, DP::None)?;
                        self.set_cursor(cursor);
                    }
                    self.cmd_insert_char(NL)?;
                    let cursor = mto_left(self, 1, DP::Nobound)?;
                    self.set_cursor(cursor).clear_sticky_col();
                    (Inner::Insert(nb.into()), Event::Noop)
                }
                Md(Mod::Open(n, DP::Right)) if n > 0 => {
                    self.insert_repeat = n - 1;
                    {
                        let cursor = mto_line_end(self, 1, DP::None)?;
                        self.set_cursor(cursor);
                    }
                    let cursor = mto_right(self, 1, DP::Nobound)?;
                    self.set_cursor(cursor).clear_sticky_col();
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

    fn handle_i_event(&mut self, evnt: Event) -> Result<Event> {
        match evnt {
            Event::Noop => Ok(Event::Noop),
            evnt @ Event::TabInsert(_) => self.ex_i_event(evnt),
            evnt @ Event::TabClear => self.ex_i_event(evnt),
            evnt => {
                self.last_inserts.push(evnt.clone());
                self.ex_i_event(evnt)
            }
        }
    }

    fn ex_i_event(&mut self, evnt: Event) -> Result<Event> {
        use crate::event::Event::{
            Backspace, Char, Delete, Enter, Esc, Mt, Tab, TabClear, TabInsert,
        };

        let evnt = match evnt {
            // movement
            Mt(Mto::Left(n, dp)) => {
                let cursor = mto_left(self, n, dp)?;
                self.set_cursor(cursor).clear_sticky_col();
                Event::Noop
            }
            Mt(Mto::Right(n, dp)) => {
                let cursor = mto_right(self, n, dp)?;
                self.set_cursor(cursor).clear_sticky_col();
                Event::Noop
            }
            Mt(Mto::Up(n, dp)) => {
                let cursor = mto_up(self, n, dp)?;
                self.set_cursor(cursor);
                Event::Noop
            }
            Mt(Mto::Down(n, dp)) => {
                let cursor = mto_down(self, n, dp)?;
                self.set_cursor(cursor);
                Event::Noop
            }
            Mt(Mto::LineHome(dp)) => {
                let cursor = mto_line_home(self, dp)?;
                self.set_cursor(cursor).set_sticky_col(dp, "home");
                Event::Noop
            }
            Mt(Mto::LineEnd(n, dp)) => {
                let cursor = mto_line_end(self, n, dp)?;
                self.set_cursor(cursor).set_sticky_col(dp, "end");
                Event::Noop
            }
            // Handle mode events.
            Esc => {
                self.repeat()?;
                let cursor = mto_left(self, 1, DP::LineBound)?;
                self.set_cursor(cursor).clear_sticky_col();
                self.mode_normal();
                Event::Noop
            }
            // on going insert
            Char(ch, _) => {
                self.cmd_insert_char(ch)?;
                Event::Noop
            }
            Backspace(_) => {
                self.cmd_backspace(1)?;
                Event::Noop
            }
            Enter(_) => {
                self.cmd_insert_char(NL)?;
                Event::Noop
            }
            Tab(_) => {
                self.cmd_insert_char('\t')?;
                Event::Noop
            }
            Delete(_) => {
                let cursor = self.to_char_cursor();
                self.cmd_delete_over(cursor..=cursor)?;
                Event::Noop
            }
            // tab completion events
            TabInsert(new) => {
                let cursor = self.to_char_cursor();
                match &self.tab_state {
                    TabState::Active(old) => {
                        let to = cursor + old.chars().count();
                        self.cmd_delete_over(cursor..to)?;
                    }
                    TabState::None => (),
                }
                self.cmd_insert(cursor, new.as_str())?;
                self.tab_state = TabState::Active(new);
                Event::Noop
            }
            TabClear => {
                match &self.tab_state {
                    TabState::Active(old) => {
                        let from = self.to_char_cursor();
                        let to = from + old.chars().count();
                        self.cmd_delete_over(from..to)?;
                    }
                    TabState::None => (),
                };
                self.tab_state = TabState::default();
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
                Backspace(_) | Delete(_) => true,
                Char(_, _) | Enter(_) | Tab(_) => true,
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
        let mut nb = NormalBuffer::default();
        nb.change = Change::start(buf);
        nb
    }

    fn set_cursor(&mut self, cursor: usize) {
        self.to_mut_change().set_cursor(cursor);
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
    fn set_cursor(&mut self, cursor: usize) {
        self.to_mut_change().set_cursor(cursor)
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
    rope: Rope,
    parent: Option<rc::Weak<RefCell<Change>>>,
    children: Vec<Rc<RefCell<Change>>>,
    cursor: usize,
}

impl Default for Change {
    fn default() -> Change {
        Change {
            rope: Rope::from_reader(io::empty()).unwrap(),
            parent: None,
            children: Vec::default(),
            cursor: 0,
        }
    }
}

impl From<Rope> for Change {
    fn from(rope: Rope) -> Change {
        Change {
            rope,
            parent: None,
            children: Vec::default(),
            cursor: 0,
        }
    }
}

impl AsRef<Rope> for Change {
    fn as_ref(&self) -> &Rope {
        &self.rope
    }
}

impl AsMut<Rope> for Change {
    fn as_mut(&mut self) -> &mut Rope {
        &mut self.rope
    }
}

impl Change {
    fn start(rope: Rope) -> Rc<RefCell<Change>> {
        Rc::new(RefCell::new(Change {
            rope,
            parent: None,
            children: Vec::default(),
            cursor: 0,
        }))
    }

    fn to_next_change(prev: &mut Rc<RefCell<Change>>) -> Rc<RefCell<Change>> {
        let next = {
            let prev_change: &Change = &prev.as_ref().borrow();
            Rc::new(RefCell::new(Change {
                rope: prev_change.as_ref().clone(),
                parent: None,
                children: Vec::default(),
                cursor: prev_change.cursor,
            }))
        };

        next.borrow_mut().children.push(Rc::clone(prev));
        prev.borrow_mut().parent = Some(Rc::downgrade(&next));

        next
    }

    #[inline]
    fn to_char_cursor(&self) -> usize {
        self.cursor
    }

    fn to_xy_cursor(&self, cursor: Option<usize>) -> Cursor {
        let cursor = cursor.unwrap_or(self.cursor);
        let row_at = self.rope.char_to_line(cursor);
        let col_at = cursor - self.rope.line_to_char(row_at);
        (col_at, row_at).into()
    }
}

impl Change {
    fn set_cursor(&mut self, cursor: usize) {
        self.cursor = cursor;
    }

    fn insert_char(&mut self, ch: char) -> Result<()> {
        self.rope.insert_char(self.cursor, ch);
        self.cursor += 1;
        Ok(())
    }

    fn insert(&mut self, char_idx: usize, text: &str) -> Result<()> {
        self.rope.insert(char_idx, text);
        Ok(())
    }

    fn backspace(&mut self, n: usize) -> Result<()> {
        if self.cursor > 0 {
            let cursor = self.cursor.saturating_sub(n);
            self.rope.remove(cursor..self.cursor);
        }
        Ok(())
    }

    fn remove_over<R>(&mut self, range: R) -> Result<()>
    where
        R: RangeBounds<usize>,
    {
        use std::ops::Bound::{Excluded, Included, Unbounded};

        let n = self.rope.len_chars();

        let from = match range.start_bound() {
            Included(from) => cmp::min(*from, n.saturating_sub(1)),
            Excluded(from) => cmp::min(from.saturating_add(1), n),
            Unbounded => 0,
        };

        let to = match range.end_bound() {
            Included(to) => cmp::min((*to).saturating_add(1), n),
            Excluded(to) => cmp::min(*to, n),
            Unbounded => n,
        };

        if from < to {
            self.rope.remove(from..to);
        }
        Ok(())
    }
}

impl Change {
    //TODO
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

    //TODO
    //fn iter<'a>(&'a self, dp: DP) -> Box<dyn Iterator<Item = char> + 'a> {
    //    let chars = self.rope.chars_at(self.cursor);
    //    match dp {
    //        DP::Left => Box::new(IterChar {
    //            _change: None,
    //            iter: chars,
    //            reverse: true,
    //        }),
    //        DP::Right => Box::new(chars),
    //        _ => unreachable!(),
    //    }
    //}
}

pub fn mto_left(buf: &Buffer, mut n: usize, dp: DP) -> Result<usize> {
    let cursor = buf.to_char_cursor();
    let home = buf.to_line_home(Some(cursor));
    let new_cursor = cursor.saturating_sub(n);

    let cursor = match dp {
        DP::LineBound if new_cursor >= home => new_cursor,
        DP::LineBound => home,
        DP::Nobound if new_cursor >= home => new_cursor,
        DP::Nobound => {
            n = n - (cursor - home);
            let mut iter = (0..buf.char_to_line(cursor)).rev();
            loop {
                match iter.next() {
                    Some(line_idx) => {
                        let s = buf.line(line_idx);
                        let home = buf.line_to_char(line_idx);
                        match text::visual_line_n(&s) {
                            m if m == n => break home,
                            m if m > n => break home + (m - n),
                            m => n = n - m,
                        }
                    }
                    None => break 0,
                }
            }
        }
        DP::None => new_cursor,
        dp => err_at!(Fatal, msg: format!("invalid direction: {}", dp))?,
    };
    Ok(cursor)
}

pub fn mto_right(buf: &Buffer, mut n: usize, dp: DP) -> Result<usize> {
    let cursor = buf.to_char_cursor();
    let line_idx = buf.char_to_line(cursor);
    let home = buf.to_line_home(Some(cursor));
    let end = {
        let s = buf.line(line_idx);
        home + text::visual_line_n(&s)
    };
    let new_cursor = cursor + n;

    let cursor = match dp {
        DP::LineBound if new_cursor < end => new_cursor,
        DP::LineBound if home < end => end.saturating_sub(1),
        DP::LineBound => end,
        DP::Nobound if new_cursor < end => new_cursor,
        DP::Nobound => {
            let mut iter = buf.lines_at(line_idx, DP::Right)?.enumerate();
            loop {
                match iter.next() {
                    Some((0, _)) => n = n - (end - cursor),
                    Some((i, line)) => {
                        let m = text::visual_line_n(&line);
                        match buf.line_to_char(line_idx + i) {
                            home if n <= m => break home + n.saturating_sub(1),
                            _ => n = n - m,
                        }
                    }
                    None => break mto_end(buf)?,
                }
            }
        }
        dp => err_at!(Fatal, msg: format!("invalid direction: {}", dp))?,
    };
    Ok(cursor)
}

pub fn mto_line_home(buf: &Buffer, pos: DP) -> Result<usize> {
    let cursor = buf.to_line_home(None);
    let cursor = match pos {
        DP::TextCol => {
            let xy = buf.to_change().to_xy_cursor(Some(cursor));
            let n = skip_whitespace(&buf.line(xy.row), xy.col, DP::Right)?;
            cursor + n
        }
        DP::StickyCol | DP::None => cursor,
        dp => err_at!(Fatal, msg: format!("invalid direction: {}", dp))?,
    };
    Ok(cursor)
}

pub fn mto_line_end(buf: &Buffer, n: usize, dp: DP) -> Result<usize> {
    // When a `n` is given also go `n-1` lines downward.
    let cursor = {
        let cursor = mto_down(buf, n.saturating_sub(1), DP::None)?;
        let s = buf.line(buf.char_to_line(cursor));
        let m = text::visual_line_n(&s).saturating_sub(1);
        buf.to_line_home(Some(cursor)) + m
    };

    let cursor = match dp {
        DP::TextCol => {
            let xy = buf.to_change().to_xy_cursor(Some(cursor));
            let n = skip_whitespace(&buf.line(xy.row), xy.col, DP::Left)?;
            cursor.saturating_sub(n)
        }
        DP::StickyCol | DP::None => cursor,
        dp => err_at!(Fatal, msg: format!("invalid direction: {}", dp))?,
    };

    Ok(cursor)
}

pub fn mto_line_middle(buf: &Buffer, p: usize) -> Result<usize> {
    let n = {
        let s = buf.line(buf.char_to_line(buf.to_char_cursor()));
        text::visual_line_n(&s)
    };
    let cursor = {
        let n = (((p as f64) / 100.0) * (n as f64)) as usize;
        buf.to_line_home(None) + n
    };
    Ok(cursor)
}

pub fn mto_column(buf: &Buffer, n: usize) -> Result<usize> {
    let home = buf.to_line_home(None);
    let n = {
        let s = buf.line(buf.char_to_line(buf.to_char_cursor()));
        cmp::min(text::visual_line_n(&s), n)
    };
    Ok(home + n.saturating_sub(1))
}

pub fn mto_char(buf: &Buffer, evnt: Mto) -> Result<usize> {
    let cursor = buf.to_char_cursor();

    let (n, ch, dp, pos) = match evnt {
        Mto::CharF(n, Some(ch), dp) => (n, ch, dp, 'f'),
        Mto::CharT(n, Some(ch), dp) => (n, ch, dp, 't'),
        Mto::CharT(_, None, _) | Mto::None => return Ok(cursor),
        mto => err_at!(Fatal, msg: format!("unexpected {}", mto))?,
    };

    let mut iter = buf.chars_at(cursor, dp)?.enumerate();
    if let DP::Right = dp {
        iter.next();
    }
    let item = iter
        .filter_map(|(i, a)| if_else!(a == ch, Some(i), None))
        .skip(n.saturating_sub(1))
        .next()
        .clone();

    let cursor = match (item, dp, pos) {
        (Some(i), DP::Right, 'f') => cursor + i,
        (Some(i), DP::Right, 't') => (cursor + i).saturating_sub(1),
        (Some(i), DP::Left, 'f') => cursor.saturating_sub(i + 1),
        (Some(i), DP::Left, 't') => cursor.saturating_sub(i),
        (None, _, _) => cursor,
        (_, dp, pos) => err_at!(Fatal, msg: format!("bad {} {}", dp, pos))?,
    };
    Ok(cursor)
}

pub fn mto_up(buf: &Buffer, n: usize, dp: DP) -> Result<usize> {
    let bc_xy = buf.to_xy_cursor(None);
    let row = bc_xy.row.saturating_sub(n);
    let line = &buf.line(row);
    let char_end = text::visual_line_n(&line).saturating_sub(1);
    let col = cmp::min(char_end, bc_xy.col);

    let home = buf.line_to_char(row);
    let cursor = match dp {
        DP::StickyCol => match buf.sticky_col {
            StickyCol::Home => home,
            StickyCol::End => home + char_end,
            StickyCol::None => home + col,
        },
        DP::TextCol => home + skip_whitespace(&line, bc_xy.col, DP::Right)?,
        DP::None => home + col,
        dp => err_at!(Fatal, msg: format!("invalid direction: {}", dp))?,
    };
    Ok(cursor)
}

pub fn mto_down(buf: &Buffer, n: usize, dp: DP) -> Result<usize> {
    let row = {
        let row = buf.char_to_line(buf.to_char_cursor()) + n;
        cmp::min(buf.to_last_line_idx(), row)
    };
    let char_end = text::visual_line_n(&buf.line(row)).saturating_sub(1);
    let col = cmp::min(char_end, buf.to_xy_cursor(None).col);

    let home = buf.line_to_char(row);
    let cursor = match dp {
        DP::StickyCol => match buf.sticky_col {
            StickyCol::Home => home,
            StickyCol::End => home + char_end,
            StickyCol::None => home + col,
        },
        DP::TextCol => {
            let xy = buf.to_change().to_xy_cursor(Some(home));
            home + skip_whitespace(&buf.line(xy.row), xy.col, DP::Right)?
        }
        DP::None => home + col,
        dp => err_at!(Fatal, msg: format!("invalid direction: {}", dp))?,
    };
    Ok(cursor)
}

pub fn mto_row(buf: &Buffer, n: usize, dp: DP) -> Result<usize> {
    let last_line = buf.to_last_line_idx();
    let cursor = match n {
        std::usize::MAX => buf.line_to_char(last_line),
        n => buf.line_to_char(cmp::min(last_line, n)),
    };
    let cursor = match dp {
        DP::TextCol => {
            let xy = buf.to_change().to_xy_cursor(Some(cursor));
            cursor + skip_whitespace(&buf.line(xy.row), xy.col, DP::Right)?
        }
        _ => cursor,
    };
    Ok(cursor)
}

pub fn mto_percent(buf: &Buffer, n: usize, dp: DP) -> Result<usize> {
    let row = {
        let n_lines = buf.to_last_line_idx() + 1;
        cmp::min(((n * n_lines) + 99) / 100, n_lines.saturating_sub(1))
    };
    let cursor = buf.line_to_char(row);
    let cursor = match dp {
        DP::TextCol => {
            let xy = buf.to_change().to_xy_cursor(Some(cursor));
            cursor + skip_whitespace(&buf.line(xy.row), xy.col, DP::Right)?
        }
        _ => cursor,
    };
    Ok(cursor)
}

pub fn mto_end(buf: &Buffer) -> Result<usize> {
    let line_idx = buf.to_last_line_idx();
    let n = text::visual_line_n(&buf.line(line_idx));
    Ok(buf.line_to_char(line_idx) + n.saturating_sub(1))
}

pub fn mto_cursor(buf: &Buffer, n: usize) -> Result<usize> {
    let cursor = buf.to_char_cursor();
    Ok(limite!(cursor + n, buf.n_chars().saturating_sub(1)))
}

macro_rules! mto_text_left {
    ($buf:ident, $n:ident, $pos:ident, $start:expr, $state:ident) => {{
        let bc_xy = $buf.to_xy_cursor(None);
        let to_chars = |line: String| -> Vec<char> {
            let line = text::visual_line(&line);
            let mut chars: Vec<char> = line.chars().collect();
            chars.reverse();
            chars
        };

        let (mut iter, row, col) = {
            let chars: Vec<char> = {
                let line = $buf.line(bc_xy.row);
                text::visual_line(&line).chars().collect()
            };
            let col = {
                let n = cmp::min(chars.len(), bc_xy.col + 1);
                if_else!($pos == DP::Start, n, bc_xy.col)
            };
            let rem_chars = chars[..col].len();
            let chars: Vec<(usize, char)> = {
                let mut chars = chars[..col].to_vec();
                chars.reverse();
                chars.into_iter().enumerate().collect()
            };
            let iter = $buf.lines_at(bc_xy.row, DP::Left)?.map(to_chars);
            (
                WIterChar::new(iter, rem_chars, chars),
                bc_xy.row,
                col.saturating_sub(1),
            )
        };

        let mut state = $start;
        let cursor = loop {
            state = match iter.next() {
                Some(item) => match state.push(DP::Left, $pos, item) {
                    $state::Fin(r, _, None) => {
                        break xy_to_cursor($buf, (row - r, 0));
                    }
                    $state::Fin(r, rc, Some(c)) => {
                        let col = {
                            let this = col.saturating_sub(c);
                            if_else!(r == 0, this, rc.saturating_sub(c + 1))
                        };
                        break xy_to_cursor($buf, (row - r, col));
                    }
                    state => state,
                },
                None if $pos == DP::Start => break last_char_idx($buf),
                None => break 0, // DP::End
            };
        };
        Ok(saturate_cursor($buf, cursor))
    }};
}

macro_rules! mto_text_right {
    ($buf:ident, $n:ident, $pos:ident, $start:expr, $state:ident) => {{
        let bc_xy = $buf.to_xy_cursor(None);
        let to_chars = |s: String| -> Vec<char> { text::visual_line(&s).chars().collect() };

        let (mut iter, row, col) = {
            let chars: Vec<char> = {
                let line = $buf.line(bc_xy.row);
                text::visual_line(&line).chars().collect()
            };
            let col = if_else!(
                $pos == DP::Start,
                bc_xy.col,
                cmp::min(chars.len(), bc_xy.col + 1)
            );
            let rem_chars = chars[col..].len();
            let chars: Vec<(usize, char)> = {
                let iter = chars[col..].to_vec().into_iter().enumerate();
                iter.collect()
            };
            let iter = $buf.lines_at(bc_xy.row + 1, DP::Right)?.map(to_chars);
            (WIterChar::new(iter, rem_chars, chars), bc_xy.row, col)
        };

        let mut state = $start;
        let cursor = loop {
            state = match iter.next() {
                Some(item) => match state.push(DP::Right, $pos, item) {
                    $state::Fin(r, _, None) => {
                        break xy_to_cursor($buf, (row + r, 0));
                    }
                    $state::Fin(r, _, Some(c)) => {
                        let col = {
                            let n = line_chars($buf, row + r).saturating_sub(1);
                            let col = if_else!(r == 0, col.saturating_add(c), c);
                            cmp::min(n, col)
                        };
                        break xy_to_cursor($buf, (row + r, col));
                    }
                    state => state,
                },
                None => break last_char_idx($buf),
            };
        };
        Ok(saturate_cursor($buf, cursor))
    }};
}

pub fn mto_words_left(buf: &Buffer, n: usize, pos: DP) -> Result<usize> {
    mto_text_left!(buf, n, pos, MtoWord::St(n), MtoWord)
}

pub fn mto_words_right(buf: &Buffer, n: usize, pos: DP) -> Result<usize> {
    mto_text_right!(buf, n, pos, MtoWord::St(n), MtoWord)
}

pub fn mto_wwords_left(buf: &Buffer, n: usize, pos: DP) -> Result<usize> {
    mto_text_left!(buf, n, pos, MtoWWord::St(n), MtoWWord)
}

pub fn mto_wwords_right(buf: &Buffer, n: usize, pos: DP) -> Result<usize> {
    mto_text_right!(buf, n, pos, MtoWWord::St(n), MtoWWord)
}

pub fn mto_sentence_left(buf: &Buffer, n: usize) -> Result<usize> {
    let pos = DP::None;
    let start = MtoSentence::St(n);
    let cursor = mto_text_left!(buf, n, pos, start, MtoSentence)?;
    let is_skip = |(_, ch): &(usize, char)| -> bool {
        match ch {
            ch if ch.is_whitespace() => true,
            '.' | ')' | ']' | '"' | '\'' => true,
            _ => false,
        }
    };

    let i = if cursor == 0 {
        0
    } else {
        let iter = buf.chars_at(cursor, DP::Right)?.enumerate();
        iter.skip_while(is_skip)
            .next()
            .clone()
            .map(|(i, _)| i)
            .unwrap_or(buf.n_chars().saturating_sub(1))
    };
    Ok(cursor + if_else!(buf.to_char_cursor() <= cursor + i, 0, i))
}

pub fn mto_sentence_right(buf: &Buffer, n: usize) -> Result<usize> {
    let pos = DP::None;
    let start = {
        let ln = {
            let bc_xy = buf.to_xy_cursor(None);
            text::visual_line_n(&buf.line(bc_xy.row))
        };
        if_else!(ln == 0, MtoSentence::Ws(1, 0, n), MtoSentence::St(n))
    };
    mto_text_right!(buf, n, pos, start, MtoSentence)
}

pub fn mto_paras_left(buf: &Buffer, mut n: usize) -> Result<usize> {
    let row = buf.to_xy_cursor(None).row;
    let mut iter = buf.lines_at(row, DP::Left)?.enumerate();
    let row = loop {
        match iter.next() {
            Some((r, line)) => match text::visual_line_n(&line) {
                0 if n == 1 => break row.saturating_sub(r + 1),
                0 => n -= 1,
                _ => (),
            },
            None => break 0,
        }
    };
    Ok(saturate_cursor(buf, buf.line_to_char(row)))
}

pub fn mto_paras_right(buf: &Buffer, mut n: usize) -> Result<usize> {
    let row = buf.to_xy_cursor(None).row;
    let mut iter = buf.lines_at(row, DP::Right)?.enumerate();
    iter.next();
    let row = loop {
        match iter.next() {
            Some((r, line)) => match text::visual_line_n(&line) {
                0 if n == 1 => break row.saturating_add(r),
                0 => n -= 1,
                _ => (),
            },
            None => break 0,
        }
    };
    Ok(saturate_cursor(buf, buf.line_to_char(row)))
}

pub fn mto_match_pair(buf: &Buffer) -> Result<usize> {
    use crate::match_pair;

    let cursor = buf.to_char_cursor();
    Ok(match_pair::match_under_cursor(buf).unwrap_or(cursor))
}

pub fn mto_unmatch_pair(buf: &Buffer, ch: char, n: usize, dir: DP) -> Result<usize> {
    use crate::match_pair;

    let cursor = buf.to_char_cursor();
    let cursor = match dir {
        DP::Left => match_pair::unmatch_before(buf, ch, n).unwrap_or(cursor),
        DP::Right => match_pair::unmatch_after(buf, ch, n).unwrap_or(cursor),
        _ => cursor,
    };
    Ok(cursor)
}

pub fn mto_bracket(buf: &mut Buffer, e: Mto) -> Result<Event> {
    let mut m = 0;
    let mut cursor = buf.to_char_cursor();
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
    let (n, patt, dp) = match evnt {
        Mto::Pattern(n, Some(patt), dp) => Ok((n, patt, dp)),
        _ => err_at!(Fatal, msg: format!("unreachable")),
    }?;

    let iter = {
        let search: Search = patt.as_str().try_into()?;
        let byte_off = {
            let char_idx = buf.to_char_cursor();
            buf.to_change().rope.char_to_byte(char_idx)
        };
        match dp {
            DP::Right => search.find_fwd(&buf.to_string(), byte_off),
            DP::Left => search.find_rev(&buf.to_string(), byte_off),
            _ => unreachable!(),
        }
    }
    .into_iter();

    let n = n.saturating_sub(1);
    let cursor = match dp {
        DP::Left => match iter.skip(n).next() {
            Some((s, _)) => Ok(s),
            None => Ok(buf.to_char_cursor()),
        },
        DP::Right => match iter.skip(n).next() {
            Some((s, _)) => Ok(s),
            None => Ok(buf.to_char_cursor()),
        },
        _ => err_at!(Fatal, msg: format!("unreachable")),
    }?;

    buf.set_cursor(cursor);
    Ok(Event::Noop)
}

// skip whitespace from `offset` in specified direction `dp` and returned
// the number of position skipped.
pub fn skip_whitespace(line: &str, off: usize, dp: DP) -> Result<usize> {
    let line = text::visual_line(&line);
    let chars: Vec<char> = line.chars().collect();
    let ln = chars.len();

    let n = match dp {
        DP::Right => {
            let item = {
                let iter = chars.into_iter().skip(off).enumerate();
                iter.skip_while(|(_, ch)| ch.is_whitespace()).next().clone()
            };
            item.map(|x| x.0)
                .unwrap_or(ln.saturating_sub(off).saturating_sub(1))
        }
        DP::Left => {
            let item = {
                let m = chars.len().saturating_sub(off).saturating_sub(1);
                let iter = chars.into_iter().rev().skip(m).enumerate();
                iter.skip_while(|(_, ch)| ch.is_whitespace()).next().clone()
            };
            item.map(|x| x.0).unwrap_or(off)
        }
        dp => err_at!(Fatal, msg: format!("invalid direction: {}", dp))?,
    };
    Ok(n)
}

pub fn to_span_line(buf: &Buffer, a: usize, z: usize) -> Result<Spanline> {
    let span: Span = {
        let iter = buf.chars_at(a, DP::Right)?.take(z - a);
        String::from_iter(iter).into()
    };
    Ok(span.into())
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd)]
enum MtoWord {
    St(usize), // start - (n,) number of words to move.
    An(usize), // Alphanumeric - (n,) number of words to move.
    Ch(usize), // non-ws - (n) number of words to move.
    Ws(usize),
    Fin(usize, usize, Option<usize>), // (row, rem_chars, col_off)
}

impl MtoWord {
    fn decr(self) -> Self {
        match self {
            MtoWord::St(n) => MtoWord::St(n.saturating_sub(1)),
            MtoWord::An(n) => MtoWord::An(n.saturating_sub(1)),
            MtoWord::Ch(n) => MtoWord::Ch(n.saturating_sub(1)),
            MtoWord::Ws(n) => MtoWord::Ws(n.saturating_sub(1)),
            MtoWord::Fin(_, _, _) => unreachable!(),
        }
    }

    // (row, rem_chars, Option<(col_off, char)>)
    fn match_char(self, dir: DP, pos: DP, item: (usize, usize, Option<(usize, char)>)) -> Self {
        use MtoWord::{An, Ch, Fin, St, Ws};

        let (row, rc, col, ch) = {
            let (row, rc, ch) = item;
            let (col, ch) = ch.unwrap();
            (row, rc, col, ch)
        };
        let last_char = rc.saturating_sub(1);

        let is_ws = ch.is_whitespace();
        let is_an = ch.is_alphanumeric() || ch == '_';

        // rotate the current state.
        let state = match pos {
            DP::Start => match self {
                St(n) if is_an => An(n),
                St(n) if is_ws => Ws(n),
                St(n) => Ch(n),
                An(n) if is_an => An(n),
                An(n) if is_ws => Ws(n),
                An(n) => Ch(n - 1),
                Ch(n) if is_an => An(n - 1),
                Ch(n) if is_ws => Ws(n),
                Ch(n) => Ch(n),
                Ws(n) if is_ws => Ws(n),
                Ws(n) if is_an => An(n - 1),
                Ws(n) => Ch(n - 1),
                _ => unreachable!(),
            },
            DP::End => match self {
                St(n) if is_an => An(n),
                St(n) if is_ws => Ws(n),
                St(n) => Ch(n),
                An(n) if is_an => An(n),
                An(n) if is_ws => Ws(n - 1),
                An(n) => Ch(n - 1),
                Ch(n) if is_an => An(n - 1),
                Ch(n) if is_ws => Ws(n - 1),
                Ch(n) => Ch(n),
                Ws(n) if is_ws => Ws(n),
                Ws(n) if is_an => An(n),
                Ws(n) => Ch(n),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        };

        // check the rotated state is a candidate for final state.
        let max_col = Some(std::usize::MAX);
        match (dir, pos) {
            (DP::Right, DP::End) if col == 0 && row > 0 => match state {
                St(0) => Fin(row.saturating_sub(1), rc, max_col),
                An(0) => Fin(row.saturating_sub(1), rc, max_col),
                Ch(0) => Fin(row.saturating_sub(1), rc, max_col),
                Ws(0) => Fin(row.saturating_sub(1), rc, max_col),
                state => state,
            },
            (DP::Left, DP::End) if col == last_char && row > 0 => match state {
                St(0) => Fin(row.saturating_sub(1), rc, Some(0)),
                An(0) => Fin(row.saturating_sub(1), rc, Some(0)),
                Ch(0) => Fin(row.saturating_sub(1), rc, Some(0)),
                Ws(0) => Fin(row.saturating_sub(1), rc, Some(0)),
                state => state,
            },
            (_, DP::Start) => match state {
                St(0) | An(0) => Fin(row, rc, Some(col)),
                Ch(0) | Ws(0) => Fin(row, rc, Some(col)),
                state => state,
            },
            (_, DP::End) => match state {
                St(0) => Fin(row, rc, Some(col.saturating_sub(1))),
                An(0) => Fin(row, rc, Some(col.saturating_sub(1))),
                Ch(0) => Fin(row, rc, Some(col.saturating_sub(1))),
                Ws(0) => Fin(row, rc, Some(col.saturating_sub(1))),
                state => state,
            },
            (_, _) => unreachable!(),
        }
    }
}

impl fmt::Display for MtoWord {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        use MtoWord::{An, Ch, Fin, St, Ws};
        match self {
            St(n) => write!(f, "St<{}>", n),
            An(n) => write!(f, "An<{}>", n),
            Ch(n) => write!(f, "Ch<{}>", n),
            Ws(n) => write!(f, "Ws<{}>", n),
            Fin(r, rc, c) => write!(f, "Fin<{},{},{:?}>", r, rc, c),
        }
    }
}

impl MtoWord {
    // (row, rem_chars, Option<(col_off, char)>)
    fn push(self, dir: DP, pos: DP, item: (usize, usize, Option<(usize, char)>)) -> Self {
        use MtoWord::{An, Ch, Fin, St, Ws};

        let state = match self {
            val @ Fin(_, _, _) => val,
            St(0) | An(0) | Ch(0) | Ws(0) => Fin(0, 0, None),
            St(n) | An(n) | Ch(n) => match item {
                (row, 0, None) => {
                    if row == 0 {
                        Ws(n)
                    } else if n == 1 {
                        Fin(row, 0, None)
                    } else {
                        self.decr()
                    }
                }
                (row, rc, None) if pos == DP::End => {
                    let this = Fin(row, rc, Some(rc.saturating_sub(1)));
                    if_else!(n == 1, this, self.decr())
                }
                (_, _, None) => Ws(n),
                (_, _, Some(_)) => self.match_char(dir, pos, item),
            },
            Ws(n) => match item {
                (row, 0, None) => {
                    if n == 1 {
                        Fin(row, 0, None)
                    } else {
                        self.decr()
                    }
                }
                (_, _, None) => Ws(n),
                (_, _, Some(_)) => self.match_char(dir, pos, item),
            },
        };
        trace!("push {:?} {} -> {}", item, self, state);
        state
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd)]
enum MtoWWord {
    St(usize), // start - (n,) number of words to move.
    Ch(usize), // char - (n,) number of words to move.
    Ws(usize),
    Fin(usize, usize, Option<usize>), // (row, rem_chars, col_off)
}

impl MtoWWord {
    fn decr(self) -> Self {
        match self {
            MtoWWord::St(n) => MtoWWord::St(n.saturating_sub(1)),
            MtoWWord::Ch(n) => MtoWWord::Ch(n.saturating_sub(1)),
            MtoWWord::Ws(n) => MtoWWord::Ws(n.saturating_sub(1)),
            MtoWWord::Fin(_, _, _) => unreachable!(),
        }
    }

    // (row, rem_chars, Option<(col_off, char)>)
    fn match_char(self, dir: DP, pos: DP, item: (usize, usize, Option<(usize, char)>)) -> Self {
        use MtoWWord::{Ch, Fin, St, Ws};

        let (row, rc, col, ch) = {
            let (row, rc, ch) = item;
            let (col, ch) = ch.unwrap();
            (row, rc, col, ch)
        };
        let last_char = rc.saturating_sub(1);

        let is_ws = ch.is_whitespace();

        // rotate the current state.
        let state = match pos {
            DP::Start => match self {
                St(n) if is_ws => Ws(n),
                St(n) => Ch(n),
                Ch(n) if is_ws => Ws(n),
                Ch(n) => Ch(n),
                Ws(n) if is_ws => Ws(n),
                Ws(n) => Ch(n - 1),
                _ => unreachable!(),
            },
            DP::End => match self {
                St(n) if is_ws => Ws(n),
                St(n) => Ch(n),
                Ch(n) if is_ws => Ws(n - 1),
                Ch(n) => Ch(n),
                Ws(n) if is_ws => Ws(n),
                Ws(n) => Ch(n),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        };

        // check the rotated state is a candidate for final state.
        let max_col = Some(std::usize::MAX);
        match (dir, pos) {
            (DP::Right, DP::End) if col == 0 && row > 0 => match state {
                St(0) => Fin(row.saturating_sub(1), rc, max_col),
                Ch(0) => Fin(row.saturating_sub(1), rc, max_col),
                Ws(0) => Fin(row.saturating_sub(1), rc, max_col),
                state => state,
            },
            (DP::Left, DP::End) if col == last_char && row > 0 => match state {
                St(0) => Fin(row.saturating_sub(1), rc, Some(0)),
                Ch(0) => Fin(row.saturating_sub(1), rc, Some(0)),
                Ws(0) => Fin(row.saturating_sub(1), rc, Some(0)),
                state => state,
            },
            (_, DP::Start) => match state {
                St(0) | Ch(0) | Ws(0) => Fin(row, rc, Some(col)),
                state => state,
            },
            (_, DP::End) => match state {
                St(0) => Fin(row, rc, Some(col.saturating_sub(1))),
                Ch(0) => Fin(row, rc, Some(col.saturating_sub(1))),
                Ws(0) => Fin(row, rc, Some(col.saturating_sub(1))),
                state => state,
            },
            (_, _) => unreachable!(),
        }
    }
}

impl fmt::Display for MtoWWord {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        use MtoWWord::{Ch, Fin, St, Ws};
        match self {
            St(n) => write!(f, "St<{}>", n),
            Ch(n) => write!(f, "Ch<{}>", n),
            Ws(n) => write!(f, "Ws<{}>", n),
            Fin(r, rc, c) => write!(f, "Fin<{},{},{:?}>", r, rc, c),
        }
    }
}

impl MtoWWord {
    // (row, rem_chars, Option<(col_off, char)>)
    fn push(self, dir: DP, pos: DP, item: (usize, usize, Option<(usize, char)>)) -> Self {
        use MtoWWord::{Ch, Fin, St, Ws};

        let state = match self {
            val @ Fin(_, _, _) => val,
            St(0) | Ch(0) | Ws(0) => Fin(0, 0, None),
            St(n) | Ch(n) => match item {
                (row, 0, None) => {
                    if row == 0 {
                        Ws(n)
                    } else if n == 1 {
                        Fin(row, 0, None)
                    } else {
                        self.decr()
                    }
                }
                (row, rc, None) if pos == DP::End => {
                    let this = Fin(row, rc, Some(rc.saturating_sub(1)));
                    if_else!(n == 1, this, self.decr())
                }
                (_, _, None) => Ws(n),
                (_, _, Some(_)) => self.match_char(dir, pos, item),
            },
            Ws(n) => match item {
                (row, 0, None) => {
                    if n == 1 {
                        Fin(row, 0, None)
                    } else {
                        self.decr()
                    }
                }
                (_, _, None) => Ws(n),
                (_, _, Some(_)) => self.match_char(dir, pos, item),
            },
        };
        trace!("push {:?} {} -> {}", item, self, state);
        state
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd)]
enum MtoSentence {
    St(usize),
    Ch(usize, usize),                 // (n-ch, n)
    Ws(usize, usize, usize),          // (n-ws, n-ch, n)
    Fin(usize, usize, Option<usize>), // (row, rem_chars, col_off)
}

impl MtoSentence {
    // (row, rem_chars, Option<(col_off, char)>)
    fn match_char(self, dir: DP, item: (usize, usize, Option<(usize, char)>)) -> Self {
        use MtoSentence::{Ch, Fin, St, Ws};

        let (row, rc, col, ch) = {
            let (row, rc, ch) = item;
            let (col, ch) = ch.unwrap();
            (row, rc, col, ch)
        };

        match dir {
            // forward, rotate the current state
            DP::Right => match self {
                St(n) => match ch {
                    '.' | '!' | '?' => Ws(0, 0, n),
                    _ => Ch(1, n),
                },
                Ch(count, n) => match ch {
                    '.' | '!' | '?' => Ws(0, count, n),
                    _ => Ch(count + 1, n),
                },
                Ws(count, nch, n) => match ch {
                    ch if ch.is_whitespace() => Ws(count + 1, nch, n),
                    ')' | ']' | '"' | '\'' => Ws(count + 1, nch, n),
                    _ if count > 0 && n == 1 => Fin(row, rc, Some(col)),
                    _ if count > 0 => Ch(1, n - 1),
                    _ => Ch(1, n),
                },
                _ => unreachable!(),
            },
            // reverse, rotate the current state
            DP::Left => match self {
                St(n) => match ch {
                    ch if ch.is_whitespace() => Ws(1, 0, n),
                    _ => Ch(1, n),
                },
                Ch(count, n) => match ch {
                    ch if ch.is_whitespace() => Ws(1, count, n),
                    _ => Ch(count + 1, n),
                },
                Ws(count, nch, n) => match ch {
                    ch if ch.is_whitespace() => Ws(count + 1, nch, n),
                    ')' | ']' | '"' | '\'' => Ws(count + 1, nch, n),
                    '.' if nch > 0 && n == 1 => Fin(row, rc, Some(col)),
                    '.' if nch > 0 => Ch(1, n - 1),
                    _ => Ch(1, n),
                },
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for MtoSentence {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        use MtoSentence::{Ch, Fin, St, Ws};
        match self {
            St(n) => write!(f, "St<{}>", n),
            Ch(m, n) => write!(f, "Ch<{},{}>", m, n),
            Ws(c, n, m) => write!(f, "Ws<{},{},{}>", c, n, m),
            Fin(r, rc, c) => write!(f, "Fin<{},{},{:?}>", r, rc, c),
        }
    }
}

impl MtoSentence {
    // (row, rem_chars, Option<(col_off, char)>)
    fn push(self, dir: DP, _pos: DP, item: (usize, usize, Option<(usize, char)>)) -> Self {
        use MtoSentence::{Ch, Fin, St, Ws};

        let state = match self {
            val @ Fin(_, _, _) => val,
            St(0) | Ch(_, 0) | Ws(_, _, 0) => Fin(0, 0, None),
            St(_) => match item {
                (0, 0, None) => self,
                (row, 0, None) => Fin(row, 0, None),
                (_, _, None) => self,
                (_, _, Some(_)) => self.match_char(dir, item),
            },
            Ch(nch, n) => match item {
                (0, 0, None) => Ws(1, nch, n),
                (row, 0, None) => Fin(row, 0, None),
                (_, _, None) if dir == DP::Left => Ws(1, nch, n),
                (_, _, None) => self,
                (_, _, Some(_)) => self.match_char(dir, item),
            },
            Ws(count, nch, n) => match item {
                (0, 0, None) => Ws(count + 1, nch, n),
                (row, 0, None) => Fin(row, 0, None),
                (_, _, None) => Ws(count + 1, nch, n),
                (_, _, Some(_)) => self.match_char(dir, item),
            },
        };
        trace!("push {:?} {} -> {}", item, self, state);
        state
    }
}

struct WIterChar<I>
where
    I: Iterator<Item = Vec<char>>,
{
    iter: I,
    rem_chars: usize,
    chars: std::vec::IntoIter<(usize, char)>,
    row: usize,
}

impl<I> WIterChar<I>
where
    I: Iterator<Item = Vec<char>>,
{
    fn new(iter: I, rchs: usize, chars: Vec<(usize, char)>) -> Self {
        WIterChar {
            iter,
            rem_chars: rchs,
            chars: chars.into_iter(),
            row: 0,
        }
    }

    fn to_next_line(&mut self) -> bool {
        match self.iter.next() {
            Some(chars) => {
                self.rem_chars = chars.len();
                self.chars = {
                    let chars: Vec<(usize, char)> = {
                        //
                        chars.into_iter().enumerate().collect()
                    };
                    chars.into_iter()
                };
                self.row += 1;
                true
            }
            None => {
                self.rem_chars = 0;
                self.chars = vec![].into_iter();
                false
            }
        }
    }
}

impl<I> Iterator for WIterChar<I>
where
    I: Iterator<Item = Vec<char>>,
{
    // (row, rem_chars, Option<(col_off, char)>)
    type Item = (usize, usize, Option<(usize, char)>);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.chars.next() {
                Some(val) => break Some((self.row, self.rem_chars, Some(val))),
                None => {
                    let (row, rem_chars) = (self.row, self.rem_chars);
                    break match self.to_next_line() {
                        true => Some((row, rem_chars, None)),
                        false => None,
                    };
                }
            }
        }
    }
}

pub struct IterLine<'a> {
    _change: cell::Ref<'a, Change>, // holding a reference.
    iter: ropey::iter::Lines<'a>,
    reverse: bool,
}

impl<'a> Iterator for IterLine<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.reverse {
            self.iter.prev().map(|l| l.to_string())
        } else {
            self.iter.next().map(|l| l.to_string())
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
    patt: Regex,
}

impl<'a> TryFrom<&'a str> for Search {
    type Error = Error;

    fn try_from(patt: &'a str) -> Result<Search> {
        let patt = err_at!(BadPattern, Regex::new(patt), format!("{}", patt))?;
        Ok(Search { patt })
    }
}

impl Search {
    fn find_fwd(&self, text: &str, byte_off: usize) -> Vec<(usize, usize)> {
        let matches: Vec<(usize, usize)> = {
            let iter = self.patt.find_iter(text).map(|m| (m.start(), m.end()));
            iter.collect()
        };

        match Self::find(byte_off, &matches[..]) {
            Some(i) => {
                let mut ms = matches[i..].to_vec();
                ms.extend(&matches[..i]);
                ms
            }
            None => matches,
        }
    }

    fn find_rev(&self, text: &str, byte_off: usize) -> Vec<(usize, usize)> {
        let mut matches = self.find_fwd(text, byte_off);
        matches.reverse();
        matches
    }

    fn find(off: usize, rs: &[(usize, usize)]) -> Option<usize> {
        match rs.len() {
            0 => None,
            1 => Some(0),
            _ => {
                let m = rs.len() / 2;
                let (s, e) = rs[m].clone();
                if e < off || off >= s {
                    Self::find(off, &rs[m..]).map(|i| m + i)
                } else {
                    Self::find(off, &rs[..m])
                }
            }
        }
    }
}

#[inline]
fn saturate_cursor(buf: &Buffer, cursor: usize) -> usize {
    if_else!(cursor >= buf.n_chars(), last_char_idx(buf), cursor)
}

fn last_char_idx(buf: &Buffer) -> usize {
    let row = buf.to_last_line_idx();
    let col = text::visual_line_n(&buf.line(row));
    xy_to_cursor(buf, (row, col.saturating_sub(1)))
}

#[inline]
fn xy_to_cursor(buf: &Buffer, (row, col): (usize, usize)) -> usize {
    buf.line_to_char(row) + col
}

#[inline]
fn line_chars(buf: &Buffer, row: usize) -> usize {
    text::visual_line_n(&buf.line(row))
}

#[cfg(test)]
#[path = "buffer_test.rs"]
mod buffer_test;
