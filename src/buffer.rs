use ropey::{self, Rope, RopeSlice};

use std::{
    borrow::Borrow,
    cell::{self, RefCell},
    cmp, io, mem,
    rc::{self, Rc},
};

use crate::{
    event::{Event, DP},
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
}

impl Buffer {
    pub fn on_event(c: &mut Context, evnt: Event) -> Result<Event> {
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

        match evnt {
            Event::Noop => Ok(Event::Noop),
            evnt => {
                let inner = {
                    let b: &mut Buffer = c.as_mut();
                    mem::replace(&mut b.inner, Default::default())
                };
                let (inner, evnt) = match inner {
                    Inner::Normal(nb) => nb.on_event(c, evnt)?,
                    Inner::Insert(ib) => ib.on_event(c, evnt)?,
                };

                c.as_mut_buffer().inner = inner;
                Ok(evnt)
            }
        }
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

impl NormalBuffer {
    fn on_event(self, c: &mut Context, e: Event) -> Result<(Inner, Event)> {
        use crate::event::{Event::*, DP::*};

        // switch to insert mode.
        //match e {
        //    N(n, evnt) if n > 1 && is_insert!(evnt.as_ref()) => {
        //        let ib: InsertBuffer = self.into();
        //        return ib.on_event(c, *evnt);
        //    }
        //    _ => (),
        //};

        let mut change = c.as_mut_buffer().to_mut_change();
        let evnt = match e {
            Noop => Noop,
            // execute motion command.
            N(n, box MtoLeft(dp)) => change.mto_left(n, dp)?,
            N(n, box MtoRight(dp)) => change.mto_right(n, dp)?,
            N(n, box MtoUp(dp)) => change.mto_up(n, dp)?,
            N(n, box MtoDown(dp)) => change.mto_down(n, dp)?,
            N(n, box MtoCol) => change.mto_column(n)?,
            N(n, box MtoRow(dp)) => change.mto_row(n, dp)?,
            N(n, box MtoPercent) => change.mto_percent(n)?,
            N(_, box MtoHome(dp)) => change.mto_home(dp)?,
            N(_, box MtoEnd) => change.mto_end()?, // TODO: make this sticky.
            N(n, box MtoCursor) => change.mto_cursor(n)?,
            N(n, e @ box MtoCharF(_, _)) => change.mto_char(n, *e)?,
            N(n, e @ box MtoCharT(_, _)) => change.mto_char(n, *e)?,
            N(n, e @ box MtoWord(_, _)) => change.mto_words(n, *e)?,
            N(n, e @ box MtoWWord(_, _)) => change.mto_wwords(n, *e)?,
            N(n, e @ box MtoSentence(_)) => change.mto_sentence(n, *e)?,
            N(n, e @ box MtoPara(_)) => change.mto_para(n, *e)?,
            N(n, e @ box MtoBracket(_, _, _)) => change.mto_bracket(n, *e)?,
            N(n, e @ box MtoPattern(Some(_), _)) => change.mto_pattern(n, *e)?,
            // execute mode switching commands
            N(n, box ModeInsert(Caret)) => {
                change.mto_home(Caret)?;
                N(n, Box::new(ModeInsert(Caret)))
            }
            N(n, e @ box ModeInsert(_)) => N(n, Box::new(*e)),
            //Char('%', _) if m.is_empty() => {
            //    self.to_mut_change().fwd_match_group();
            //    Ok(Noop)
            //}
            evnt => evnt,
        };

        Ok((Inner::Normal(self), evnt))
    }
}

#[derive(Clone)]
struct InsertBuffer {
    repeat: usize,
    last_inserts: Vec<Event>,
    change: Rc<RefCell<Change>>,
}

impl From<NormalBuffer> for InsertBuffer {
    fn from(nb: NormalBuffer) -> InsertBuffer {
        InsertBuffer {
            repeat: 1,
            change: nb.change,
            last_inserts: Default::default(),
        }
    }
}

impl Default for InsertBuffer {
    fn default() -> InsertBuffer {
        InsertBuffer {
            repeat: 1,
            change: Default::default(),
            last_inserts: Default::default(),
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

    fn to_repeat_evnts(&mut self) -> Vec<Event> {
        use crate::event::Event::*;

        let evnts: Vec<Event> = self.last_inserts.drain(..).collect();
        let valid = evnts.iter().all(|evnt| match evnt {
            Char(_, _) | Backspace | Enter | Tab | Delete => true,
            _ => false,
        });

        if valid {
            evnts
        } else {
            vec![]
        }
    }
}

impl InsertBuffer {
    fn on_event(mut self, c: &mut Context, e: Event) -> Result<(Inner, Event)> {
        use crate::event::Event::*;

        let insert_only = {
            let b = c.as_mut_buffer();
            b.last_inserts.push(e.clone());
            b.insert_only
        };

        match self.exec_event(c, e)? {
            ModeEsc if !insert_only => {
                self.repeat(c)?;
                Ok((Inner::Normal(self.into()), Noop))
            }
            evnt => Ok((Inner::Insert(self), evnt)),
        }
    }

    fn repeat(&mut self, c: &mut Context) -> Result<()> {
        let last_inserts: Vec<Event> = self.to_repeat_evnts();
        for _ in 0..self.repeat {
            for evnt in last_inserts.iter() {
                self.exec_event(c, evnt.clone())?;
            }
        }
        c.as_mut_buffer().last_inserts = last_inserts;
        Ok(())
    }

    fn exec_event(&mut self, _: &mut Context, evnt: Event) -> Result<Event> {
        use crate::event::{Event::*, DP::*};

        match evnt {
            // Start mode.
            N(n, box ModeInsert(_)) if n > 1 => {
                self.repeat = n - 1;
                Ok(Noop)
            }
            N(n, box ModeAppend(Right)) if n > 1 => {
                self.repeat = n - 1;
                change!(self, mto_right, 1, Nobound)
            }
            N(n, box ModeAppend(End)) if n > 1 => {
                self.repeat = n - 1;
                change!(self, mto_end)?;
                change!(self, mto_right, 1, LineBound)
            }
            N(n, box ModeOpen(Left)) if n > 1 => {
                self.repeat = n - 1;
                change!(self, mto_home, Nope)?;
                change!(self, insert_char, NL);
                change!(self, mto_left, 1, Nobound)
            }
            N(n, box ModeOpen(Right)) if n > 1 => {
                self.repeat = n - 1;
                change!(self, mto_end)?;
                change!(self, mto_right, 1, Nobound)?;
                change!(self, insert_char, NL);
                Ok(Noop)
            }
            // movement
            MtoLeft(dp) => change!(self, mto_left, 1, dp),
            MtoRight(dp) => change!(self, mto_right, 1, dp),
            MtoUp(dp) => change!(self, mto_up, 1, dp),
            MtoDown(dp) => change!(self, mto_down, 1, dp),
            MtoHome(dp) => change!(self, mto_home, dp),
            MtoEnd => change!(self, mto_end),
            // Handle mode events.
            Esc => {
                change!(self, mto_left, 1, LineBound)?;
                Ok(ModeEsc)
            }
            // on going insert
            Char(ch, _) => {
                self.change = Change::to_next_change(&mut self.change);
                change!(self, insert_char, ch);
                Ok(Noop)
            }
            Backspace => {
                self.change = Change::to_next_change(&mut self.change);
                change!(self, backspace, 1);
                Ok(Noop)
            }
            Enter => {
                self.change = Change::to_next_change(&mut self.change);
                change!(self, insert_char, NL);
                Ok(Noop)
            }
            Tab => {
                self.change = Change::to_next_change(&mut self.change);
                change!(self, insert_char, '\t');
                Ok(Noop)
            }
            Delete => {
                self.change = Change::to_next_change(&mut self.change);
                change!(self, remove_at);
                Ok(Noop)
            }
            evnt => Ok(evnt),
        }
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

    fn insert_char(&mut self, ch: char) {
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
    fn mto_left(&mut self, n: usize, dp: DP) -> Result<Event> {
        use crate::event::DP::*;

        self.cursor = match dp {
            LineBound => {
                let row = self.buf.char_to_line(self.cursor);
                let home = self.buf.line_to_char(row);
                let new_cursor = self.cursor.saturating_sub(n);
                Ok(if_else!(new_cursor > home, new_cursor, home))
            }
            Nobound => Ok(self.cursor.saturating_sub(n)),
            _ => err_at!(Fatal, msg: format!("unreachable")),
        }?;

        Ok(Event::Noop)
    }

    fn mto_right(&mut self, n: usize, dp: DP) -> Result<Event> {
        for ch in self.buf.chars_at(self.cursor).take(n) {
            if dp == DP::LineBound && ch == NL {
                break;
            }
            self.cursor += 1
        }

        Ok(Event::Noop)
    }

    fn mto_up(&mut self, n: usize, pos: DP) -> Result<Event> {
        use crate::event::DP::*;

        match self.buf.char_to_line(self.cursor) {
            0 => Ok(Event::Noop),
            row => {
                let row = row.saturating_sub(n);
                self.cursor = {
                    let col = cmp::min(
                        self.buf.line(row).len_chars().saturating_sub(2),
                        self.to_col(),
                    );
                    self.buf.line_to_char(row) + col
                };
                if pos == Caret {
                    self.mto_home(Caret)?;
                }
                Ok(Event::Noop)
            }
        }
    }

    fn mto_down(&mut self, n: usize, pos: DP) -> Result<Event> {
        use crate::event::DP::*;

        let row = self.buf.char_to_line(self.cursor);
        match self.buf.len_lines() {
            0 => Ok(Event::Noop),
            n_rows if row == n_rows => Ok(Event::Noop),
            n_rows => {
                let row = limite!(row.saturating_add(n), n_rows);
                self.cursor = {
                    let col = cmp::min(
                        self.buf.line(row).len_chars().saturating_sub(2),
                        self.to_col(),
                    );
                    self.buf.line_to_char(row) + col
                };
                if pos == Caret {
                    self.mto_home(Caret)?;
                }
                Ok(Event::Noop)
            }
        }
    }

    fn mto_column(&mut self, n: usize) -> Result<Event> {
        for ch in self.buf.chars_at(self.cursor).take(n) {
            if ch == NL {
                break;
            }
            self.cursor += 1;
        }
        Ok(Event::Noop)
    }

    fn mto_row(&mut self, n: usize, pos: DP) -> Result<Event> {
        let row = self.buf.char_to_line(self.cursor);
        match self.buf.len_lines() {
            0 => Ok(Event::Noop),
            _ if n < row => self.mto_up(row - n, pos),
            n_rows if n < n_rows => self.mto_up(n - row, pos),
            _ => Ok(Event::Noop),
        }
    }

    fn mto_percent(&mut self, n: usize) -> Result<Event> {
        use crate::event::DP::*;

        let row = self.buf.char_to_line(self.cursor);
        match self.buf.len_lines() {
            0 => Ok(Event::Noop),
            mut n_rows if n < 100 => {
                n_rows -= 1;
                let n = (((n_rows as f64) * (n as f64)) / (100 as f64)) as usize;
                if n < row {
                    self.mto_up(row - n, Nope)
                } else {
                    self.mto_down(n - row, Nope)
                }
            }
            _ => Ok(Event::Noop),
        }
    }

    fn mto_cursor(&mut self, n: usize) -> Result<Event> {
        self.cursor = limite!(self.cursor + n, self.buf.len_chars());
        Ok(Event::Noop)
    }

    fn mto_home(&mut self, pos: DP) -> Result<Event> {
        use crate::event::DP::*;

        self.cursor = self.buf.line_to_char(self.buf.char_to_line(self.cursor));
        if pos == Caret {
            self.skip_whitespace(Right);
        }
        Ok(Event::Noop)
    }

    fn mto_end(&mut self) -> Result<Event> {
        let mut iter = self.buf.chars_at(self.cursor);
        let mut cursor = self.cursor;
        loop {
            match iter.next() {
                Some(NL) => break (),
                Some(_) => cursor += 1,
                None => break (),
            }
        }
        self.cursor = cursor;
        Ok(Event::Noop)
    }

    fn mto_char(&mut self, mut n: usize, evnt: Event) -> Result<Event> {
        use crate::event::DP::*;

        let (ch, dp, pos) = match evnt {
            Event::MtoCharF(Some(ch), dp) => (ch, dp, Find),
            Event::MtoCharT(Some(ch), dp) => (ch, dp, Till),
            _ => unreachable!(),
        };

        self.cursor = match dp {
            Right => {
                let mut iter = self.iter(dp).enumerate();
                loop {
                    match iter.next() {
                        Some((_, NL)) => break self.cursor,
                        Some((i, c)) if c == ch && n == 0 && pos == Till => {
                            break self.cursor.saturating_add(i);
                        }
                        Some((i, c)) if c == ch && n == 0 => {
                            break self.cursor.saturating_add(i - 1);
                        }
                        Some((_, c)) if c == ch => n -= 1,
                        _ => (),
                    }
                }
            }
            Left => {
                let mut iter = self.iter(dp).enumerate();
                loop {
                    match iter.next() {
                        Some((_, NL)) => break self.cursor,
                        Some((i, c)) if c == ch && n == 0 && pos == Till => {
                            break self.cursor.saturating_add(i);
                        }
                        Some((i, c)) if c == ch && n == 0 => {
                            break self.cursor.saturating_add(i + 1);
                        }
                        Some((_, c)) if c == ch => n -= 1,
                        _ => (),
                    }
                }
            }
            _ => unreachable!(),
        };

        Ok(Event::Noop)
    }

    fn mto_words(&mut self, n: usize, evnt: Event) -> Result<Event> {
        use crate::event::{Event::*, DP::*};

        match evnt {
            MtoWord(Left, pos) => {
                for _ in 0..n {
                    let n = self.skip_whitespace(Left);
                    match pos {
                        End if n == 0 => {
                            self.skip_alphanumeric(Left);
                            self.mto_right(1, Nobound)?;
                        }
                        End => {
                            self.skip_alphanumeric(Left);
                            self.mto_right(1, Nobound)?;
                        }
                        Start if n == 0 => {
                            self.skip_alphanumeric(Left);
                            self.skip_whitespace(Left);
                        }
                        Start => (),
                        _ => unreachable!(),
                    }
                }
                Ok(Event::Noop)
            }
            MtoWord(Right, pos) => {
                for _ in 0..n {
                    let n = self.skip_whitespace(Right);
                    match pos {
                        End if n == 0 => {
                            self.skip_alphanumeric(Right);
                            self.mto_left(1, Nobound)?;
                        }
                        End => {
                            self.skip_alphanumeric(Right);
                            self.mto_left(1, Nobound)?;
                        }
                        Start if n == 0 => {
                            self.skip_alphanumeric(Right);
                            self.skip_whitespace(Right);
                        }
                        Start => (),
                        _ => unreachable!(),
                    }
                }
                Ok(Event::Noop)
            }
            _ => err_at!(Fatal, msg: format!("unreachable")),
        }
    }

    fn mto_wwords(&mut self, n: usize, evnt: Event) -> Result<Event> {
        use crate::event::{Event::*, DP::*};

        match evnt {
            MtoWWord(Left, pos) => {
                for _ in 0..n {
                    let n = self.skip_whitespace(Left);
                    match pos {
                        Start if n == 0 => {
                            self.skip_non_whitespace(Left);
                            self.mto_right(1, Nobound)?;
                        }
                        Start => {
                            self.skip_non_whitespace(Left);
                            self.mto_right(1, Nobound)?;
                        }
                        End if n == 0 => {
                            self.skip_non_whitespace(Left);
                            self.skip_whitespace(Left);
                        }
                        End => (),
                        _ => unreachable!(),
                    }
                }
                Ok(Event::Noop)
            }
            MtoWWord(Right, pos) => {
                for _ in 0..n {
                    let n = self.skip_whitespace(Right);
                    match pos {
                        End if n == 0 => {
                            self.skip_non_whitespace(Right);
                            self.mto_left(1, Nobound)?;
                        }
                        End => {
                            self.skip_non_whitespace(Right);
                            self.mto_left(1, Nobound)?;
                        }
                        Start if n == 0 => {
                            self.skip_non_whitespace(Right);
                            self.skip_whitespace(Right);
                        }
                        Start => (),
                        _ => unreachable!(),
                    }
                }
                Ok(Event::Noop)
            }
            _ => err_at!(Fatal, msg: format!("unreachable")),
        }
    }

    fn mto_sentence(&mut self, mut n: usize, e: Event) -> Result<Event> {
        use crate::event::{Event::*, DP::*};

        let is_ws = |ch: char| ch.is_whitespace();

        let mut pch: Option<char> = None;
        self.cursor = match e {
            MtoSentence(Left) => {
                let mut iter = self.iter(Left).enumerate();
                Ok(loop {
                    pch = match (iter.next(), pch) {
                        (Some((i, '.')), Some(pch)) if is_ws(pch) => {
                            if n > 1 {
                                n -= 1;
                            } else {
                                break self.cursor.saturating_sub(i);
                            }
                            Some('.')
                        }
                        (Some((i, NL)), Some(NL)) => {
                            if n > 1 {
                                n -= 1;
                            } else {
                                break self.cursor.saturating_sub(i);
                            }
                            Some(NL)
                        }
                        (Some((_, ch)), _) => Some(ch),
                        (None, _) => break 0,
                    };
                })
            }
            MtoSentence(Right) => {
                let mut iter = self.iter(Right).enumerate();
                Ok(loop {
                    pch = match (pch, iter.next()) {
                        (Some('.'), Some((i, ch))) if is_ws(ch) => {
                            if n > 1 {
                                n -= 1;
                            } else {
                                break self.cursor.saturating_add(i);
                            }
                            Some('.')
                        }
                        (Some(NL), Some((i, NL))) => {
                            if n > 1 {
                                n -= 1;
                            } else {
                                break self.cursor.saturating_add(i);
                            }
                            Some(NL)
                        }
                        (_, Some((_, ch))) => Some(ch),
                        (_, None) => {
                            break self.buf.len_chars().saturating_sub(1);
                        }
                    };
                })
            }
            _ => err_at!(Fatal, msg: format!("unreachable")),
        }?;

        self.skip_whitespace(Right);

        Ok(Event::Noop)
    }

    fn mto_para(&mut self, mut n: usize, evnt: Event) -> Result<Event> {
        use crate::event::{Event::*, DP::*};

        let row = self.buf.char_to_line(self.cursor);
        self.cursor = match evnt {
            MtoPara(Left) => {
                let mut iter = self.iter_line(Left).enumerate();
                let cursor = loop {
                    match iter.next() {
                        Some((i, line)) => match line.chars().next() {
                            Some(NL) if n == 0 => {
                                break self.buf.line_to_char(row - (i + 1));
                            }
                            Some(NL) => n -= 1,
                            Some(_) => (),
                            None => break self.buf.line_to_char(row - (i + 1)),
                        },
                        None => break 0,
                    }
                };
                Ok(cursor)
            }
            MtoPara(Right) => {
                let mut iter = self.iter_line(Right).enumerate();
                let cursor = loop {
                    match iter.next() {
                        Some((i, line)) => match line.chars().next() {
                            Some(NL) if n == 0 => {
                                break self.buf.line_to_char(row + i);
                            }
                            Some(NL) => n -= 1,
                            Some(_) => (),
                            None => break self.buf.line_to_char(row + i),
                        },
                        None => break self.buf.len_chars().saturating_sub(1),
                    }
                };
                Ok(cursor)
            }
            _ => err_at!(Fatal, msg: format!("unreachable")),
        }?;

        Ok(Event::Noop)
    }

    fn mto_bracket(&mut self, mut n: usize, e: Event) -> Result<Event> {
        use crate::event::{Event::*, DP::*};

        let mut m = 0;
        let mut cursor = self.cursor;
        match e {
            MtoBracket(yin, yan, Left) => {
                let mut iter = self.iter(Left).enumerate();
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
            MtoBracket(yin, yan, Right) => {
                let mut iter = self.iter(Right).enumerate();
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

        self.cursor = cursor;
        Ok(Event::Noop)
    }

    fn mto_pattern(&mut self, n: usize, evnt: Event) -> Result<Event> {
        use crate::event::{Event::*, DP::*};

        let (pattern, dp) = match evnt {
            MtoPattern(Some(pattern), dp) => Ok((pattern, dp)),
            _ => err_at!(Fatal, msg: format!("unreachable")),
        }?;

        let text = self.buf.to_string();
        let search = Search::new(&pattern, &text, dp)?;
        let byte_off = self.buf.char_to_byte(self.cursor);

        let n = n.saturating_sub(1);
        self.cursor = match dp {
            Left => {
                let item = search.rev(byte_off).skip(n).next();
                match item {
                    Some((s, _)) => Ok(s),
                    None => Ok(self.cursor),
                }
            }
            Right => {
                let item = search.iter(byte_off).skip(n).next();
                match item {
                    Some((s, _)) => Ok(s),
                    None => Ok(self.cursor),
                }
            }
            _ => err_at!(Fatal, msg: format!("unreachable")),
        }?;

        Ok(Event::Noop)
    }

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
