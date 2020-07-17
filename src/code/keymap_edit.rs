use crossterm::event::KeyModifiers;
#[allow(unused_imports)]
use log::{debug, trace};

use std::mem;

use crate::{
    buffer::Buffer,
    code,
    event::{Event, Mod, Mto, DP},
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
    pub fn fold(&mut self, _: &code::Code, buf: &Buffer, evnt: Event) -> Result<Event> {
        match buf.to_mode() {
            "insert" => self.insert_fold(evnt),
            "normal" => self.normal_fold(evnt),
            _ => err_at!(Fatal, msg: format!("unreachable")),
        }
    }

    pub fn to_event_prefix(&self) -> Event {
        self.prefix.clone()
    }
}

impl KeyEdit {
    fn insert_fold(&mut self, e: Event) -> Result<Event> {
        Ok(e)
    }

    fn normal_fold(&mut self, evnt: Event) -> Result<Event> {
        use crate::event::Event::*;
        use crate::event::{Code, Opr};

        let noop = Event::Noop;

        let prefix = mem::replace(&mut self.prefix, Default::default());
        let (m_empty, ctrl) = {
            let m = evnt.to_modifiers();
            (m.is_empty(), m == KeyModifiers::CONTROL)
        };

        let (prefix, evnt) = match prefix {
            Event::Noop if m_empty => match evnt {
                // motion command - characterwise
                Backspace(_m) => (noop, Mt(Mto::Left(1, DP::Nobound))),
                Left(_m) => (noop, Mt(Mto::Left(1, DP::LineBound))),
                Right(_m) => (noop, Mt(Mto::Right(1, DP::LineBound))),
                Home(_m) => (noop, Mt(Mto::LineHome(DP::StickyCol))),
                End(_m) => (noop, Mt(Mto::LineEnd(1, DP::StickyCol))),
                Char('h', _m) => (noop, Mt(Mto::Left(1, DP::LineBound))),
                Char(' ', _m) => (noop, Mt(Mto::Right(1, DP::Nobound))),
                Char('l', _m) => (noop, Mt(Mto::Right(1, DP::LineBound))),
                Char('0', _m) => (noop, Mt(Mto::LineHome(DP::None))),
                Char('^', _m) => (noop, Mt(Mto::LineHome(DP::TextCol))),
                Char('$', _m) => (noop, Mt(Mto::LineEnd(1, DP::StickyCol))),
                Char('|', _m) => (noop, Mt(Mto::Col(1))),
                Char('f', _m) => (F(1, DP::Right), noop),
                Char('F', _m) => (F(1, DP::Left), noop),
                Char('t', _m) => (T(1, DP::Right), noop),
                Char('T', _m) => (T(1, DP::Left), noop),
                Char(';', _m) => (noop, Mt(Mto::CharR(1, DP::Right))),
                Char(',', _m) => (noop, Mt(Mto::CharR(1, DP::Left))),
                // motion command - linewise
                Char('k', _m) => (noop, Mt(Mto::Up(1, DP::StickyCol))),
                Up(_m) => (noop, Mt(Mto::Up(1, DP::StickyCol))),
                Char('-', _m) => (noop, Mt(Mto::Up(1, DP::TextCol))),
                Char('j', _m) => (noop, Mt(Mto::Down(1, DP::StickyCol))),
                Down(_m) => (noop, Mt(Mto::Down(1, DP::StickyCol))),
                Enter(_m) => (noop, Mt(Mto::Down(1, DP::TextCol))),
                Char('+', _m) => (noop, Mt(Mto::Down(1, DP::TextCol))),
                Char('G', _) => (noop, Mt(Mto::Row(0, DP::TextCol))),
                Char('%', _) => (noop, Mt(Mto::Percent(1, DP::None))),

                Char('b', _) => (noop, Mt(Mto::Word(1, DP::Left, DP::Start))),
                Char('B', _) => (noop, Mt(Mto::WWord(1, DP::Left, DP::Start))),
                Char('e', _) => (noop, Mt(Mto::Word(1, DP::Right, DP::End))),
                Char('E', _) => (noop, Mt(Mto::WWord(1, DP::Right, DP::End))),
                Char('{', _) => (noop, Mt(Mto::Para(1, DP::Left))),
                Char('}', _) => (noop, Mt(Mto::Para(1, DP::Right))),
                Char('(', _) => (noop, Mt(Mto::Sentence(1, DP::Left))),
                Char(')', _) => (noop, Mt(Mto::Sentence(1, DP::Right))),
                Char('w', _) => (noop, Mt(Mto::Word(1, DP::Right, DP::Start))),
                Char('W', _) => (noop, Mt(Mto::WWord(1, DP::Right, DP::Start))),
                Char('n', _) => (noop, Mt(Mto::PatternR(1, DP::Right))),
                Char('N', _) => (noop, Mt(Mto::PatternR(1, DP::Left))),
                //
                Char('I', _) => (noop, Md(Mod::Insert(1, DP::TextCol))),
                Char('i', _) => (noop, Md(Mod::Insert(1, DP::None))),
                Char('a', _) => (noop, Md(Mod::Append(1, DP::Right))),
                Char('A', _) => (noop, Md(Mod::Append(1, DP::End))),
                Char('O', _) => (noop, Md(Mod::Open(1, DP::Left))),
                Char('o', _) => (noop, Md(Mod::Open(1, DP::Right))),
                Md(Mod::Insert(n, p)) => (noop, Md(Mod::Insert(n, p))),
                //
                Char('[', _) => (B(1, DP::Left), noop),
                Char(']', _) => (B(1, DP::Right), noop),
                Char('g', _) if ctrl => (noop, Event::Code(Code::StatusFile)),
                Char('g', _) => (G(1), noop),
                // operation prefix
                Char('c', _) => (Op(Opr::Change(1, Mto::None)), noop),
                Char('d', _) => (Op(Opr::Delete(1, Mto::None)), noop),
                Char('y', _) => (Op(Opr::Yank(1, Mto::None)), noop),
                Char('~', _) => (Op(Opr::Swapcase(1, Mto::None)), noop),
                Char('!', _) => (Op(Opr::Filter(1, Mto::None)), noop),
                Char('=', _) => (Op(Opr::Equal(1, Mto::None)), noop),
                Char('<', _) => (Op(Opr::RShift(1, Mto::None)), noop),
                Char('>', _) => (Op(Opr::LShift(1, Mto::None)), noop),
                // numeric prefix
                Char(ch @ '0'..='9', _) => (N(parse_n!(0, ch)), noop),
                evnt => (noop, evnt),
            },
            N(n) if m_empty => match evnt {
                // motion command - characterwise
                Backspace(_m) => (noop, Mt(Mto::Left(n, DP::Nobound))),
                Left(_m) => (noop, Mt(Mto::Left(n, DP::LineBound))),
                Right(_m) => (noop, Mt(Mto::Right(n, DP::LineBound))),
                Home(_m) => (noop, Mt(Mto::LineHome(DP::StickyCol))),
                End(_m) => (noop, Mt(Mto::LineEnd(n, DP::StickyCol))),
                Char('h', _m) => (noop, Mt(Mto::Left(n, DP::LineBound))),
                Char(' ', _m) => (noop, Mt(Mto::Right(n, DP::Nobound))),
                Char('l', _m) => (noop, Mt(Mto::Right(n, DP::LineBound))),
                Char('^', _m) => (noop, Mt(Mto::LineHome(DP::TextCol))),
                Char('$', _m) => (noop, Mt(Mto::LineEnd(n, DP::StickyCol))),
                Char('|', _m) => (noop, Mt(Mto::Col(n))),
                Char('f', _m) => (F(n, DP::Right), noop),
                Char('F', _m) => (F(n, DP::Left), noop),
                Char('t', _m) => (T(n, DP::Right), noop),
                Char('T', _m) => (T(n, DP::Left), noop),
                Char(';', _m) => (noop, Mt(Mto::CharR(n, DP::Right))),
                Char(',', _m) => (noop, Mt(Mto::CharR(n, DP::Left))),
                // motion command - linewise
                Char('k', _m) => (noop, Mt(Mto::Up(n, DP::StickyCol))),
                Up(_m) => (noop, Mt(Mto::Up(n, DP::StickyCol))),
                Char('-', _m) => (noop, Mt(Mto::Up(n, DP::TextCol))),
                Char('j', _m) => (noop, Mt(Mto::Down(n, DP::StickyCol))),
                Down(_m) => (noop, Mt(Mto::Down(n, DP::StickyCol))),
                Enter(_m) => (noop, Mt(Mto::Down(n, DP::TextCol))),
                Char('+', _m) => (noop, Mt(Mto::Down(n, DP::TextCol))),
                Char('G', _) => (noop, Mt(Mto::Row(n, DP::TextCol))),
                Char('%', _) => (noop, Mt(Mto::Percent(n, DP::None))),

                Char('b', _) => (noop, Mt(Mto::Word(n, DP::Left, DP::Start))),
                Char('B', _) => (noop, Mt(Mto::WWord(n, DP::Left, DP::Start))),
                Char('e', _) => (noop, Mt(Mto::Word(n, DP::Right, DP::End))),
                Char('E', _) => (noop, Mt(Mto::WWord(n, DP::Right, DP::End))),
                Char('{', _) => (noop, Mt(Mto::Para(n, DP::Left))),
                Char('}', _) => (noop, Mt(Mto::Para(n, DP::Right))),
                Char('(', _) => (noop, Mt(Mto::Sentence(n, DP::Left))),
                Char(')', _) => (noop, Mt(Mto::Sentence(n, DP::Right))),
                Char('w', _) => (noop, Mt(Mto::Word(n, DP::Right, DP::Start))),
                Char('W', _) => (noop, Mt(Mto::WWord(n, DP::Right, DP::Start))),
                Char('n', _) => (noop, Mt(Mto::PatternR(n, DP::Right))),
                Char('N', _) => (noop, Mt(Mto::PatternR(n, DP::Left))),
                //
                Char('I', _) => (noop, Md(Mod::Insert(n, DP::TextCol))),
                Char('i', _) => (noop, Md(Mod::Insert(n, DP::None))),
                Char('a', _) => (noop, Md(Mod::Append(n, DP::Right))),
                Char('A', _) => (noop, Md(Mod::Append(n, DP::End))),
                Char('O', _) => (noop, Md(Mod::Open(n, DP::Left))),
                Char('o', _) => (noop, Md(Mod::Open(n, DP::Right))),
                Md(Mod::Insert(m, p)) => (noop, Md(Mod::Insert(n * m, p))),
                //
                Char('[', _) => (B(n, DP::Left), noop),
                Char(']', _) => (B(n, DP::Right), noop),
                Char('g', _) if ctrl => (noop, Event::Code(Code::StatusFile)),
                Char('g', _) => (G(n), noop),
                // operation prefix
                Char('c', _) => (Op(Opr::Change(n, Mto::None)), noop),
                Char('d', _) => (Op(Opr::Delete(n, Mto::None)), noop),
                Char('y', _) => (Op(Opr::Yank(n, Mto::None)), noop),
                Char('~', _) => (Op(Opr::Swapcase(n, Mto::None)), noop),
                Char('!', _) => (Op(Opr::Filter(n, Mto::None)), noop),
                Char('=', _) => (Op(Opr::Equal(n, Mto::None)), noop),
                Char('<', _) => (Op(Opr::RShift(n, Mto::None)), noop),
                Char('>', _) => (Op(Opr::LShift(n, Mto::None)), noop),
                // continue with numberic prefix
                Char(ch @ '0'..='9', _) => (N(parse_n!(n, ch)), noop),
                evnt => (noop, evnt),
            },
            G(n) if m_empty => match evnt {
                Char('g', _) if ctrl => (noop, Event::Code(Code::StatusCursor)),
                // motion command - characterwise
                Home(_m) => (noop, Mt(Mto::ScreenHome(DP::None))),
                End(_m) => (noop, Mt(Mto::ScreenEnd(n, DP::None))),
                Char('_', _m) => (noop, Mt(Mto::LineEnd(n, DP::TextCol))),
                Char('0', _m) => (noop, Mt(Mto::ScreenHome(DP::None))),
                Char('^', _m) => (noop, Mt(Mto::ScreenHome(DP::TextCol))),
                Char('$', _m) => (noop, Mt(Mto::ScreenEnd(n, DP::None))),
                Char('m', _m) => (noop, Mt(Mto::ScreenMiddle)),
                Char('M', _m) => (noop, Mt(Mto::LineMiddle(n, DP::None))),
                // motion command - linewise
                Char('k', _m) => (noop, Mt(Mto::ScreenUp(n, DP::None))),
                Up(_m) => (noop, Mt(Mto::ScreenUp(n, DP::None))),
                Char('j', _m) => (noop, Mt(Mto::ScreenDown(n, DP::None))),
                Down(_m) => (noop, Mt(Mto::ScreenDown(n, DP::None))),
                Char('g', _) => (noop, Mt(Mto::Row(1, DP::TextCol))),

                Char('e', _) => (noop, Mt(Mto::Word(n, DP::Left, DP::End))),
                Char('E', _) => (noop, Mt(Mto::WWord(n, DP::Left, DP::End))),
                Char('o', _) => (noop, Mt(Mto::Cursor(n))),
                Char('I', _) => (noop, Md(Mod::Insert(n, DP::TextCol))),
                // operation prefix
                Char('~', _) => (Op(Opr::Swapcase(n, Mto::None)), noop),
                Char('u', _) => (Op(Opr::Lowercase(n, Mto::None)), noop),
                Char('U', _) => (Op(Opr::Uppercase(n, Mto::None)), noop),
                Char('w', _) => (Op(Opr::Format(n, Mto::None)), noop),
                Char('?', _) => (Op(Opr::Encode(n, Mto::None)), noop),
                Char('@', _) => (Op(Opr::Func(n, Mto::None)), noop),
                evnt => (noop, evnt),
            },
            B(n, d) if m_empty => match evnt {
                Char('(', _) => (noop, Mt(Mto::Bracket(n, '(', ')', d))),
                Char(')', _) => (noop, Mt(Mto::Bracket(n, ')', '(', d))),
                Char('{', _) => (noop, Mt(Mto::Bracket(n, '{', '}', d))),
                Char('}', _) => (noop, Mt(Mto::Bracket(n, '}', '{', d))),
                evnt => (noop, evnt),
            },
            F(n, d) if m_empty => match evnt {
                Char(ch, _) => (noop, Mt(Mto::CharF(n, Some(ch), d))),
                evnt => (noop, evnt),
            },
            T(n, d) if m_empty => match evnt {
                Char(ch, _) => (noop, Mt(Mto::CharT(n, Some(ch), d))),
                evnt => (noop, evnt),
            },
            prefix => (prefix, evnt),
        };

        debug!("prefix:{} event:{}", prefix, evnt);

        self.prefix = prefix;
        Ok(evnt)
    }
}
