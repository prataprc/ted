use crate::window::Context;

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
enum FT {
    Text(Text),
}

impl Default for FT {
    fn default() -> FT {
        FT::Text(Default::default())
    }
}

impl FT {
    fn on_event(&mut self, c: &mut Context, evnt, Event) -> Result<Event> {
        match self {
            FT::Text(t) => t.on_event(c, evnt)
        }
    }
}

#[derive(Clone)]
struct Text;

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
        }
    }

    fn on_n_event(&mut self, c: &mut Context, evnt: Event) -> Result<Event> {
        use crate::event::{Event::*, DP::*};

        // switch to insert mode.
        match evnt {
            evnt @ N(n, _) if n > 1 && evnt.is_insert() => {
                let b = c.as_mut_buffer();
                b.mode_insert()?;
                b.on_event(c, evnt);
            }
            _ => (),
        };

        let evnt = match e {
            Noop => Noop,
            // execute motion command.
            N(n, box MtoLeft(dp)) => self.mto_left(n, dp)?,
            N(n, box MtoRight(dp)) => self.mto_right(n, dp)?,
            N(n, box MtoUp(dp)) => self.mto_up(n, dp)?,
            N(n, box MtoDown(dp)) => self.mto_down(n, dp)?,
            N(n, box MtoCol) => self.mto_column(n)?,
            N(n, box MtoRow(dp)) => self.mto_row(n, dp)?,
            N(n, box MtoPercent) => self.mto_percent(n)?,
            N(_, box MtoHome(dp)) => self.mto_home(dp)?,
            N(_, box MtoEnd) => self.mto_end()?, // TODO: make this sticky.
            N(n, box MtoCursor) => self.mto_cursor(n)?,
            N(n, e @ box MtoCharF(_, _)) => self.mto_char(n, *e)?,
            N(n, e @ box MtoCharT(_, _)) => self.mto_char(n, *e)?,
            N(n, e @ box MtoWord(_, _)) => self.mto_words(n, *e)?,
            N(n, e @ box MtoWWord(_, _)) => self.mto_wwords(n, *e)?,
            N(n, e @ box MtoSentence(_)) => self.mto_sentence(n, *e)?,
            N(n, e @ box MtoPara(_)) => self.mto_para(n, *e)?,
            N(n, e @ box MtoBracket(_, _, _)) => self.mto_bracket(n, *e)?,
            N(n, e @ box MtoPattern(Some(_), _)) => self.mto_pattern(n, *e)?,
            // execute mode switching commands
            N(n, box ModeInsert(Caret)) => {
                self.mto_home(Caret)?;
                N(n, Box::new(ModeInsert(Caret)))
            }
            N(n, e @ box ModeInsert(_)) => N(n, Box::new(*e)),
            //Char('%', _) if m.is_empty() => {
            //    self.to_mut_change().fwd_match_group();
            //    Ok(Noop)
            //}
            evnt => evnt,
        };

        evnt
    }

    fn on_i_event(&mut self, c: &mut Context, evnt: Event) -> Result<Event> {
        todo!()
    }
}

impl Text {
    //fn mto_left(&mut self, n: usize, dp: DP) -> Result<Event> {
    //    use crate::event::DP::*;

    //    let mut cursor = self.as_mut_buffer().to_cursor();
    //    cursor = match dp {
    //        LineBound => {
    //            let home = self.as_mut_buffer().line_start();
    //            let new_cursor = cursor.saturating_sub(n);
    //            Ok(if_else!(new_cursor > home, new_cursor, home))
    //        }
    //        Nobound => Ok(cursor.saturating_sub(n)),
    //        _ => err_at!(Fatal, msg: format!("unreachable")),
    //    }?;

    //    self.as_mut_buffer().set_cursor(cursor);
    //    Ok(Event::Noop)
    //}

    //fn mto_right(&mut self, n: usize, dp: DP) -> Result<Event> {
    //    let b = self.as_mut_buffer();
    //    let mut cursor = b.to_cursor();
    //    for ch in b.chars_at(cursor).take(n) {
    //        if dp == DP::LineBound && ch == NL {
    //            break;
    //        }
    //        cursor += 1
    //    }

    //    b.set_cursor(cursor);
    //    Ok(Event::Noop)
    //}

    //fn mto_up(&mut self, n: usize, pos: DP) -> Result<Event> {
    //    use crate::event::DP::*;

    //    let b = self.as_mut_buffer();
    //    let mut cursor = b.to_cursor();
    //    match b.char_to_line(cursor) {
    //        0 => Ok(Event::Noop),
    //        row => {
    //            let row = row.saturating_sub(n);
    //            cursor = {
    //                let col = cmp::min(
    //                    self.buf.line(row).len_chars().saturating_sub(2),
    //                    self.to_col(),
    //                );
    //                self.buf.line_to_char(row) + col
    //            };
    //            b.set_cursor(cursor);
    //            match pos {
    //                Caret => self.mto_home(Caret)?,
    //                _ => (),
    //            };
    //            Ok(Event::Noop)
    //        }
    //    }
    //}

    //fn mto_down(&mut self, n: usize, pos: DP) -> Result<Event> {
    //    use crate::event::DP::*;

    //    let row = self.buf.char_to_line(self.cursor);
    //    match self.buf.len_lines() {
    //        0 => Ok(Event::Noop),
    //        n_rows if row == n_rows => Ok(Event::Noop),
    //        n_rows => {
    //            let row = limite!(row.saturating_add(n), n_rows);
    //            self.cursor = {
    //                let col = cmp::min(
    //                    self.buf.line(row).len_chars().saturating_sub(2),
    //                    self.to_col(),
    //                );
    //                self.buf.line_to_char(row) + col
    //            };
    //            if pos == Caret {
    //                self.mto_home(Caret)?;
    //            }
    //            Ok(Event::Noop)
    //        }
    //    }
    //}

    //fn mto_column(&mut self, n: usize) -> Result<Event> {
    //    for ch in self.buf.chars_at(self.cursor).take(n) {
    //        if ch == NL {
    //            break;
    //        }
    //        self.cursor += 1;
    //    }
    //    Ok(Event::Noop)
    //}

    //fn mto_row(&mut self, n: usize, pos: DP) -> Result<Event> {
    //    let row = self.buf.char_to_line(self.cursor);
    //    match self.buf.len_lines() {
    //        0 => Ok(Event::Noop),
    //        _ if n < row => self.mto_up(row - n, pos),
    //        n_rows if n < n_rows => self.mto_up(n - row, pos),
    //        _ => Ok(Event::Noop),
    //    }
    //}

    //fn mto_percent(&mut self, n: usize) -> Result<Event> {
    //    use crate::event::DP::*;

    //    let row = self.buf.char_to_line(self.cursor);
    //    match self.buf.len_lines() {
    //        0 => Ok(Event::Noop),
    //        mut n_rows if n < 100 => {
    //            n_rows -= 1;
    //            let n = (((n_rows as f64) * (n as f64)) / (100 as f64)) as usize;
    //            if n < row {
    //                self.mto_up(row - n, Nope)
    //            } else {
    //                self.mto_down(n - row, Nope)
    //            }
    //        }
    //        _ => Ok(Event::Noop),
    //    }
    //}

    //fn mto_cursor(&mut self, n: usize) -> Result<Event> {
    //    self.cursor = limite!(self.cursor + n, self.buf.len_chars());
    //    Ok(Event::Noop)
    //}

    //fn mto_home(&mut self, pos: DP) -> Result<Event> {
    //    use crate::event::DP::*;

    //    self.cursor = self.buf.line_to_char(self.buf.char_to_line(self.cursor));
    //    if pos == Caret {
    //        self.skip_whitespace(Right);
    //    }
    //    Ok(Event::Noop)
    //}

    //fn mto_end(&mut self) -> Result<Event> {
    //    let mut iter = self.buf.chars_at(self.cursor);
    //    let mut cursor = self.cursor;
    //    loop {
    //        match iter.next() {
    //            Some(NL) => break (),
    //            Some(_) => cursor += 1,
    //            None => break (),
    //        }
    //    }
    //    self.cursor = cursor;
    //    Ok(Event::Noop)
    //}

    //fn mto_char(&mut self, mut n: usize, evnt: Event) -> Result<Event> {
    //    use crate::event::DP::*;

    //    let (ch, dp, pos) = match evnt {
    //        Event::MtoCharF(Some(ch), dp) => (ch, dp, Find),
    //        Event::MtoCharT(Some(ch), dp) => (ch, dp, Till),
    //        _ => unreachable!(),
    //    };

    //    self.cursor = match dp {
    //        Right => {
    //            let mut iter = self.iter(dp).enumerate();
    //            loop {
    //                match iter.next() {
    //                    Some((_, NL)) => break self.cursor,
    //                    Some((i, c)) if c == ch && n == 0 && pos == Till => {
    //                        break self.cursor.saturating_add(i);
    //                    }
    //                    Some((i, c)) if c == ch && n == 0 => {
    //                        break self.cursor.saturating_add(i - 1);
    //                    }
    //                    Some((_, c)) if c == ch => n -= 1,
    //                    _ => (),
    //                }
    //            }
    //        }
    //        Left => {
    //            let mut iter = self.iter(dp).enumerate();
    //            loop {
    //                match iter.next() {
    //                    Some((_, NL)) => break self.cursor,
    //                    Some((i, c)) if c == ch && n == 0 && pos == Till => {
    //                        break self.cursor.saturating_add(i);
    //                    }
    //                    Some((i, c)) if c == ch && n == 0 => {
    //                        break self.cursor.saturating_add(i + 1);
    //                    }
    //                    Some((_, c)) if c == ch => n -= 1,
    //                    _ => (),
    //                }
    //            }
    //        }
    //        _ => unreachable!(),
    //    };

    //    Ok(Event::Noop)
    //}

    //fn mto_words(&mut self, n: usize, evnt: Event) -> Result<Event> {
    //    use crate::event::{Event::*, DP::*};

    //    match evnt {
    //        MtoWord(Left, pos) => {
    //            for _ in 0..n {
    //                let n = self.skip_whitespace(Left);
    //                match pos {
    //                    End if n == 0 => {
    //                        self.skip_alphanumeric(Left);
    //                        self.mto_right(1, Nobound)?;
    //                    }
    //                    End => {
    //                        self.skip_alphanumeric(Left);
    //                        self.mto_right(1, Nobound)?;
    //                    }
    //                    Start if n == 0 => {
    //                        self.skip_alphanumeric(Left);
    //                        self.skip_whitespace(Left);
    //                    }
    //                    Start => (),
    //                    _ => unreachable!(),
    //                }
    //            }
    //            Ok(Event::Noop)
    //        }
    //        MtoWord(Right, pos) => {
    //            for _ in 0..n {
    //                let n = self.skip_whitespace(Right);
    //                match pos {
    //                    End if n == 0 => {
    //                        self.skip_alphanumeric(Right);
    //                        self.mto_left(1, Nobound)?;
    //                    }
    //                    End => {
    //                        self.skip_alphanumeric(Right);
    //                        self.mto_left(1, Nobound)?;
    //                    }
    //                    Start if n == 0 => {
    //                        self.skip_alphanumeric(Right);
    //                        self.skip_whitespace(Right);
    //                    }
    //                    Start => (),
    //                    _ => unreachable!(),
    //                }
    //            }
    //            Ok(Event::Noop)
    //        }
    //        _ => err_at!(Fatal, msg: format!("unreachable")),
    //    }
    //}

    //fn mto_wwords(&mut self, n: usize, evnt: Event) -> Result<Event> {
    //    use crate::event::{Event::*, DP::*};

    //    match evnt {
    //        MtoWWord(Left, pos) => {
    //            for _ in 0..n {
    //                let n = self.skip_whitespace(Left);
    //                match pos {
    //                    Start if n == 0 => {
    //                        self.skip_non_whitespace(Left);
    //                        self.mto_right(1, Nobound)?;
    //                    }
    //                    Start => {
    //                        self.skip_non_whitespace(Left);
    //                        self.mto_right(1, Nobound)?;
    //                    }
    //                    End if n == 0 => {
    //                        self.skip_non_whitespace(Left);
    //                        self.skip_whitespace(Left);
    //                    }
    //                    End => (),
    //                    _ => unreachable!(),
    //                }
    //            }
    //            Ok(Event::Noop)
    //        }
    //        MtoWWord(Right, pos) => {
    //            for _ in 0..n {
    //                let n = self.skip_whitespace(Right);
    //                match pos {
    //                    End if n == 0 => {
    //                        self.skip_non_whitespace(Right);
    //                        self.mto_left(1, Nobound)?;
    //                    }
    //                    End => {
    //                        self.skip_non_whitespace(Right);
    //                        self.mto_left(1, Nobound)?;
    //                    }
    //                    Start if n == 0 => {
    //                        self.skip_non_whitespace(Right);
    //                        self.skip_whitespace(Right);
    //                    }
    //                    Start => (),
    //                    _ => unreachable!(),
    //                }
    //            }
    //            Ok(Event::Noop)
    //        }
    //        _ => err_at!(Fatal, msg: format!("unreachable")),
    //    }
    //}

    //fn mto_sentence(&mut self, mut n: usize, e: Event) -> Result<Event> {
    //    use crate::event::{Event::*, DP::*};

    //    let is_ws = |ch: char| ch.is_whitespace();

    //    let mut pch: Option<char> = None;
    //    self.cursor = match e {
    //        MtoSentence(Left) => {
    //            let mut iter = self.iter(Left).enumerate();
    //            Ok(loop {
    //                pch = match (iter.next(), pch) {
    //                    (Some((i, '.')), Some(pch)) if is_ws(pch) => {
    //                        if n > 1 {
    //                            n -= 1;
    //                        } else {
    //                            break self.cursor.saturating_sub(i);
    //                        }
    //                        Some('.')
    //                    }
    //                    (Some((i, NL)), Some(NL)) => {
    //                        if n > 1 {
    //                            n -= 1;
    //                        } else {
    //                            break self.cursor.saturating_sub(i);
    //                        }
    //                        Some(NL)
    //                    }
    //                    (Some((_, ch)), _) => Some(ch),
    //                    (None, _) => break 0,
    //                };
    //            })
    //        }
    //        MtoSentence(Right) => {
    //            let mut iter = self.iter(Right).enumerate();
    //            Ok(loop {
    //                pch = match (pch, iter.next()) {
    //                    (Some('.'), Some((i, ch))) if is_ws(ch) => {
    //                        if n > 1 {
    //                            n -= 1;
    //                        } else {
    //                            break self.cursor.saturating_add(i);
    //                        }
    //                        Some('.')
    //                    }
    //                    (Some(NL), Some((i, NL))) => {
    //                        if n > 1 {
    //                            n -= 1;
    //                        } else {
    //                            break self.cursor.saturating_add(i);
    //                        }
    //                        Some(NL)
    //                    }
    //                    (_, Some((_, ch))) => Some(ch),
    //                    (_, None) => {
    //                        break self.buf.len_chars().saturating_sub(1);
    //                    }
    //                };
    //            })
    //        }
    //        _ => err_at!(Fatal, msg: format!("unreachable")),
    //    }?;

    //    self.skip_whitespace(Right);

    //    Ok(Event::Noop)
    //}

    //fn mto_para(&mut self, mut n: usize, evnt: Event) -> Result<Event> {
    //    use crate::event::{Event::*, DP::*};

    //    let row = self.buf.char_to_line(self.cursor);
    //    self.cursor = match evnt {
    //        MtoPara(Left) => {
    //            let mut iter = self.iter_line(Left).enumerate();
    //            let cursor = loop {
    //                match iter.next() {
    //                    Some((i, line)) => match line.chars().next() {
    //                        Some(NL) if n == 0 => {
    //                            break self.buf.line_to_char(row - (i + 1));
    //                        }
    //                        Some(NL) => n -= 1,
    //                        Some(_) => (),
    //                        None => break self.buf.line_to_char(row - (i + 1)),
    //                    },
    //                    None => break 0,
    //                }
    //            };
    //            Ok(cursor)
    //        }
    //        MtoPara(Right) => {
    //            let mut iter = self.iter_line(Right).enumerate();
    //            let cursor = loop {
    //                match iter.next() {
    //                    Some((i, line)) => match line.chars().next() {
    //                        Some(NL) if n == 0 => {
    //                            break self.buf.line_to_char(row + i);
    //                        }
    //                        Some(NL) => n -= 1,
    //                        Some(_) => (),
    //                        None => break self.buf.line_to_char(row + i),
    //                    },
    //                    None => break self.buf.len_chars().saturating_sub(1),
    //                }
    //            };
    //            Ok(cursor)
    //        }
    //        _ => err_at!(Fatal, msg: format!("unreachable")),
    //    }?;

    //    Ok(Event::Noop)
    //}

    //fn mto_bracket(&mut self, mut n: usize, e: Event) -> Result<Event> {
    //    use crate::event::{Event::*, DP::*};

    //    let mut m = 0;
    //    let mut cursor = self.cursor;
    //    match e {
    //        MtoBracket(yin, yan, Left) => {
    //            let mut iter = self.iter(Left).enumerate();
    //            cursor -= loop {
    //                match iter.next() {
    //                    Some((_, ch)) if ch == yin && m > 0 => m -= 1,
    //                    Some((i, ch)) if ch == yin && n == 0 => break i + 1,
    //                    Some((_, ch)) if ch == yin => n -= 1,
    //                    Some((_, ch)) if ch == yan => m += 1,
    //                    Some(_) => (),
    //                    None => break 0,
    //                }
    //            };
    //        }
    //        MtoBracket(yin, yan, Right) => {
    //            let mut iter = self.iter(Right).enumerate();
    //            cursor += {
    //                loop {
    //                    match iter.next() {
    //                        Some((_, ch)) if ch == yin && m > 0 => m -= 1,
    //                        Some((i, ch)) if ch == yin && n == 0 => break i,
    //                        Some((_, ch)) if ch == yin => n -= 1,
    //                        Some((_, ch)) if ch == yan => m += 1,
    //                        Some(_) => (),
    //                        None => break 0,
    //                    }
    //                }
    //            };
    //        }
    //        _ => err_at!(Fatal, msg: format!("unreachable"))?,
    //    }

    //    self.cursor = cursor;
    //    Ok(Event::Noop)
    //}

    //fn mto_pattern(&mut self, n: usize, evnt: Event) -> Result<Event> {
    //    use crate::event::{Event::*, DP::*};

    //    let (pattern, dp) = match evnt {
    //        MtoPattern(Some(pattern), dp) => Ok((pattern, dp)),
    //        _ => err_at!(Fatal, msg: format!("unreachable")),
    //    }?;

    //    let text = self.buf.to_string();
    //    let search = Search::new(&pattern, &text, dp)?;
    //    let byte_off = self.buf.char_to_byte(self.cursor);

    //    let n = n.saturating_sub(1);
    //    self.cursor = match dp {
    //        Left => {
    //            let item = search.rev(byte_off).skip(n).next();
    //            match item {
    //                Some((s, _)) => Ok(s),
    //                None => Ok(self.cursor),
    //            }
    //        }
    //        Right => {
    //            let item = search.iter(byte_off).skip(n).next();
    //            match item {
    //                Some((s, _)) => Ok(s),
    //                None => Ok(self.cursor),
    //            }
    //        }
    //        _ => err_at!(Fatal, msg: format!("unreachable")),
    //    }?;

    //    Ok(Event::Noop)
    //}

    //fn skip_whitespace(&mut self, dp: DP) -> usize {
    //    use crate::event::DP::*;

    //    let mut n = 0;
    //    let n = loop {
    //        match self.iter(dp).next() {
    //            Some(ch) if ch.is_whitespace() => n += 1,
    //            Some(_) => break n,
    //            None => break n,
    //        }
    //    };
    //    self.cursor = if_else!(dp == Right, self.cursor + n, self.cursor - n);
    //    n
    //}

    //fn skip_non_whitespace(&mut self, dp: DP) -> usize {
    //    use crate::event::DP::*;

    //    let mut n = 0;
    //    let n = loop {
    //        match self.iter(dp).next() {
    //            Some(ch) if ch.is_whitespace() => n += 1,
    //            Some(_) => break n,
    //            None => break n,
    //        }
    //    };
    //    self.cursor = if_else!(dp == Right, self.cursor + n, self.cursor - n);
    //    n
    //}

    //fn skip_alphanumeric(&mut self, dp: DP) -> usize {
    //    use crate::event::DP::*;

    //    let mut n = 0;
    //    let n = loop {
    //        match self.iter(dp).next() {
    //            Some(ch) if ch.is_alphanumeric() => n += 1,
    //            Some(_) => break n,
    //            None => break n,
    //        }
    //    };
    //    self.cursor = if_else!(dp == Right, self.cursor + n, self.cursor - n);
    //    n
    //}

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
