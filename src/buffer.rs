use lazy_static::lazy_static;
use log::trace;
use ropey::{self, Rope, RopeSlice};

use std::{
    borrow::Borrow,
    cell::{self, RefCell},
    cmp, fmt, io, mem,
    ops::Bound,
    rc::{self, Rc},
    result,
    sync::Mutex,
};

use crate::{
    event::{Event, Mto, DP},
    ftypes::FType,
    keymap::Keymap,
    location::Location,
    search::Search,
    window::Context,
    {err_at, Error, Result},
};

pub const NL: char = '\n';

lazy_static! {
    static ref BUFFER_NUM: Mutex<usize> = Mutex::new(0);
}

// Cursor within the buffer, starts from (0, 0)
#[derive(Clone, Copy, Default, Debug, Eq, PartialEq)]
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
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.row == other.row {
            self.col.partial_cmp(&other.col)
        } else {
            self.row.partial_cmp(&other.row)
        }
    }
}

impl Ord for Cursor {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        if self.row == other.row {
            self.row.cmp(&other.row)
        } else {
            self.col.cmp(&other.col)
        }
    }
}

// all bits and pieces of content is managed by buffer.
#[derive(Clone)]
pub struct Buffer {
    pub num: usize, // buffer number
    pub location: Location,
    pub read_only: bool,
    pub insert_only: bool,
    pub mto_pattern: Mto,
    pub mto_find_char: Mto,
    pub insert_repeat: usize,
    pub last_inserts: Vec<Event>,
    pub keymap: Keymap,
    pub ftype: FType,

    inner: Inner,
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

impl Buffer {
    pub fn from_reader<R>(data: R) -> Result<Buffer>
    where
        R: io::Read,
    {
        let buf = err_at!(FailBuffer, Rope::from_reader(data))?;
        let mut num = BUFFER_NUM.lock().unwrap();
        *num = *num + 1;
        let b = Buffer {
            num: *num,
            location: Default::default(),
            read_only: false,
            insert_only: false,
            insert_repeat: Default::default(),
            last_inserts: Default::default(),
            mto_find_char: Default::default(),
            mto_pattern: Default::default(),
            keymap: Default::default(),
            ftype: Default::default(),

            inner: Inner::Normal(NormalBuffer::new(buf)),
        };

        Ok(b)
    }

    pub fn empty() -> Result<Buffer> {
        let buf = vec![];
        Self::from_reader(buf.as_slice())
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

    pub fn set_insert_only(&mut self, insert_only: bool) -> &mut Self {
        self.insert_only = insert_only;
        self
    }

    pub fn set_keymap(&mut self, km: Keymap) -> &mut Self {
        self.keymap = km;
        self
    }

    pub fn set_ftype(&mut self, ftype: FType) -> &mut Self {
        self.ftype = ftype;
        self
    }

    pub fn set_event_prefix(&mut self, prefix: Event) -> &mut Self {
        match &mut self.inner {
            Inner::Insert(_) => (),
            Inner::Normal(NormalBuffer { evnt_prefix, .. }) => {
                *evnt_prefix = prefix;
            }
        };
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
    #[inline]
    pub fn is_read_only(&self) -> bool {
        self.read_only
    }

    #[inline]
    pub fn is_insert_only(&self) -> bool {
        self.insert_only
    }

    #[inline]
    pub fn is_modified(&self) -> bool {
        let change = self.to_change();
        change.parent.is_some() || !change.children.is_empty()
    }

    #[inline]
    pub fn to_mode(&self) -> &'static str {
        match &self.inner {
            Inner::Insert(_) => "insert",
            Inner::Normal(_) => "normal",
        }
    }

    #[inline]
    pub fn to_id(&self) -> String {
        match self.to_location() {
            Location::Anonymous(s) => s,
            Location::Disk(s) => s.to_str().unwrap().to_string(),
        }
    }

    #[inline]
    pub fn to_num(&self) -> usize {
        self.num
    }

    #[inline]
    pub fn to_file_type(&self) -> String {
        self.ftype.to_type_name()
    }

    #[inline]
    pub fn to_location(&self) -> Location {
        self.location.clone()
    }

    pub fn to_event_prefix(&self) -> Event {
        match &self.inner {
            Inner::Insert(_) => Event::Noop,
            Inner::Normal(NormalBuffer { evnt_prefix, .. }) => {
                //
                evnt_prefix.clone()
            }
        }
    }

    pub fn to_inserts(&self) -> Vec<Event> {
        self.last_inserts.clone()
    }
}

impl Buffer {
    #[inline]
    pub fn to_string(&self) -> String {
        self.to_change().as_ref().to_string()
    }

    #[inline]
    pub fn to_cursor(&self) -> usize {
        self.to_change().to_cursor()
    }

    #[inline]
    pub fn to_xy_cursor(&self) -> Cursor {
        self.to_change().to_xy_cursor()
    }

    pub fn to_col(&self) -> usize {
        let cursor = self.to_cursor();
        let a_char = {
            let change = self.to_change();
            change.buf.line_to_char(change.buf.char_to_line(cursor))
        };
        cursor - a_char
    }

    #[inline]
    pub fn len_lines(&self) -> usize {
        let change = self.to_change();
        change.buf.len_lines()
    }

    #[inline]
    pub fn lines_at<'a>(
        //
        &'a self,
        r: usize,
        dp: DP,
    ) -> Result<Box<dyn Iterator<Item = RopeSlice> + 'a>> {
        let change = self.to_change();
        match dp {
            DP::Right => {
                let iter = unsafe {
                    let change: &Change = change.borrow();
                    (change as *const Change).as_ref().unwrap().buf.lines_at(r)
                };
                Ok(Box::new(Iter {
                    _change: change,
                    iter,
                }))
            }
            DP::Left => {
                let iter = unsafe {
                    let change: &Change = change.borrow();
                    (change as *const Change).as_ref().unwrap().buf.lines_at(r)
                };
                Ok(Box::new(ReverseIter {
                    _change: Some(change),
                    iter,
                }))
            }
            _ => err_at!(Fatal, msg: format!("unreachable")),
        }
    }

    #[inline]
    pub fn line_to_char(&self, row: usize) -> usize {
        self.to_change().buf.line_to_char(row)
    }

    #[inline]
    pub fn line_home(&self) -> usize {
        let change = self.to_change();
        change
            .buf
            .line_to_char(change.buf.char_to_line(self.to_cursor()))
    }

    #[inline]
    pub fn char_to_line(&self, cursor: usize) -> usize {
        self.to_change().buf.char_to_line(cursor)
    }

    #[inline]
    pub fn char_to_byte(&self, cursor: usize) -> usize {
        self.to_change().buf.char_to_byte(cursor)
    }

    pub fn chars_at<'a>(
        //
        &'a self,
        n: usize,
        dp: DP,
    ) -> Result<Box<dyn Iterator<Item = char> + 'a>> {
        let change = self.to_change();
        match dp {
            DP::Right => {
                let iter = unsafe {
                    let change: &Change = change.borrow();
                    let r: &Rope = {
                        let c = (change as *const Change).as_ref().unwrap();
                        c.as_ref()
                    };
                    r.chars_at(n)
                };
                Ok(Box::new(Iter {
                    _change: change,
                    iter,
                }))
            }
            DP::Left => {
                let iter = unsafe {
                    let change: &Change = change.borrow();
                    let r: &Rope = {
                        let c = (change as *const Change).as_ref().unwrap();
                        c.as_ref()
                    };
                    r.chars_at(n)
                };
                Ok(Box::new(ReverseIter {
                    _change: Some(change),
                    iter,
                }))
            }
            _ => err_at!(Fatal, msg: format!("unreachable")),
        }
    }

    #[inline]
    pub fn len_line(&self, row: usize) -> usize {
        let change = self.to_change();
        change.buf.line(row).len_chars()
    }

    #[inline]
    pub fn len_chars(&self) -> usize {
        let change = self.to_change();
        change.buf.len_chars()
    }
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
    pub fn insert_char(&mut self, ch: char) -> Result<()> {
        let change = match &mut self.inner {
            Inner::Normal(nb) => &mut nb.change,
            Inner::Insert(ib) => &mut ib.change,
        };

        *change = Change::to_next_change(change);
        self.to_mut_change().insert_char(ch)
    }

    #[inline]
    pub fn backspace(&mut self, n: usize) -> Result<()> {
        let change = match &mut self.inner {
            Inner::Normal(nb) => &mut nb.change,
            Inner::Insert(ib) => &mut ib.change,
        };

        *change = Change::to_next_change(change);
        self.to_mut_change().backspace(n)
    }

    #[inline]
    pub fn remove_at(&mut self, f: Bound<usize>, t: Bound<usize>) -> Result<()> {
        let change = match &mut self.inner {
            Inner::Normal(nb) => &mut nb.change,
            Inner::Insert(ib) => &mut ib.change,
        };

        *change = Change::to_next_change(change);
        self.to_mut_change().remove_at(f, t)
    }
}

impl Buffer {
    pub fn on_event(c: &mut Context, evnt: Event) -> Result<Event> {
        // fold events.
        let (prefix, evnt) = {
            let mut keymap = {
                let b = c.as_mut_buffer();
                mem::replace(&mut b.keymap, Default::default())
            };
            let (prefix, evnt) = keymap.fold(c, evnt)?;
            c.as_mut_buffer().keymap = keymap;
            trace!("folded event, {} {}", prefix, evnt);
            (prefix, evnt)
        };

        c.as_mut_buffer().set_event_prefix(prefix);

        let evnt_up = {
            let mut ftype = {
                let b = c.as_mut_buffer();
                mem::replace(&mut b.ftype, Default::default())
            };
            let evnt_up = ftype.on_event(c, evnt.clone())?;
            c.as_mut_buffer().ftype = ftype;
            evnt_up
        };
        let evnt = if evnt_up == evnt {
            Self::handle_event(c, evnt)?
        } else {
            evnt_up
        };

        Ok(evnt)
    }

    pub fn mode_normal(&mut self) -> Result<()> {
        self.inner = match mem::replace(&mut self.inner, Default::default()) {
            Inner::Insert(ib) => Inner::Normal(ib.into()),
            inner @ Inner::Normal(_) => inner,
        };
        Ok(())
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

    fn ex_n_insert(c: &mut Context, evnt: Event) -> Result<Event> {
        use crate::event::{Event::Md, Mod};

        let nr = mem::replace(&mut c.as_mut_buffer().inner, Default::default());
        let (inner, evnt) = match nr {
            Inner::Normal(nb) => match evnt {
                Md(Mod::Insert(n, pos)) if n > 0 => {
                    c.as_mut_buffer().insert_repeat = n - 1;
                    if pos == DP::Caret {
                        mto_home(c, DP::Caret)?;
                    }
                    (Inner::Insert(nb.into()), Event::Noop)
                }
                Md(Mod::Append(n, pos)) if n > 0 => {
                    c.as_mut_buffer().insert_repeat = n - 1;
                    if pos == DP::End {
                        mto_end(c)?;
                    }
                    mto_right(c, 1, DP::Nobound)?;
                    (Inner::Insert(nb.into()), Event::Noop)
                }
                Md(Mod::Open(n, DP::Left)) if n > 0 => {
                    c.as_mut_buffer().insert_repeat = n - 1;
                    mto_home(c, DP::Nope)?;
                    c.as_mut_buffer().insert_char(NL)?;
                    mto_left(c, 1, DP::Nobound)?;
                    (Inner::Insert(nb.into()), Event::Noop)
                }
                Md(Mod::Open(n, DP::Right)) if n > 0 => {
                    c.as_mut_buffer().insert_repeat = n - 1;
                    mto_end(c)?;
                    mto_right(c, 1, DP::Nobound)?;
                    c.as_mut_buffer().insert_char(NL)?;
                    (Inner::Insert(nb.into()), Event::Noop)
                }
                _ => (Inner::Normal(nb), Event::Noop),
            },
            inner @ Inner::Insert(_) => (inner, evnt),
        };

        c.as_mut_buffer().inner = inner;
        Ok(evnt)
    }

    fn handle_event(c: &mut Context, evnt: Event) -> Result<Event> {
        match c.as_buffer().to_mode() {
            "insert" => Self::handle_i_event(c, evnt),
            "normal" => Self::handle_n_event(c, evnt),
            _ => err_at!(Fatal, msg: format!("unreachable")),
        }
    }

    fn handle_n_event(c: &mut Context, evnt: Event) -> Result<Event> {
        use crate::event::Event::Mt;

        // switch to insert mode.
        let evnt = match Self::to_insert_n(evnt) {
            (Some(n), evnt) if n > 0 => {
                let evnt = Self::ex_n_insert(c, evnt)?;
                return Self::handle_i_event(c, evnt);
            }
            (_, evnt) => evnt,
        };

        let evnt = match evnt {
            Event::Noop => Event::Noop,
            // execute motion command.
            Mt(Mto::Left(n, dp)) => mto_left(c, n, dp)?,
            Mt(Mto::Right(n, dp)) => mto_right(c, n, dp)?,
            Mt(Mto::Up(n, dp)) => mto_up(c, n, dp)?,
            Mt(Mto::Down(n, dp)) => mto_down(c, n, dp)?,
            Mt(Mto::Col(n)) => mto_column(c, n)?,
            Mt(Mto::Home(dp)) => mto_home(c, dp)?,
            Mt(Mto::End) => mto_end(c)?,
            Mt(Mto::Row(n, dp)) => mto_row(c, n, dp)?,
            Mt(Mto::Percent(n)) => mto_percent(c, n)?,
            Mt(Mto::Cursor(n)) => mto_cursor(c, n)?,
            Mt(e @ Mto::CharF(_, _, _)) => {
                c.as_mut_buffer().mto_find_char = e.clone();
                mto_char(c, e)?
            }
            Mt(e @ Mto::CharT(_, _, _)) => {
                c.as_mut_buffer().mto_find_char = e.clone();
                mto_char(c, e)?
            }
            Mt(Mto::CharR(n, dir)) => {
                let e = c.as_mut_buffer().mto_find_char.clone();
                mto_char(c, e.transform(n, dir)?)?
            }
            Mt(e @ Mto::Word(_, _, _)) => mto_words(c, e)?,
            Mt(e @ Mto::WWord(_, _, _)) => mto_wwords(c, e)?,
            Mt(e @ Mto::Sentence(_, _)) => mto_sentence(c, e)?,
            Mt(e @ Mto::Para(_, _)) => mto_para(c, e)?,
            Mt(e @ Mto::Bracket(_, _, _, _)) => mto_bracket(c, e)?,
            Mt(e @ Mto::Pattern(_, Some(_), _)) => {
                c.as_mut_buffer().mto_pattern = e.clone();
                mto_pattern(c, e)?
            }
            Mt(Mto::PatternR(n, dir)) => {
                let e = c.as_mut_buffer().mto_pattern.clone();
                mto_pattern(c, e.transform(n, dir)?)?
            }
            evnt => evnt,
        };

        Ok(evnt)
    }

    fn handle_i_event(c: &mut Context, evnt: Event) -> Result<Event> {
        match evnt {
            Event::Noop => Ok(Event::Noop),
            evnt => {
                c.as_mut_buffer().last_inserts.push(evnt.clone());
                Self::ex_i_event(c, evnt)
            }
        }
    }

    fn ex_i_event(c: &mut Context, evnt: Event) -> Result<Event> {
        use crate::event::Event::{Backspace, Char, Delete, Enter, Esc, Mt, Tab};

        let evnt = match evnt {
            // movement
            Mt(Mto::Left(n, dp)) => mto_left(c, n, dp)?,
            Mt(Mto::Right(n, dp)) => mto_right(c, n, dp)?,
            Mt(Mto::Up(n, dp)) => mto_up(c, n, dp)?,
            Mt(Mto::Down(n, dp)) => mto_down(c, n, dp)?,
            Mt(Mto::Home(dp)) => mto_home(c, dp)?,
            Mt(Mto::End) => mto_end(c)?,
            // Handle mode events.
            Esc => {
                Self::repeat(c)?;
                mto_left(c, 1, DP::LineBound)?;
                c.as_mut_buffer().mode_normal()?;
                Event::Noop
            }
            // on going insert
            Char(ch, _) => {
                c.as_mut_buffer().insert_char(ch)?;
                Event::Noop
            }
            Backspace => {
                c.as_mut_buffer().backspace(1)?;
                Event::Noop
            }
            Enter => {
                c.as_mut_buffer().insert_char(NL)?;
                Event::Noop
            }
            Tab => {
                c.as_mut_buffer().insert_char('\t')?;
                Event::Noop
            }
            Delete => {
                let from = Bound::Included(c.as_mut_buffer().to_cursor());
                let to = from.clone();
                c.as_mut_buffer().remove_at(from, to)?;
                Event::Noop
            }
            evnt => evnt,
        };

        Ok(evnt)
    }

    fn repeat(c: &mut Context) -> Result<()> {
        use crate::event::Event::{Backspace, Char, Delete, Enter, Tab};
        let (last_inserts, insert_repeat) = {
            let b = c.as_mut_buffer();
            let evnts: Vec<Event> = b.last_inserts.drain(..).collect();
            let valid = evnts.iter().all(|evnt| match evnt {
                Char(_, _) | Enter | Tab | Backspace | Delete => true,
                _ => false,
            });
            if valid {
                (evnts, b.insert_repeat)
            } else {
                (vec![], b.insert_repeat)
            }
        };

        for _ in 0..insert_repeat {
            for evnt in last_inserts.iter() {
                Self::ex_i_event(c, evnt.clone())?;
            }
        }

        let b = c.as_mut_buffer();
        b.insert_repeat = 0;
        b.last_inserts = last_inserts;
        Ok(())
    }
}

#[derive(Clone)]
struct NormalBuffer {
    evnt_prefix: Event,
    change: Rc<RefCell<Change>>,
}

impl Default for NormalBuffer {
    fn default() -> NormalBuffer {
        NormalBuffer {
            evnt_prefix: Default::default(),
            change: Default::default(),
        }
    }
}

impl From<InsertBuffer> for NormalBuffer {
    fn from(ib: InsertBuffer) -> NormalBuffer {
        NormalBuffer {
            evnt_prefix: Default::default(),
            change: ib.change,
        }
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
            DP::Left => Box::new(ReverseIter {
                _change: None,
                iter: chars,
            }),
            DP::Right => Box::new(chars),
            _ => unreachable!(),
        }
    }
}

pub fn mto_left(c: &mut Context, n: usize, dp: DP) -> Result<Event> {
    let mut cursor = c.as_buffer().to_cursor();
    cursor = match dp {
        DP::LineBound => {
            let home = c.as_buffer().line_home();
            let new_cursor = cursor.saturating_sub(n);
            Ok(if_else!(new_cursor > home, new_cursor, home))
        }
        DP::Nobound => Ok(cursor.saturating_sub(n)),
        _ => err_at!(Fatal, msg: format!("unreachable")),
    }?;

    c.as_mut_buffer().set_cursor(cursor);
    Ok(Event::Noop)
}

pub fn mto_right(c: &mut Context, n: usize, dp: DP) -> Result<Event> {
    let b = c.as_mut_buffer();
    let mut cursor = b.to_cursor();
    for ch in b.chars_at(cursor, DP::Right)?.take(n) {
        match dp {
            DP::LineBound if ch == NL => break,
            DP::Nobound | DP::LineBound => (),
            _ => err_at!(Fatal, msg: format!("unreachable"))?,
        }
        cursor += 1
    }

    b.set_cursor(cursor);
    Ok(Event::Noop)
}

pub fn mto_home(c: &mut Context, pos: DP) -> Result<Event> {
    let b = c.as_mut_buffer();
    b.set_cursor(b.line_home());
    match pos {
        DP::Caret => {
            b.skip_whitespace(DP::Right);
        }
        DP::Nope => (),
        _ => err_at!(Fatal, msg: format!("unreachable"))?,
    }
    Ok(Event::Noop)
}

pub fn mto_up(c: &mut Context, n: usize, pos: DP) -> Result<Event> {
    let b = c.as_mut_buffer();
    let mut cursor = b.to_cursor();
    match b.char_to_line(cursor) {
        0 => Ok(Event::Noop),
        row => {
            let row = row.saturating_sub(n);
            cursor = {
                let col = {
                    let n_chars = b.len_line(row);
                    cmp::min(n_chars.saturating_sub(2), b.to_col())
                };
                b.line_to_char(row) + col
            };
            b.set_cursor(cursor);
            match pos {
                DP::Caret => mto_home(c, DP::Caret),
                DP::Nope => Ok(Event::Noop),
                _ => {
                    err_at!(Fatal, msg: format!("unreachable"))?;
                    Ok(Event::Noop)
                }
            }
        }
    }
}

pub fn mto_down(c: &mut Context, n: usize, pos: DP) -> Result<Event> {
    let b = c.as_mut_buffer();
    let row = b.char_to_line(b.to_cursor());
    match b.len_lines() {
        0 => Ok(Event::Noop),
        n_rows if row == n_rows => Ok(Event::Noop),
        n_rows => {
            let row = limite!(row.saturating_add(n), n_rows);
            let cursor = {
                let n_chars = b.len_line(row);
                let col = cmp::min(n_chars.saturating_sub(2), b.to_col());
                b.line_to_char(row) + col
            };
            b.set_cursor(cursor);
            match pos {
                DP::Caret => mto_home(c, DP::Caret),
                DP::Nope => Ok(Event::Noop),
                _ => {
                    err_at!(Fatal, msg: format!("unreachable"))?;
                    Ok(Event::Noop)
                }
            }
        }
    }
}

pub fn mto_column(c: &mut Context, n: usize) -> Result<Event> {
    let b = c.as_mut_buffer();
    let n = {
        let m = b.len_line(b.char_to_line(b.to_cursor())).saturating_sub(1);
        cmp::min(m, n).saturating_sub(1)
    };
    b.set_cursor(b.line_home() + n);
    Ok(Event::Noop)
}

pub fn mto_row(c: &mut Context, n: usize, pos: DP) -> Result<Event> {
    let b = c.as_buffer();
    let row = b.char_to_line(b.to_cursor());
    let n = n.saturating_sub(1);
    match b.len_lines() {
        0 => Ok(Event::Noop),
        n_rows if n == 0 => mto_down(c, n_rows.saturating_sub(1), pos),
        _ if n < row => mto_up(c, row - n, pos),
        n_rows if n <= n_rows => mto_down(c, n - row, pos),
        n_rows => mto_down(c, n_rows.saturating_sub(1), pos),
    }
}

pub fn mto_percent(c: &mut Context, n: usize) -> Result<Event> {
    let b = c.as_buffer();
    let row = b.char_to_line(b.to_cursor());
    match b.len_lines() {
        0 => Ok(Event::Noop),
        mut n_rows if n < 100 => {
            n_rows = n_rows.saturating_sub(1);
            match (((n_rows as f64) * (n as f64)) / (100 as f64)) as usize {
                n if n < row => mto_up(c, row - n, DP::Caret),
                n => mto_down(c, n - row, DP::Caret),
            }
        }
        n_rows => mto_down(c, n_rows.saturating_sub(1), DP::Caret),
    }
}

pub fn mto_cursor(c: &mut Context, n: usize) -> Result<Event> {
    let b = c.as_mut_buffer();
    let cursor = b.to_cursor();
    b.set_cursor(limite!(cursor + n, b.len_chars()));
    Ok(Event::Noop)
}

// TODO: create an option of having sticky cursor.
pub fn mto_end(c: &mut Context) -> Result<Event> {
    let b = c.as_mut_buffer();
    let mut cursor = b.to_cursor();
    {
        let mut iter = b.chars_at(b.to_cursor(), DP::Right)?;
        loop {
            match iter.next() {
                Some(NL) => break (),
                Some(_) => cursor += 1,
                None => break (),
            }
        }
    }
    b.set_cursor(cursor);
    Ok(Event::Noop)
}

pub fn mto_char(c: &mut Context, evnt: Mto) -> Result<Event> {
    let (mut n, ch, dp, pos) = match evnt {
        Mto::CharF(n, Some(ch), dp) => (n, ch, dp, DP::Find),
        Mto::CharT(n, Some(ch), dp) => (n, ch, dp, DP::Till),
        Mto::None => return Ok(Event::Noop),
        _ => unreachable!(),
    };

    let b = c.as_mut_buffer();
    let mut cursor = b.to_cursor();
    let home = b.line_home();
    cursor = match dp {
        DP::Right => {
            let mut iter = b.chars_at(cursor, DP::Right)?.enumerate();
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
            let mut iter = b.chars_at(cursor, DP::Left)?.enumerate();
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

    b.set_cursor(if_else!(cursor > home, cursor, home));
    Ok(Event::Noop)
}

pub fn mto_words(c: &mut Context, evnt: Mto) -> Result<Event> {
    match evnt {
        Mto::Word(n, DP::Left, pos) => {
            for _ in 0..n {
                let n = c.as_mut_buffer().skip_whitespace(DP::Left);
                match pos {
                    DP::End if n == 0 => {
                        c.as_mut_buffer().skip_alphanumeric(DP::Left);
                        mto_right(c, 1, DP::Nobound)?;
                    }
                    DP::End => {
                        c.as_mut_buffer().skip_alphanumeric(DP::Left);
                        mto_right(c, 1, DP::Nobound)?;
                    }
                    DP::Start if n == 0 => {
                        c.as_mut_buffer().skip_alphanumeric(DP::Left);
                        c.as_mut_buffer().skip_whitespace(DP::Left);
                    }
                    DP::Start => (),
                    _ => unreachable!(),
                }
            }
            Ok(Event::Noop)
        }
        Mto::Word(n, DP::Right, pos) => {
            for _ in 0..n {
                let n = c.as_mut_buffer().skip_whitespace(DP::Right);
                match pos {
                    DP::End if n == 0 => {
                        c.as_mut_buffer().skip_alphanumeric(DP::Right);
                        mto_left(c, 1, DP::Nobound)?;
                    }
                    DP::End => {
                        c.as_mut_buffer().skip_alphanumeric(DP::Right);
                        mto_left(c, 1, DP::Nobound)?;
                    }
                    DP::Start if n == 0 => {
                        c.as_mut_buffer().skip_alphanumeric(DP::Right);
                        c.as_mut_buffer().skip_whitespace(DP::Right);
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

pub fn mto_wwords(c: &mut Context, evnt: Mto) -> Result<Event> {
    match evnt {
        Mto::WWord(n, DP::Left, pos) => {
            for _ in 0..n {
                let n = c.as_mut_buffer().skip_whitespace(DP::Left);
                match pos {
                    DP::Start if n == 0 => {
                        c.as_mut_buffer().skip_non_whitespace(DP::Left);
                        mto_right(c, 1, DP::Nobound)?;
                    }
                    DP::Start => {
                        c.as_mut_buffer().skip_non_whitespace(DP::Left);
                        mto_right(c, 1, DP::Nobound)?;
                    }
                    DP::End if n == 0 => {
                        c.as_mut_buffer().skip_non_whitespace(DP::Left);
                        c.as_mut_buffer().skip_whitespace(DP::Left);
                    }
                    DP::End => (),
                    _ => unreachable!(),
                }
            }
            Ok(Event::Noop)
        }
        Mto::WWord(n, DP::Right, pos) => {
            for _ in 0..n {
                let n = c.as_mut_buffer().skip_whitespace(DP::Right);
                match pos {
                    DP::End if n == 0 => {
                        c.as_mut_buffer().skip_non_whitespace(DP::Right);
                        mto_left(c, 1, DP::Nobound)?;
                    }
                    DP::End => {
                        c.as_mut_buffer().skip_non_whitespace(DP::Right);
                        mto_left(c, 1, DP::Nobound)?;
                    }
                    DP::Start if n == 0 => {
                        c.as_mut_buffer().skip_non_whitespace(DP::Right);
                        c.as_mut_buffer().skip_whitespace(DP::Right);
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

pub fn mto_sentence(c: &mut Context, e: Mto) -> Result<Event> {
    let is_ws = |ch: char| ch.is_whitespace();

    let b = c.as_mut_buffer();
    let mut cursor = b.to_cursor();
    let mut pch: Option<char> = None;
    cursor = match e {
        Mto::Sentence(mut n, DP::Left) => {
            let mut iter = b.chars_at(cursor, DP::Left)?.enumerate();
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
            let mut iter = b.chars_at(cursor, DP::Right)?.enumerate();
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
                        break b.len_chars().saturating_sub(1);
                    }
                };
            })
        }
        _ => err_at!(Fatal, msg: format!("unreachable")),
    }?;

    b.set_cursor(cursor);
    b.skip_whitespace(DP::Right);

    Ok(Event::Noop)
}

pub fn mto_para(c: &mut Context, evnt: Mto) -> Result<Event> {
    let b = c.as_mut_buffer();
    let mut cursor = b.to_cursor();
    let row = b.char_to_line(cursor);
    cursor = match evnt {
        Mto::Para(mut n, DP::Left) => {
            let mut iter = b.lines_at(row, DP::Left)?.enumerate();
            let cursor = loop {
                match iter.next() {
                    Some((i, line)) => match line.chars().next() {
                        Some(NL) if n == 0 => {
                            break b.line_to_char(row - (i + 1));
                        }
                        Some(NL) => n -= 1,
                        Some(_) => (),
                        None => break b.line_to_char(row - (i + 1)),
                    },
                    None => break 0,
                }
            };
            Ok(cursor)
        }
        Mto::Para(mut n, DP::Right) => {
            let mut iter = b.lines_at(row, DP::Right)?.enumerate();
            let cursor = loop {
                match iter.next() {
                    Some((i, line)) => match line.chars().next() {
                        Some(NL) if n == 0 => {
                            break b.line_to_char(row + i);
                        }
                        Some(NL) => n -= 1,
                        Some(_) => (),
                        None => break b.line_to_char(row + i),
                    },
                    None => break b.len_chars().saturating_sub(1),
                }
            };
            Ok(cursor)
        }
        _ => err_at!(Fatal, msg: format!("unreachable")),
    }?;

    b.set_cursor(cursor);
    Ok(Event::Noop)
}

pub fn mto_bracket(c: &mut Context, e: Mto) -> Result<Event> {
    let mut m = 0;
    let b = c.as_mut_buffer();
    let mut cursor = b.to_cursor();
    match e {
        Mto::Bracket(mut n, yin, yan, DP::Left) => {
            let mut iter = b.chars_at(cursor, DP::Left)?.enumerate();
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
            let mut iter = b.chars_at(cursor, DP::Right)?.enumerate();
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

    b.set_cursor(cursor);
    Ok(Event::Noop)
}

pub fn mto_pattern(c: &mut Context, evnt: Mto) -> Result<Event> {
    let (n, pattern, dp) = match evnt {
        Mto::Pattern(n, Some(pattern), dp) => Ok((n, pattern, dp)),
        _ => err_at!(Fatal, msg: format!("unreachable")),
    }?;

    let b = c.as_mut_buffer();
    let search = {
        let text = b.to_string();
        Search::new(&pattern, &text, dp)?
    };
    let mut cursor = b.to_cursor();
    let byte_off = b.char_to_byte(cursor);

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

    b.set_cursor(cursor);
    Ok(Event::Noop)
}

struct Iter<'a, I, T>
where
    I: Iterator<Item = T>,
{
    _change: cell::Ref<'a, Change>, // holding a reference.
    iter: I,
}

impl<'a> Iterator for Iter<'a, ropey::iter::Chars<'a>, char> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a> Iterator for Iter<'a, ropey::iter::Lines<'a>, RopeSlice<'a>> {
    type Item = RopeSlice<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

struct ReverseIter<'a, I, T>
where
    I: Iterator<Item = T>,
{
    _change: Option<cell::Ref<'a, Change>>, // holding a reference.
    iter: I,
}

impl<'a> Iterator for ReverseIter<'a, ropey::iter::Chars<'a>, char> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.prev()
    }
}

impl<'a> Iterator for ReverseIter<'a, ropey::iter::Lines<'a>, RopeSlice<'a>> {
    type Item = RopeSlice<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.prev()
    }
}
