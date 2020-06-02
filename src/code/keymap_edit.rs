use crossterm::event::KeyModifiers;
use log::trace;

use std::mem;

use crate::{
    buffer::Buffer,
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
    pub fn fold(&mut self, buf: &Buffer, evnt: Event) -> Result<Event> {
        match buf.to_mode() {
            "insert" => self.insert_fold(evnt),
            "normal" => self.normal_fold(evnt),
            _ => err_at!(Fatal, msg: format!("unreachable")),
        }
    }

    fn insert_fold(&mut self, e: Event) -> Result<Event> {
        Ok(e)
    }

    fn normal_fold(&mut self, evnt: Event) -> Result<Event> {
        use crate::event::Code::{StatusCursor, StatusFile};
        use crate::event::Event::{Backspace, Char, Enter};
        use crate::event::Event::{Code, Md, Mt, B, F, G, N, T};

        let noop = Event::Noop;

        let prefix = mem::replace(&mut self.prefix, Default::default());
        let (m_empty, ctrl) = {
            let m = evnt.to_modifiers();
            (m.is_empty(), m == KeyModifiers::CONTROL)
        };

        let (prefix, evnt) = match prefix {
            Event::Noop if m_empty => match evnt {
                Backspace => (noop, Mt(Mto::Left(1, DP::Nobound))),
                Enter => (noop, Mt(Mto::Down(1, DP::Caret))),
                Char('h', _) => (noop, Mt(Mto::Left(1, DP::LineBound))),
                Char(' ', _) => (noop, Mt(Mto::Right(1, DP::Nobound))),
                Char('l', _) => (noop, Mt(Mto::Right(1, DP::LineBound))),
                Char('-', _) => (noop, Mt(Mto::Up(1, DP::Caret))),
                Char('j', _) => (noop, Mt(Mto::Up(1, DP::Nope))),
                Char('k', _) => (noop, Mt(Mto::Down(1, DP::Nope))),
                Char('+', _) => (noop, Mt(Mto::Down(1, DP::Caret))),
                Char('|', _) => (noop, Mt(Mto::Col(1))),
                Char('G', _) => (noop, Mt(Mto::Row(1, DP::Caret))),
                Char('%', _) => (noop, Mt(Mto::Percent(1))),
                Char('0', _) => (noop, Mt(Mto::Home(DP::Nope))),
                Char('^', _) => (noop, Mt(Mto::Home(DP::Caret))),
                Char('$', _) => (noop, Mt(Mto::End)),
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
                Char(';', _) => (noop, Mt(Mto::CharR(1, DP::Right))),
                Char(',', _) => (noop, Mt(Mto::CharR(1, DP::Left))),
                Char('n', _) => (noop, Mt(Mto::PatternR(1, DP::Right))),
                Char('N', _) => (noop, Mt(Mto::PatternR(1, DP::Left))),
                //
                Char('I', _) => (noop, Md(Mod::Insert(1, DP::Caret))),
                Char('i', _) => (noop, Md(Mod::Insert(1, DP::Nope))),
                Char('a', _) => (noop, Md(Mod::Append(1, DP::Right))),
                Char('A', _) => (noop, Md(Mod::Append(1, DP::End))),
                Char('O', _) => (noop, Md(Mod::Open(1, DP::Left))),
                Char('o', _) => (noop, Md(Mod::Open(1, DP::Right))),
                Md(Mod::Insert(n, p)) => (noop, Md(Mod::Insert(n, p))),
                //
                Char('[', _) => (B(1, DP::Left), Event::Noop),
                Char(']', _) => (B(1, DP::Right), Event::Noop),
                Char('g', _) => (G(1), Event::Noop),
                Char('f', _) => (F(1, DP::Right), Event::Noop),
                Char('F', _) => (F(1, DP::Left), Event::Noop),
                Char('t', _) => (T(1, DP::Right), Event::Noop),
                Char('T', _) => (T(1, DP::Left), Event::Noop),
                Char(ch @ '0'..='9', _) => (N(parse_n!(1, ch)), Event::Noop),
                evnt => (noop, evnt),
            },
            B(n, d) if m_empty => match evnt {
                Char('(', _) => (noop, Mt(Mto::Bracket(n, '(', ')', d))),
                Char(')', _) => (noop, Mt(Mto::Bracket(n, ')', '(', d))),
                Char('{', _) => (noop, Mt(Mto::Bracket(n, '{', '}', d))),
                Char('}', _) => (noop, Mt(Mto::Bracket(n, '}', '{', d))),
                evnt => (noop, evnt),
            },
            G(n) if m_empty => match evnt {
                Char('g', _) if ctrl => (noop, Code(StatusCursor)),
                Char('g', _) => (noop, Mt(Mto::Row(n, DP::Caret))),
                Char('e', _) => (noop, Mt(Mto::Word(n, DP::Left, DP::End))),
                Char('E', _) => (noop, Mt(Mto::WWord(n, DP::Left, DP::End))),
                Char('o', _) => (noop, Mt(Mto::Cursor(n))),
                Char('I', _) => (noop, Md(Mod::Insert(n, DP::Caret))),
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
            N(n) if m_empty => match evnt {
                Backspace => (noop, Mt(Mto::Left(n, DP::Nobound))),
                Enter => (noop, Mt(Mto::Down(n, DP::Caret))),
                Char('h', _) => (noop, Mt(Mto::Left(n, DP::LineBound))),
                Char(' ', _) => (noop, Mt(Mto::Right(n, DP::Nobound))),
                Char('l', _) => (noop, Mt(Mto::Right(n, DP::LineBound))),
                Char('-', _) => (noop, Mt(Mto::Up(n, DP::Caret))),
                Char('j', _) => (noop, Mt(Mto::Up(n, DP::Nope))),
                Char('k', _) => (noop, Mt(Mto::Down(n, DP::Nope))),
                Char('+', _) => (noop, Mt(Mto::Down(n, DP::Caret))),
                Char('|', _) => (noop, Mt(Mto::Col(n))),
                Char('G', _) => (noop, Mt(Mto::Row(n, DP::Caret))),
                Char('%', _) => (noop, Mt(Mto::Percent(n))),
                Char('0', _) => (noop, Mt(Mto::Home(DP::Nope))),
                Char('^', _) => (noop, Mt(Mto::Home(DP::Caret))),
                Char('$', _) => (noop, Mt(Mto::End)),
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
                Char(';', _) => (noop, Mt(Mto::CharR(n, DP::Right))),
                Char(',', _) => (noop, Mt(Mto::CharR(n, DP::Left))),
                Char('n', _) => (noop, Mt(Mto::PatternR(n, DP::Right))),
                Char('N', _) => (noop, Mt(Mto::PatternR(n, DP::Left))),
                //
                Char('I', _) => (noop, Md(Mod::Insert(n, DP::Caret))),
                Char('i', _) => (noop, Md(Mod::Insert(n, DP::Nope))),
                Char('a', _) => (noop, Md(Mod::Append(n, DP::Right))),
                Char('A', _) => (noop, Md(Mod::Append(n, DP::End))),
                Char('O', _) => (noop, Md(Mod::Open(n, DP::Left))),
                Char('o', _) => (noop, Md(Mod::Open(n, DP::Right))),
                Md(Mod::Insert(m, p)) => (noop, Md(Mod::Insert(n * m, p))),
                //
                Char('[', _) => (B(n, DP::Left), Event::Noop),
                Char(']', _) => (B(n, DP::Right), Event::Noop),
                Char('g', _) => (G(n), Event::Noop),
                Char('f', _) => (F(n, DP::Right), Event::Noop),
                Char('F', _) => (F(n, DP::Left), Event::Noop),
                Char('t', _) => (T(n, DP::Right), Event::Noop),
                Char('T', _) => (T(n, DP::Left), Event::Noop),
                Char(ch @ '0'..='9', _) => (N(parse_n!(n, ch)), Event::Noop),
                evnt => (noop, evnt),
            },
            // control commands
            Event::Noop | N(_) => match evnt {
                Char('g', _) if ctrl => (noop, Code(StatusFile)),
                evnt => (prefix, evnt),
            },
            prefix => (prefix, evnt),
        };

        trace!("folded event, {} {}", prefix, evnt);

        self.prefix = prefix;
        Ok(evnt)
    }
}
