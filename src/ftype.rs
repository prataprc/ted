use std::cmp;

use crate::{
    buffer::NL,
    event::{Event, DP},
    search::Search,
    window::Context,
    Error, Result,
};

#[derive(Clone)]
pub struct FType {
    p: FT,
    fallback: FT,
}

impl Default for FType {
    fn default() -> FType {
        FType {
            p: Default::default(),
            fallback: Default::default(),
        }
    }
}

impl FType {
    pub fn new(p: FT, fallback: FT) -> FType {
        FType { p, fallback }
    }

    pub fn on_event(&mut self, c: &mut Context, evnt: Event) -> Result<Event> {
        match self.p.on_event(c, evnt)? {
            Event::Noop => Ok(Event::Noop),
            evnt => self.fallback.on_event(c, evnt),
        }
    }
}

#[derive(Clone)]
pub enum FT {
    Text(Text),
}

impl Default for FT {
    fn default() -> FT {
        FT::Text(Default::default())
    }
}

impl FT {
    fn on_event(&mut self, c: &mut Context, evnt: Event) -> Result<Event> {
        match self {
            FT::Text(t) => t.on_event(c, evnt),
        }
    }
}

#[derive(Clone)]
pub struct Text;

impl Default for Text {
    fn default() -> Text {
        Text
    }
}

impl Text {
    pub fn on_event(&mut self, c: &mut Context, evnt: Event) -> Result<Event> {
        match c.as_buffer().to_mode() {
            "insert" => self.on_i_event(c, evnt),
            "normal" => self.on_n_event(c, evnt),
            _ => err_at!(Fatal, msg: format!("unreachable")),
        }
    }

    fn on_n_event(&mut self, c: &mut Context, evnt: Event) -> Result<Event> {
        use crate::event::{Event::*, DP::*};

        // switch to insert mode.
        //let evnt = match evnt {
        //    evnt @ N(n, _) if n > 1 && evnt.is_insert() => {
        //        let b = c.as_mut_buffer();
        //        b.mode_insert()?;
        //        return b.on_event(c, evnt);
        //    }
        //    evnt => evnt,
        //};

        //let evnt = match evnt {
        //    Noop => Noop,
        //    // execute motion command.
        //    N(n, box MtoLeft(dp)) => self.mto_left(n, dp)?,
        //    N(n, box MtoRight(dp)) => self.mto_right(n, dp)?,
        //    N(n, box MtoUp(dp)) => self.mto_up(n, dp)?,
        //    N(n, box MtoDown(dp)) => self.mto_down(n, dp)?,
        //    N(n, box MtoCol) => self.mto_column(n)?,
        //    N(n, box MtoRow(dp)) => self.mto_row(n, dp)?,
        //    N(n, box MtoPercent) => self.mto_percent(n)?,
        //    N(_, box MtoHome(dp)) => self.mto_home(dp)?,
        //    N(_, box MtoEnd) => self.mto_end()?, // TODO: make this sticky.
        //    N(n, box MtoCursor) => self.mto_cursor(n)?,
        //    N(n, e @ box MtoCharF(_, _)) => self.mto_char(n, *e)?,
        //    N(n, e @ box MtoCharT(_, _)) => self.mto_char(n, *e)?,
        //    N(n, e @ box MtoWord(_, _)) => self.mto_words(n, *e)?,
        //    N(n, e @ box MtoWWord(_, _)) => self.mto_wwords(n, *e)?,
        //    N(n, e @ box MtoSentence(_)) => self.mto_sentence(n, *e)?,
        //    N(n, e @ box MtoPara(_)) => self.mto_para(n, *e)?,
        //    N(n, e @ box MtoBracket(_, _, _)) => self.mto_bracket(n, *e)?,
        //    N(n, e @ box MtoPattern(Some(_), _)) => self.mto_pattern(n, *e)?,
        //    // execute mode switching commands
        //    N(n, box ModeInsert(Caret)) => {
        //        self.mto_home(Caret)?;
        //        N(n, Box::new(ModeInsert(Caret)))
        //    }
        //    N(n, e @ box ModeInsert(_)) => N(n, Box::new(*e)),
        //    //Char('%', _) if m.is_empty() => {
        //    //    self.to_mut_change().fwd_match_group();
        //    //    Ok(Noop)
        //    //}
        //    evnt => evnt,
        //};

        Ok(evnt)
    }

    fn on_i_event(&mut self, c: &mut Context, evnt: Event) -> Result<Event> {
        todo!()
    }
}

impl Text {
    fn mto_left(&mut self, c: &mut Context, n: usize, dp: DP) -> Result<Event> {
        use crate::event::DP::*;

        let mut cursor = c.as_buffer().to_cursor();
        cursor = match dp {
            LineBound => {
                let home = c.as_buffer().line_home();
                let new_cursor = cursor.saturating_sub(n);
                Ok(if_else!(new_cursor > home, new_cursor, home))
            }
            Nobound => Ok(cursor.saturating_sub(n)),
            _ => err_at!(Fatal, msg: format!("unreachable")),
        }?;

        c.as_mut_buffer().set_cursor(cursor);
        Ok(Event::Noop)
    }

    fn mto_right(&mut self, c: &mut Context, n: usize, dp: DP) -> Result<Event> {
        let b = c.as_mut_buffer();
        let mut cursor = b.to_cursor();
        for ch in b.chars_at(cursor, DP::Right)?.take(n) {
            match dp {
                DP::LineBound if ch == NL => break,
                DP::Nobound => (),
                _ => err_at!(Fatal, msg: format!("unreachable"))?,
            }
            cursor += 1
        }

        b.set_cursor(cursor);
        Ok(Event::Noop)
    }

    fn mto_home(&mut self, c: &mut Context, pos: DP) -> Result<Event> {
        use crate::event::DP::*;

        let b = c.as_mut_buffer();
        b.set_cursor(b.line_home());
        match pos {
            Caret => {
                b.skip_whitespace(Right);
            }
            Nope => (),
            _ => err_at!(Fatal, msg: format!("unreachable"))?,
        }
        Ok(Event::Noop)
    }

    fn mto_up(&mut self, c: &mut Context, n: usize, pos: DP) -> Result<Event> {
        use crate::event::DP::*;

        let b = c.as_mut_buffer();
        let mut cursor = b.to_cursor();
        match b.char_to_line(cursor) {
            0 => Ok(Event::Noop),
            row => {
                let row = row.saturating_sub(n);
                cursor = {
                    let col = {
                        let n_chars = b.len_line(row);
                        cmp::min(n_chars.saturating_sub(2), b.to_col())
                    };
                    b.line_to_char(row) + col
                };
                b.set_cursor(cursor);
                match pos {
                    Caret => self.mto_home(c, Caret),
                    Nope => Ok(Event::Noop),
                    _ => {
                        err_at!(Fatal, msg: format!("unreachable"))?;
                        Ok(Event::Noop)
                    }
                }
            }
        }
    }

    fn mto_down(&mut self, c: &mut Context, n: usize, pos: DP) -> Result<Event> {
        use crate::event::DP::*;

        let b = c.as_mut_buffer();
        let row = b.char_to_line(b.to_cursor());
        match b.len_lines() {
            0 => Ok(Event::Noop),
            n_rows if row == n_rows => Ok(Event::Noop),
            n_rows => {
                let row = limite!(row.saturating_add(n), n_rows);
                let cursor = {
                    let n_chars = b.len_line(row);
                    let col = cmp::min(n_chars.saturating_sub(2), b.to_col());
                    b.line_to_char(row) + col
                };
                b.set_cursor(cursor);
                match pos {
                    Caret => self.mto_home(c, Caret),
                    Nope => Ok(Event::Noop),
                    _ => {
                        err_at!(Fatal, msg: format!("unreachable"))?;
                        Ok(Event::Noop)
                    }
                }
            }
        }
    }

    fn mto_column(&mut self, c: &mut Context, n: usize) -> Result<Event> {
        let b = c.as_mut_buffer();
        let n = {
            let m = b.len_line(b.char_to_line(b.to_cursor())).saturating_sub(1);
            cmp::min(m, n).saturating_sub(1)
        };
        b.set_cursor(b.line_home() + n);
        Ok(Event::Noop)
    }

    fn mto_row(&mut self, c: &mut Context, n: usize, pos: DP) -> Result<Event> {
        let b = c.as_buffer();
        let row = b.char_to_line(b.to_cursor());
        let n = n.saturating_sub(1);
        match b.len_lines() {
            0 => Ok(Event::Noop),
            n_rows if n == 0 => self.mto_down(c, n_rows.saturating_sub(1), pos),
            _ if n < row => self.mto_up(c, row - n, pos),
            n_rows if n <= n_rows => self.mto_down(c, n - row, pos),
            n_rows => self.mto_down(c, n_rows.saturating_sub(1), pos),
        }
    }

    fn mto_percent(&mut self, c: &mut Context, n: usize) -> Result<Event> {
        use crate::event::DP::*;

        let b = c.as_buffer();
        let row = b.char_to_line(b.to_cursor());
        match b.len_lines() {
            0 => Ok(Event::Noop),
            mut n_rows if n < 100 => {
                n_rows = n_rows.saturating_sub(1);
                match (((n_rows as f64) * (n as f64)) / (100 as f64)) as usize {
                    n if n < row => self.mto_up(c, row - n, Caret),
                    n => self.mto_down(c, n - row, Caret),
                }
            }
            n_rows => self.mto_down(c, n_rows.saturating_sub(1), Caret),
        }
    }

    fn mto_cursor(&mut self, c: &mut Context, n: usize) -> Result<Event> {
        let b = c.as_mut_buffer();
        let cursor = b.to_cursor();
        b.set_cursor(limite!(cursor + n, b.len_chars()));
        Ok(Event::Noop)
    }

    fn mto_end(&mut self, c: &mut Context) -> Result<Event> {
        let b = c.as_mut_buffer();
        let mut cursor = b.to_cursor();
        {
            let mut iter = b.chars_at(b.to_cursor(), DP::Right)?;
            loop {
                match iter.next() {
                    Some(NL) => break (),
                    Some(_) => cursor += 1,
                    None => break (),
                }
            }
        }
        b.set_cursor(cursor);
        Ok(Event::Noop)
    }

    fn mto_char(
        //
        &mut self,
        c: &mut Context,
        mut n: usize,
        evnt: Event,
    ) -> Result<Event> {
        use crate::event::DP::*;

        let (ch, dp, pos) = match evnt {
            Event::MtoCharF(Some(ch), dp) => (ch, dp, Find),
            Event::MtoCharT(Some(ch), dp) => (ch, dp, Till),
            _ => unreachable!(),
        };

        let b = c.as_mut_buffer();
        let mut cursor = b.to_cursor();
        let home = b.line_home();
        cursor = match dp {
            Right => {
                let mut iter = b.chars_at(cursor, Right)?.enumerate();
                loop {
                    match iter.next() {
                        Some((_, NL)) => break cursor,
                        Some((i, c)) if c == ch && n == 0 && pos == Find => {
                            break cursor.saturating_add(i);
                        }
                        Some((i, c)) if c == ch && n == 0 => {
                            break cursor.saturating_add(i.saturating_sub(1));
                        }
                        Some((_, c)) if c == ch => n -= 1,
                        _ => (),
                    }
                }
            }
            Left => {
                let mut iter = b.chars_at(cursor, Left)?.enumerate();
                loop {
                    match iter.next() {
                        Some((_, NL)) => break cursor,
                        Some((i, c)) if c == ch && n == 0 && pos == Find => {
                            break cursor.saturating_sub(i + 1);
                        }
                        Some((i, c)) if c == ch && n == 0 => {
                            break cursor.saturating_sub(i);
                        }
                        Some((_, c)) if c == ch => n -= 1,
                        _ => (),
                    }
                }
            }
            _ => unreachable!(),
        };

        b.set_cursor(if_else!(cursor > home, cursor, home));
        Ok(Event::Noop)
    }

    fn mto_words(
        //
        &mut self,
        c: &mut Context,
        n: usize,
        evnt: Event,
    ) -> Result<Event> {
        use crate::event::{Event::*, DP::*};

        match evnt {
            MtoWord(Left, pos) => {
                for _ in 0..n {
                    let n = c.as_mut_buffer().skip_whitespace(Left);
                    match pos {
                        End if n == 0 => {
                            c.as_mut_buffer().skip_alphanumeric(Left);
                            self.mto_right(c, 1, Nobound)?;
                        }
                        End => {
                            c.as_mut_buffer().skip_alphanumeric(Left);
                            self.mto_right(c, 1, Nobound)?;
                        }
                        Start if n == 0 => {
                            c.as_mut_buffer().skip_alphanumeric(Left);
                            c.as_mut_buffer().skip_whitespace(Left);
                        }
                        Start => (),
                        _ => unreachable!(),
                    }
                }
                Ok(Event::Noop)
            }
            MtoWord(Right, pos) => {
                for _ in 0..n {
                    let n = c.as_mut_buffer().skip_whitespace(Right);
                    match pos {
                        End if n == 0 => {
                            c.as_mut_buffer().skip_alphanumeric(Right);
                            self.mto_left(c, 1, Nobound)?;
                        }
                        End => {
                            c.as_mut_buffer().skip_alphanumeric(Right);
                            self.mto_left(c, 1, Nobound)?;
                        }
                        Start if n == 0 => {
                            c.as_mut_buffer().skip_alphanumeric(Right);
                            c.as_mut_buffer().skip_whitespace(Right);
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

    fn mto_wwords(
        //
        &mut self,
        c: &mut Context,
        n: usize,
        evnt: Event,
    ) -> Result<Event> {
        use crate::event::{Event::*, DP::*};

        match evnt {
            MtoWWord(Left, pos) => {
                for _ in 0..n {
                    let n = c.as_mut_buffer().skip_whitespace(Left);
                    match pos {
                        Start if n == 0 => {
                            c.as_mut_buffer().skip_non_whitespace(Left);
                            self.mto_right(c, 1, Nobound)?;
                        }
                        Start => {
                            c.as_mut_buffer().skip_non_whitespace(Left);
                            self.mto_right(c, 1, Nobound)?;
                        }
                        End if n == 0 => {
                            c.as_mut_buffer().skip_non_whitespace(Left);
                            c.as_mut_buffer().skip_whitespace(Left);
                        }
                        End => (),
                        _ => unreachable!(),
                    }
                }
                Ok(Event::Noop)
            }
            MtoWWord(Right, pos) => {
                for _ in 0..n {
                    let n = c.as_mut_buffer().skip_whitespace(Right);
                    match pos {
                        End if n == 0 => {
                            c.as_mut_buffer().skip_non_whitespace(Right);
                            self.mto_left(c, 1, Nobound)?;
                        }
                        End => {
                            c.as_mut_buffer().skip_non_whitespace(Right);
                            self.mto_left(c, 1, Nobound)?;
                        }
                        Start if n == 0 => {
                            c.as_mut_buffer().skip_non_whitespace(Right);
                            c.as_mut_buffer().skip_whitespace(Right);
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

    fn mto_sentence(
        //
        &mut self,
        c: &mut Context,
        mut n: usize,
        e: Event,
    ) -> Result<Event> {
        use crate::event::{Event::*, DP::*};

        let is_ws = |ch: char| ch.is_whitespace();

        let b = c.as_mut_buffer();
        let mut cursor = b.to_cursor();
        let mut pch: Option<char> = None;
        cursor = match e {
            MtoSentence(Left) => {
                let mut iter = b.chars_at(cursor, Left)?.enumerate();
                Ok(loop {
                    pch = match (iter.next(), pch) {
                        (Some((i, '.')), Some(pch)) if is_ws(pch) => {
                            if n > 1 {
                                n -= 1;
                            } else {
                                break cursor.saturating_sub(i);
                            }
                            Some('.')
                        }
                        (Some((i, NL)), Some(NL)) => {
                            if n > 1 {
                                n -= 1;
                            } else {
                                break cursor.saturating_sub(i);
                            }
                            Some(NL)
                        }
                        (Some((_, ch)), _) => Some(ch),
                        (None, _) => break 0,
                    };
                })
            }
            MtoSentence(Right) => {
                let mut iter = b.chars_at(cursor, Right)?.enumerate();
                Ok(loop {
                    pch = match (pch, iter.next()) {
                        (Some('.'), Some((i, ch))) if is_ws(ch) => {
                            if n > 1 {
                                n -= 1;
                            } else {
                                break cursor.saturating_add(i);
                            }
                            Some('.')
                        }
                        (Some(NL), Some((i, NL))) => {
                            if n > 1 {
                                n -= 1;
                            } else {
                                break cursor.saturating_add(i);
                            }
                            Some(NL)
                        }
                        (_, Some((_, ch))) => Some(ch),
                        (_, None) => {
                            break b.len_chars().saturating_sub(1);
                        }
                    };
                })
            }
            _ => err_at!(Fatal, msg: format!("unreachable")),
        }?;

        b.set_cursor(cursor);
        b.skip_whitespace(Right);

        Ok(Event::Noop)
    }

    fn mto_para(
        //
        &mut self,
        c: &mut Context,
        mut n: usize,
        evnt: Event,
    ) -> Result<Event> {
        use crate::event::{Event::*, DP::*};

        let b = c.as_mut_buffer();
        let mut cursor = b.to_cursor();
        let row = b.char_to_line(cursor);
        cursor = match evnt {
            MtoPara(Left) => {
                let mut iter = b.lines_at(row, Left)?.enumerate();
                let cursor = loop {
                    match iter.next() {
                        Some((i, line)) => match line.chars().next() {
                            Some(NL) if n == 0 => {
                                break b.line_to_char(row - (i + 1));
                            }
                            Some(NL) => n -= 1,
                            Some(_) => (),
                            None => break b.line_to_char(row - (i + 1)),
                        },
                        None => break 0,
                    }
                };
                Ok(cursor)
            }
            MtoPara(Right) => {
                let mut iter = b.lines_at(row, Right)?.enumerate();
                let cursor = loop {
                    match iter.next() {
                        Some((i, line)) => match line.chars().next() {
                            Some(NL) if n == 0 => {
                                break b.line_to_char(row + i);
                            }
                            Some(NL) => n -= 1,
                            Some(_) => (),
                            None => break b.line_to_char(row + i),
                        },
                        None => break b.len_chars().saturating_sub(1),
                    }
                };
                Ok(cursor)
            }
            _ => err_at!(Fatal, msg: format!("unreachable")),
        }?;

        b.set_cursor(cursor);
        Ok(Event::Noop)
    }

    fn mto_bracket(
        //
        &mut self,
        c: &mut Context,
        mut n: usize,
        e: Event,
    ) -> Result<Event> {
        use crate::event::{Event::*, DP::*};

        let mut m = 0;
        let b = c.as_mut_buffer();
        let mut cursor = b.to_cursor();
        match e {
            MtoBracket(yin, yan, Left) => {
                let mut iter = b.chars_at(cursor, Left)?.enumerate();
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
                let mut iter = b.chars_at(cursor, Right)?.enumerate();
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

        b.set_cursor(cursor);
        Ok(Event::Noop)
    }

    fn mto_pattern(
        //
        &mut self,
        c: &mut Context,
        n: usize,
        evnt: Event,
    ) -> Result<Event> {
        use crate::event::{Event::*, DP::*};

        let (pattern, dp) = match evnt {
            MtoPattern(Some(pattern), dp) => Ok((pattern, dp)),
            _ => err_at!(Fatal, msg: format!("unreachable")),
        }?;

        let b = c.as_mut_buffer();
        let search = {
            let text = b.to_string();
            Search::new(&pattern, &text, dp)?
        };
        let mut cursor = b.to_cursor();
        let byte_off = b.char_to_byte(cursor);

        let n = n.saturating_sub(1);
        cursor = match dp {
            Left => {
                let item = search.rev(byte_off).skip(n).next();
                match item {
                    Some((s, _)) => Ok(s),
                    None => Ok(cursor),
                }
            }
            Right => {
                let item = search.iter(byte_off).skip(n).next();
                match item {
                    Some((s, _)) => Ok(s),
                    None => Ok(cursor),
                }
            }
            _ => err_at!(Fatal, msg: format!("unreachable")),
        }?;

        b.set_cursor(cursor);
        Ok(Event::Noop)
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
