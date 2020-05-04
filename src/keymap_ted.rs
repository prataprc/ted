use crossterm::event::KeyModifiers;

use std::iter::FromIterator;

use crate::{
    event::{Event, Mod, Mto, Ted, DP},
    window::Context,
    Error, Result,
};

macro_rules! parse_n {
    ($ns:expr) => {
        err_at!(
            FailConvert,
            String::from_iter($ns.drain(..)).parse::<usize>()
        )
    };
}

#[derive(Clone, Default)]
pub struct Ted;

impl Ted {
    pub fn fold(&mut self, c: &Context, evnt: Event) -> Result<(Event, Event)> {
        match c.as_buffer().to_mode() {
            "insert" => self.insert_fold(c, evnt),
            "normal" => self.normal_fold(c, evnt),
            _ => err_at!(Fatal, msg: format!("unreachable")),
        }
    }

    fn insert_fold(
        //
        &mut self,
        _: &Context,
        evnt: Event,
    ) -> Result<(Event, Event)> {
        Ok((Event::Noop, evnt))
    }

    fn normal_fold(
        //
        &mut self,
        c: &Context,
        evnt: Event,
    ) -> Result<(Event, Event)> {
        use crate::event::Event::*;

        let prefix = c.to_event_prefix();
        let (fc, pn) = {
            let b = c.as_buffer();
            (b.evnt_mto_char.clone(), b.evnt_mto_patt.clone())
        };

        let m = evnt.to_modifiers();
        let empty = m.is_empty();
        let ctrl = m == KeyModifiers::CONTROL;

        let evnt = match prefix {
            Noop if empty => match evnt {
                Backspace => (Noop, Mt(Mto::Left(1, DP::Nobound))),
                Enter => (Noop, Mt(Mto::Down(1, DP::Caret))),
                Char('h', _) => (Noop, Mt(Mto::Left(1, DP::LineBound))),
                Char(' ', _) => (Noop, Mt(Mto::Right(1, DP::Nobound))),
                Char('l', _) => (Noop, Mt(Mto::Right(1, DP::LineBound))),
                Char('-', _) => (Noop, Mt(Mto::Up(1, DP::Caret))),
                Char('j', _) => (Noop, Mt(Mto::Up(1, DP::Nope))),
                Char('k', _) => (Noop, Mt(Mto::Down(1, DP::Nope))),
                Char('+', _) => (Noop, Mt(Mto::Down(1, DP::Caret))),
                Char('|', _) => (Noop, Mt(Mto::Col(1))),
                Char('G', _) => (Noop, Mt(Mto::Row(1, DP::Caret))),
                Char('%', _) => (Noop, Mt(Mto::Percent(1))),
                Char('0', _) => (Noop, Mt(Mto::Home(DP::Nope))),
                Char('^', _) => (Noop, Mt(Mto::Home(DP::Caret))),
                Char('$', _) => (Noop, Mt(Mto::End)),
                Char('F', _) => (Noop, Mt(Mto::CharF(1, None, DP::Left))),
                Char('f', _) => (Noop, Mt(Mto::CharF(1, None, DP::Right))),
                Char('T', _) => (Noop, Mt(Mto::CharT(1, None, DP::Left))),
                Char('t', _) => (Noop, Mt(Mto::CharT(1, None, DP::Right))),
                Char('b', _) => (Noop, Mt(Mto::Word(1, DP::Left, DP::Start))),
                Char('B', _) => (Noop, Mt(Mto::WWord(1, DP::Left, DP::Start))),
                Char('e', _) => (Noop, Mt(Mto::Word(1, DP::Right, DP::End))),
                Char('E', _) => (Noop, Mt(Mto::WWord(1, DP::Right, DP::End))),
                Char('{', _) => (Noop, Mt(Mto::Para(1, DP::Left))),
                Char('}', _) => (Noop, Mt(Mto::Para(1, DP::Right))),
                Char('(', _) => (Noop, Mt(Mto::Sentence(1, DP::Left))),
                Char(')', _) => (Noop, Mt(Mto::Sentence(1, DP::Right))),
                Char('w', _) => (Noop, Mt(Mto::Word(1, DP::Right, DP::Start))),
                Char('W', _) => (Noop, Mt(Mto::WWord(1, DP::Right, DP::Start))),
                Char(';', _) => (Noop, Mt(Mto::CharR(1, DP::Right))),
                Char(',', _) => (Noop, Mt(Mto::CharR(1, DP::Left))),
                Char('n', _) => (Noop, Mt(Mto::Pattern(1, None, DP::Right))),
                Char('N', _) => (Noop, Mt(Mto::Pattern(1, None, DP::Left))),
                //
                Char('I', _) => (Noop, Md(Mod::Insert(1, DP::Caret))),
                Char('i', _) => (Noop, Md(Mod::Insert(1, DP::Nope))),
                Char('a', _) => (Noop, Md(Mod::Append(1, DP::Right))),
                Char('A', _) => (Noop, Md(Mod::Append(1, DP::End))),
                Char('O', _) => (Noop, Md(Mod::Open(1, DP::Left))),
                Char('o', _) => (Noop, Md(Mod::Open(1, DP::Right))),
                Md(Mod::Insert(n, p)) => (Noop, Md(Mod::Insert(n, p))),
                //
                Char('[', _) => (B(1, DP::Left), Noop),
                Char(']', _) => (B(1, DP::Right), Noop),
                Char('g', _) => (G(1), Noop),
                Char('f', _) => (F(1, DP::Right), Noop),
                Char('F', _) => (F(1, DP::Left), Noop),
                Char('t', _) => (T(1, DP::Right), Noop),
                Char('T', _) => (T(1, DP::Left), Noop),
                Char('[', _) => (B(1, DP::Left), Noop),
                Char(']', _) => (B(1, DP::Right), Noop),
                Char('g', _) => (G(1), Noop),
                Char('f', _) => (F(1, DP::Right), Noop),
                Char('F', _) => (F(1, DP::Left), Noop),
                Char('t', _) => (T(1, DP::Right), Noop),
                Char('T', _) => (T(1, DP::Left), Noop),
                Char(ch @ '0'..='9', _) => (N(parse_n!(ch)?), Noop),
                _ => (Noop, Noop),
            },
            B(n, d) if empty => match evnt {
                Char('(', _) => (Noop, Mt(Mto::Bracket(n, '(', ')', d))),
                Char(')', _) => (Noop, Mt(Mto::Bracket(n, ')', '(', d))),
                Char('{', _) => (Noop, Mt(Mto::Bracket(n, '{', '}', d))),
                Char('}', _) => (Noop, Mt(Mto::Bracket(n, '}', '{', d))),
                _ => (Noop, Noop),
            },
            G(n) if empty => match evnt {
                Char('g', _) => (Noop, Mt(Mto::Row(n, DP::Caret))),
                Char('e', _) => (Noop, Mt(Mto::Word(n, DP::Left, DP::End))),
                Char('E', _) => (Noop, Mt(Mto::WWord(n, DP::Left, DP::End))),
                Char('o', _) => (Noop, Mt(Mto::Cursor(n))),
                Char('I', _) => (Noop, Mt(Mod::Insert(n))),
                _ => (Noop, Noop),
            },
            F(n, d) if empty => match evnt {
                Char(ch, _) => (Noop, Mt(Mto::CharF(n, Some(ch), d))),
                _ => (Noop, Noop),
            },
            T(n, d) if empty => match evnt {
                Char(ch, _) => (Noop, Mt(Mto::CharT(n, Some(ch), d))),
                _ => (Noop, Noop),
            },
            N(n) if empty => match evnt {
                Backspace => (Noop, Mt(Mto::Left(n, DP::Nobound))),
                Enter => (Noop, Mt(Mto::Down(n, DP::Caret))),
                Char('h', _) => (Noop, Mt(Mto::Left(n, DP::LineBound))),
                Char(' ', _) => (Noop, Mt(Mto::Right(n, DP::Nobound))),
                Char('l', _) => (Noop, Mt(Mto::Right(n, DP::LineBound))),
                Char('-', _) => (Noop, Mt(Mto::Up(n, DP::Caret))),
                Char('j', _) => (Noop, Mt(Mto::Up(n, DP::Nope))),
                Char('k', _) => (Noop, Mt(Mto::Down(n, DP::Nope))),
                Char('+', _) => (Noop, Mt(Mto::Down(n, DP::Caret))),
                Char('|', _) => (Noop, Mt(Mto::Col(n))),
                Char('G', _) => (Noop, Mt(Mto::Row(n, DP::Caret))),
                Char('%', _) => (Noop, Mt(Mto::Percent(n))),
                Char('0', _) => (Noop, Mt(Mto::Home(DP::Nope))),
                Char('^', _) => (Noop, Mt(Mto::Home(DP::Caret))),
                Char('$', _) => (Noop, Mt(Mto::End)),
                Char('F', _) => (Noop, Mt(Mto::CharF(n, None, DP::Left))),
                Char('f', _) => (Noop, Mt(Mto::CharF(n, None, DP::Right))),
                Char('T', _) => (Noop, Mt(Mto::CharT(n, None, DP::Left))),
                Char('t', _) => (Noop, Mt(Mto::CharT(n, None, DP::Right))),
                Char('b', _) => (Noop, Mt(Mto::Word(n, DP::Left, DP::Start))),
                Char('B', _) => (Noop, Mt(Mto::WWord(n, DP::Left, DP::Start))),
                Char('e', _) => (Noop, Mt(Mto::Word(n, DP::Right, DP::End))),
                Char('E', _) => (Noop, Mt(Mto::WWord(n, DP::Right, DP::End))),
                Char('{', _) => (Noop, Mt(Mto::Para(n, DP::Left))),
                Char('}', _) => (Noop, Mt(Mto::Para(n, DP::Right))),
                Char('(', _) => (Noop, Mt(Mto::Sentence(n, DP::Left))),
                Char(')', _) => (Noop, Mt(Mto::Sentence(n, DP::Right))),
                Char('w', _) => (Noop, Mt(Mto::Word(n, DP::Right, DP::Start))),
                Char('W', _) => (Noop, Mt(Mto::WWord(n, DP::Right, DP::Start))),
                Char(';', _) => (Noop, Mt(Mto::CharR(n, DP::Right))),
                Char(',', _) => (Noop, Mt(Mto::CharR(n, DP::Left))),
                Char('n', _) => (Noop, Mt(Mto::Pattern(n, None, DP::Right))),
                Char('N', _) => (Noop, Mt(Mto::Pattern(n, None, DP::Left))),
                //
                Char('I', _) => (Noop, Md(Mod::Insert(n, DP::Caret))),
                Char('i', _) => (Noop, Md(Mod::Insert(n, DP::Nope))),
                Char('a', _) => (Noop, Md(Mod::Append(n, DP::Right))),
                Char('A', _) => (Noop, Md(Mod::Append(n, DP::End))),
                Char('O', _) => (Noop, Md(Mod::Open(n, DP::Left))),
                Char('o', _) => (Noop, Md(Mod::Open(n, DP::Right))),
                Md(Mod::Insert(m, p)) => (Noop, Md(Mod::Insert(n * m, p))),
                //
                Char('[', _) => (B(n, DP::Left), Noop),
                Char(']', _) => (B(n, DP::Right), Noop),
                Char('g', _) => (G(n), Noop),
                Char('f', _) => (F(n, DP::Right), Noop),
                Char('F', _) => (F(n, DP::Left), Noop),
                Char('t', _) => (T(n, DP::Right), Noop),
                Char('T', _) => (T(n, DP::Left), Noop),
                Char('[', _) => (B(n, DP::Left), Noop),
                Char(']', _) => (B(n, DP::Right), Noop),
                Char('g', _) => (G(n), Noop),
                Char('f', _) => (F(n, DP::Right), Noop),
                Char('F', _) => (F(n, DP::Left), Noop),
                Char('t', _) => (T(n, DP::Right), Noop),
                Char('T', _) => (T(n, DP::Left), Noop),
                Char(ch @ '0'..='9', _) => (N((n * 10) + parse_n!(ch)?), Noop),
                _ => (Noop, Noop),
            },
            // control commands
            Noop | N(_) => match evnt {
                Char('g', _) if ctrl => (Noop, Td(Ted::StatusFile)),
                evnt => (prefix, event),
            },
            prefix => (prefix, evnt),
        };

        Ok((prefix, evnt))
    }
}
