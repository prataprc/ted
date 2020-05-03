use ropey::{self, Rope, RopeSlice};

use std::{
    borrow::Borrow,
    cell::{self, RefCell},
    cmp, io, mem,
    rc::{self, Rc},
};

use crate::{
    event::{Event, DP},
    ftype::FType,
    keymap::Keymap,
    location::Location,
    search::Search,
    window::Context,
    {err_at, Error, Result},
};

pub const NL: char = '\n';

macro_rules! change {
    ($self:ident,$method:ident) => {
        $self.to_mut_change().$method()
    };
    ($self:ident,$method:ident, $($s:expr),*) => {
        $self.to_mut_change().$method($($s),*)
    };
}

// all bits and pieces of content is managed by buffer.
#[derive(Clone)]
pub struct Buffer {
    pub location: Location,
    pub read_only: bool,
    pub insert_only: bool,
    pub evnt_mto_char: Event,
    pub evnt_mto_patt: Event,
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

impl Default for Buffer {
    fn default() -> Buffer {
        Buffer {
            location: Default::default(),
            read_only: false,
            insert_only: false,
            evnt_mto_char: Event::Noop,
            evnt_mto_patt: Event::Noop,
            last_inserts: Default::default(),
            keymap: Default::default(),
            ftype: Default::default(),

            inner: Default::default(),
        }
    }
}

impl Buffer {
    pub fn from_reader<R>(data: R) -> Result<Buffer>
    where
        R: io::Read,
    {
        let buf = err_at!(FailBuffer, Rope::from_reader(data))?;
        Ok(Buffer {
            location: Default::default(),
            read_only: false,
            insert_only: false,
            evnt_mto_char: Event::Noop,
            evnt_mto_patt: Event::Noop,
            last_inserts: Default::default(),
            keymap: Default::default(),
            ftype: Default::default(),

            inner: Inner::Normal(NormalBuffer::new(buf)),
        })
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
    pub fn is_read_only(&self) -> bool {
        self.read_only
    }

    pub fn is_insert_only(&self) -> bool {
        self.insert_only
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

    pub fn to_mto_char(&self) -> Event {
        self.evnt_mto_char.clone()
    }

    pub fn to_mto_pattern(&self) -> Event {
        self.evnt_mto_patt.clone()
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
    pub fn to_xy_cursor(&self) -> (usize, usize) {
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
        let change = match self.inner {
            Inner::Normal(nb) => &mut nb.change,
            Inner::Insert(ib) => &mut ib.change,
        };

        *change = Change::to_next_change(change);
        self.to_mut_change().insert_char(ch)
    }

    #[inline]
    pub fn backspace(&mut self, n: usize) -> Result<()> {
        let change = match self.inner {
            Inner::Normal(nb) => &mut nb.change,
            Inner::Insert(ib) => &mut ib.change,
        };

        *change = Change::to_next_change(change);
        self.to_mut_change().backspace(n)
    }

    #[inline]
    pub fn remove_at(&mut self, f: Bound<usize>, t: Bound<usize>) -> Result<()> {
        let change = match self.inner {
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
            (prefix, evnt)
        };
        c.as_mut_buffer().set_event_prefix(prefix);

        let mut ftype = {
            let b = c.as_mut_buffer();
            mem::replace(&mut b.ftype, Default::default());
        };
        ftype.on_event(c, event);
        c.as_mut_buffer().ftype = ftype;
    }

    fn mode_insert(&mut self) -> Result<()> {
        self.inner = match mem::replace(&mut self.inner, Default::default()) {
            Inner::Normal(nb) => Inner::Insert(nb.into()),
            inner @ Inner::Insert(_) => inner,
        };
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
        let prev_change: &Change = &prev.as_ref().borrow();
        let next = Rc::new(RefCell::new(Change {
            buf: prev_change.as_ref().clone(),
            parent: None,
            children: Default::default(),
            cursor: prev_change.cursor,
        }));

        next.borrow_mut().children.push(Rc::clone(prev));
        prev.borrow_mut().parent = Some(Rc::downgrade(&next));

        next
    }

    fn to_cursor(&self) -> usize {
        self.cursor
    }

    fn to_xy_cursor(&self) -> (usize, usize) {
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
        use std::ops::Bound::{Included, Excluded, Unbound};

        let n = self.buf.len_chars();
        let from = match from => {
            Included(from) => cmp::min(from, n.saturating_sub(1)),
            Excluded(from) => cmp::min(from.saturating_add(1), n),
            Unbounded => 0,
        };
        let to = match to = {
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
        use crate::event::DP::*;

        let mut n = 0;
        let n = loop {
            match self.iter(dp).next() {
                Some(ch) if ch.is_whitespace() => n += 1,
                Some(_) => break n,
                None => break n,
            }
        };
        self.cursor = if_else!(dp == Right, self.cursor + n, self.cursor - n);
        n
    }

    fn skip_non_whitespace(&mut self, dp: DP) -> usize {
        use crate::event::DP::*;

        let mut n = 0;
        let n = loop {
            match self.iter(dp).next() {
                Some(ch) if ch.is_whitespace() => n += 1,
                Some(_) => break n,
                None => break n,
            }
        };
        self.cursor = if_else!(dp == Right, self.cursor + n, self.cursor - n);
        n
    }

    fn skip_alphanumeric(&mut self, dp: DP) -> usize {
        use crate::event::DP::*;

        let mut n = 0;
        let n = loop {
            match self.iter(dp).next() {
                Some(ch) if ch.is_alphanumeric() => n += 1,
                Some(_) => break n,
                None => break n,
            }
        };
        self.cursor = if_else!(dp == Right, self.cursor + n, self.cursor - n);
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
    fn to_col(&self) -> usize {
        let a_char = self.buf.line_to_char(self.buf.char_to_line(self.cursor));
        self.cursor - a_char
    }

    fn iter<'a>(&'a self, dp: DP) -> Box<dyn Iterator<Item = char> + 'a> {
        use crate::event::DP::*;

        let chars = self.buf.chars_at(self.cursor);
        match dp {
            Left => Box::new(ReverseIter {
                _change: None,
                iter: chars,
            }),
            Right => Box::new(chars),
            _ => unreachable!(),
        }
    }

    fn iter_line<'a>(
        //
        &'a self,
        dp: DP,
    ) -> Box<dyn Iterator<Item = RopeSlice> + 'a> {
        use crate::event::DP::*;

        let lines = self.buf.lines_at(self.buf.char_to_line(self.cursor));
        match dp {
            Left => Box::new(ReverseIter {
                _change: None,
                iter: lines,
            }),
            Right => Box::new(lines),
            _ => unreachable!(),
        }
    }
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
