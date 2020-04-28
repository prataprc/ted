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

macro_rules! want_char {
    ($pe:expr) => {
        use Event::{MtoCharF, MtoCharT};

        if let Some(pe) = $pe {
            match pe {
                MtoCharF(_, _, _) | MtoCharT(_, _, _) => true,
                _ => false,
            }
        } else {
            false
        }
    };
}

macro_rules! normal_event {
    ($evnt:expr, $pe:expr) => {{
        let m = $evnt.to_modifiers();
        let wc = want_char!($pe);
        match $evnt {
            Enter => DownA(1),
            Backspace(n) => MtoLeft(n, false /*line-bound*/),
            Char(ch, _) if wc => MtoChar(ch),
            Char(ch @ '0'..='9', _) if m.is_empty() => Dec(vec![ch]),
            Char('F', _) if m.is_empty() => MtoCharF(None, Left, 1),
            Char('f', _) if m.is_empty() => MtoCharF(None, Right, 1),
            Char('T', _) if m.is_empty() => MtoCharT(None, Left, 1),
            Char('t', _) if m.is_empty() => MtoCharT(None, Right, 1),
            Char('a', _) if m.is_empty() => ModeAppend(1, Left),
            Char('A', _) if m.is_empty() => ModeAppend(1, Right),
            Char('i', _) if m.is_empty() => ModeAppend(1, Left),
            Char('I', _) if m.is_empty() => ModeAppend(1, Right),
            Char('h', _) if m.is_empty() => MtoLeft(1, LineBound),
            Char('l', _) if m.is_empty() => MtoRight(1, LineBound),
            Char(' ', _) if m.is_empty() => MtoRight(1, LineBound),
            Char('j', _) if m.is_empty() => MtoUp(1, Dir::None),
            Char('k', _) if m.is_empty() => MtoDown(1, Dir::None),
            Char('0', _) if m.is_empty() => MtoHome(Dir::None),
            Char('^', _) if m.is_empty() => MtoHome(Caret),
            Char('|', _) if m.is_empty() => MtoCol(1),
            evnt => evnt,
        }
    }};
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

    fn to_prefix_event(&mut self, evnt: Event) -> Result<Option<Event>> {
        use Event::Search;
        use Event::{Append, Char, GotoCol, PrefixN, Right};
        use Event::{Bracket, Insert, OpenDown, OpenUp, PrefixBB, PrefixFB};
        use Event::{Down, DownA, GotoRowA, PrefixG, TChar, Up, UpA};
        use Event::{GotoN, GotoPercent, Paragraph, Sentence, WWord, Word};

        use Event::Dec;
        use Event::Left;

        let m = evnt.to_modifiers();

        let (pe, e) = match (self.evnt_prefix.take(), evnt) {
            // Simple move commands.
            (None, e @ MtoLeft(_, _)) => (None, Some(e)),
            (None, e @ MtoCharF(_, _, _)) => (Some(e), None),
            (None, e @ MtoCharT(_, _, _)) => (Some(e), None),
            (None, e @ MtoPattern(_, _, _)) => (None, Some(e)),
            // simple mode commands
            (None, e @ ModeInsert(_)) => (None, Some(e)),
            (None, e @ ModeAppend(_, _)) => (None, Some(e)),
            // gather N prefix
            (None, Dec(ns)) => (Some(Dec(ns)), None),
            (Dec(mut ns), Dec(ns)) => {
                ns.extend(&ns);
                (Some(Dec(ns)), None)
            }
            // N prefix move command
            (Some(Dec(ns)), MtoCharF(None, dir, n)) => {
                let n = parse_n!(ns)?.saturating_mul(n);
                (Some(MtoCharF(None, dir, n)), None)
            }
            (Some(Dec(ns)), MtoCharT(None, dir, n)) => {
                let n = parse_n!(ns)?.saturating_mul(n);
                (Some(MtoCharT(None, dir, n)), None)
            }
            (Some(Dec(ns)), MtoLeft(n, line_bound)) => (
                let n = parse_n!(xs)?.saturating_mul(n);
                (None, Some(MtoLeft(n, line_bound)))
            )
            (Some(Dec(ns)), MtoPattern(n, pattern, dir)) => (
                let n = parse_n!(ns)?.saturating_mul(n);
                (None, Some(MtoPattern(n, pattern, dir)))
            )
            // N prefix mode command
            (Some(Dec(ns)), ModeInsert(n)) => (
                let n = parse_n!(ns)?.saturating_mul(n);
                (None, Some(ModeInsert(n)))
            )
            (Some(Dec(ns)), ModeAppend(n, tail)) => (
                let n = parse_n!(ns)?.saturating_mul(n);
                (None, Some(ModeAppend(n, tail)))
            )
            // Move commands
            (Some(MtoCharF(None, dir, n)), MtoChar(ch))  => (
                None,
                Some(MtoCharF(Some(ch), dir, n)),
            )
            (Some(MtoCharT(None, dir, n)), MtoChar(ch))  => (
                None,
                Some(MtoCharT(Some(ch), dir, n)),
            )
            (pe, e) => (pe, Some(e)),
        };

        self.evnt_prefix = pe;

        match e {
            Some(PrefixN(mut xs)) if m.is_empty() => match evnt {
                Char('i', _) => (None, Some(Insert(parse_n!(xs)?))),
                Char('I', _) => {
                    self.as_mut_change().home();
                    self.as_mut_change().skip_whitespace(true /*forward*/);
                    (None, Some(Insert(parse_n!(xs)?)))
                }
                Char('o', _) => (None, Some(OpenUp(parse_n!(xs)?))),
                Char('O', _) => (None, Some(OpenDown(parse_n!(xs)?))),
                Char('h', _) => (None, Some(Left(parse_n!(xs)?, true))),
                Char('l', _) => (None, Some(Right(parse_n!(xs)?, true))),
                Char('k', _) => (None, Some(Up(parse_n!(xs)?))),
                Char('j', _) => (None, Some(Down(parse_n!(xs)?))),
                Char('-', _) => (None, Some(UpA(parse_n!(xs)?))),
                Char('+', _) => (None, Some(DownA(parse_n!(xs)?))),
                Char(' ', _) => (None, Some(Right(parse_n!(xs)?, false))),
                Char('|', _) => (None, Some(GotoCol(parse_n!(xs)?))),
                Char(';', _) if self.c.evnt_find_char.is_some() => {
                    let m = parse_n!(xs)?;
                    let evnt_fc = self.c.evnt_find_char.clone().unwrap();
                    let e = match evnt_fc {
                        FChar(_, None, _) => None,
                        FChar(_, Some(ch), d) => Some(FChar(m, Some(ch), d)),
                        TChar(_, None, _) => None,
                        TChar(_, Some(ch), d) => Some(FChar(m, Some(ch), d)),
                        _ => err_at!(Fatal, msg: format!("unreachable"))?,
                    };
                    (None, e)
                }
                Char(';', _) => (None, None),
                Char(',', _) if self.c.evnt_find_char.is_some() => {
                    let m = parse_n!(xs)?;
                    let e = match self.c.evnt_find_char.clone().unwrap() {
                        FChar(_, None, _) => None,
                        FChar(_, Some(ch), d) => Some(FChar(m, Some(ch), !d)),
                        TChar(_, None, _) => None,
                        TChar(_, Some(ch), d) => Some(FChar(m, Some(ch), !d)),
                        _ => err_at!(Fatal, msg: format!("unreachable"))?,
                    };
                    (None, e)
                }
                Char(',', _) => (None, None),
                Char('G', _) => (None, Some(GotoRowA(parse_n!(xs)?))),
                Char('g', _) => (Some(PrefixG(parse_n!(xs)?)), None),
                Char('%', _) => (Some(GotoPercent(parse_n!(xs)?)), None),
                Char('w', _) => (None, Some(Word(parse_n!(xs)?, false, false))),
                Char('e', _) => (None, Some(Word(parse_n!(xs)?, false, true))),
                Char('b', _) => (None, Some(Word(parse_n!(xs)?, true, false))),
                Char('W', _) => (None, Some(WWord(parse_n!(xs)?, false, false))),
                Char('E', _) => (None, Some(WWord(parse_n!(xs)?, false, true))),
                Char('B', _) => (None, Some(WWord(parse_n!(xs)?, true, false))),
                Char(')', _) => (None, Some(Sentence(parse_n!(xs)?, true))),
                Char('(', _) => (None, Some(Sentence(parse_n!(xs)?, false))),
                Char('}', _) => (None, Some(Paragraph(parse_n!(xs)?, true))),
                Char('{', _) => (None, Some(Paragraph(parse_n!(xs)?, false))),
                Char('[', _) => (Some(PrefixBB(parse_n!(xs)?)), None),
                Char(']', _) => (Some(PrefixFB(parse_n!(xs)?)), None),
                Char('n', _) => (None, Some(Search(parse_n!(xs)?, None, true))),
                Char('N', _) => (None, Some(Search(parse_n!(xs)?, None, false))),
                evnt => (None, Some(evnt)),
            },
            Some(PrefixG(n)) if m.is_empty() => match evnt {
                Char('g', _) => (None, Some(GotoRowA(n))),
                Char('e', _) => (None, Some(Word(n, true, true))),
                Char('E', _) => (None, Some(WWord(n, true, true))),
                Char('o', _) => (None, Some(GotoN(n))),
                Char('I', _) => {
                    self.as_mut_change().home();
                    (None, Some(Insert(n)))
                }
                _ => (None, Some(evnt)),
            },
            Some(PrefixBB(n)) if m.is_empty() => match evnt {
                Char('(', _) => (None, Some(Bracket(n, '(', ')', false))),
                Char('{', _) => (None, Some(Bracket(n, '{', '}', false))),
                _ => (None, Some(evnt)),
            },
            Some(PrefixFB(n)) if m.is_empty() => match evnt {
                Char(')', _) => (None, Some(Bracket(n, ')', '(', true))),
                Char('}', _) => (None, Some(Bracket(n, '}', '{', true))),
                _ => (None, Some(evnt)),
            },
            pe => (pe, Some(evnt)),
        };

        Ok(e)
    }

    fn handle_event(&mut self, mut evnt: Event) -> Result<Option<Event>> {
        evnt = match self.to_prefix_event(evnt)? {
            Some(evnt) => evnt,
            None => return Ok(None),
        };

        match evnt {
            // execute motion command.
            e @ MtoLeft(_, _) => self.as_mut_change().mto_left(e),
            e @ MtoRight(_, _) => self.as_mut_change().mto_right(e),
            e @ MtoUp(_, _) => self.as_mut_change().mto_up(e),
            e @ MtoDown(_, _) => self.as_mut_change().mto_down(e),
            e @ MtoCol(_, _) => self.as_mut_change().mto_column(e),
            e @ MtoRow(_, _) => self.as_mut_change().mto_row(e),
            e @ MtoPercent(n) => self.as_mut_change().mto_percent(n),
            e @ MtoHome(pos) => self.as_mut_change().mto_home(pos),
            e @ MtoEnd => self.as_mut_change().mto_end(),
            e @ MtoCharF(_, _, _) => self.as_mut_change().mto_char(e),
            e @ MtoCharT(_, _, _) => self.as_mut_change().mto_char(e),
            MtoCursor(n) => self.as_mut_change().mto_cursor(n),
            e @ MtoWords(_, _, _) => self.as_mut_change().mto_words(e),
            WWord(n, true, tail) if m.is_empty() => {
                self.as_mut_change().fwd_wwords(n, tail);
                Ok(None)
            }
            WWord(n, _, tail) if m.is_empty() => {
                self.as_mut_change().rev_wwords(n, tail);
                Ok(None)
            }
            Char('%', _) if m.is_empty() => {
                self.as_mut_change().fwd_match_group();
                Ok(None)
            }
            Sentence(n, true) if m.is_empty() => {
                self.as_mut_change().fwd_sentence(n);
                Ok(None)
            }
            Sentence(n, _) if m.is_empty() => {
                self.as_mut_change().rev_sentence(n);
                Ok(None)
            }
            Paragraph(n, true) if m.is_empty() => {
                self.as_mut_change().fwd_para(n);
                Ok(None)
            }
            Paragraph(n, _) if m.is_empty() => {
                self.as_mut_change().rev_para(n);
                Ok(None)
            }
            Bracket(n, yin, yan, true) if m.is_empty() => {
                self.as_mut_change().fwd_bracket(yin, yan, n);
                Ok(None)
            }
            Bracket(n, yin, yan, _) if m.is_empty() => {
                self.as_mut_change().rev_bracket(yin, yan, n);
                Ok(None)
            }
            MtoPattern(n, None, fwd) => match self.c.evnt_search.clone() {
                Some(Search(_, Some(patt), fwdo)) => {
                    let fwd = if fwd { fwdo } else { !fwdo };
                    self.as_mut_change().start_search(n, &patt, fwd)?;
                    Ok(None)
                }
                Some(_) | None => Ok(None),
            },
            MtoPattern(n, Some(pattern), fwd) => {
                self.as_mut_change().start_search(n, &pattern, fwd)?;
                self.c.evnt_search = Some(Search(n, Some(pattern), fwd));
                Ok(None)
            }
            TChar(n, Some(ch), d) if d => {
                self.as_mut_change().fwd_char(n, ch, true /*till*/);
                Ok(None)
            }
            TChar(n, Some(ch), _) => {
                self.as_mut_change().rev_char(n, ch, true /*till*/);
                Ok(None)
            }
            TChar(_, _, _) => Ok(None),
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
        use Event::{Backspace, Char, Delete, Enter, Tab};

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
        use Event::{Append, Insert, OpenDown, OpenUp};

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
    fn mto_left(&mut self, evnt: Event) -> Result<Option<Event>> {
        self.cursor = match evnt {
            MtoLeft(n, Dir::LineBound) => {
                let row = self.buf.char_to_line(self.cursor);
                let home = self.buf.line_to_char(row);
                let new_cursor = self.cursor.saturating_sub(n);
                Ok(if_else!(new_cursor > home, new_cursor, home))
            }
            MtoLeft(n, Dir::Unbound) => {
                Ok(self.cursor.saturating_sub(n))
            }
            _ => err_at!(Fatal, msg: format!("unreachable")),
        }?;
        Ok(None)
    }

    fn mto_right(&mut self, evnt: Event) -> Result<Option<Event>> {
        let (n, dir) = match evnt {
            MtoRight(n, dir) => Ok((n, dir)),
            _ => err_at!(Fatal, msg: format!("unreachable")),
        }?;

        for ch in self.buf.chars_at(self.cursor).take(n) {
            if line_bound && ch == NEW_LINE_CHAR {
                break;
            }
            self.cursor += 1
        }

        Ok(None)
    }

    fn mto_up(&mut self, evnt: Event) -> Result<Option<Event>> {
        match evnt {
            MtoUp(n, pos) => {
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
            _ => err_at!(Fatal, msg: format!("unreachable")),
        }
    }

    fn mto_down(&mut self, evnt: Event) -> Result<Option<Event> {
        let row = self.buf.char_to_line(self.cursor);
        let n_lines = self.buf.len_lines();
        match evnt {
            MtoDown(_, _) if n_lines == 0 => Ok(None),
            MtoDown(_, _) if row == n_lines => Ok(None),
            MtoDown(n, pos) => {
                let row = limite!(row.saturating_add(n), n_lines);
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
            }
        }
    }

    fn mto_column(&mut self, evnt: Event) -> Result<Option<Event>> {
        match evnt {
            MtoCol(n) => {
                for ch in self.buf.chars_at(self.cursor).take(n) {
                    if ch == NEW_LINE_CHAR {
                        break;
                    }
                    self.cursor += 1;
                }
            }
        }
    }

    fn mto_row(&mut self, evnt: Event) -> Result<Option<Event>> {
        let row = self.buf.char_to_line(self.cursor);
        let n_lines = self.buf.len_lines();
        match evnt {
            MtoRow(_, _) if n_lines == 0 => Ok(None),
            MtoRow(n, pos) if n < row => {
                self.mto_up(MtoUp(row-n, pos));
                Ok(None)
            }
            MtoRow(n, pos) if n < n_lines => {
                self.mto_up(MtoUp(n-row, pos));
                Ok(None)
            }
            _ => Ok(None)
        }
    }

    fn mto_percent(&mut self, n: usize) -> Result<Option<Event>> {
        let row = self.buf.char_to_line(self.cursor);
        let mut n_lines = self.buf.len_lines();
        match evnt {
            MtoPercent(_) if n_lines == 0 => Ok(None),
            MtoPercent(n) if n < 100 => {
                n_lines -= 1;
                let n = (((n_lines as f64) * (n as f64)) / (100 as f64)) as usize;
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
            Event::MtoCharF(Some(ch), dir, n) => (ch, dir, n, Find),
            Event::MtoCharT(Some(ch), dir, n) => (ch, dir, n, Till),
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

    fn rev_wwords(&mut self, n: usize, tail: bool) {
        let (fwd, line_bound) = (false, false);
        for _ in 0..n {
            let n = self.skip_whitespace(fwd);
            match tail {
                false if n == 0 => {
                    self.skip_non_whitespace(fwd);
                    self.move_right(1, line_bound);
                }
                false => {
                    self.skip_non_whitespace(fwd);
                    self.move_right(1, line_bound);
                }
                true if n == 0 => {
                    self.skip_non_whitespace(fwd);
                    self.skip_whitespace(fwd);
                }
                true => (),
            }
        }
    }

    fn fwd_wwords(&mut self, n: usize, tail: bool) {
        let (fwd, line_bound) = (true, false);
        for _ in 0..n {
            let n = self.skip_whitespace(fwd);
            match tail {
                true if n == 0 => {
                    self.skip_non_whitespace(fwd);
                    self.move_left(1, line_bound);
                }
                true => {
                    self.skip_non_whitespace(fwd);
                    self.move_left(1, line_bound);
                }
                false if n == 0 => {
                    self.skip_non_whitespace(fwd);
                    self.skip_whitespace(fwd);
                }
                false => (),
            }
        }
    }

    fn rev_sentence(&mut self, mut n: usize) {
        let (cursor, nw) = {
            let mut iter = self.iter(false /*forward*/).enumerate();
            let mut prev_ch: Option<char> = None;
            loop {
                prev_ch = match (iter.next(), prev_ch) {
                    (Some((i, '.')), None) if n == 0 => {
                        break (self.cursor.saturating_sub(i + 1), true);
                    }
                    (Some((_, '.')), None) => {
                        n -= 1;
                        Some('.')
                    }
                    (Some((i, NEW_LINE_CHAR)), Some(NEW_LINE_CHAR)) => {
                        if n == 0 {
                            break (self.cursor.saturating_sub(i + 1), false);
                        } else {
                            n -= 1;
                            Some(NEW_LINE_CHAR)
                        }
                    }
                    (Some((_, ch)), _) => Some(ch),
                    (None, _) => {
                        break (0, false);
                    }
                };
            }
        };
        self.cursor = cursor;
        if nw {
            self.fwd_words(1, false /*tail*/);
        }
    }

    fn fwd_sentence(&mut self, mut n: usize) {
        let (cursor, nw) = {
            let mut iter = self.iter(true /*forward*/).enumerate();
            let mut prev_ch: Option<char> = None;
            loop {
                prev_ch = match (iter.next(), prev_ch) {
                    (Some((i, '.')), None) if n == 0 => {
                        break (self.cursor + i, true);
                    }
                    (Some((_, '.')), None) => {
                        n -= 1;
                        Some('.')
                    }
                    (Some((i, NEW_LINE_CHAR)), Some(NEW_LINE_CHAR)) => {
                        if n == 0 {
                            break (self.cursor + i, false);
                        } else {
                            n -= 1;
                            Some(NEW_LINE_CHAR)
                        }
                    }
                    (Some((_, ch)), _) => Some(ch),
                    (None, _) => {
                        break (self.buf.len_chars().saturating_sub(1), false);
                    }
                }
            }
        };
        self.cursor = cursor;
        if nw {
            self.fwd_words(1, false /*tail*/);
        }
    }

    fn rev_para(&mut self, mut n: usize) {
        self.cursor = {
            let row = self.buf.char_to_line(self.cursor);
            let mut iter = self.iter_line(false).enumerate();
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
    }

    fn fwd_para(&mut self, mut n: usize) {
        self.cursor = {
            let row = self.buf.char_to_line(self.cursor);
            let mut iter = self.iter_line(true /*forward*/).enumerate();
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
        };
    }

    fn fwd_bracket(&mut self, yin: char, yan: char, mut n: usize) {
        self.cursor += {
            let mut iter = self.iter(true /*forward*/).enumerate();
            let mut m = 0;
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

    fn rev_bracket(&mut self, yin: char, yan: char, mut n: usize) {
        self.cursor -= {
            let mut iter = self.iter(false /*forward*/).enumerate();
            let mut m = 0;
            loop {
                match iter.next() {
                    Some((_, ch)) if ch == yin && m > 0 => m -= 1,
                    Some((i, ch)) if ch == yin && n == 0 => break i + 1,
                    Some((_, ch)) if ch == yin => n -= 1,
                    Some((_, ch)) if ch == yan => m += 1,
                    Some(_) => (),
                    None => break 0,
                }
            }
        };
    }


    fn move_to_pattern(&mut self, evnt: Event) -> Result<Option<Event>> {
        let text = self.buf.to_string();
        let search = Search::new(p, &text, fwd)?;
        let byte_off = self.buf.char_to_byte(self.cursor);

        let n = n.saturating_sub(1);
        match fwd {
            true => match search.iter(byte_off).skip(n).next() {
                Some((s, _)) => self.cursor = s,
                None => (),
            },
            false => match search.rev(byte_off).skip(n).next() {
                Some((s, _)) => self.cursor = s,
                None => (),
            },
        }

        Ok(search)
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
            Dir::Right => Box::new(chars),
            Dir::Left => Box::new(ReverseIter::new(chars)),
        }
    }

    fn iter_at<'a>(
        //
        &'a self,
        fwd: bool,
        cursor: usize,
    ) -> Box<dyn Iterator<Item = char> + 'a> {
        if fwd {
            Box::new(self.buf.chars_at(cursor))
        } else {
            Box::new(ReverseIter::new(self.buf.chars_at(cursor)))
        }
    }

    fn iter_line<'a>(
        //
        &'a self,
        fwd: bool,
    ) -> Box<dyn Iterator<Item = RopeSlice> + 'a> {
        let row = self.buf.char_to_line(self.cursor);
        if fwd {
            Box::new(self.buf.lines_at(row))
        } else {
            Box::new(ReverseIter::new(self.buf.lines_at(row)))
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
