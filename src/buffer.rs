use ropey::{self, Rope, RopeSlice};

use std::{
    borrow::Borrow,
    cell::{self, RefCell},
    cmp, io,
    iter::FromIterator,
    mem,
    rc::{self, Rc},
};

use crate::{
    event::{Event, DP},
    location::Location,
    plugin::Plugin,
    search::Search,
    window::State,
    {err_at, Error, Result},
};

const NL: char = '\n';

macro_rules! parse_n {
    ($xs:expr) => {
        err_at!(
            FailConvert,
            String::from_iter($xs.drain(..)).parse::<usize>()
        )
    };
}

macro_rules! want_char {
    ($ep:expr) => {{
        use crate::event::Event::*;

        match $ep {
            B(_) | MtoCharF(_, _) | MtoCharT(_, _) => true,
            _ => false,
        }
    }};
}

macro_rules! g_prefix {
    ($ep:expr) => {{
        use crate::event::Event::*;

        match $ep {
            G(_) => true,
            _ => false,
        }
    }};
}

macro_rules! is_insert {
    ($e:expr) => {{
        use crate::event::Event::*;

        match $e {
            ModeInsert(_) | ModeAppend(_) | ModeOpen(_) => true,
            _ => false,
        }
    }};
}

macro_rules! change {
    ($self:ident,$method:ident) => {
        $self.as_mut_change().$method()
    };
    ($self:ident,$method:ident, $($s:expr),*) => {
        $self.as_mut_change().$method($($s),*)
    };
}

#[derive(Clone)]
pub struct Context {
    s: Option<State>,
    e: Option<Event>,
    location: Location,
    read_only: bool,
    insert_only: bool,
    evnt_mto_char: Event,
    evnt_mto_patt: Event,
    last_inserts: Vec<Event>,
}

impl Default for Context {
    fn default() -> Context {
        use crate::event::Event::*;

        Context {
            location: Default::default(),
            read_only: false,
            insert_only: false,
            evnt_mto_char: Noop,
            evnt_mto_patt: Noop,
            last_inserts: Default::default(),
        }
    }
}

impl Context {
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

    pub fn is_read_only(&self) -> bool {
        self.read_only
    }

    pub fn is_insert_only(&self) -> bool {
        self.insert_only
    }

    pub fn to_location(&self) -> Location {
        self.location.clone()
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

// all bits and pieces of content is managed by buffer.
#[derive(Clone)]
pub struct Buffer {
    c: Context,
    p: Plugin,
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
            c: Default::default(),
            p: Default::default(),
            inner: Default::default(),
        }
    }
}

impl AsRef<Context> for Buffer {
    fn as_ref(&self) -> &Context {
        &self.c
    }
}

impl AsMut<Context> for Buffer {
    fn as_mut(&mut self) -> &mut Context {
        &mut self.c
    }
}

impl Buffer {
    pub fn from_reader<R>(data: R) -> Result<Buffer>
    where
        R: io::Read,
    {
        let buf = err_at!(FailBuffer, Rope::from_reader(data))?;
        Ok(Buffer {
            c: Default::default(),
            p: Default::default(),
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

    pub fn set_plugin(&mut self, p: Plugin) -> &mut Self {
        self.p = p;
        self
    }
}

impl Buffer {
    pub fn as_mut_context(&mut self) -> &mut Context {
        &mut self.c
    }

    fn as_change(&self) -> cell::Ref<Change> {
        match &self.inner {
            Inner::Normal(val) => val.as_change(),
            Inner::Insert(val) => val.as_change(),
        }
    }
}

impl Buffer {
    #[inline]
    pub fn to_string(&self) -> String {
        self.as_change().as_ref().to_string()
    }

    #[inline]
    pub fn to_id(&self) -> String {
        match self.c.to_location() {
            Location::Anonymous(s) => s,
            Location::Disk(s) => s.to_str().unwrap().to_string(),
        }
    }

    #[inline]
    pub fn to_cursor(&self) -> usize {
        self.as_change().to_cursor()
    }

    #[inline]
    pub fn to_xy_cursor(&self) -> (usize, usize) {
        self.as_change().to_xy_cursor()
    }

    #[inline]
    pub fn lines_at(&self, n_row: usize) -> impl Iterator<Item = RopeSlice> {
        let change = self.as_change();
        Iter::new_lines_at(change, n_row)
    }

    pub fn on_event(&mut self, mut s: State) -> Result<State> {
        use crate::event::Event::*;

        let evnt = mem::replace(&mut s.event, Default::default());
        let inner = mem::replace(&mut self.inner, Default::default());
        let (inner, evnt) = match inner {
            Inner::Normal(mut nb) => match nb.on_event(s, &self.c, evnt)? {
                Noop => (Inner::Normal(nb), Ok(Noop)),
                N(n, e) if n > 1 && is_insert!(e.as_ref()) => {
                    let ib = {
                        let mut ib: InsertBuffer = nb.into();
                        ib.on_event(s, *e, false /*repeat*/)?;
                        ib.repeat = n - 1;
                        ib
                    };
                    (Inner::Insert(ib), Ok(Noop))
                }
                evnt => (Inner::Normal(nb), Ok(evnt)),
            },
            Inner::Insert(mut ib) => match ib.on_event(s, evnt, false)? {
                ModeEsc if !self.c.insert_only => {
                    self.c.last_inserts = ib.repeat()?;
                    (Inner::Normal(ib.into()), Ok(Noop))
                }
                evnt => (Inner::Insert(ib), Ok(evnt)),
            },
        };

        self.inner = inner;
        s.event = evnt;

        Ok(s)
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
        self.as_mut_change().set_cursor(cursor);
        self
    }
}

impl NormalBuffer {
    fn as_change(&self) -> cell::Ref<Change> {
        self.change.as_ref().borrow()
    }

    fn as_mut_change(&mut self) -> cell::RefMut<Change> {
        self.change.as_ref().borrow_mut()
    }
}

impl NormalBuffer {
    fn fold_event(c: &Context, ep: Event, evnt: Event) -> Result<(Event, Event)> {
        use crate::event::{Event::*, DP::*};

        let m = evnt.to_modifiers();
        let wc = want_char!(ep);
        let gp = g_prefix!(ep);

        let evnt = match evnt {
            // find char
            Char(ch, _) if wc && m.is_empty() => MtoChar(ch),
            // g-prefix
            Char('g', _) if gp && m.is_empty() => MtoRow(Caret),
            Char('e', _) if gp && m.is_empty() => MtoWord(Left, End),
            Char('E', _) if gp && m.is_empty() => MtoWWord(Left, End),
            Char('o', _) if gp && m.is_empty() => MtoCursor,
            Char('I', _) if gp && m.is_empty() => ModeInsert(Nope),
            //
            Char(ch @ '0'..='9', _) if m.is_empty() => Dec(vec![ch]),
            // mode commands
            Char('I', _) if m.is_empty() => ModeInsert(Caret),
            Char('i', _) if m.is_empty() => ModeInsert(Nope),
            Char('a', _) if m.is_empty() => ModeAppend(Left),
            Char('A', _) if m.is_empty() => ModeAppend(End),
            Char('O', _) if m.is_empty() => ModeOpen(Left),
            Char('o', _) if m.is_empty() => ModeOpen(Right),
            // move commands
            Backspace if m.is_empty() => MtoLeft(Nobound),
            Char('h', _) if m.is_empty() => MtoLeft(LineBound),
            Char(' ', _) if m.is_empty() => MtoRight(LineBound),
            Char(' ', _) if m.is_empty() => MtoRight(Nobound),
            Char('l', _) if m.is_empty() => MtoRight(LineBound),
            Char('-', _) if m.is_empty() => MtoUp(Caret),
            Char('j', _) if m.is_empty() => MtoUp(Nope),
            Char('k', _) if m.is_empty() => MtoDown(Nope),
            Char('+', _) if m.is_empty() => MtoDown(Caret),
            Enter if m.is_empty() => MtoDown(Caret),
            Char('|', _) if m.is_empty() => MtoCol,
            Char('G', _) if m.is_empty() => MtoRow(Caret),
            Char('%', _) if m.is_empty() => MtoPercent,
            Char('0', _) if m.is_empty() => MtoHome(Nope),
            Char('^', _) if m.is_empty() => MtoHome(Caret),
            Char('$', _) if m.is_empty() => MtoEnd,
            Char('F', _) if m.is_empty() => MtoCharF(None, Left),
            Char('f', _) if m.is_empty() => MtoCharF(None, Right),
            Char('T', _) if m.is_empty() => MtoCharT(None, Left),
            Char('t', _) if m.is_empty() => MtoCharT(None, Right),
            Char('b', _) if m.is_empty() => MtoWord(Left, Start),
            Char('B', _) if m.is_empty() => MtoWWord(Left, Start),
            Char('e', _) if m.is_empty() => MtoWord(Right, End),
            Char('E', _) if m.is_empty() => MtoWWord(Right, End),
            Char('{', _) if m.is_empty() => MtoPara(Left),
            Char('}', _) if m.is_empty() => MtoPara(Right),
            Char('(', _) if m.is_empty() => MtoSentence(Left),
            Char(')', _) if m.is_empty() => MtoSentence(Right),
            Char('w', _) if m.is_empty() => MtoWord(Right, Start),
            Char('W', _) if m.is_empty() => MtoWWord(Right, Start),
            Char(';', _) if m.is_empty() => MtoCharR(Right),
            Char(',', _) if m.is_empty() => MtoCharR(Left),
            Char('n', _) if m.is_empty() => MtoPattern(None, Right),
            Char('N', _) if m.is_empty() => MtoPattern(None, Left),
            // prefix event
            Char('g', _) if m.is_empty() => G(Box::new(Event::Noop)),
            Char('[', _) if m.is_empty() => B(Left),
            Char(']', _) if m.is_empty() => B(Right),
            evnt => evnt,
        };

        let fc = c.evnt_mto_char.clone();
        let pn = c.evnt_mto_patt.clone();

        let (ep, evnt) = match (ep, evnt) {
            // Simple Move Prefix
            (Noop, e @ MtoCharF(_, _)) => (e, Noop),
            (Noop, e @ MtoCharT(_, _)) => (e, Noop),
            // N prefix
            (Noop, Dec(ns)) => (Dec(ns), Noop),
            (Dec(mut ns), Dec(ms)) => {
                ns.extend(&ms);
                (Dec(ns), Noop)
            }
            // N-G-prefix
            (N(n, box G(_)), e @ MtoRow(_)) => (Noop, N(n, Box::new(e))),
            (N(n, box G(_)), e @ MtoWord(_, _)) => (Noop, N(n, Box::new(e))),
            (N(n, box G(_)), e @ MtoWWord(_, _)) => (Noop, N(n, Box::new(e))),
            (N(n, box G(_)), e @ MtoCursor) => (Noop, N(n, Box::new(e))),
            (N(n, box G(_)), e @ ModeInsert(_)) => (Noop, N(n, Box::new(e))),
            // N-B-prefix
            (N(n, box B(dp)), MtoChar(ch)) => match ch {
                '(' => (Noop, N(n, Box::new(MtoBracket('(', ')', dp)))),
                ')' => (Noop, N(n, Box::new(MtoBracket(')', '(', dp)))),
                '{' => (Noop, N(n, Box::new(MtoBracket('{', '}', dp)))),
                '}' => (Noop, N(n, Box::new(MtoBracket('}', '{', dp)))),
                _ => unreachable!(),
            },
            (N(n, box MtoCharF(None, dp)), MtoChar(ch)) => {
                let f_prefix = Box::new(MtoCharF(Some(ch), dp));
                (N(n, f_prefix), Noop)
            }
            (N(n, box MtoCharT(None, dp)), MtoChar(ch)) => {
                let f_prefix = Box::new(MtoCharT(Some(ch), dp));
                (N(n, f_prefix), Noop)
            }
            (N(_, _), _) => {
                err_at!(Fatal, msg: format!("unreachable"))?;
                (Noop, Noop)
            }
            // Commands
            (Dec(mut ns), MtoCharR(dp)) => match fc {
                Noop => (Noop, Noop),
                fc => (N(parse_n!(ns)?, Box::new(fc.transform(dp)?)), Noop),
            },
            (Dec(mut ns), MtoPattern(None, dp)) => match pn {
                Noop => (Noop, Noop),
                pn => (N(parse_n!(ns)?, Box::new(pn.transform(dp)?)), Noop),
            },
            (Dec(mut ns), e) => (N(parse_n!(ns)?, Box::new(e)), Noop),
            (Noop, e) => (Noop, N(1, Box::new(e))),
            (ep, e) => (ep, e),
        };

        Ok((ep, evnt))
    }

    fn on_event(&mut self, c: &Context, evnt: Event) -> Result<Event> {
        use crate::event::{Event::*, DP::*};

        let (pe, evnt) = Self::fold_event(c, self.evnt_prefix.clone(), evnt)?;
        self.evnt_prefix = pe;

        let mut change = self.as_mut_change();
        match evnt {
            Noop => Ok(Noop),
            // execute motion command.
            N(n, box MtoLeft(dp)) => change.mto_left(n, dp),
            N(n, box MtoRight(dp)) => change.mto_right(n, dp),
            N(n, box MtoUp(dp)) => change.mto_up(n, dp),
            N(n, box MtoDown(dp)) => change.mto_down(n, dp),
            N(n, box MtoCol) => change.mto_column(n),
            N(n, box MtoRow(dp)) => change.mto_row(n, dp),
            N(n, box MtoPercent) => change.mto_percent(n),
            N(_, box MtoHome(dp)) => change.mto_home(dp),
            N(_, box MtoEnd) => change.mto_end(), // TODO: make this sticky.
            N(n, box MtoCursor) => change.mto_cursor(n),
            N(n, e @ box MtoCharF(_, _)) => change.mto_char(n, *e),
            N(n, e @ box MtoCharT(_, _)) => change.mto_char(n, *e),
            N(n, e @ box MtoWord(_, _)) => change.mto_words(n, *e),
            N(n, e @ box MtoWWord(_, _)) => change.mto_wwords(n, *e),
            N(n, e @ box MtoSentence(_)) => change.mto_sentence(n, *e),
            N(n, e @ box MtoPara(_)) => change.mto_para(n, *e),
            N(n, e @ box MtoBracket(_, _, _)) => change.mto_bracket(n, *e),
            N(n, e @ box MtoPattern(Some(_), _)) => change.mto_pattern(n, *e),
            // execute mode switching commands
            N(n, box ModeInsert(Caret)) => {
                change.mto_home(Caret)?;
                Ok(N(n, Box::new(ModeInsert(Caret))))
            }
            N(n, e @ box ModeInsert(_)) => Ok(N(n, Box::new(*e))),
            //Char('%', _) if m.is_empty() => {
            //    self.as_mut_change().fwd_match_group();
            //    Ok(Noop)
            //}
            evnt => Ok(evnt),
        }
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
        self.as_mut_change().set_cursor(cursor);
        self
    }
}

impl InsertBuffer {
    fn as_change(&self) -> cell::Ref<Change> {
        self.change.as_ref().borrow()
    }

    fn as_mut_change(&mut self) -> cell::RefMut<Change> {
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

    fn repeat(&mut self) -> Result<Vec<Event>> {
        let last_inserts: Vec<Event> = self.to_repeat_evnts();
        for _ in 0..self.repeat {
            for evnt in last_inserts.iter() {
                self.on_event(evnt.clone(), true)?;
            }
        }

        Ok(last_inserts)
    }
}

impl InsertBuffer {
    fn on_event(&mut self, evnt: Event, repeat: bool) -> Result<Event> {
        use crate::event::{Event::*, DP::*};

        if !repeat {
            self.last_inserts.push(evnt.clone());
        }

        match evnt {
            // Start mode.
            ModeInsert(_) => Ok(Noop),
            ModeAppend(Right) => change!(self, mto_right, 1, Nobound),
            ModeAppend(End) => {
                change!(self, mto_end)?;
                change!(self, mto_right, 1, LineBound)
            }
            ModeOpen(Left) => {
                change!(self, mto_home, Nope)?;
                change!(self, insert_char, NL);
                change!(self, mto_left, 1, Nobound)
            }
            ModeOpen(Right) => {
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

    fn lines_at(&self, n_row: usize) -> ropey::iter::Lines {
        self.buf.lines_at(n_row)
    }

    fn iter<'a>(&'a self, dp: DP) -> Box<dyn Iterator<Item = char> + 'a> {
        use crate::event::DP::*;

        let chars = self.buf.chars_at(self.cursor);
        match dp {
            Left => Box::new(ReverseIter::new(chars)),
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
            Left => Box::new(ReverseIter::new(lines)),
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

impl<'a> Iter<'a, ropey::iter::Lines<'a>, RopeSlice<'a>> {
    fn new_lines_at(
        //
        change: cell::Ref<'a, Change>,
        n_row: usize,
    ) -> Iter<'a, ropey::iter::Lines<'a>, RopeSlice<'a>> {
        let iter = unsafe {
            let change: &Change = change.borrow();
            (change as *const Change).as_ref().unwrap().lines_at(n_row)
        };
        Iter {
            _change: change,
            iter,
        }
    }
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

struct ReverseIter<I, T>
where
    I: Iterator<Item = T>,
{
    iter: I,
}

impl<I, T> ReverseIter<I, T>
where
    I: Iterator<Item = T>,
{
    fn new(iter: I) -> ReverseIter<I, T> {
        ReverseIter { iter }
    }
}

impl<'a> Iterator for ReverseIter<ropey::iter::Chars<'a>, char> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.prev()
    }
}

impl<'a> Iterator for ReverseIter<ropey::iter::Lines<'a>, RopeSlice<'a>> {
    type Item = RopeSlice<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.prev()
    }
}
