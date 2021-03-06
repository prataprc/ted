//! Module `buffer` implement editing and cursor movement commands
//! over text content.

use lazy_static::lazy_static;
#[allow(unused_imports)]
use log::{debug, trace};
use ropey::{self, Rope};

use std::{
    borrow::Borrow,
    cell::{self, RefCell},
    cmp, fmt, io,
    iter::FromIterator,
    mem,
    ops::RangeBounds,
    rc::{self, Rc},
    result,
    sync::Mutex,
    vec,
};

use crate::{
    event::{Event, Mod, Mto, DP},
    location::Location,
    mark,
    term::{Span, Spanline},
    text,
    window::WinBuffer,
    {err_at, Error, Result},
};

/// Maximum number of lines supported by this buffer implementation.
pub const MAX_LINES: usize = 1_000_000_000;

/// Number of spaces to use for each step of indent.
pub const SHIFT_WIDTH: usize = 4;

lazy_static! {
    static ref BUFFER_NUM: Mutex<usize> = Mutex::new(0);
}

/// Cursor within buffer in two-dimensional coordinate.
#[derive(Clone, Copy, Default, Debug)]
pub struct Cursor {
    /// Cursor column start from ZERO.
    pub col: usize,
    /// Cursor row start from ZERO.
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

/// All bits and pieces of content is managed by buffer.
///
/// Content is to be found in location, refer [Location] for details. Use
/// `set_` methods to configure the buffer instance.
#[derive(Clone)]
pub struct Buffer {
    /// Source for this buffer, typically a file from local disk.
    pub location: Location,
    /// Text-format for this buffer.
    pub format: text::Format,
    /// Shift-width, number of spaces to use for each step of indent.
    pub shift_width: usize,
    /// Buffer number, for easy picking. Make sure to set unique numbers
    /// for each buffer.
    pub num: usize, // buffer number

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
}

#[derive(Clone)]
enum Inner {
    Normal(NormalBuffer),
    Insert(InsertBuffer),
    Replace(ReplaceBuffer),
    None,
}

impl Default for Inner {
    fn default() -> Inner {
        Inner::None
    }
}

impl From<NormalBuffer> for Inner {
    fn from(nb: NormalBuffer) -> Inner {
        Inner::Normal(nb)
    }
}

impl From<InsertBuffer> for Inner {
    fn from(ib: InsertBuffer) -> Inner {
        Inner::Insert(ib)
    }
}

impl From<ReplaceBuffer> for Inner {
    fn from(rb: ReplaceBuffer) -> Inner {
        Inner::Replace(rb)
    }
}

impl Inner {
    fn do_repeat(buf: &mut Buffer) -> Result<Event> {
        match &mut buf.inner {
            Inner::Normal(_) => err_at!(Fatal, msg: format!("not supported")),
            Inner::Insert(ib) => {
                let i_evnts = ib.i_evnts.clone();
                InsertBuffer::do_repeat(ib.repeat, i_evnts, buf)
            }
            Inner::Replace(_) => todo!(),
            Inner::None => err_at!(Fatal, msg: format!("unreachable"))?,
        }
    }

    fn into_normal_buffer(self) -> Result<Inner> {
        match self {
            inner @ Inner::Normal(_) => Ok(inner),
            Inner::Insert(ib) => Ok(Inner::Normal(ib.into())),
            Inner::Replace(_) => todo!(),
            Inner::None => err_at!(Fatal, msg: format!("unreachable"))?,
        }
    }

    fn insert_event(&mut self, evnt: Event) -> Result<()> {
        match self {
            Inner::Normal(_) => Ok(()),
            Inner::Insert(ib) => ib.insert_event(evnt),
            Inner::Replace(_) => todo!(),
            Inner::None => err_at!(Fatal, msg: format!("unreachable"))?,
        }
    }

    #[inline]
    fn cud_newlines(&mut self, cursor: Option<usize>, nl: &str, n: usize) -> Result<usize> {
        match self {
            Inner::Normal(nb) => nb.cud_newlines(cursor, nl, n),
            Inner::Insert(ib) => ib.cud_newlines(cursor, nl, n),
            Inner::Replace(rb) => rb.cud_newlines(cursor, nl, n),
            Inner::None => err_at!(Fatal, msg: format!("unreachable")),
        }
    }

    #[inline]
    fn cud_char(&mut self, cursor: Option<usize>, ch: char) -> Result<usize> {
        match self {
            Inner::Normal(nb) => nb.cud_char(cursor, ch),
            Inner::Insert(ib) => ib.cud_char(cursor, ch),
            Inner::Replace(rb) => rb.cud_char(cursor, ch),
            Inner::None => err_at!(Fatal, msg: format!("unreachable")),
        }
    }

    #[inline]
    fn cud_str(&mut self, cursor: Option<usize>, txt: &str) -> Result<usize> {
        match self {
            Inner::Normal(nb) => nb.cud_str(cursor, txt),
            Inner::Insert(ib) => ib.cud_str(cursor, txt),
            Inner::Replace(rb) => rb.cud_str(cursor, txt),
            Inner::None => err_at!(Fatal, msg: format!("unreachable")),
        }
    }

    #[inline]
    fn cud_delete<R>(&mut self, range: R) -> Result<()>
    where
        R: RangeBounds<usize>,
    {
        match self {
            Inner::Normal(nb) => nb.cud_delete(range),
            Inner::Insert(ib) => ib.cud_delete(range),
            Inner::Replace(rb) => rb.cud_delete(range),
            Inner::None => err_at!(Fatal, msg: format!("unreachable")),
        }
    }
}

/// Create and configure a text buffer.
impl Buffer {
    /// Create a new instance of buffer pre-populating it with
    /// content from `loc`. Refer [Location] for details.
    pub fn from_reader(loc: Location) -> Result<Buffer> {
        let buf = {
            let bytes = loc.to_bytes()?;
            err_at!(FailBuffer, Rope::from_reader(bytes.as_slice()))?
        };
        let mut num = BUFFER_NUM.lock().unwrap();
        *num = *num + 1;
        let b = Buffer {
            location: loc,
            format: text::Format::default(),
            shift_width: SHIFT_WIDTH,
            num: *num,

            inner: Inner::Normal(NormalBuffer::new(buf)),

            tab_state: TabState::default(),
            marks: mark::new_marks(),
            sticky_col: StickyCol::default(),
            mto_pattern: Mto::default(),
            mto_find_char: Mto::default(),
        };

        Ok(b)
    }

    /// Create an empty buffer backed by transient in-memory location.
    pub fn empty() -> Buffer {
        Self::from_reader(Location::default()).unwrap()
    }

    /// Set cursor postion within the buffer, cursor is character index.
    pub fn set_cursor(&mut self, cursor: usize) -> &mut Self {
        match &mut self.inner {
            Inner::Normal(val) => val.set_cursor(cursor),
            Inner::Insert(val) => val.set_cursor(cursor),
            Inner::Replace(val) => val.set_cursor(cursor),
            Inner::None => (),
        };
        self
    }

    /// Configure buffer as read-only.
    pub fn set_read_only(&mut self, read_only: bool) -> &mut Self {
        self.location.set_read_only(read_only);
        self
    }

    /// Configure buffer's text-format. Refer [text::Format] for details.
    pub fn set_format(&mut self, format: text::Format) -> &mut Self {
        self.format = format;
        self
    }

    /// Configure shift-width for text-indentation.
    pub fn set_shift_width(&mut self, shift_width: usize) -> &mut Self {
        self.shift_width = shift_width;
        self
    }

    /// Clear sticky-column for this buffer. Certian buffer commands can
    /// make the cursor stick to the end-of-the-line or beginning-of-the-line.
    /// Refer [Buffer::set_sticky_col] for details.
    pub fn clear_sticky_col(&mut self) -> &mut Self {
        self.sticky_col = StickyCol::default();
        self
    }

    /// Set cursor to stick to end-of-line or beginning-of-line.
    ///
    /// * End-of-line: `pos` == [DP::StickyCol] && at == "end"
    /// * Beginning-of-line: `pos` == [DP::StickyCol] && at == "home"
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

    /// Switch buffer to `Normal` mode.
    pub fn set_normal_mode(&mut self) {
        self.inner = match mem::replace(&mut self.inner, Inner::default()) {
            Inner::Insert(ib) => Inner::Normal(ib.into()),
            Inner::Replace(_rb) => todo!(),
            inner @ Inner::Normal(_) => inner,
            inner @ Inner::None => inner,
        };
    }

    /// Switch buffer to `Insert` mode.
    pub fn set_insert_mode(&mut self) {
        self.inner = match mem::replace(&mut self.inner, Inner::default()) {
            Inner::Normal(mut nb) => {
                let change = mem::replace(&mut nb.change, Default::default());
                Inner::Insert(InsertBuffer::new(1, change))
            }
            Inner::Replace(_rb) => todo!(),
            inner @ Inner::Insert(_) => inner,
            inner @ Inner::None => inner,
        };
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

    #[inline]
    fn n_lines(&self) -> usize {
        let change = &self.to_change();
        change.rope.len_lines()
    }

    fn to_last_line_idx(&self) -> usize {
        let n_lines = {
            let change = &self.to_change();
            change.rope.len_lines()
        };
        let n_chars = self.n_chars();
        for line_idx in (0..n_lines).rev() {
            let home = self.line_to_char(line_idx);
            if n_chars != home {
                return line_idx;
            }
            // n_chars == home, the last line is just `new-line`.
            // and the next iteration will break.
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
        self.location.is_read_only()
    }

    /// Return whether buffer is marked as modified.
    #[inline]
    pub fn is_modified(&self) -> bool {
        self.to_change().is_modified()
    }

    /// Return current buffer state as string.
    #[inline]
    pub fn to_mode(&self) -> &'static str {
        match &self.inner {
            Inner::Normal(_) => "normal",
            Inner::Insert(_) => "insert",
            Inner::Replace(_) => "replace",
            Inner::None => "none",
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

    /// Convert byte-index to valid character-index within buffer.
    #[inline]
    pub fn byte_to_char(&self, byte_idx: usize) -> usize {
        self.to_change().as_ref().byte_to_char(byte_idx)
    }

    /// Convert character-index to valid byte-index within buffer.
    #[inline]
    pub fn char_to_byte(&self, char_idx: usize) -> usize {
        self.to_change().rope.char_to_byte(char_idx)
    }
}

impl Buffer {
    #[inline]
    fn to_change(&self) -> cell::Ref<Change> {
        match &self.inner {
            Inner::Normal(val) => val.to_change(),
            Inner::Insert(val) => val.to_change(),
            Inner::Replace(val) => val.to_change(),
            Inner::None => unreachable!(),
        }
    }

    #[inline]
    fn as_mut_change(&mut self) -> &mut Rc<RefCell<Change>> {
        match &mut self.inner {
            Inner::Normal(val) => val.as_mut_change(),
            Inner::Insert(val) => val.as_mut_change(),
            Inner::Replace(val) => val.as_mut_change(),
            Inner::None => unreachable!(),
        }
    }

    #[inline]
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

    /// Return the last character index in buffer.
    ///
    /// Must be same as `buffer.n_chars().saturating_sub(1)`.
    pub fn last_char_idx(&self) -> usize {
        let row = self.to_last_line_idx();
        let col = text::visual_line_n(&self.line(row));
        xy_to_cursor(self, (row, col.saturating_sub(1)))
    }

    /// Convert `buffer[a..z]` into Spanline that can be rendered.
    pub fn to_span_line(&self, a: usize, z: usize) -> Result<Spanline> {
        let span: Span = {
            let iter = self.chars_at(a, DP::Right)?.take(z - a);
            String::from_iter(iter).into()
        };
        Ok(span.into())
    }
}

impl Buffer {
    /// Handle ZERO or more buffer specific events. If event cannot be
    /// handled, they are simply retured as is, preserving their order
    /// in which they are present in the `evnts` list.
    pub fn on_event(&mut self, evnts: Event) -> Result<Event> {
        match &self.inner {
            Inner::Normal(_) => NormalBuffer::on_event(self, evnts),
            Inner::Insert(_) => InsertBuffer::on_event(self, evnts),
            Inner::Replace(_) => todo!(),
            Inner::None => err_at!(Fatal, msg: format!("unreachable")),
        }
    }
}

/// Create-Update-Delete operations on buffer.
impl Buffer {
    /// Insert `n` newlines, specified by `nl`, at `cursor` position.
    /// If cursor is `None`, use the current cursor position. Return
    /// the new cursor postion.
    #[inline]
    pub fn cud_newlines(&mut self, cursor: Option<usize>, nl: &str, n: usize) -> Result<usize> {
        self.inner.cud_newlines(cursor, nl, n)
    }

    /// Insert character `ch` at `cursor` position. If cursor is None, use the
    /// current cursor position. Return the new cursor position.
    #[inline]
    pub fn cud_char(&mut self, cursor: Option<usize>, ch: char) -> Result<usize> {
        self.inner.cud_char(cursor, ch)
    }

    /// Insert string `txt` at `cursor position. If cursor is None, use the
    /// current cursor position. Return the new cursor position.
    #[inline]
    pub fn cud_str(&mut self, cursor: Option<usize>, txt: &str) -> Result<usize> {
        self.inner.cud_str(cursor, txt)
    }

    /// Delete text between the specified range. Range shall be specified
    /// as character-index.
    #[inline]
    pub fn cud_delete<R>(&mut self, range: R) -> Result<()>
    where
        R: RangeBounds<usize>,
    {
        self.inner.cud_delete(range)
    }
}

#[derive(Clone)]
struct NormalBuffer {
    i_evnts: Event,
    change: Rc<RefCell<Change>>,
}

impl From<InsertBuffer> for NormalBuffer {
    fn from(ib: InsertBuffer) -> NormalBuffer {
        NormalBuffer {
            i_evnts: ib.i_evnts,
            change: ib.change,
        }
    }
}

impl NormalBuffer {
    fn new(buf: Rope) -> NormalBuffer {
        NormalBuffer {
            i_evnts: Event::default(),
            change: Change::start(buf),
        }
    }

    fn set_cursor(&mut self, cursor: usize) {
        self.to_mut_change().set_cursor(cursor);
    }

    #[inline]
    fn cud_newlines(&mut self, cursor: Option<usize>, nl: &str, n: usize) -> Result<usize> {
        self.to_mut_change().cud_newlines(cursor, nl, n)
    }

    #[inline]
    fn cud_char(&mut self, cursor: Option<usize>, ch: char) -> Result<usize> {
        self.to_mut_change().cud_char(cursor, ch)
    }

    #[inline]
    fn cud_str(&mut self, cursor: Option<usize>, text: &str) -> Result<usize> {
        self.to_mut_change().cud_str(cursor, text)
    }

    #[inline]
    fn cud_delete<R>(&mut self, range: R) -> Result<()>
    where
        R: RangeBounds<usize>,
    {
        self.to_mut_change().cud_delete(range)
    }
}

impl NormalBuffer {
    fn to_change(&self) -> cell::Ref<Change> {
        self.change.as_ref().borrow()
    }

    fn to_mut_change(&mut self) -> cell::RefMut<Change> {
        self.change.as_ref().borrow_mut()
    }

    fn as_mut_change(&mut self) -> &mut Rc<RefCell<Change>> {
        &mut self.change
    }
}

impl NormalBuffer {
    fn on_event(buf: &mut Buffer, mut evnts: Event) -> Result<Event> {
        let mut res_evnts = Event::Noop;
        loop {
            match evnts.next() {
                Some(evnt) => {
                    res_evnts.push(Self::do_on_event(buf, evnt)?);
                    match &buf.inner {
                        Inner::Normal(_) => (),
                        Inner::Insert(_) => {
                            res_evnts.push(buf.on_event(evnts)?);
                            break;
                        }
                        Inner::Replace(_) => todo!(),
                        Inner::None => err_at!(Fatal, msg: format!("unreachable"))?,
                    }
                }
                None => break,
            };
        }
        Ok(res_evnts)
    }

    fn do_on_event(buf: &mut Buffer, evnt: Event) -> Result<Event> {
        debug!("{}", evnt);

        let evnt = match evnt {
            // first, try switching to insert mode, if event is insert.
            Event::Md(Mod::Insert(n, pos)) if n > 0 => mod_insert(buf, n, pos)?,
            Event::Md(Mod::Append(n, pos)) if n > 0 => mod_append(buf, n, pos)?,
            Event::Md(Mod::Open(n, pos)) if n > 0 => mod_open(buf, n, pos)?,
            Event::Md(Mod::Insert(_, _)) => Event::Noop,
            Event::Md(Mod::Append(_, _)) => Event::Noop,
            Event::Md(Mod::Open(_, _)) => Event::Noop,
            // motion command - characterwise.
            Event::Mt(Mto::Left(n, dp)) => {
                let cursor = mto_left(buf, n, dp)?;
                buf.set_cursor(cursor).clear_sticky_col();
                Event::Noop
            }
            Event::Mt(Mto::Right(n, dp)) => {
                let cursor = mto_right(buf, n, dp)?;
                buf.set_cursor(cursor).clear_sticky_col();
                Event::Noop
            }
            Event::Mt(Mto::LineHome(dp)) => {
                let cursor = mto_line_home(buf, dp)?;
                buf.set_cursor(cursor).set_sticky_col(dp, "home");
                Event::Noop
            }
            Event::Mt(Mto::LineEnd(n, dp)) => {
                let cursor = mto_line_end(buf, n, dp)?;
                buf.set_cursor(cursor).set_sticky_col(dp, "end");
                Event::Noop
            }
            Event::Mt(Mto::LineMiddle(p, _)) if p < 1 => {
                let cursor = mto_line_middle(buf, 50)?;
                buf.set_cursor(cursor).clear_sticky_col();
                Event::Noop
            }
            Event::Mt(Mto::LineMiddle(p, _)) => {
                let cursor = mto_line_middle(buf, p)?;
                buf.set_cursor(cursor).clear_sticky_col();
                Event::Noop
            }
            Event::Mt(Mto::Col(n)) => {
                let cursor = mto_column(buf, n)?;
                buf.set_cursor(cursor).clear_sticky_col();
                Event::Noop
            }
            Event::Mt(e @ Mto::CharF(_, _, _)) => {
                buf.mto_find_char = e.clone();
                let cursor = mto_char(buf, e)?;
                buf.set_cursor(cursor).clear_sticky_col();
                Event::Noop
            }
            Event::Mt(e @ Mto::CharT(_, _, _)) => {
                buf.mto_find_char = e.clone();
                let cursor = mto_char(buf, e)?;
                buf.set_cursor(cursor).clear_sticky_col();
                Event::Noop
            }
            Event::Mt(Mto::CharR(n, dir)) => {
                let e = buf.mto_find_char.clone();
                let cursor = mto_char(buf, e.dir_xor(n, dir)?)?;
                buf.set_cursor(cursor).clear_sticky_col();
                Event::Noop
            }
            // motion command - linewise.
            Event::Mt(Mto::Up(n, dp)) => {
                let cursor = mto_up(buf, n, dp)?;
                buf.set_cursor(cursor);
                Event::Noop
            }
            Event::Mt(Mto::Down(n, dp)) => {
                let cursor = mto_down(buf, n, dp)?;
                buf.set_cursor(cursor);
                Event::Noop
            }
            Event::Mt(Mto::Row(n, dp)) => {
                let n = n.saturating_sub(1);
                let cursor = mto_row(buf, n, dp)?;
                buf.set_cursor(cursor);
                Event::Noop
            }
            Event::Mt(Mto::Percent(n, dp)) => {
                let cursor = mto_percent(buf, n, dp)?;
                buf.set_cursor(cursor);
                Event::Noop
            }
            Event::Mt(Mto::Cursor(n)) => {
                let cursor = mto_cursor(buf, n)?;
                buf.set_cursor(cursor);
                Event::Noop
            }
            // motion command - word/sentence/para wise
            Event::Mt(Mto::Word(n, DP::Left, pos)) => {
                let cursor = mto_words_left(buf, n, pos)?;
                buf.set_cursor(cursor).clear_sticky_col();
                Event::Noop
            }
            Event::Mt(Mto::Word(n, DP::Right, pos)) => {
                let cursor = mto_words_right(buf, n, pos)?;
                buf.set_cursor(cursor).clear_sticky_col();
                Event::Noop
            }
            Event::Mt(Mto::WWord(n, DP::Left, pos)) => {
                let cursor = mto_wwords_left(buf, n, pos)?;
                buf.set_cursor(cursor).clear_sticky_col();
                Event::Noop
            }
            Event::Mt(Mto::WWord(n, DP::Right, pos)) => {
                let cursor = mto_wwords_right(buf, n, pos)?;
                buf.set_cursor(cursor).clear_sticky_col();
                Event::Noop
            }
            Event::Mt(Mto::Sentence(n, DP::Left)) => {
                let cursor = mto_sentence_left(buf, n)?;
                buf.set_cursor(cursor).clear_sticky_col();
                Event::Noop
            }
            Event::Mt(Mto::Sentence(n, DP::Right)) => {
                let cursor = mto_sentence_right(buf, n)?;
                buf.set_cursor(cursor).clear_sticky_col();
                Event::Noop
            }
            Event::Mt(Mto::Para(n, DP::Left)) => {
                let cursor = mto_paras_left(buf, n)?;
                buf.set_cursor(cursor).clear_sticky_col();
                Event::Noop
            }
            Event::Mt(Mto::Para(n, DP::Right)) => {
                let cursor = mto_paras_right(buf, n)?;
                buf.set_cursor(cursor).clear_sticky_col();
                Event::Noop
            }
            // motion command, other motions.
            Event::Mt(Mto::MatchPair) => {
                let cursor = mto_match_pair(buf)?;
                buf.set_cursor(cursor).clear_sticky_col();
                Event::Noop
            }
            Event::Mt(Mto::UnmatchPair(n, ch, dir)) => {
                let cursor = mto_unmatch_pair(buf, ch, n, dir)?;
                buf.set_cursor(cursor).clear_sticky_col();
                Event::Noop
            }
            // motion command marks and jumps
            Event::Mt(Mto::Jump(typ, mindex)) => {
                let mrk = mark::get_mark(&buf.marks, mindex);
                match mrk {
                    Some(mrk) if typ == '`' => {
                        let cursor = mrk.to_cursor();
                        buf.set_cursor(cursor).clear_sticky_col();
                        Event::Noop
                    }
                    Some(mrk) if typ == '\'' => {
                        buf.set_cursor(mrk.to_cursor());
                        let cursor = mto_line_home(buf, DP::TextCol)?;
                        buf.set_cursor(cursor).clear_sticky_col();
                        Event::Noop
                    }
                    _ => Event::Mt(Mto::Jump(typ, mindex)),
                }
            }
            Event::Mr(mrk) => match mrk.to_index() {
                'a'..='z' | '\'' | '`' => {
                    let mrk = mrk.into_mark(buf);
                    mark::set_mark(&mut buf.marks, mrk);
                    Event::Noop
                }
                'A'..='Z' => Event::Mr(mrk.into_mark(buf)),
                _ => Event::Noop,
            },
            Event::Mt(e @ Mto::Bracket(_, _, _, _)) => mto_bracket(buf, e)?,
            Event::Mt(e @ Mto::Pattern(_, Some(_), _)) => {
                buf.mto_pattern = e.clone();

                let cursor = mto_pattern(buf, e)?;
                buf.set_cursor(cursor).clear_sticky_col();
                Event::Noop
            }
            Event::Mt(Mto::PatternR(n, dir)) => {
                let e = buf.mto_pattern.clone();

                let cursor = mto_pattern(buf, e.dir_xor(n, dir)?)?;
                buf.set_cursor(cursor).clear_sticky_col();
                Event::Noop
            }
            evnt => evnt,
        };

        Ok(evnt)
    }
}

#[derive(Clone)]
struct InsertBuffer {
    repeat: usize,
    i_evnts: Event,
    change: Rc<RefCell<Change>>,
}

impl InsertBuffer {
    fn new(repeat: usize, change: Rc<RefCell<Change>>) -> Self {
        InsertBuffer {
            repeat: repeat.saturating_sub(1),
            i_evnts: Event::default(),
            change,
        }
    }

    fn set_cursor(&mut self, cursor: usize) {
        self.to_mut_change().set_cursor(cursor)
    }

    #[inline]
    fn cud_newlines(&mut self, cursor: Option<usize>, nl: &str, n: usize) -> Result<usize> {
        self.to_mut_change().cud_newlines(cursor, nl, n)
    }

    #[inline]
    fn cud_char(&mut self, cursor: Option<usize>, ch: char) -> Result<usize> {
        self.to_mut_change().cud_char(cursor, ch)
    }

    #[inline]
    fn cud_str(&mut self, cursor: Option<usize>, text: &str) -> Result<usize> {
        self.to_mut_change().cud_str(cursor, text)
    }

    #[inline]
    fn cud_delete<R>(&mut self, range: R) -> Result<()>
    where
        R: RangeBounds<usize>,
    {
        self.to_mut_change().cud_delete(range)
    }
}

impl InsertBuffer {
    fn to_change(&self) -> cell::Ref<Change> {
        self.change.as_ref().borrow()
    }

    fn to_mut_change(&mut self) -> cell::RefMut<Change> {
        self.change.as_ref().borrow_mut()
    }

    fn as_mut_change(&mut self) -> &mut Rc<RefCell<Change>> {
        &mut self.change
    }
}

impl InsertBuffer {
    fn do_repeat(repeat: usize, i_evnts: Event, buf: &mut Buffer) -> Result<Event> {
        let mut res_evnts = Event::Noop;
        // repeat insert, if any, before exiting the insert-mode.
        for _ in 0..repeat {
            for evnt in i_evnts.clone().into_iter() {
                res_evnts.push(Self::do_on_event(buf, evnt)?);
            }
        }
        Ok(res_evnts)
    }

    fn insert_event(&mut self, evnt: Event) -> Result<()> {
        self.i_evnts.push(evnt);
        Ok(())
    }

    fn on_event(buf: &mut Buffer, mut evnts: Event) -> Result<Event> {
        use crate::event::Event::*;

        let mut res_evnts = Event::Noop;
        let evnt = loop {
            let evnt = match evnts.next() {
                // first, handle mode events.
                Some(Md(Mod::Esc)) => {
                    let evnt = Inner::do_repeat(buf)?;
                    let cursor = mto_left(buf, 1, DP::LineBound)?;
                    buf.set_cursor(cursor).clear_sticky_col();
                    buf.inner = {
                        let inner = mem::replace(&mut buf.inner, Inner::default());
                        inner.into_normal_buffer()?
                    };
                    break evnt;
                }
                // then other evnts
                Some(Event::Noop) => Event::Noop,
                Some(evnt @ TabInsert(_)) => Self::do_on_event(buf, evnt)?,
                Some(evnt @ TabClear) => Self::do_on_event(buf, evnt)?,
                Some(evnt) => {
                    // save into this insert-session.
                    buf.inner.insert_event(evnt.clone())?;
                    Self::do_on_event(buf, evnt)?
                }
                None => break Event::Noop,
            };
            res_evnts.push(evnt)
        };
        res_evnts.push(evnt);
        Ok(res_evnts)
    }

    fn do_on_event(buf: &mut Buffer, evnt: Event) -> Result<Event> {
        use crate::event::{self, Cud, Event::*};
        use std::iter::repeat;

        debug!("{}", evnt);

        // handle tab-rotation for completion.
        let evnt = match (buf.tab_state.clone(), evnt) {
            (TabState::None, TabInsert(newt)) => {
                let cursor = {
                    let cursor = buf.to_char_cursor();
                    buf.cud_str(Some(cursor), newt.as_str())?
                };
                buf.set_cursor(cursor).clear_sticky_col();

                buf.tab_state = TabState::Active(newt.clone());
                Edit(event::Edit::new_ins(cursor, newt))
            }
            (TabState::None, TabClear) => Event::Noop,
            (TabState::None, evnt) => evnt,
            (TabState::Active(oldt), TabInsert(newt)) => {
                let cursor = {
                    let cursor = buf.to_char_cursor();
                    cursor.saturating_sub(text::width(oldt.chars()))
                };
                let to = cursor + oldt.chars().count();
                buf.cud_delete(cursor..to)?;
                let cursor = buf.cud_str(Some(cursor), newt.as_str())?;
                buf.set_cursor(cursor).clear_sticky_col();

                buf.tab_state = TabState::Active(newt.clone());
                Edit(event::Edit::new_chg(cursor, oldt, newt))
            }
            (TabState::Active(oldt), TabClear) => {
                let cursor = {
                    let cursor = buf.to_char_cursor();
                    cursor.saturating_sub(text::width(oldt.chars()))
                };
                let to = cursor + oldt.chars().count();
                buf.cud_delete(cursor..to)?;
                buf.set_cursor(cursor);

                buf.tab_state = TabState::default();
                Edit(event::Edit::new_del(cursor, oldt))
            }
            (TabState::Active(_), evnt) => {
                buf.tab_state = TabState::default();
                evnt
            }
        };

        let evnt = match evnt {
            // cursor movement
            Mt(Mto::Left(n, dp)) => {
                let cursor = mto_left(buf, n, dp)?;
                buf.set_cursor(cursor).clear_sticky_col();
                Event::Noop
            }
            Mt(Mto::Right(n, dp)) => {
                let cursor = mto_right(buf, n, dp)?;
                buf.set_cursor(cursor).clear_sticky_col();
                Event::Noop
            }
            Mt(Mto::Up(n, dp)) => {
                let cursor = mto_up(buf, n, dp)?;
                buf.set_cursor(cursor);
                Event::Noop
            }
            Mt(Mto::Down(n, dp)) => {
                let cursor = mto_down(buf, n, dp)?;
                buf.set_cursor(cursor);
                Event::Noop
            }
            Mt(Mto::LineHome(dp)) => {
                let cursor = mto_line_home(buf, dp)?;
                buf.set_cursor(cursor).set_sticky_col(dp, "home");
                Event::Noop
            }
            Mt(Mto::LineEnd(n, dp)) => {
                let cursor = mto_line_end(buf, n, dp)?;
                buf.set_cursor(cursor).set_sticky_col(dp, "end");
                Event::Noop
            }
            // insert session
            Wr(Cud::Char(ch)) => {
                let cursor = buf.to_char_cursor();
                {
                    let cursor = buf.cud_char(None, ch)?;
                    buf.set_cursor(cursor).clear_sticky_col();
                }
                Edit(event::Edit::new_ins(cursor, ch.into()))
            }
            Wr(Cud::Tab(n)) => {
                let cursor = buf.to_char_cursor();
                let n = buf.shift_width * n;
                let txt = String::from_iter(repeat(' ').take(n));
                {
                    let cursor = buf.cud_str(None, &txt)?;
                    buf.set_cursor(cursor).clear_sticky_col();
                }
                Edit(event::Edit::new_ins(cursor, txt))
            }
            Wr(Cud::Enter(n)) => {
                let cursor = buf.to_char_cursor();
                let to = buf.cud_newlines(Some(cursor), buf.format.newline(), n)?;
                buf.set_cursor(cursor).clear_sticky_col();
                Edit(event::Edit::new_ins(cursor, buf.slice(cursor..to)))
            }
            Wr(Cud::Backspace(n)) if buf.to_char_cursor() > 0 => {
                let cursor = buf.to_char_cursor();
                let from = cursor.saturating_sub(n);
                buf.cud_delete(from..cursor)?;
                buf.set_cursor(from);
                Edit(event::Edit::new_del(cursor, buf.slice(from..cursor)))
            }
            Wr(Cud::Delete(n)) => {
                let cursor = buf.to_char_cursor();
                let to = cmp::min(cursor.saturating_add(n), buf.n_chars());
                buf.cud_delete(cursor..to)?;
                Edit(event::Edit::new_del(cursor, buf.slice(cursor..to)))
            }
            evnt => evnt,
        };

        Ok(evnt)
    }
}

#[derive(Clone)]
struct ReplaceBuffer {
    change: Rc<RefCell<Change>>,
}

impl From<NormalBuffer> for ReplaceBuffer {
    fn from(nb: NormalBuffer) -> ReplaceBuffer {
        ReplaceBuffer { change: nb.change }
    }
}

impl ReplaceBuffer {
    fn set_cursor(&mut self, cursor: usize) {
        self.to_mut_change().set_cursor(cursor)
    }

    #[inline]
    fn cud_newlines(&mut self, cursor: Option<usize>, nl: &str, n: usize) -> Result<usize> {
        self.to_mut_change().cud_newlines(cursor, nl, n)
    }

    #[inline]
    fn cud_char(&mut self, _cursor: Option<usize>, _ch: char) -> Result<usize> {
        todo!()
    }

    #[inline]
    fn cud_str(&mut self, _cursor: Option<usize>, _text: &str) -> Result<usize> {
        todo!()
    }

    #[inline]
    fn cud_delete<R>(&mut self, _range: R) -> Result<()>
    where
        R: RangeBounds<usize>,
    {
        todo!()
    }
}

impl ReplaceBuffer {
    fn to_change(&self) -> cell::Ref<Change> {
        self.change.as_ref().borrow()
    }

    fn to_mut_change(&mut self) -> cell::RefMut<Change> {
        self.change.as_ref().borrow_mut()
    }

    fn as_mut_change(&mut self) -> &mut Rc<RefCell<Change>> {
        &mut self.change
    }
}

// A change captures a single session of CUD commands. The main
// purpose is to implement undo/redo and few other associated features.
//
// NOTE: Visualize the change tree, maintained via `past` and `news`
// references, as inverted tree of changes, with root being the oldest
// change.
#[derive(Clone)]
struct Change {
    // persistent clone shared among all previous changes.
    rope: Rope,
    // a change always come from a single past.
    past: Option<rc::Weak<RefCell<Change>>>,
    // and it can have ZERO or more futures.
    news: Vec<Rc<RefCell<Change>>>,
    // redo path, offset into `news`.
    redo: Option<usize>,
    // list of events for this change-session.
    cuds: Event,
    // last and latest cursor position for this change.
    cursor: usize,
}

impl Default for Change {
    fn default() -> Change {
        Change {
            rope: Rope::from_reader(io::empty()).unwrap(),
            past: None,
            news: Vec::default(),
            redo: None,
            cuds: Event::Noop,
            cursor: 0,
        }
    }
}

impl AsRef<Rope> for Change {
    fn as_ref(&self) -> &Rope {
        &self.rope
    }
}

impl Change {
    // Start from the first change.
    fn start(rope: Rope) -> Rc<RefCell<Change>> {
        Rc::new(RefCell::new(Change {
            rope,
            past: None,
            news: Vec::default(),
            redo: None,
            cuds: Event::Noop,
            cursor: 0,
        }))
    }

    // fork from current `change`, where current change can be the
    // leaf node of the inverted tree, or an intermediate node
    // of the inverted tree.
    //
    // a fork always create a new leaf.
    fn fork(change: &mut Rc<RefCell<Change>>) -> Rc<RefCell<Change>> {
        let leaf = {
            let cc: &Change = &change.as_ref().borrow();
            Rc::new(RefCell::new(Change {
                rope: cc.as_ref().clone(),
                past: Some(Rc::downgrade(change)),
                news: Vec::default(),
                redo: None,
                cuds: Event::Noop,
                cursor: cc.cursor, // inherit the cursor position.
            }))
        };
        {
            let cc: &mut Change = &mut change.as_ref().borrow_mut();
            let redo = cc.news.len();
            cc.news.push(Rc::clone(&leaf));
            cc.redo = Some(redo);
        }
        leaf
    }

    #[inline]
    fn to_char_cursor(&self) -> usize {
        self.cursor
    }

    fn to_xy_cursor(&self, cursor: Option<usize>) -> Cursor {
        let cursor = cursor.unwrap_or(self.cursor);
        let row_at = self.rope.char_to_line(cursor);
        let col_at = cursor.saturating_sub(self.rope.line_to_char(row_at));
        (col_at, row_at).into()
    }

    fn is_modified(&self) -> bool {
        let mut modified = match self.cuds {
            Event::Noop => false,
            _ => true,
        };
        modified = modified || self.past.is_some();
        modified = modified || self.news.len() > 0;
        modified
    }
}

impl Change {
    fn set_cursor(&mut self, cursor: usize) {
        self.cursor = cursor;
    }

    fn cud_newlines(&mut self, cursor: Option<usize>, nl: &str, n: usize) -> Result<usize> {
        let cursor = cursor.unwrap_or(self.cursor);

        let mut new_line = String::default();
        (0..n).for_each(|_| new_line.push_str(nl));
        self.rope.insert(cursor, &new_line);

        Ok(cursor + n)
    }

    fn cud_char(&mut self, cursor: Option<usize>, ch: char) -> Result<usize> {
        let cursor = cursor.unwrap_or(self.cursor);
        self.rope.insert_char(cursor, ch);
        Ok(cursor + 1)
    }

    fn cud_str(&mut self, cursor: Option<usize>, txt: &str) -> Result<usize> {
        let cursor = cursor.unwrap_or(self.cursor);
        self.rope.insert(cursor, txt);
        Ok(cursor + text::width(txt.chars()))
    }

    fn cud_delete<R>(&mut self, range: R) -> Result<()>
    where
        R: RangeBounds<usize>,
    {
        use std::ops::Bound::{Excluded, Included, Unbounded};

        let n_chars = self.rope.len_chars();

        let from = match range.start_bound() {
            Included(from) => cmp::min(*from, n_chars.saturating_sub(1)),
            Excluded(from) => cmp::min(from.saturating_add(1), n_chars),
            Unbounded => 0,
        };

        let to = match range.end_bound() {
            Included(to) => cmp::min((*to).saturating_add(1), n_chars),
            Excluded(to) => cmp::min(*to, n_chars),
            Unbounded => n_chars,
        };

        if from < to {
            self.rope.remove(from..to);
        }
        Ok(())
    }
}

fn mto_left(buf: &Buffer, mut n: usize, dp: DP) -> Result<usize> {
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

fn mto_right(buf: &Buffer, mut n: usize, dp: DP) -> Result<usize> {
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

fn mto_line_home(buf: &Buffer, pos: DP) -> Result<usize> {
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

fn mto_line_end(buf: &Buffer, n: usize, dp: DP) -> Result<usize> {
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

fn mto_line_middle(buf: &Buffer, p: usize) -> Result<usize> {
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

fn mto_column(buf: &Buffer, n: usize) -> Result<usize> {
    let home = buf.to_line_home(None);
    let n = {
        let s = buf.line(buf.char_to_line(buf.to_char_cursor()));
        cmp::min(text::visual_line_n(&s), n)
    };
    Ok(home + n.saturating_sub(1))
}

fn mto_char(buf: &Buffer, evnt: Mto) -> Result<usize> {
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

fn mto_up(buf: &Buffer, n: usize, dp: DP) -> Result<usize> {
    let bc_xy = buf.to_xy_cursor(None);
    let row = bc_xy.row.saturating_sub(n);
    let line = &buf.line(row);
    let char_end = text::visual_line_n(&line).saturating_sub(1);
    let col = cmp::min(bc_xy.col, char_end);

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

fn mto_down(buf: &Buffer, n: usize, dp: DP) -> Result<usize> {
    let row = {
        let row = buf.char_to_line(buf.to_char_cursor()) + n;
        cmp::min(row, buf.to_last_line_idx())
    };
    let char_end = text::visual_line_n(&buf.line(row)).saturating_sub(1);
    let col = cmp::min(buf.to_xy_cursor(None).col, char_end);

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

fn mto_row(buf: &Buffer, n: usize, dp: DP) -> Result<usize> {
    let cursor = buf.line_to_char(cmp::min(n, buf.to_last_line_idx()));
    let cursor = match dp {
        DP::TextCol => {
            let xy = buf.to_xy_cursor(Some(cursor));
            cursor + skip_whitespace(&buf.line(xy.row), xy.col, DP::Right)?
        }
        _ => cursor,
    };
    Ok(cursor)
}

fn mto_percent(buf: &Buffer, n: usize, dp: DP) -> Result<usize> {
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

fn mto_end(buf: &Buffer) -> Result<usize> {
    let line_idx = buf.to_last_line_idx();
    let n = text::visual_line_n(&buf.line(line_idx));
    Ok(buf.line_to_char(line_idx) + n.saturating_sub(1))
}

fn mto_cursor(buf: &Buffer, n: usize) -> Result<usize> {
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
                let n = cmp::min(bc_xy.col + 1, chars.len());
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
                None if $pos == DP::Start => break $buf.last_char_idx(),
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
                None => break $buf.last_char_idx(),
            };
        };
        Ok(saturate_cursor($buf, cursor))
    }};
}

fn mto_words_left(buf: &Buffer, n: usize, pos: DP) -> Result<usize> {
    mto_text_left!(buf, n, pos, MtoWord::St(n), MtoWord)
}

fn mto_words_right(buf: &Buffer, n: usize, pos: DP) -> Result<usize> {
    mto_text_right!(buf, n, pos, MtoWord::St(n), MtoWord)
}

fn mto_wwords_left(buf: &Buffer, n: usize, pos: DP) -> Result<usize> {
    mto_text_left!(buf, n, pos, MtoWWord::St(n), MtoWWord)
}

fn mto_wwords_right(buf: &Buffer, n: usize, pos: DP) -> Result<usize> {
    mto_text_right!(buf, n, pos, MtoWWord::St(n), MtoWWord)
}

fn mto_sentence_left(buf: &Buffer, n: usize) -> Result<usize> {
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

fn mto_sentence_right(buf: &Buffer, n: usize) -> Result<usize> {
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

fn mto_paras_left(buf: &Buffer, mut n: usize) -> Result<usize> {
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

fn mto_paras_right(buf: &Buffer, mut n: usize) -> Result<usize> {
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

fn mto_match_pair(buf: &Buffer) -> Result<usize> {
    use crate::match_pair;

    let cursor = buf.to_char_cursor();
    Ok(match_pair::match_under_cursor(buf).unwrap_or(cursor))
}

fn mto_unmatch_pair(buf: &Buffer, ch: char, n: usize, dir: DP) -> Result<usize> {
    use crate::match_pair;

    let cursor = buf.to_char_cursor();
    let cursor = match dir {
        DP::Left => match_pair::unmatch_before(buf, ch, n).unwrap_or(cursor),
        DP::Right => match_pair::unmatch_after(buf, ch, n).unwrap_or(cursor),
        _ => cursor,
    };
    Ok(cursor)
}

fn mto_bracket(buf: &mut Buffer, e: Mto) -> Result<Event> {
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

fn mto_pattern(buf: &mut Buffer, evnt: Mto) -> Result<usize> {
    use regex::Regex;

    let (n, patt, dp) = match evnt.clone() {
        Mto::Pattern(n, Some(patt), dp) => {
            let msg = format!("{}", patt);
            let patt = err_at!(BadPattern, Regex::new(&patt), msg)?;
            (n, patt, dp)
        }
        _ => err_at!(Fatal, msg: format!("unreachable"))?,
    };

    let get_matches = |txt: String| -> Vec<(usize, usize)> {
        let iter = patt.find_iter(&txt).map(|m| (m.start(), m.end()));
        iter.collect()
    };

    let cursor = buf.to_char_cursor();
    match dp {
        DP::Left => {
            let txt = buf.slice(..cursor);
            let matches: Vec<(usize, usize)> = get_matches(txt);
            match matches.into_iter().rev().skip(n.saturating_sub(1)).next() {
                Some((start, _)) => Ok(start),
                None => Ok(cursor),
            }
        }
        DP::Right => {
            let from = cursor.saturating_add(1);
            let txt = buf.slice(from..);
            let matches: Vec<(usize, usize)> = get_matches(txt);
            match matches.into_iter().skip(n.saturating_sub(1)).next() {
                Some((start, _)) => Ok(from + start),
                None => Ok(cursor),
            }
        }
        _ => err_at!(Fatal, msg: format!("unreachable"))?,
    }
}

fn mod_insert(buf: &mut Buffer, repeat: usize, pos: DP) -> Result<Event> {
    if pos == DP::TextCol {
        let cursor = mto_line_home(buf, pos)?;
        buf.set_cursor(cursor).set_sticky_col(pos, "home");
    }
    buf.inner = {
        let ib = InsertBuffer::new(repeat, Change::fork(buf.as_mut_change()));
        ib.into()
    };
    Ok(Event::Noop)
}

fn mod_append(buf: &mut Buffer, repeat: usize, pos: DP) -> Result<Event> {
    let cursor = match pos {
        DP::End => {
            let cursor = buf.to_char_cursor();
            let n = text::visual_line_n(&buf.line(buf.char_to_line(cursor)));
            buf.to_line_home(None) + n
        }
        DP::Right => buf.to_char_cursor().saturating_add(1),
        _ => err_at!(Fatal, msg: format!("unreachable"))?,
    };
    buf.set_cursor(cursor).clear_sticky_col();

    buf.inner = {
        let ib = InsertBuffer::new(repeat, Change::fork(buf.as_mut_change()));
        ib.into()
    };
    Ok(Event::Noop)
}

fn mod_open(buf: &mut Buffer, repeat: usize, pos: DP) -> Result<Event> {
    match pos {
        DP::Left => {
            let cursor = {
                let cursor = buf.line_to_char(buf.to_xy_cursor(None).row);
                buf.cud_newlines(Some(cursor), buf.format.newline(), 1)?;
                cursor
            };
            buf.set_cursor(cursor).clear_sticky_col();
        }
        DP::Right => {
            let cursor = {
                let cursor = buf.to_char_cursor();
                let n = text::visual_line_n(&buf.line(buf.char_to_line(cursor)));
                buf.to_line_home(None) + n
            };
            let cursor = buf.cud_newlines(Some(cursor), buf.format.newline(), 1)?;
            buf.set_cursor(cursor).clear_sticky_col();
        }
        _ => err_at!(Fatal, msg: format!("unreachable"))?,
    }
    buf.inner = {
        let ib = InsertBuffer::new(repeat, Change::fork(buf.as_mut_change()));
        ib.into()
    };
    Ok(Event::Noop)
}

/// Function to skip whitespace in a line.
///
/// Cursor position within the line is specified by `off` and direction
/// in which to skip the whitespace is specified by `dp`. Return the number
/// of position skipped.
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

/// Bi-directional, line-by-line iterator for Buffer.
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

/// Bi-directional, char-by-char iterator for Buffer.
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

#[inline]
fn saturate_cursor(buf: &Buffer, cursor: usize) -> usize {
    if_else!(cursor >= buf.n_chars(), buf.last_char_idx(), cursor)
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
