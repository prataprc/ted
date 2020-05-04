use crossterm::event::KeyModifiers;

use std::iter::FromIterator;

use crate::{window::Context, Error, Event, Result};

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
        use crate::event::{Event::*, DP::*};

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
                Backspace => (Noop, Mt(Mto::Left(1, Nobound))),
                Enter => (Noop, Mt(MtoDown(1, Caret))),
                Char('h', _) => (Noop, Mt(Mto::Left(1, LineBound))),
                Char(' ', _) => (Noop, Mt(Mto::Right(1, Nobound))),
                Char('l', _) => (Noop, Mt(Mto::Right(1, LineBound))),
                Char('-', _) => (Noop, Mt(Mto::Up(1, Caret))),
                Char('j', _) => (Noop, Mt(Mto::Up(1, Nope))),
                Char('k', _) => (Noop, Mt(Mto::Down(1, Nope))),
                Char('+', _) => (Noop, Mt(Mto::Down(1, Caret))),
                Char('|', _) => (Noop, Mt(Mto::Col(1))),
                Char('G', _) => (Noop, Mt(Mto::Row(1, Caret))),
                Char('%', _) => (Noop, Mt(Mto::Percent(1))),
                Char('0', _) => (Noop, Mt(Mto::Home(Nope))),
                Char('^', _) => (Noop, Mt(Mto::Home(Caret)),
                Char('$', _) => (Noop, Mt(Mto::End)),
                Char('F', _) => (Noop, Mt(Mto::CharF(1, None, Left))),
                Char('f', _) => (Noop, Mt(Mto::CharF(1, None, Right))),
                Char('T', _) => (Noop, Mt(Mto::CharT(1, None, Left))),
                Char('t', _) => (Noop, Mt(Mto::CharT(1, None, Right))),
                Char('b', _) => (Noop, Mt(Mto::Word(1, Left, Start))),
                Char('B', _) => (Noop, Mt(Mto::WWord(1, Left, Start))),
                Char('e', _) => (Noop, Mt(Mto::Word(1, Right, End))),
                Char('E', _) => (Noop, Mt(Mto::WWord(1, Right, End))),
                Char('{', _) => (Noop, Mt(Mto::Para(1, Left))),
                Char('}', _) => (Noop, Mt(Mto::Para(1, Right))),
                Char('(', _) => (Noop, Mt(Mto::Sentence(1, Left))),
                Char(')', _) => (Noop, Mt(Mto::Sentence(1, Right))),
                Char('w', _) => (Noop, Mt(Mto::Word(1, Right, Start))),
                Char('W', _) => (Noop, Mt(Mto::WWord(1, Right, Start))),
                Char(';', _) => (Noop, Mt(Mto::CharR(1, Right))),
                Char(',', _) => (Noop, Mt(Mto::CharR(1, Left))),
                Char('n', _) => (Noop, Mt(Mto::Pattern(1, None, Right))),
                Char('N', _) => (Noop, Mt(Mto::Pattern(1, None, Left))),
                //
                Char('I', _) => (Noop, Md(Mod::Insert(1, Caret))),
                Char('i', _) => (Noop, Md(Mod::Insert(1, Nope))),
                Char('a', _) => (Noop, Md(Mod::Append(1, Right))),
                Char('A', _) => (Noop, Md(Mod::Append(1, End))),
                Char('O', _) => (Noop, Md(Mod::Open(1, Left))),
                Char('o', _) => (Noop, Md(Mod::Open(1, Right))),
                Md(Mod::Insert(n, p)) => (Noop, Md(Mod::Insert(n, p))),
                //
                Char('[', _) => (B(1, Left), Noop),
                Char(']', _) => (B(1, Right), Noop),
                Char('g', _) => (G(1), Noop),
                Char('f', _) => (F(1, Right), Noop),
                Char('F', _) => (F(1, Left), Noop),
                Char('t', _) => (T(1, Right), Noop),
                Char('T', _) => (T(1, Left), Noop),
                Char('[', _) => (B(1, Left), Noop),
                Char(']', _) => (B(1, Right), Noop),
                Char('g', _) => (G(1), Noop),
                Char('f', _) => (F(1, Right), Noop),
                Char('F', _) => (F(1, Left), Noop),
                Char('t', _) => (T(1, Right), Noop),
                Char('T', _) => (T(1, Left), Noop),
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
                Char('g', _) => (Noop, Mt(Mto::Row(n, Caret))),
                Char('e', _) => (Noop, Mt(Mto::Word(n, Left, End))),
                Char('E', _) => (Noop, Mt(Mto::WWord(n, Left, End))),
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
                Backspace => (Noop, Mt(Mto::Left(n, Nobound))),
                Enter => (Noop, Mt(MtoDown(n, Caret))),
                Char('h', _) => (Noop, Mt(Mto::Left(n, LineBound))),
                Char(' ', _) => (Noop, Mt(Mto::Right(n, Nobound))),
                Char('l', _) => (Noop, Mt(Mto::Right(n, LineBound))),
                Char('-', _) => (Noop, Mt(Mto::Up(n, Caret))),
                Char('j', _) => (Noop, Mt(Mto::Up(n, Nope))),
                Char('k', _) => (Noop, Mt(Mto::Down(n, Nope))),
                Char('+', _) => (Noop, Mt(Mto::Down(n, Caret))),
                Char('|', _) => (Noop, Mt(Mto::Col(n))),
                Char('G', _) => (Noop, Mt(Mto::Row(n, Caret))),
                Char('%', _) => (Noop, Mt(Mto::Percent(n))),
                Char('0', _) => (Noop, Mt(Mto::Home(Nope))),
                Char('^', _) => (Noop, Mt(Mto::Home(Caret)),
                Char('$', _) => (Noop, Mt(Mto::End)),
                Char('F', _) => (Noop, Mt(Mto::CharF(n, None, Left))),
                Char('f', _) => (Noop, Mt(Mto::CharF(n, None, Right))),
                Char('T', _) => (Noop, Mt(Mto::CharT(n, None, Left))),
                Char('t', _) => (Noop, Mt(Mto::CharT(n, None, Right))),
                Char('b', _) => (Noop, Mt(Mto::Word(n, Left, Start))),
                Char('B', _) => (Noop, Mt(Mto::WWord(n, Left, Start))),
                Char('e', _) => (Noop, Mt(Mto::Word(n, Right, End))),
                Char('E', _) => (Noop, Mt(Mto::WWord(n, Right, End))),
                Char('{', _) => (Noop, Mt(Mto::Para(n, Left))),
                Char('}', _) => (Noop, Mt(Mto::Para(n, Right))),
                Char('(', _) => (Noop, Mt(Mto::Sentence(n, Left))),
                Char(')', _) => (Noop, Mt(Mto::Sentence(n, Right))),
                Char('w', _) => (Noop, Mt(Mto::Word(n, Right, Start))),
                Char('W', _) => (Noop, Mt(Mto::WWord(n, Right, Start))),
                Char(';', _) => (Noop, Mt(Mto::CharR(n, Right))),
                Char(',', _) => (Noop, Mt(Mto::CharR(n, Left))),
                Char('n', _) => (Noop, Mt(Mto::Pattern(n, None, Right))),
                Char('N', _) => (Noop, Mt(Mto::Pattern(n, None, Left))),
                //
                Char('I', _) => (Noop, Md(Mod::Insert(n, Caret))),
                Char('i', _) => (Noop, Md(Mod::Insert(n, Nope))),
                Char('a', _) => (Noop, Md(Mod::Append(n, Right))),
                Char('A', _) => (Noop, Md(Mod::Append(n, End))),
                Char('O', _) => (Noop, Md(Mod::Open(n, Left))),
                Char('o', _) => (Noop, Md(Mod::Open(n, Right))),
                Md(Mod::Insert(m, p)) => (Noop, Md(Mod::Insert(n * m, p))),
                //
                Char('[', _) => (B(n, Left), Noop),
                Char(']', _) => (B(n, Right), Noop),
                Char('g', _) => (G(n), Noop),
                Char('f', _) => (F(n, Right), Noop),
                Char('F', _) => (F(n, Left), Noop),
                Char('t', _) => (T(n, Right), Noop),
                Char('T', _) => (T(n, Left), Noop),
                Char('[', _) => (B(n, Left), Noop),
                Char(']', _) => (B(n, Right), Noop),
                Char('g', _) => (G(n), Noop),
                Char('f', _) => (F(n, Right), Noop),
                Char('F', _) => (F(n, Left), Noop),
                Char('t', _) => (T(n, Right), Noop),
                Char('T', _) => (T(n, Left), Noop),
                Char(ch @ '0'..='9', _)) => (N((n * 10) + parse_n!(ch)?), Noop),
                _ => (Noop, Noop),
            }
            // control commands
            Noop | N(_) => match evnt {
                Char('g', _) if ctrl => StatusFile,
            }
            prefix => (prefix, evnt),
        };

        Ok((prefix, evnt))
    }
}
