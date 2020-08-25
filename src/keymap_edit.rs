#[allow(unused_imports)]
use log::{debug, trace};

use std::mem;

use crate::{
    buffer::Buffer,
    event::{self, Event, Mod, Mto, Scroll, DP},
    Error, Result,
};

macro_rules! parse_n {
    ($n:expr, $ch:expr) => {{
        let m = $ch.to_digit(10).unwrap_or(1) as usize;
        ($n * 10) + m
    }};
}

#[derive(Clone, Default)]
pub struct KeyEdit {
    prefix: Event,
}

impl KeyEdit {
    pub fn fold(&mut self, buf: &Buffer, evnt: Event) -> Result<Event> {
        match buf.to_mode() {
            "insert" => self.insert_fold(buf, evnt),
            "normal" => self.normal_fold(buf, evnt),
            _ => err_at!(Fatal, msg: format!("unreachable")),
        }
    }

    pub fn to_event_prefix(&self) -> Event {
        self.prefix.clone()
    }
}

impl KeyEdit {
    fn insert_fold(&mut self, _: &Buffer, evnt: Event) -> Result<Event> {
        use crate::event::Event::*;
        use crate::event::Insrt;

        let noop = Event::Noop;

        let prefix = mem::replace(&mut self.prefix, Event::default());
        let (empty, ctrl) = {
            use crossterm::event::KeyModifiers as KM;
            let m = evnt.to_modifiers();
            (m.is_empty(), m.contains(KM::CONTROL))
        };

        let (prefix, evnt) = match prefix {
            Event::Noop if empty => match evnt {
                Event::Esc => (noop, Md(Mod::Esc)),
                evnt @ Event::Delete(_) => (noop, evnt),
                evnt @ Event::Tab(_) => (noop, evnt),
                evnt @ Event::Enter(_) => (noop, evnt),
                evnt @ Event::Up(_) => (noop, evnt),
                evnt @ Event::Down(_) => (noop, evnt),
                evnt @ Event::Left(_) => (noop, evnt),
                evnt @ Event::Right(_) => (noop, evnt),
                evnt @ Event::Home(_) => (noop, evnt),
                evnt @ Event::End(_) => (noop, evnt),
                evnt @ Event::PageUp(_) => (noop, evnt),
                evnt @ Event::PageDown(_) => (noop, evnt),
                Event::Char('h', m) => (noop, Backspace(m)),
                evnt => (noop, evnt),
            },
            Event::Noop if ctrl => match evnt {
                Event::Char('[', _) => (noop, Md(Mod::Esc)),
                Event::Char('a', _) => (noop, In(Insrt::ReInsert)),
                Event::Char('@', _) => {
                    let mut evnt = In(Insrt::ReInsert);
                    evnt.push(Md(Mod::Esc));
                    (noop, evnt)
                }
                Event::Char('w', _) => (noop, In(Insrt::RemoveWord)),
                Event::Char('u', _) => (noop, In(Insrt::RemoveLine)),
                Event::Char('n', _) => (noop, In(Insrt::NextWord)),
                Event::Char('p', _) => (noop, In(Insrt::PrevWord)),
                Event::Char('t', _) => (noop, In(Insrt::RShift(1))),
                Event::Char('d', _) => (noop, In(Insrt::LShift(1))),
                evnt => (noop, evnt),
            },
            prefix => (prefix, evnt),
        };

        debug!("insert prefix:{} event:{}", prefix, evnt);

        self.prefix = prefix;
        Ok(evnt)
    }

    fn normal_fold(&mut self, _: &Buffer, evnt: Event) -> Result<Event> {
        use crate::event::Event::*;

        let noop = Event::Noop;

        let prefix = mem::replace(&mut self.prefix, Event::default());
        let (empty, ctrl, shift) = {
            use crossterm::event::KeyModifiers as KM;
            let m = evnt.to_modifiers();
            (m.is_empty(), m.contains(KM::CONTROL), m.contains(KM::SHIFT))
        };

        let (prefix, evnt) = match prefix {
            Event::Noop if empty | shift => match evnt {
                // motion command - characterwise
                Backspace(_) => (noop, Mt(Mto::Left(1, DP::Nobound))),
                Left(_) => (noop, Mt(Mto::Left(1, DP::LineBound))),
                Right(_) => (noop, Mt(Mto::Right(1, DP::LineBound))),
                Home(_) => (noop, Mt(Mto::LineHome(DP::StickyCol))),
                End(_) => (noop, Mt(Mto::LineEnd(1, DP::StickyCol))),
                Char('h', _) => (noop, Mt(Mto::Left(1, DP::LineBound))),
                Char(' ', _) => (noop, Mt(Mto::Right(1, DP::Nobound))),
                Char('l', _) => (noop, Mt(Mto::Right(1, DP::LineBound))),
                Char('0', _) => (noop, Mt(Mto::LineHome(DP::None))),
                Char('^', _) => (noop, Mt(Mto::LineHome(DP::TextCol))),
                Char('$', _) => (noop, Mt(Mto::LineEnd(1, DP::StickyCol))),
                Char('|', _) => (noop, Mt(Mto::Col(1))),
                Char('f', _) => (F(1, DP::Right), noop),
                Char('F', _) => (F(1, DP::Left), noop),
                Char('t', _) => (T(1, DP::Right), noop),
                Char('T', _) => (T(1, DP::Left), noop),
                Char(';', _) => (noop, Mt(Mto::CharR(1, DP::Right))),
                Char(',', _) => (noop, Mt(Mto::CharR(1, DP::Left))),
                // motion command - linewise
                Char('k', _) => (noop, Mt(Mto::Up(1, DP::StickyCol))),
                Up(_) => (noop, Mt(Mto::Up(1, DP::StickyCol))),
                Char('-', _) => (noop, Mt(Mto::Up(1, DP::TextCol))),
                Char('j', _) => (noop, Mt(Mto::Down(1, DP::StickyCol))),
                Down(_) => (noop, Mt(Mto::Down(1, DP::StickyCol))),
                Enter(_) => (noop, Mt(Mto::Down(1, DP::TextCol))),
                Char('+', _) => (noop, Mt(Mto::Down(1, DP::TextCol))),
                Char('G', _) => (noop, Mt(Mto::Row(std::usize::MAX, DP::TextCol))),
                Char('%', _) => (noop, Mt(Mto::MatchPair)),
                // motion command - word/sentence/para
                Char('w', _) => (noop, Mt(Mto::Word(1, DP::Right, DP::Start))),
                Char('W', _) => (noop, Mt(Mto::WWord(1, DP::Right, DP::Start))),
                Char('e', _) => (noop, Mt(Mto::Word(1, DP::Right, DP::End))),
                Char('E', _) => (noop, Mt(Mto::WWord(1, DP::Right, DP::End))),
                Char('b', _) => (noop, Mt(Mto::Word(1, DP::Left, DP::End))),
                Char('B', _) => (noop, Mt(Mto::WWord(1, DP::Left, DP::End))),
                Char('(', _) => (noop, Mt(Mto::Sentence(1, DP::Left))),
                Char(')', _) => (noop, Mt(Mto::Sentence(1, DP::Right))),
                Char('{', _) => (noop, Mt(Mto::Para(1, DP::Left))),
                Char('}', _) => (noop, Mt(Mto::Para(1, DP::Right))),
                // motion command - various
                Char('H', _) => (noop, Mt(Mto::WinH(1))),
                Char('M', _) => (noop, Mt(Mto::WinM)),
                Char('L', _) => (noop, Mt(Mto::WinL(1))),
                // motion command - mark, jumps and searches
                Char('m', _) => (M, noop),
                Char('\'', _) => (J('\''), noop),
                Char('`', _) => (J('`'), noop),
                Char('n', _) => (noop, Mt(Mto::PatternR(1, DP::Right))),
                Char('N', _) => (noop, Mt(Mto::PatternR(1, DP::Left))),
                // prefix commands
                Char(ch @ '0'..='9', _) => (N(parse_n!(0, ch)), noop),
                Char('[', _) => (B(1, DP::Left), noop),
                Char(']', _) => (B(1, DP::Right), noop),
                Char('g', _) => (G(1), noop),
                Char('z', _) => (Z(0), noop),
                // operation prefix
                Char('c', _) => (Op(event::Opr::Change(1, Mto::None)), noop),
                Char('d', _) => (Op(event::Opr::Delete(1, Mto::None)), noop),
                Char('y', _) => (Op(event::Opr::Yank(1, Mto::None)), noop),
                Char('~', _) => (Op(event::Opr::Swapcase(1, Mto::None)), noop),
                Char('!', _) => (Op(event::Opr::Filter(1, Mto::None)), noop),
                Char('=', _) => (Op(event::Opr::Equal(1, Mto::None)), noop),
                Char('<', _) => (Op(event::Opr::RShift(1, Mto::None)), noop),
                Char('>', _) => (Op(event::Opr::LShift(1, Mto::None)), noop),
                //
                Char('I', _) => (noop, Md(Mod::Insert(1, DP::TextCol))),
                Char('i', _) => (noop, Md(Mod::Insert(1, DP::None))),
                Char('a', _) => (noop, Md(Mod::Append(1, DP::Right))),
                Char('A', _) => (noop, Md(Mod::Append(1, DP::End))),
                Char('O', _) => (noop, Md(Mod::Open(1, DP::Left))),
                Char('o', _) => (noop, Md(Mod::Open(1, DP::Right))),
                Md(Mod::Insert(n, p)) => (noop, Md(Mod::Insert(n, p))),
                evnt => (noop, evnt),
            },
            Event::Noop if ctrl => match evnt {
                // motion commands, window scroll.
                Char('g', _) => {
                    let evnt = Event::Appn(event::Appn::StatusFile);
                    (noop, evnt)
                }
                Char('e', _) => {
                    let evnt = Mt(Mto::WinScroll(1, Scroll::Ones, DP::Right));
                    (noop, evnt)
                }
                Char('d', _) => {
                    let evnt = Mt(Mto::WinScroll(1, Scroll::Lines, DP::Right));
                    (noop, evnt)
                }
                Char('f', _) | PageDown(_) => {
                    let evnt = Mt(Mto::WinScroll(1, Scroll::Pages, DP::Right));
                    (noop, evnt)
                }
                Char('y', _) => {
                    let evnt = Mt(Mto::WinScroll(1, Scroll::Ones, DP::Left));
                    (noop, evnt)
                }
                Char('u', _) => {
                    let evnt = Mt(Mto::WinScroll(1, Scroll::Lines, DP::Left));
                    (noop, evnt)
                }
                Char('b', _) | PageUp(_) => {
                    let evnt = Mt(Mto::WinScroll(1, Scroll::Pages, DP::Left));
                    (noop, evnt)
                }
                evnt => (noop, evnt),
            },
            N(n) if empty | shift => match evnt {
                // motion command - characterwise
                Backspace(_) => (noop, Mt(Mto::Left(n, DP::Nobound))),
                Left(_) => (noop, Mt(Mto::Left(n, DP::LineBound))),
                Right(_) => (noop, Mt(Mto::Right(n, DP::LineBound))),
                Home(_) => (noop, Mt(Mto::LineHome(DP::StickyCol))),
                End(_) => (noop, Mt(Mto::LineEnd(n, DP::StickyCol))),
                Char('h', _) => (noop, Mt(Mto::Left(n, DP::LineBound))),
                Char(' ', _) => (noop, Mt(Mto::Right(n, DP::Nobound))),
                Char('l', _) => (noop, Mt(Mto::Right(n, DP::LineBound))),
                Char('^', _) => (noop, Mt(Mto::LineHome(DP::TextCol))),
                Char('$', _) => (noop, Mt(Mto::LineEnd(n, DP::StickyCol))),
                Char('|', _) => (noop, Mt(Mto::Col(n))),
                Char('f', _) => (F(n, DP::Right), noop),
                Char('F', _) => (F(n, DP::Left), noop),
                Char('t', _) => (T(n, DP::Right), noop),
                Char('T', _) => (T(n, DP::Left), noop),
                Char(';', _) => (noop, Mt(Mto::CharR(n, DP::Right))),
                Char(',', _) => (noop, Mt(Mto::CharR(n, DP::Left))),
                // motion command - linewise
                Char('k', _) => (noop, Mt(Mto::Up(n, DP::StickyCol))),
                Up(_) => (noop, Mt(Mto::Up(n, DP::StickyCol))),
                Char('-', _) => (noop, Mt(Mto::Up(n, DP::TextCol))),
                Char('j', _) => (noop, Mt(Mto::Down(n, DP::StickyCol))),
                Down(_) => (noop, Mt(Mto::Down(n, DP::StickyCol))),
                Enter(_) => (noop, Mt(Mto::Down(n, DP::TextCol))),
                Char('+', _) => (noop, Mt(Mto::Down(n, DP::TextCol))),
                Char('G', _) => (noop, Mt(Mto::Row(n, DP::TextCol))),
                Char('%', _) => (noop, Mt(Mto::Percent(n, DP::TextCol))),
                // motion command - word/sentence/para
                Char('w', _) => (noop, Mt(Mto::Word(n, DP::Right, DP::Start))),
                Char('W', _) => (noop, Mt(Mto::WWord(n, DP::Right, DP::Start))),
                Char('e', _) => (noop, Mt(Mto::Word(n, DP::Right, DP::End))),
                Char('E', _) => (noop, Mt(Mto::WWord(n, DP::Right, DP::End))),
                Char('b', _) => (noop, Mt(Mto::Word(n, DP::Left, DP::End))),
                Char('B', _) => (noop, Mt(Mto::WWord(n, DP::Left, DP::End))),
                Char('(', _) => (noop, Mt(Mto::Sentence(n, DP::Left))),
                Char(')', _) => (noop, Mt(Mto::Sentence(n, DP::Right))),
                Char('{', _) => (noop, Mt(Mto::Para(n, DP::Left))),
                Char('}', _) => (noop, Mt(Mto::Para(n, DP::Right))),
                // motion command - various
                Char('H', _) => (noop, Mt(Mto::WinH(n))),
                Char('M', _) => (noop, Mt(Mto::WinM)),
                Char('L', _) => (noop, Mt(Mto::WinL(n))),
                // motion command - mark, jumps and searches
                Char('m', _) => (M, noop),
                Char('\'', _) => (J('\''), noop),
                Char('`', _) => (J('`'), noop),
                Char('n', _) => (noop, Mt(Mto::PatternR(n, DP::Right))),
                Char('N', _) => (noop, Mt(Mto::PatternR(n, DP::Left))),
                // prefix commands
                Char(ch @ '0'..='9', _) => (N(parse_n!(n, ch)), noop),
                Char('[', _) => (B(n, DP::Left), noop),
                Char(']', _) => (B(n, DP::Right), noop),
                Char('g', _) => (G(n), noop),
                Char('z', _) => (Z(n), noop),
                // operation prefix
                Char('c', _) => (Op(event::Opr::Change(n, Mto::None)), noop),
                Char('d', _) => (Op(event::Opr::Delete(n, Mto::None)), noop),
                Char('y', _) => (Op(event::Opr::Yank(n, Mto::None)), noop),
                Char('~', _) => (Op(event::Opr::Swapcase(n, Mto::None)), noop),
                Char('!', _) => (Op(event::Opr::Filter(n, Mto::None)), noop),
                Char('=', _) => (Op(event::Opr::Equal(n, Mto::None)), noop),
                Char('<', _) => (Op(event::Opr::RShift(n, Mto::None)), noop),
                Char('>', _) => (Op(event::Opr::LShift(n, Mto::None)), noop),
                //
                Char('I', _) => (noop, Md(Mod::Insert(n, DP::TextCol))),
                Char('i', _) => (noop, Md(Mod::Insert(n, DP::None))),
                Char('a', _) => (noop, Md(Mod::Append(n, DP::Right))),
                Char('A', _) => (noop, Md(Mod::Append(n, DP::End))),
                Char('O', _) => (noop, Md(Mod::Open(n, DP::Left))),
                Char('o', _) => (noop, Md(Mod::Open(n, DP::Right))),
                Md(Mod::Insert(m, p)) => (noop, Md(Mod::Insert(n * m, p))),
                evnt => (noop, evnt),
            },
            N(n) if ctrl => match evnt {
                // motion commands, window scroll.
                Char('g', _) => {
                    let evnt = Event::Appn(event::Appn::StatusFile);
                    (noop, evnt)
                }
                Char('e', _) => {
                    let evnt = Mt(Mto::WinScroll(n, Scroll::Ones, DP::Right));
                    (noop, evnt)
                }
                Char('d', _) => {
                    let evnt = Mt(Mto::WinScroll(n, Scroll::Lines, DP::Right));
                    (noop, evnt)
                }
                Char('f', _) => {
                    let evnt = Mt(Mto::WinScroll(n, Scroll::Pages, DP::Right));
                    (noop, evnt)
                }
                Char('y', _) => {
                    let evnt = Mt(Mto::WinScroll(n, Scroll::Ones, DP::Left));
                    (noop, evnt)
                }
                Char('u', _) => {
                    let evnt = Mt(Mto::WinScroll(n, Scroll::Lines, DP::Left));
                    (noop, evnt)
                }
                Char('b', _) | PageUp(_) => {
                    let evnt = Mt(Mto::WinScroll(n, Scroll::Pages, DP::Left));
                    (noop, evnt)
                }
                evnt => (noop, evnt),
            },
            G(n) if empty | shift => match evnt {
                // motion command - characterwise
                Home(_) => (noop, Mt(Mto::ScreenHome(DP::None))),
                End(_) => (noop, Mt(Mto::ScreenEnd(n, DP::None))),
                Char('_', _) => (noop, Mt(Mto::LineEnd(n, DP::TextCol))),
                Char('0', _) => (noop, Mt(Mto::ScreenHome(DP::None))),
                Char('^', _) => (noop, Mt(Mto::ScreenHome(DP::TextCol))),
                Char('$', _) => (noop, Mt(Mto::ScreenEnd(n, DP::None))),
                Char('m', _) => (noop, Mt(Mto::ScreenMiddle)),
                Char('M', _) => (noop, Mt(Mto::LineMiddle(n, DP::None))),
                // motion command - linewise
                Char('k', _) => (noop, Mt(Mto::ScreenUp(n, DP::None))),
                Up(_) => (noop, Mt(Mto::ScreenUp(n, DP::None))),
                Char('j', _) => (noop, Mt(Mto::ScreenDown(n, DP::None))),
                Down(_) => (noop, Mt(Mto::ScreenDown(n, DP::None))),
                Char('g', _) => (noop, Mt(Mto::Row(n, DP::TextCol))),
                // motion command - wordwise
                Char('e', _) => (noop, Mt(Mto::Word(n, DP::Left, DP::Start))),
                Char('E', _) => (noop, Mt(Mto::WWord(n, DP::Left, DP::Start))),

                Char('o', _) => (noop, Mt(Mto::Cursor(n))),
                Char('I', _) => (noop, Md(Mod::Insert(n, DP::TextCol))),
                // operation prefix
                Char('~', _) => (Op(event::Opr::Swapcase(n, Mto::None)), noop),
                Char('u', _) => (Op(event::Opr::Lowercase(n, Mto::None)), noop),
                Char('U', _) => (Op(event::Opr::Uppercase(n, Mto::None)), noop),
                Char('w', _) => (Op(event::Opr::Format(n, Mto::None)), noop),
                Char('?', _) => (Op(event::Opr::Encode(n, Mto::None)), noop),
                Char('@', _) => (Op(event::Opr::Func(n, Mto::None)), noop),
                evnt => (noop, evnt),
            },
            G(_) if ctrl => match evnt {
                Char('g', _) => {
                    let evnt = Event::Appn(event::Appn::StatusCursor);
                    (noop, evnt)
                }
                evnt => (noop, evnt),
            },
            B(n, d) if empty => match evnt {
                Char('(', _) => (noop, Mt(Mto::UnmatchPair(n, '(', d))),
                Char(')', _) => (noop, Mt(Mto::UnmatchPair(n, ')', d))),
                Char('{', _) => (noop, Mt(Mto::UnmatchPair(n, '{', d))),
                Char('}', _) => (noop, Mt(Mto::UnmatchPair(n, '}', d))),
                evnt => (noop, evnt),
            },
            F(n, d) if empty => match evnt {
                Char(ch, _) => (noop, Mt(Mto::CharF(n, Some(ch), d))),
                evnt => (noop, evnt),
            },
            T(n, d) if empty => match evnt {
                Char(ch, _) => (noop, Mt(Mto::CharT(n, Some(ch), d))),
                evnt => (noop, evnt),
            },
            Z(n) if empty => match evnt {
                // motion commands, window scroll - vertical
                Enter(_) => {
                    let evnt = Mt(Mto::WinScroll(n, Scroll::TextUp, DP::TextCol));
                    (noop, evnt)
                }
                Char('+', _) => {
                    let evnt = Mt(Mto::WinScroll(n, Scroll::Cursor, DP::Right));
                    (noop, evnt)
                }
                Char('^', _) => {
                    let evnt = Mt(Mto::WinScroll(n, Scroll::Cursor, DP::Left));
                    (noop, evnt)
                }
                Char('t', _) => {
                    let evnt = Mt(Mto::WinScroll(n, Scroll::TextUp, DP::None));
                    (noop, evnt)
                }
                Char('.', _) => {
                    let evnt = Mt(Mto::WinScroll(n, Scroll::TextCenter, DP::TextCol));
                    (noop, evnt)
                }
                Char('z', _) => {
                    let evnt = Mt(Mto::WinScroll(n, Scroll::TextCenter, DP::None));
                    (noop, evnt)
                }
                Char('-', _) => {
                    let evnt = Mt(Mto::WinScroll(n, Scroll::TextBottom, DP::TextCol));
                    (noop, evnt)
                }
                Char('b', _) => {
                    let evnt = Mt(Mto::WinScroll(n, Scroll::TextBottom, DP::None));
                    (noop, evnt)
                }
                // motion commands, window scroll - horizontal
                Char('l', _) => {
                    let evnt = Mt(Mto::WinScroll(n, Scroll::Chars, DP::Right));
                    (noop, evnt)
                }
                Char('h', _) => {
                    let evnt = Mt(Mto::WinScroll(n, Scroll::Chars, DP::Left));
                    (noop, evnt)
                }
                Char('L', _) => {
                    let evnt = Mt(Mto::WinScroll(n, Scroll::Slide, DP::Right));
                    (noop, evnt)
                }
                Char('H', _) => {
                    let evnt = Mt(Mto::WinScroll(n, Scroll::Slide, DP::Left));
                    (noop, evnt)
                }
                Char('s', _) => {
                    let evnt = Mt(Mto::WinScroll(n, Scroll::Align, DP::Right));
                    (noop, evnt)
                }
                Char('e', _) => {
                    let evnt = Mt(Mto::WinScroll(n, Scroll::Align, DP::Left));
                    (noop, evnt)
                }
                evnt => (noop, evnt),
            },
            M if empty => match evnt {
                Char(ch, _) => match ch {
                    'a'..='z' | 'A'..='Z' | '\'' | '`' => (noop, Mr(ch.into())),
                    _ => (noop, evnt),
                },
                evnt => (noop, evnt),
            },
            J(typ) if empty => match evnt {
                Char(ch, _) => match ch {
                    '\'' | '`' => (noop, Mt(Mto::Jump(typ, ch))),
                    'a'..='z' => (noop, Mt(Mto::Jump(typ, ch))),
                    'A'..='Z' => (noop, Mt(Mto::Jump(typ, ch))),
                    _ => (noop, evnt),
                },
                evnt => (noop, evnt),
            },
            prefix => (prefix, evnt),
        };

        debug!("normal prefix:{} event:{}", prefix, evnt);

        self.prefix = prefix;
        Ok(evnt)
    }
}
