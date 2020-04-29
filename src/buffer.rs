use crossterm::event::KeyModifiers;
use lazy_static::lazy_static;
use log::trace;
use ropey::{self, Rope, RopeSlice};

use std::{
    cell::{self, RefCell},
    cmp, ffi, fmt, io,
    iter::FromIterator,
    rc::{self, Rc},
    result,
    sync::Mutex,
};

use crate::{
    event::Event,
    search::Search,
    {err_at, Error, Result},
};

const NEW_LINE_CHAR: char = '\n';

macro_rules! parse_n {
    ($xs:expr) => {
        err_at!(
            FailConvert,
            String::from_iter($xs.drain(..)).parse::<usize>()
        )
    };
}

#[derive(Clone)]
pub struct Context {
    location: Location,
    read_only: bool,
    insert_only: bool,
    evnt_mto_char: Option<Event>,
    evnt_search: Option<Event>,
    last_inserts: Vec<Event>,
}

// all bits and pieces of content is managed by buffer.
#[derive(Clone)]
pub enum Buffer {
    InsertBuffer(InsertBuffer),
    NormalBuffer(NormalBuffer),
}

impl Default for Buffer {
    fn default() -> Buffer {
        Buffer::NormalBuffer(Default::default())
    }
}

impl Buffer {
    pub fn from_reader<R>(data: R) -> Result<Buffer>
    where
        R: io::Read,
    {
        let buf = err_at!(FailBuffer, Rope::from_reader(data))?;
        // trace!("first {:p}", &buf);
        Ok(Buffer::NormalBuffer(NormalBuffer::new(buf)))
    }

    pub fn empty() -> Result<Buffer> {
        let buf = vec![];
        Self::from_reader(buf.as_slice())
    }

    pub fn set_location(&mut self, loc: Location) -> &mut Self {
        match self {
            Buffer::NormalBuffer(val) => {
                val.set_location(loc);
            }
            Buffer::InsertBuffer(val) => {
                val.set_location(loc);
            }
        }
        self
    }

    pub fn set_read_only(&mut self, read_only: bool) -> &mut Self {
        match self {
            Buffer::NormalBuffer(val) => {
                val.set_read_only(read_only);
            }
            Buffer::InsertBuffer(val) => {
                val.set_read_only(read_only);
            }
        }
        self
    }

    pub fn set_insert_only(&mut self, insert_only: bool) -> &mut Self {
        match self {
            Buffer::NormalBuffer(val) => {
                val.set_insert_only(insert_only);
            }
            Buffer::InsertBuffer(val) => {
                val.set_insert_only(insert_only);
            }
        }
        self
    }

    pub fn set_cursor(&mut self, cursor: usize) -> &mut Self {
        match self {
            Buffer::NormalBuffer(val) => {
                val.set_cursor(cursor);
            }
            Buffer::InsertBuffer(val) => {
                val.set_cursor(cursor);
            }
        }
        self
    }

    pub fn as_change(&self) -> cell::Ref<Change> {
        match self {
            Buffer::NormalBuffer(val) => val.as_change(),
            Buffer::InsertBuffer(val) => val.as_change(),
        }
    }

    pub fn as_mut_change(&mut self) -> cell::RefMut<Change> {
        match self {
            Buffer::NormalBuffer(val) => val.as_mut_change(),
            Buffer::InsertBuffer(val) => val.as_mut_change(),
        }
    }
}

impl Buffer {
    pub fn to_string(&self) -> String {
        match self {
            Buffer::NormalBuffer(val) => val.as_change().as_ref().to_string(),
            Buffer::InsertBuffer(val) => val.as_change().as_ref().to_string(),
        }
    }

    pub fn to_location(&self) -> Location {
        match self {
            Buffer::NormalBuffer(val) => val.as_context().location.clone(),
            Buffer::InsertBuffer(val) => val.as_context().location.clone(),
        }
    }

    pub fn to_id(&self) -> String {
        match self.to_location() {
            Location::Anonymous(s) => s,
            Location::Disk(s) => s.to_str().unwrap().to_string(),
        }
    }
}

impl Buffer {
    pub fn to_cursor(&self) -> usize {
        match self {
            Buffer::NormalBuffer(val) => val.as_change().to_cursor(),
            Buffer::InsertBuffer(val) => val.as_change().to_cursor(),
        }
    }

    pub fn to_xy_cursor(&self) -> (usize, usize) {
        match self {
            Buffer::NormalBuffer(val) => val.as_change().to_xy_cursor(),
            Buffer::InsertBuffer(val) => val.as_change().to_xy_cursor(),
        }
    }

    pub fn handle_event(&mut self, evnt: Event) -> Result<Option<Event>> {
        match self {
            Buffer::NormalBuffer(nb) => {
                let evnt = {
                    let evnt = normal_event!(evnt, nb.evnt_prefix.clone());
                    nb.handle_event(evnt)?
                };
                match evnt {
                    None => Ok(None),
                    Some(e @ ModeInsert(_))
                    | Some(e @ ModeAppend(_))
                    | Some(e @ ModeOpen(_, _)) => {
                            *self = {
                                let mut ib: InsertBuffer = nb.clone().into();
                                ib.handle_event(e, false /*repeat*/)?;
                                Buffer::InsertBuffer(ib)
                            };
                            Ok(None)
                    }
                    evnt => Ok(Some(evnt))
                }
            }
            Buffer::InsertBuffer(ib) => match ib.handle_event(evnt, false)? {
                Some(ModeEsc) if !ib.c.insert_only => {
                    ib.c.last_inserts = ib.repeat()?;
                    *self = Buffer::NormalBuffer(ib.clone().into());
                    Ok(None)
                }
                Some(evnt) => Ok(Some(evnt)),
                None => Ok(None),
            },
        }
    }
}

macro_rules! want_char {
    ($pe:expr) => {
        match $pe {
            Some(B(_)) | Some(MtoCharF(_, _)) | Some(MtoCharT(_, _)) => true,
            None => false
        }
    };
}

macro_rules! g_prefix {
    ($pe:expr) => {
        match $pe {
            Some(G(_)) => true
            None => false
        }
    };
}

macro_rules! normal_event {
    ($evnt:expr, $pe:expr) => {{
        let m = $evnt.to_modifiers();
        let wc = want_char!($pe);
        let gp = g_prefix!($pe);
        match $evnt {
            // find char
            Char(ch, _) if wc m.is_empty() => MtoChar(ch),
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
            Char('A', _) if m.is_empty() => ModeAppend(Right),
            Char('O', _) if m.is_empty() => ModeOpen(Left),
            Char('o', _) if m.is_empty() => ModeOpen(Right),
            // move commands
            Backspace if m.is_empty() => MtoLeft(Unbound),
            Char('h', _) if m.is_empty() => MtoLeft(LineBound),
            Char(' ', _) if m.is_empty() => MtoRight(LineBound),
            Char(' ', _) if m.is_empty() => MtoRight(Unbound),
            Char('l', _) if m.is_empty() => MtoRight(LineBound),
            Char('-', _) if m.is_empty() => MtoUp(Caret),
            Char('j', _) if m.is_empty() => MtoUp(Nope),
            Char('k', _) if m.is_empty() => MtoDown(Nope),
            Char('+', _) if m.is_empty() => MtoDown(Caret),
            Enter if m.is_empty() =>        MtoDown(Caret),
            Char('0', _) if m.is_empty() => MtoHome(Nope),
            Char('^', _) if m.is_empty() => MtoHome(Caret),
            Char('|', _) if m.is_empty() => MtoCol,
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
            Char('G', _) if m.is_empty() => MtoRow(Caret),
            Char('n', _) if m.is_empty() => MtoPattern(None, Right),
            Char('N', _) if m.is_empty() => MtoPattern(None, Left),
            Char('%', _) if m.is_empty() => MtoPercent,
            // prefix event
            Char('g', _) if m.is_empty() => G(Event::None),
            Char('[', _) if m.is_empty() => B(Event::None, Left),
            Char(']', _) if m.is_empty() => B(Event::None, Right),
            evnt => evnt,
        }
    }};
}

#[derive(Clone)]
pub struct NormalBuffer {
    c: Context,
    evnt_prefix: Option<Event>,
    change: Rc<RefCell<Change>>,
}

impl From<InsertBuffer> for NormalBuffer {
    fn from(ib: InsertBuffer) -> NormalBuffer {
        NormalBuffer {
            c: ib.c,
            evnt_prefix: None,
            change: ib.change,
        }
    }
}

impl Default for NormalBuffer {
    fn default() -> NormalBuffer {
        let c = Context {
            location: Default::default(),
            read_only: false,
            insert_only: false,
            evnt_mto_char: None,
            evnt_search: None,
            last_inserts: Default::default(),
        };

        NormalBuffer {
            c,
            evnt_prefix: None,
            change: Default::default(),
        }
    }
}

impl NormalBuffer {
    fn new(buf: Rope) -> NormalBuffer {
        let mut nb: NormalBuffer = Default::default();
        nb.change = Change::start(buf);
        nb
    }

    fn set_location(&mut self, loc: Location) -> &mut Self {
        self.c.location = loc;
        self
    }

    fn set_read_only(&mut self, read_only: bool) -> &mut Self {
        self.c.read_only = read_only;
        self
    }

    fn set_insert_only(&mut self, insert_only: bool) -> &mut Self {
        self.c.insert_only = insert_only;
        self
    }

    fn set_cursor(&mut self, cursor: usize) -> &mut Self {
        self.as_mut_change().set_cursor(cursor);
        self
    }

    fn as_change(&self) -> cell::Ref<Change> {
        self.change.as_ref().borrow()
    }

    fn as_mut_change(&mut self) -> cell::RefMut<Change> {
        self.change.as_ref().borrow_mut()
    }

    fn as_context(&self) -> &Context {
        &self.c
    }

    fn as_mut_context(&mut self) -> &mut Context {
        &mut self.c
    }

    fn fold_event(&mut self, evnt: Event) -> Result<Option<Event>> {
        let m = evnt.to_modifiers();
        let fc = self.c.evnt_find_char.clone();
        let pn = self.c.evnt_mto_pattern.clone();
        let (pe, e) = match (self.evnt_prefix.take(), evnt) {
            // Simple Move Prefix
            (None, e @ MtoCharF(_, _)) => (Some(e), None),
            (None, e @ MtoCharT(_, _)) => (Some(e), None),
            (None, e) => (None, N(1, Box::new(e))),
            // N prefix
            (None, Dec(ns)) => (Some(Dec(ns)), None),
            (Dec(mut ns), Dec(ns)) => {
                ns.extend(&ns);
                (Some(Dec(ns)), None)
            }
            // N-Char prefix
            (Some(N(n, e)), MtoChar(ch)) => match e {
                MtoCharF(None, dir) => {
                    let f_prefix = Box::new(MtoCharF(Some(ch), dir));
                    (Some(N(n, f_prefix)),  None)
                },
                MtoCharT(None, dir) => {
                    let f_prefix = Box::new(MtoCharT(Some(ch), dir));
                    (Some(N(n, f_prefix)),  None)
                },
            },
            // N-G-prefix
            (Some(N(n, G(_))), MtoRow(p)) => (None, Some(N(n, MtoRow(p)))),
            (Some(N(n, G(_))), MtoWord(p)) => (None, Some(N(n, MtoWord(p)))),
            (Some(N(n, G(_))), MtoWWord(p)) => (None, Some(N(n, MtoWWord(p)))),
            (Some(N(n, G(_))), MtoCursor) => (None, Some(N(n, MtoCursor))),
            (Some(N(n, G(_))), ModeInsert(pos) => {
                //
                (None, Some(N(n, ModeInsert(pos))))
            }
            // N-B-prefix
            (Some(N(n, B(dir))), e @ MtoChar(ch)) => match ch {
                '(' => (None, Some(N(n, MtoBracket('(', ')', dir)))),
                ')' => (None, Some(N(n, MtoBracket(')', '(', dir)))),
                '{' => (None, Some(N(n, MtoBracket('{', '}', dir)))),
                '}' => (None, Some(N(n, MtoBracket('}', '{', dir)))),
            }
            // Commands
            (Some(Dec(ns)), e) => (Some(N(parse_n!(ns)?, Box::new(e))),  None),
            (Some(Dec(ns)), MtoCharR(dir)) if fc.is_none() => (None, None),
            (Some(Dec(ns)), MtoCharR(dir)) if fc.is_none() => {
                let e = match (fc.unwrap(), dir) {
                    (MtoCharF(ch, Left), Right) => Ok(MtoCharF(ch, Left)),
                    (MtoCharF(ch, Left), Left) => Ok(MtoCharF(ch, Right)),
                    (MtoCharF(ch, Right), Right) => Ok(MtoCharF(ch, Right)),
                    (MtoCharF(ch, Right), Left) => Ok(MtoCharF(ch, Left)),
                    _ => err_at!(Fatal, msg: format!("unreachable")),
                }?;
                (Some(N(parse_n!(ns)?, Box::new(e))),  None)
            }
            (Some(Dec(ns)), MtoPattern(dir)) if pn.is_none() => (None, None),
            (Some(Dec(ns)), MtoPattern(dir)) => {
                let e = match (pn.unwrap(), dir) {
                    (MtoPattern(ch, Left), Right) => Ok(MtoPattern(ch, Left)),
                    (MtoPattern(ch, Left), Left) => Ok(MtoPattern(ch, Right)),
                    (MtoPattern(ch, Right), Right) => Ok(MtoPattern(ch, Right)),
                    (MtoPattern(ch, Right), Left) => Ok(MtoPattern(ch, Left)),
                    _ => err_at!(Fatal, msg: format!("unreachable")),
                }?;
                (Some(N(parse_n!(ns)?, Box::new(e))),  None)
            }
            (pe, e) => (pe, Some(e)),
        };

        self.evnt_prefix = pe;

        Ok(e)
    }

    fn handle_event(&mut self, mut evnt: Event) -> Result<Option<Event>> {
        evnt = match self.fold_event(evnt)? {
            Some(evnt) => evnt,
            None => return Ok(None),
        };
        let change = self.as_mut_change();
        match evnt {
            // execute motion command.
            N(n, MtoLeft(dir)) => change.mto_left(n, dir),
            N(n, MtoRight(dir)) => change.mto_right(n, dir),
            N(n, MtoUp(dir) => change.mto_up(n, dir),
            N(n, MtoDown(dir) => change.mto_down(n, dir),
            N(n, MtoCol => change.mto_column(n),
            N(n, MtoRow(dir) => change.mto_row(n, dir),
            N(n, MtoPercent => change.mto_percent(n),
            N(n, MtoHome(pos) => change.mto_home(pos),
            N(n, MtoEnd => change.mto_end(),
            N(n, MtoCursor => change.mto_cursor(n),
            N(n, e @ MtoCharF(_, _) => change.mto_char(e),
            N(n, e @ MtoCharT(_, _) => change.mto_char(e),
            N(n, e @ MtoCharR(_, _) => change.mto_char(e),
            N(n, e @ MtoWord(_, _) => change..mto_words(e),
            N(n, e @ MtoWWord(_, _) => change.mto_wwords(e),
            N(n, e @ MtoSentence(_) => change.mto_sentence(e),
            N(n, e @ MtoPara(_) => change.mto_para(e),
            N(n, e @ MtoBracket(_, _, _) => change.mto_bracket(e),
            N(n, e @ MtoPattern(Some(_), _) => change.mto_pattern(e),
            // execute mode switching commands
            N(n, e @ ModeInsert(_, Caret) => {
                self.mto_home(MtoHome(Caret));
                Some(e)
            }
            N(n, e @ ModeInsert(_, _) => Some(e),
            //Char('%', _) if m.is_empty() => {
            //    self.as_mut_change().fwd_match_group();
            //    Ok(None)
            //}
            evnt => Ok(Some(evnt)),
        }
    }
}

#[derive(Clone)]
pub struct InsertBuffer {
    c: Context,
    change: Rc<RefCell<Change>>,
    last_inserts: Vec<Event>,
}

impl From<NormalBuffer> for InsertBuffer {
    fn from(nb: NormalBuffer) -> InsertBuffer {
        InsertBuffer {
            c: nb.c,
            change: nb.change,
            last_inserts: Default::default(),
        }
    }
}

impl Default for InsertBuffer {
    fn default() -> InsertBuffer {
        let c = Context {
            location: Default::default(),
            read_only: false,
            insert_only: false,
            evnt_find_char: None,
            evnt_search: None,
            last_inserts: Default::default(),
        };

        InsertBuffer {
            c,
            change: Default::default(),
            last_inserts: Default::default(),
        }
    }
}

impl InsertBuffer {
    fn new(buf: Rope) -> InsertBuffer {
        let mut ib: InsertBuffer = Default::default();
        ib.change = Change::start(buf);
        ib
    }

    fn set_location(&mut self, loc: Location) -> &mut Self {
        self.c.location = loc;
        self
    }

    fn set_read_only(&mut self, read_only: bool) -> &mut Self {
        self.c.read_only = read_only;
        self
    }

    fn set_insert_only(&mut self, insert_only: bool) -> &mut Self {
        self.c.insert_only = insert_only;
        self
    }

    fn set_cursor(&mut self, cursor: usize) -> &mut Self {
        self.as_mut_change().set_cursor(cursor);
        self
    }

    fn as_change(&self) -> cell::Ref<Change> {
        self.change.as_ref().borrow()
    }

    pub fn as_mut_change(&mut self) -> cell::RefMut<Change> {
        self.change.as_ref().borrow_mut()
    }

    fn as_context(&self) -> &Context {
        &self.c
    }

    pub fn as_mut_context(&mut self) -> &mut Context {
        &mut self.c
    }

    fn to_repeat_evnts(&mut self) -> Vec<Event> {
        let evnts: Vec<Event> = self.last_inserts.drain(..).collect();
        let valid = evnts.iter().all(|evnt| match evnt {
            Char(_, _) | Backspace(_) | Enter | Tab | Delete => true,
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
        let mut first = last_inserts.first().map(Clone::clone);
        loop {
            first = match first {
                Some(Insert(n)) if n > 1 => Ok(Some(Insert(n - 1))),
                Some(Insert(_)) => Ok(None),
                Some(Append(n)) if n > 1 => Ok(Some(Append(n - 1))),
                Some(Append(_)) => Ok(None),
                Some(OpenUp(n)) if n > 1 => Ok(Some(OpenUp(n - 1))),
                Some(OpenUp(_)) => Ok(None),
                Some(OpenDown(n)) if n > 1 => Ok(Some(OpenDown(n - 1))),
                Some(OpenDown(_)) => Ok(None),
                Some(_) => err_at!(Fatal, msg: format!("unreachable")),
                None => break Ok(last_inserts),
            }?;
            match first {
                Some(_) => {
                    for evnt in last_inserts.iter() {
                        self.handle_event(evnt.clone(), true)?;
                    }
                }
                None => break Ok(last_inserts),
            }
        }
    }

    fn handle_event(
        //
        &mut self,
        evnt: Event,
        repeat: bool,
    ) -> Result<Option<Event>> {
        if !repeat {
            self.last_inserts.push(evnt.clone());
        }

        match evnt {
            // Begin insert.
            ModeInsert(_) => Ok(None),
            ModeAppend(_, Left) => {
                self.as_mut_change().move_right(1, false /*line_bound*/);
                Ok(None)
            }
            ModeAppend(_, Right) => {
                self.as_mut_change().end();
                self.as_mut_change().move_right(1, false /*line_bound*/);
                Ok(None)
            }
            OpenUp(_) => {
                self.as_mut_change().home();
                self.as_mut_change().insert_char(NEW_LINE_CHAR);
                self.as_mut_change().move_left(1, false /*line_bound*/);
                Ok(None)
            }
            OpenDown(_) => {
                self.as_mut_change().end();
                self.as_mut_change().move_right(1, false /*line_bound*/);
                self.as_mut_change().insert_char(NEW_LINE_CHAR);
                Ok(None)
            }
            // insert
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
            // movement
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
            // Handle mode events.
            Esc => {
                self.as_mut_change().move_left(1, true /*line_bound*/);
                Ok(Some(ModeEsc))
            }
            evnt => Ok(Some(evnt)),
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
    fn mto_left(&mut self, n: usize, dir: Dir) -> Result<Option<Event>> {
        self.cursor = match dir {
            Dir::LineBound => {
                let row = self.buf.char_to_line(self.cursor);
                let home = self.buf.line_to_char(row);
                let new_cursor = self.cursor.saturating_sub(n);
                Ok(if_else!(new_cursor > home, new_cursor, home))
            }
            Dir::Unbound => {
                Ok(self.cursor.saturating_sub(n))
            }
            _ => err_at!(Fatal, msg: format!("unreachable")),
        }?;

        Ok(None)
    }

    fn mto_right(&mut self, n: usize, dir: Dir) -> Result<Option<Event>> {
        for ch in self.buf.chars_at(self.cursor).take(n) {
            if line_bound && ch == NEW_LINE_CHAR {
                break;
            }
            self.cursor += 1
        }

        Ok(None)
    }

    fn mto_up(&mut self, n: usize, pop: Dir) -> Result<Option<Event>> {
        match self.buf.char_to_line(self.cursor) {
            0 => Ok(None),
            row => {
                let row = row.saturating_sub(n);
                self.cursor = {
                    let col = cmp::min(
                        self.buf.line(row).len_chars().saturating_sub(2),
                        self.to_col(),
                    );
                    self.buf.line_to_char(row) + col
                };
                if pos == Dir::Caret {
                    self.mto_home(MtoHome(Dir::Caret));
                }
                Ok(None)
            }
        }
    }

    fn mto_down(&mut self, n: usize, pos: Dir) -> Result<Option<Event> {
        let row = self.buf.char_to_line(self.cursor);
        match self.buf.len_lines() {
            0 => Ok(None),
            n_rows if row == n_rows => Ok(None),
            n_rows => {
                let row = limite!(row.saturating_add(n), n_rows);
                self.cursor = {
                    let col = cmp::min(
                        self.buf.line(row).len_chars().saturating_sub(2),
                        self.to_col(),
                    );
                    self.buf.line_to_char(row) + col
                };
                if pos == Dir::Caret {
                    self.mto_home(MtoHome(Dir::Caret));
                }
                Ok(None)
            }
        }
    }

    fn mto_column(&mut self, n: usize) -> Result<Option<Event>> {
        for ch in self.buf.chars_at(self.cursor).take(n) {
            if ch == NEW_LINE_CHAR {
                break;
            }
            self.cursor += 1;
        }
    }

    fn mto_row(&mut self, n: usize, pos: Dir) -> Result<Option<Event>> {
        let row = self.buf.char_to_line(self.cursor);
        match self.buf.len_lines() {
            0 => (),
            _ if n < row => self.mto_up(MtoUp(row-n, pos)),
            n_rows if n < n_rows => self.mto_up(MtoUp(n-row, pos)),
            _ => (),
        }
        Ok(None)
    }

    fn mto_percent(&mut self, n: usize) -> Result<Option<Event>> {
        let row = self.buf.char_to_line(self.cursor);
        match self.buf.len_lines() {
            0 => Ok(None),
            mut n_rows if n < 100 => {
                n_rows -= 1;
                let n = (((n_rows as f64) * (n as f64)) / (100 as f64)) as usize;
                if n < row {
                    self.mto_up(MtoUp(row-n, None))
                } else {
                    self.mto_down(MtoDown(n - row, None))
                }
            }
            _ => Ok(None),
        }
    }

    fn mto_cursor(&mut self, n: usize) -> Result<Option<Event>> {
        self.cursor = limite!(self.cursor + n, self.buf.len_chars());
        Ok(None)
    }

    fn mto_home(&mut self, pos: Dir) -> Result<Option<Event>> {
        self.cursor = self.buf.line_to_char(self.buf.char_to_line(self.cursor));
        self.skip_whitespace(Dir::Right);
        Ok(None)
    }

    fn mto_end(&mut self, pos: Dir) -> Result<Option<Event>> {
        let iter =self.buf.chars_at(self.cursor);
        loop { 
            match iter.next() {
                Some(NEW_LINE_CHAR) => break Ok(None),
                Some(_) => self.cursor += 1,
                None => break Ok(None),
            }
        }
    }

    fn mto_char(&mut self, evnt: Evnt) -> Result<Option<Event>> {
        let mut iter = self.iter(dir).enumerate();
        let (ch, dir, mut n, pos) => match evnt {
            Event::MtoCharF(n, Some(ch), dir) => (ch, dir, n, Find),
            Event::MtoCharT(n, Some(ch), dir) => (ch, dir, n, Till),
        };

        self.cursor = match dir {
            Dir::Right => loop {
                match iter.next() {
                    Some((_, NEW_LINE_CHAR)) => break self.cursor,
                    Some((i, c)) if c == ch && n == 0 && pos == Dir::Till => {
                        break self.cursor + i;
                    }
                    Some((i, c)) if c == ch && n == 0 => {
                        break self.cursor + (i - 1);
                    }
                    Some((_, c)) if c == ch => n -= 1,
                    _ => (),
                }
            }
            Dir::Left => loop {
                match iter.next() {
                    Some((_, NEW_LINE_CHAR)) => break self.cursor,
                    Some((i, c)) if c == ch && n == 0 && pos == Dir::Till => {
                        break self.cursor + i;
                    }
                    Some((i, c)) if c == ch && n == 0 => {
                        break self.cursor + i + 1;
                    }
                    Some((_, c)) if c == ch => n -= 1,
                    _ => (),
                }
            }
        }

        Ok(None)
    }

    fn mto_words(&mut self, evnt: Event) -> Result<Option<Event>> {
        match evnt {
            MtoWords(n, Left, pos) => {
                for _ in 0..n {
                    let n = self.skip_whitespace(Left);
                    match pos {
                        Dir::End if n == 0 => {
                            self.skip_alphanumeric(Left);
                            self.mto_right(MtoRight(1, Dir::Unbound));
                        }
                        Dir::End => {
                            self.skip_alphanumeric(Left);
                            self.mto_right(MtoRight(1, Dir::Unbound));
                        }
                        Dir::Start if n == 0 => {
                            self.skip_alphanumeric(Left);
                            self.skip_whitespace(Left);
                        }
                        Dir::Start => (),
                    }
                }
                Ok(None)
            },
            MtoWords(n, Right, pos) => {
                for _ in 0..n {
                    let n = self.skip_whitespace(Right);
                    match pos {
                        Dir::End if n == 0 => {
                            self.skip_alphanumeric(Right);
                            self.mto_left(MtoLeft(1, Dir::Unbound));
                        }
                        Dir::End => {
                            self.skip_alphanumeric(Right);
                            self.mto_left(MtoLeft(1, Dir::Unbound));
                        }
                        Dir::Start if n == 0 => {
                            self.skip_alphanumeric(Right);
                            self.skip_whitespace(Right);
                        }
                        Dir::Start => (),
                    }
                }
                Ok(None)
            }
            _ => err_at!(Fatal, msg: format!("unreachable")),
        }
    }

    fn mto_wwords(&mut self, evnt: Event) -> Result<Option<Event>> {
        match evnt {
            MtoWWords(m, Left, pos) => {
                for _ in 0..n {
                    let n = self.skip_whitespace(Left);
                    match tail {
                        false if n == 0 => {
                            self.skip_non_whitespace(Left);
                            self.mto_right(MtoRight(1, Dir::Unbound));
                        }
                        false => {
                            self.skip_non_whitespace(Left);
                            self.mto_right(MtoRight(1, Dir::Unbound));
                        }
                        true if n == 0 => {
                            self.skip_non_whitespace(Left);
                            self.skip_whitespace(Left);
                        }
                        true => (),
                    }
                }
                Ok(None)
            }
            MtoWWords(n, Right, pos) => {
                for _ in 0..n {
                    let n = self.skip_whitespace(Right);
                    match tail {
                        true if n == 0 => {
                            self.skip_non_whitespace(Right);
                            self.mto_left(MtoLeft(1, Dir::Unbound));
                        }
                        true => {
                            self.skip_non_whitespace(Right);
                            self.mto_left(MtoLeft(1, Dir::Unbound));
                        }
                        false if n == 0 => {
                            self.skip_non_whitespace(Right);
                            self.skip_whitespace(Right);
                        }
                        false => (),
                    }
                }
                Ok(None)
            }
            _ => err_at!(Fatal, msg: format!("unreachable")),
        }
    }

    fn mto_sentence(&mut self, evnt: Event) -> Result<Option<Event>> {
        let mut pch: Option<char> = None;
        match evnt {
            MtoSentence(n, Left) => {
                let mut iter = self.iter(Left).enumerate();
                loop {
                    pch = match (iter.next(), pch) {
                        (Some((i, '.')), pch) if pch.is_whitespace()
                        | (Some((i, NEW_LINE_CHAR)), NEW_LINE_CHAR) => {
                            if n > 1 {
                                n -= 1;
                            } else {
                                self.cursor = self.cursor.saturating_sub(i+1);
                            }
                        },
                        None => self.cursor = 0,
                    };
                }
            }
            MtoSentence(n, Right) => {
                let mut iter = self.iter(Right).enumerate();
                loop {
                    pch = match (iter.next(), pch) {
                        (Some((i, ch)), '.') if ch.is_whitespace()
                        | (Some((i, NEW_LINE_CHAR)), NEW_LINE_CHAR) {
                            if n > 1 {
                                n -= 1;
                            } else {
                                self.cursor = self.cursor.saturating_add(i);
                            }
                        },
                        None => {
                            self.cursor = self.buf.len_chars.saturating_sub(1);
                        }
                    };
                }
            }
        }

        self.skip_whitespace(Right);

        Ok(None)
    }

    fn mto_para(&mut self, evnt: Event) -> Result<Option<Event>> {
        let row = self.buf.char_to_line(self.cursor);
        self.cursor = match evnt {
            MtoPara(n, Left) => {
                let mut iter = self.iter_line(Left).enumerate();
                loop {
                    match iter.next() {
                        Some((i, line)) => match line.chars().next() {
                            Some(NEW_LINE_CHAR) if n == 0 => {
                                break self.buf.line_to_char(row - (i + 1));
                            }
                            Some(NEW_LINE_CHAR) => n -= 1,
                            Some(_) => (),
                            None => break self.buf.line_to_char(row - (i + 1)),
                        },
                        None => break 0,
                    }
                }
            }
            MtoPara(n, Right) => {
                let mut iter = self.iter_line(Right).enumerate();
                loop {
                    match iter.next() {
                        Some((i, line)) => match line.chars().next() {
                            Some(NEW_LINE_CHAR) if n == 0 => {
                                break self.buf.line_to_char(row + i);
                            }
                            Some(NEW_LINE_CHAR) => n -= 1,
                            Some(_) => (),
                            None => break self.buf.line_to_char(row + i),
                        },
                        None => break self.buf.len_chars().saturating_sub(1),
                    }
                }
            }
            _ => err_at!(Fatal, msg: format!("unreachable")),
        }
    }

    fn mto_bracket(&mut self, evnt: Event) -> Result<Option<Event>> {
        let mut m = 0;
        match evnt {
            MtoBracket(n, yin, yan, Left) => {
                let mut iter = self.iter(Left).enumerate();
                self.cursor -= loop {
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
            MtoBracket(n, yin, yan, Right) => {
                let mut iter = self.iter(Right).enumerate();
                self.cursor += {
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
            _ => err_at!(Fatal, msg: format!("unreachable")),
        }
    }

    fn mto_pattern(&mut self, evnt: Event) -> Result<Option<Event>> {
        let text = self.buf.to_string();
        let search = Search::new(p, &text, fwd)?;
        let byte_off = self.buf.char_to_byte(self.cursor);

        let n = n.saturating_sub(1);
        self.cursor = match evnt {
            MtoPattern(n, pattern, Left) => {
                let item = search.rev(byte_off).skip(n).next();
                match item {
                    Some((s, _)) => s,
                    None => self.cursor,
                }
            },
            }
            MtoPattern(n, pattern, Right) => {
                let item = search.iter(byte_off).skip(n).next();
                match item {
                    Some((s, _)) => s,
                    None => self.cursor,
                }
            }
            _ => err_at!(Fatal, msg: format!("unreachable")),
        }

        Ok(None)
    }


    fn skip_whitespace(&mut self, dir: Dir) -> usize {
        let mut n = 0;
        let n = loop {
            match self.iter(dir).next() {
                Some(ch) if ch.is_whitespace() => n += 1,
                Some(_) => break n,
                None => break n,
            }
        };
        self.cursor = if_else!(dir == Right, self.cursor + n, self.cursor - n);
        n
    }

    fn skip_non_whitespace(&mut self, dir: Dir) -> usize {
        let mut n = 0;
        let n = loop {
            match self.iter(dir).next() {
                Some(ch) if ch.is_whitespace() => n += 1,
                Some(_) => break n,
                None => break n,
            }
        };
        self.cursor = if_else!(dir == Right, self.cursor + n, self.cursor - n);
        n
    }

    fn skip_alphanumeric(&mut self, dir: Dir) -> usize {
        let mut n = 0;
        let n = loop {
            match self.iter(dir).next() {
                Some(ch) if ch.is_alphanumeric() => n += 1,
                Some(_) => break n,
                None => break n,
            }
        };
        self.cursor = if_else!(dir == Right, self.cursor + n, self.cursor - n);
        n
    }

    fn fwd_match_group(&mut self) {
        self.cursor = {
            let mut iter = self.iter(true /*fwd*/).enumerate();
            let res = loop {
                match iter.next() {
                    Some((i, '(')) => break Some((')', i + 1, true)),
                    Some((i, ')')) => break Some(('(', i, false)),
                    Some((i, '{')) => break Some(('}', i + 1, true)),
                    Some((i, '}')) => break Some(('{', i, false)),
                    Some((i, '<')) => break Some(('>', i + 1, true)),
                    Some((i, '>')) => break Some(('<', i, false)),
                    Some((i, '[')) => break Some(('[', i + 1, true)),
                    Some((i, ']')) => break Some(('[', i, false)),
                    Some((_, NEW_LINE_CHAR)) => break None,
                    Some(_) => (),
                    None => break None,
                };
            };
            if let Some((nch, noff, fwd)) = res {
                let cursor = self.cursor + noff;
                let mut iter = self.iter_at(fwd, cursor).enumerate();
                loop {
                    match iter.next() {
                        Some((i, ch)) if ch == nch && fwd => {
                            break cursor + i;
                        }
                        Some((i, ch)) if ch == nch => {
                            break cursor - i - 1;
                        }
                        Some(_) => (),
                        None => break cursor,
                    }
                }
            } else {
                self.cursor
            }
        };
    }
}

impl Change {
    pub fn lines_at(&self, line_idx: usize) -> ropey::iter::Lines {
        self.buf.lines_at(line_idx)
    }

    fn iter<'a>(&'a self, dir: Dir) -> Box<dyn Iterator<Item = char> + 'a> {
        let chars = self.buf.chars_at(self.cursor);
        match dir {
            Dir::Left => Box::new(ReverseIter::new(chars)),
            Dir::Right => Box::new(chars),
        }
    }

    fn iter_at<'a>(
        //
        &'a self,
        dir: Dir,
        cursor: usize,
    ) -> Box<dyn Iterator<Item = char> + 'a> {
        let chars = self.buf.chars_atxcursor);
        match dir {
            Dir::Left => Box::new(ReverseIter::new(chars)),
            Dir::Right => Box::new(chars),
        }
    }

    fn iter_line<'a>(
        //
        &'a self,
        dir: Dir,
    ) -> Box<dyn Iterator<Item = RopeSlice> + 'a> {
        let lines = self.buf.lines_at(self.buf.char_to_line(self.cursor));
        match dir {
            Dir::Left => Box::new(ReverseIter::new(lines)),
            Dir::Right => Box::new(lines),
        }
    }

    fn to_col(&self) -> usize {
        let a_char = self.buf.line_to_char(self.buf.char_to_line(self.cursor));
        self.cursor - a_char
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
