use crossterm::event::KeyModifiers;

use crate::{
    event::{Event, Mod, Mto, Ted, DP},
    window::Context,
    Error, Result,
};

macro_rules! parse_n {
    ($n:expr, $ch:expr) => {{
        let m = $ch.to_digit(10).unwrap_or(1) as usize;
        ($n * 10) + m
    }};
}

#[derive(Clone, Default)]
pub struct KeyTed;

impl KeyTed {
    pub fn fold(&mut self, c: &Context, evnt: Event) -> Result<(Event, Event)> {
        match c.as_buffer().to_mode() {
            "insert" => self.insert_fold(c, evnt),
            "normal" => self.normal_fold(c, evnt),
            _ => err_at!(Fatal, msg: format!("unreachable")),
        }
    }

    fn insert_fold(&mut self, _: &Context, e: Event) -> Result<(Event, Event)> {
        Ok((Event::Noop, e))
    }

    fn normal_fold(
        //
        &mut self,
        c: &Context,
        evnt: Event,
    ) -> Result<(Event, Event)> {
        use crate::event::Event::{Backspace, Char, Enter};
        use crate::event::Event::{Md, Mt, Td, B, F, G, N, T};
        let eno = Event::Noop;

        let prefix = c.to_event_prefix();
        let (empty, ctrl) = {
            let m = evnt.to_modifiers();
            (m.is_empty(), m == KeyModifiers::CONTROL)
        };

        let (prefix, evnt) = match prefix {
            Event::Noop if empty => match evnt {
                Backspace => (eno, Mt(Mto::Left(1, DP::Nobound))),
                Enter => (eno, Mt(Mto::Down(1, DP::Caret))),
                Char('h', _) => (eno, Mt(Mto::Left(1, DP::LineBound))),
                Char(' ', _) => (eno, Mt(Mto::Right(1, DP::Nobound))),
                Char('l', _) => (eno, Mt(Mto::Right(1, DP::LineBound))),
                Char('-', _) => (eno, Mt(Mto::Up(1, DP::Caret))),
                Char('j', _) => (eno, Mt(Mto::Up(1, DP::Nope))),
                Char('k', _) => (eno, Mt(Mto::Down(1, DP::Nope))),
                Char('+', _) => (eno, Mt(Mto::Down(1, DP::Caret))),
                Char('|', _) => (eno, Mt(Mto::Col(1))),
                Char('G', _) => (eno, Mt(Mto::Row(1, DP::Caret))),
                Char('%', _) => (eno, Mt(Mto::Percent(1))),
                Char('0', _) => (eno, Mt(Mto::Home(DP::Nope))),
                Char('^', _) => (eno, Mt(Mto::Home(DP::Caret))),
                Char('$', _) => (eno, Mt(Mto::End)),
                Char('b', _) => (eno, Mt(Mto::Word(1, DP::Left, DP::Start))),
                Char('B', _) => (eno, Mt(Mto::WWord(1, DP::Left, DP::Start))),
                Char('e', _) => (eno, Mt(Mto::Word(1, DP::Right, DP::End))),
                Char('E', _) => (eno, Mt(Mto::WWord(1, DP::Right, DP::End))),
                Char('{', _) => (eno, Mt(Mto::Para(1, DP::Left))),
                Char('}', _) => (eno, Mt(Mto::Para(1, DP::Right))),
                Char('(', _) => (eno, Mt(Mto::Sentence(1, DP::Left))),
                Char(')', _) => (eno, Mt(Mto::Sentence(1, DP::Right))),
                Char('w', _) => (eno, Mt(Mto::Word(1, DP::Right, DP::Start))),
                Char('W', _) => (eno, Mt(Mto::WWord(1, DP::Right, DP::Start))),
                Char(';', _) => (eno, Mt(Mto::CharR(1, DP::Right))),
                Char(',', _) => (eno, Mt(Mto::CharR(1, DP::Left))),
                Char('n', _) => (eno, Mt(Mto::PatternR(1, DP::Right))),
                Char('N', _) => (eno, Mt(Mto::PatternR(1, DP::Left))),
                //
                Char('I', _) => (eno, Md(Mod::Insert(1, DP::Caret))),
                Char('i', _) => (eno, Md(Mod::Insert(1, DP::Nope))),
                Char('a', _) => (eno, Md(Mod::Append(1, DP::Right))),
                Char('A', _) => (eno, Md(Mod::Append(1, DP::End))),
                Char('O', _) => (eno, Md(Mod::Open(1, DP::Left))),
                Char('o', _) => (eno, Md(Mod::Open(1, DP::Right))),
                Md(Mod::Insert(n, p)) => (eno, Md(Mod::Insert(n, p))),
                //
                Char('[', _) => (B(1, DP::Left), Event::Noop),
                Char(']', _) => (B(1, DP::Right), Event::Noop),
                Char('g', _) => (G(1), Event::Noop),
                Char('f', _) => (F(1, DP::Right), Event::Noop),
                Char('F', _) => (F(1, DP::Left), Event::Noop),
                Char('t', _) => (T(1, DP::Right), Event::Noop),
                Char('T', _) => (T(1, DP::Left), Event::Noop),
                Char(ch @ '0'..='9', _) => (N(parse_n!(1, ch)), Event::Noop),
                _ => (eno, Event::Noop),
            },
            B(n, d) if empty => match evnt {
                Char('(', _) => (eno, Mt(Mto::Bracket(n, '(', ')', d))),
                Char(')', _) => (eno, Mt(Mto::Bracket(n, ')', '(', d))),
                Char('{', _) => (eno, Mt(Mto::Bracket(n, '{', '}', d))),
                Char('}', _) => (eno, Mt(Mto::Bracket(n, '}', '{', d))),
                _ => (eno, Event::Noop),
            },
            G(n) if empty => match evnt {
                Char('g', _) if ctrl => {
                    let spanline = Default::default();
                    (eno, Td(Ted::StatusCursor { spanline }))
                }
                Char('g', _) => (eno, Mt(Mto::Row(n, DP::Caret))),
                Char('e', _) => (eno, Mt(Mto::Word(n, DP::Left, DP::End))),
                Char('E', _) => (eno, Mt(Mto::WWord(n, DP::Left, DP::End))),
                Char('o', _) => (eno, Mt(Mto::Cursor(n))),
                Char('I', _) => (eno, Md(Mod::Insert(n, DP::Caret))),
                _ => (eno, Event::Noop),
            },
            F(n, d) if empty => match evnt {
                Char(ch, _) => (eno, Mt(Mto::CharF(n, Some(ch), d))),
                _ => (eno, Event::Noop),
            },
            T(n, d) if empty => match evnt {
                Char(ch, _) => (eno, Mt(Mto::CharT(n, Some(ch), d))),
                _ => (eno, Event::Noop),
            },
            N(n) if empty => match evnt {
                Backspace => (eno, Mt(Mto::Left(n, DP::Nobound))),
                Enter => (eno, Mt(Mto::Down(n, DP::Caret))),
                Char('h', _) => (eno, Mt(Mto::Left(n, DP::LineBound))),
                Char(' ', _) => (eno, Mt(Mto::Right(n, DP::Nobound))),
                Char('l', _) => (eno, Mt(Mto::Right(n, DP::LineBound))),
                Char('-', _) => (eno, Mt(Mto::Up(n, DP::Caret))),
                Char('j', _) => (eno, Mt(Mto::Up(n, DP::Nope))),
                Char('k', _) => (eno, Mt(Mto::Down(n, DP::Nope))),
                Char('+', _) => (eno, Mt(Mto::Down(n, DP::Caret))),
                Char('|', _) => (eno, Mt(Mto::Col(n))),
                Char('G', _) => (eno, Mt(Mto::Row(n, DP::Caret))),
                Char('%', _) => (eno, Mt(Mto::Percent(n))),
                Char('0', _) => (eno, Mt(Mto::Home(DP::Nope))),
                Char('^', _) => (eno, Mt(Mto::Home(DP::Caret))),
                Char('$', _) => (eno, Mt(Mto::End)),
                Char('b', _) => (eno, Mt(Mto::Word(n, DP::Left, DP::Start))),
                Char('B', _) => (eno, Mt(Mto::WWord(n, DP::Left, DP::Start))),
                Char('e', _) => (eno, Mt(Mto::Word(n, DP::Right, DP::End))),
                Char('E', _) => (eno, Mt(Mto::WWord(n, DP::Right, DP::End))),
                Char('{', _) => (eno, Mt(Mto::Para(n, DP::Left))),
                Char('}', _) => (eno, Mt(Mto::Para(n, DP::Right))),
                Char('(', _) => (eno, Mt(Mto::Sentence(n, DP::Left))),
                Char(')', _) => (eno, Mt(Mto::Sentence(n, DP::Right))),
                Char('w', _) => (eno, Mt(Mto::Word(n, DP::Right, DP::Start))),
                Char('W', _) => (eno, Mt(Mto::WWord(n, DP::Right, DP::Start))),
                Char(';', _) => (eno, Mt(Mto::CharR(n, DP::Right))),
                Char(',', _) => (eno, Mt(Mto::CharR(n, DP::Left))),
                Char('n', _) => (eno, Mt(Mto::PatternR(n, DP::Right))),
                Char('N', _) => (eno, Mt(Mto::PatternR(n, DP::Left))),
                //
                Char('I', _) => (eno, Md(Mod::Insert(n, DP::Caret))),
                Char('i', _) => (eno, Md(Mod::Insert(n, DP::Nope))),
                Char('a', _) => (eno, Md(Mod::Append(n, DP::Right))),
                Char('A', _) => (eno, Md(Mod::Append(n, DP::End))),
                Char('O', _) => (eno, Md(Mod::Open(n, DP::Left))),
                Char('o', _) => (eno, Md(Mod::Open(n, DP::Right))),
                Md(Mod::Insert(m, p)) => (eno, Md(Mod::Insert(n * m, p))),
                //
                Char('[', _) => (B(n, DP::Left), Event::Noop),
                Char(']', _) => (B(n, DP::Right), Event::Noop),
                Char('g', _) => (G(n), Event::Noop),
                Char('f', _) => (F(n, DP::Right), Event::Noop),
                Char('F', _) => (F(n, DP::Left), Event::Noop),
                Char('t', _) => (T(n, DP::Right), Event::Noop),
                Char('T', _) => (T(n, DP::Left), Event::Noop),
                Char(ch @ '0'..='9', _) => (N(parse_n!(n, ch)), Event::Noop),
                _ => (eno, Event::Noop),
            },
            // control commands
            Event::Noop | N(_) => match evnt {
                Char('g', _) if ctrl => (eno, Td(Ted::StatusFile)),
                evnt => (prefix, evnt),
            },
            prefix => (prefix, evnt),
        };

        Ok((prefix, evnt))
    }
}
