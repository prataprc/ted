use crossterm::event::KeyModifiers;

use std::iter::FromIterator;

use crate::{window::Context, Error, Event, Result};

macro_rules! want_char {
    ($prefix:expr) => {{
        use crate::event::Event::*;

        match $prefix {
            B(_) | MtoCharF(_, _) | MtoCharT(_, _) => true,
            _ => false,
        }
    }};
}

macro_rules! g_prefix {
    ($prefix:expr) => {{
        use crate::event::Event::*;

        match $prefix {
            G(_) => true,
            _ => false,
        }
    }};
}

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
        let ctrl = m == KeyModifiers::CONTROL;
        let wc = want_char!(prefix);
        let gp = g_prefix!(prefix);

        let evnt = match evnt {
            // find char
            Char(ch, _) if wc && m.is_empty() => MtoChar(ch),
            // g-prefix
            Char('g', _) if gp && m.is_empty() => MtoRow(Caret),
            Char('e', _) if gp && m.is_empty() => MtoWord(Left, End),
            Char('E', _) if gp && m.is_empty() => MtoWWord(Left, End),
            Char('o', _) if gp && m.is_empty() => MtoCursor,
            Char('I', _) if gp && m.is_empty() => ModeInsert(Nope),
            // mode commands
            Char('I', _) if m.is_empty() => ModeInsert(Caret),
            Char('i', _) if m.is_empty() => ModeInsert(Nope),
            Char('a', _) if m.is_empty() => ModeAppend(Right),
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
            Char('g', _) if m.is_empty() => G(Box::new(Noop)),
            Char('[', _) if m.is_empty() => B(Left),
            Char(']', _) if m.is_empty() => B(Right),
            //
            Char(ch @ '0'..='9', _) if m.is_empty() => Dec(vec![ch]),
            // control commands
            Char('g', _) if ctrl => StatusFile,
            evnt => evnt,
        };

        let (prefix, evnt) = match (prefix, evnt) {
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
            (prefix, e) => (prefix, e),
        };

        Ok((prefix, evnt))
    }
}
