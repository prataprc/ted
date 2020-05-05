use tree_sitter as ts;

use std::{cmp, ops::Bound};

use crate::{
    buffer::NL,
    event::{Event, Mod, Mto, DP},
    search::Search,
    window::Context,
    Error, Result,
};

extern "C" {
    fn tree_sitter_txt_en() -> ts::Language;
}

pub struct Text {
    parser: ts::Parser,
    tree: Option<ts::Tree>,
    mto_find_char: Mto,
    mto_pattern: Mto,
    insert_repeat: usize,
    last_inserts: Vec<Event>,
}

impl Default for Text {
    fn default() -> Text {
        let parser = {
            let mut p = ts::Parser::new();
            let language = unsafe { tree_sitter_txt_en() };
            p.set_language(language).unwrap();
            p
        };

        Text {
            parser,
            tree: None,
            insert_repeat: Default::default(),
            last_inserts: Default::default(),
            mto_find_char: Mto::None,
            mto_pattern: Mto::None,
        }
    }
}

impl Clone for Text {
    fn clone(&self) -> Text {
        let parser = {
            let mut p = ts::Parser::new();
            let language = unsafe { tree_sitter_txt_en() };
            p.set_language(language).unwrap();
            p
        };

        Text {
            parser,
            tree: None,
            insert_repeat: self.insert_repeat.clone(),
            last_inserts: self.last_inserts.clone(),
            mto_find_char: self.mto_find_char.clone(),
            mto_pattern: self.mto_pattern.clone(),
        }
    }
}

impl Text {
    pub fn to_type_name(&self) -> String {
        "txt".to_string()
    }

    pub fn on_event(&mut self, c: &mut Context, evnt: Event) -> Result<Event> {
        match c.as_buffer().to_mode() {
            "insert" => self.on_i_event(c, evnt),
            "normal" => self.on_n_event(c, evnt),
            _ => err_at!(Fatal, msg: format!("unreachable")),
        }
    }
}

impl Text {
    fn to_insert_n(evnt: Event) -> (Option<usize>, Event) {
        use crate::event::Event::Md;

        match evnt {
            Md(Mod::Insert(n, dp)) => (Some(n), Md(Mod::Insert(n, dp))),
            Md(Mod::Append(n, dp)) => (Some(n), Md(Mod::Append(n, dp))),
            Md(Mod::Open(n, dp)) => (Some(n), Md(Mod::Open(n, dp))),
            _ => (None, evnt),
        }
    }

    fn on_n_event(&mut self, c: &mut Context, evnt: Event) -> Result<Event> {
        use crate::event::Event::Mt;

        self.tree = match self.tree.take() {
            tree @ Some(_) => tree,
            None => {
                let b = c.as_mut_buffer();
                self.parser.parse(&b.to_string(), None)
            }
        };

        // switch to insert mode.
        let evnt = match Self::to_insert_n(evnt) {
            (Some(n), evnt) if n > 1 => {
                let b = c.as_mut_buffer();
                b.mode_insert()?;
                return self.on_i_event(c, evnt);
            }
            (_, evnt) => evnt,
        };

        let evnt = match evnt {
            Event::Noop => Event::Noop,
            // execute motion command.
            Mt(Mto::Left(n, dp)) => self.mto_left(c, n, dp)?,
            Mt(Mto::Right(n, dp)) => self.mto_right(c, n, dp)?,
            Mt(Mto::Up(n, dp)) => self.mto_up(c, n, dp)?,
            Mt(Mto::Down(n, dp)) => self.mto_down(c, n, dp)?,
            Mt(Mto::Col(n)) => self.mto_column(c, n)?,
            Mt(Mto::Home(dp)) => self.mto_home(c, dp)?,
            Mt(Mto::End) => self.mto_end(c)?,
            Mt(Mto::Row(n, dp)) => self.mto_row(c, n, dp)?,
            Mt(Mto::Percent(n)) => self.mto_percent(c, n)?,
            Mt(Mto::Cursor(n)) => self.mto_cursor(c, n)?,
            Mt(e @ Mto::CharF(_, _, _)) => self.mto_char(c, e)?,
            Mt(e @ Mto::CharT(_, _, _)) => self.mto_char(c, e)?,
            Mt(Mto::CharR(n, dir)) => {
                let e = self.mto_find_char.clone().transform(n, dir)?;
                self.mto_char(c, e)?
            }
            Mt(e @ Mto::Word(_, _, _)) => self.mto_words(c, e)?,
            Mt(e @ Mto::WWord(_, _, _)) => self.mto_wwords(c, e)?,
            Mt(e @ Mto::Sentence(_, _)) => self.mto_sentence(c, e)?,
            Mt(e @ Mto::Para(_, _)) => self.mto_para(c, e)?,
            Mt(e @ Mto::Bracket(_, _, _, _)) => self.mto_bracket(c, e)?,
            Mt(e @ Mto::Pattern(_, Some(_), _)) => self.mto_pattern(c, e)?,
            Mt(Mto::PatternR(n, dir)) => {
                let e = self.mto_pattern.clone().transform(n, dir)?;
                self.mto_pattern(c, e)?
            }
            evnt => evnt,
        };

        Ok(evnt)
    }

    fn on_i_event(&mut self, c: &mut Context, mut evnt: Event) -> Result<Event> {
        use crate::event::Event::Md;

        evnt = match self.ex_i_event(c, evnt)? {
            Md(Mod::Insert(n, pos)) if n > 0 => {
                self.insert_repeat = n - 1;
                if pos == DP::Caret {
                    self.mto_home(c, DP::Caret)?;
                }
                Event::Noop
            }
            Md(Mod::Insert(_, _)) => Event::Noop,
            Md(Mod::Append(n, pos)) if n > 0 => {
                self.insert_repeat = n - 1;
                if pos == DP::End {
                    self.mto_end(c)?;
                }
                self.mto_right(c, 1, DP::Nobound)?;
                Event::Noop
            }
            Md(Mod::Append(_, _)) => Event::Noop,
            Md(Mod::Open(n, DP::Left)) if n > 0 => {
                self.insert_repeat = n - 1;
                self.mto_home(c, DP::Nope)?;
                c.as_mut_buffer().insert_char(NL)?;
                self.mto_left(c, 1, DP::Nobound)?;
                Event::Noop
            }
            Md(Mod::Open(n, DP::Right)) if n > 0 => {
                self.insert_repeat = n - 1;
                self.mto_end(c)?;
                self.mto_right(c, 1, DP::Nobound)?;
                c.as_mut_buffer().insert_char(NL)?;
                Event::Noop
            }
            Md(Mod::Open(_, _)) => Event::Noop,
            evnt => {
                self.last_inserts.push(evnt.clone());
                evnt
            }
        };

        Ok(self.ex_i_event(c, evnt)?)
    }

    fn ex_i_event(&mut self, c: &mut Context, evnt: Event) -> Result<Event> {
        use crate::event::Event::Delete;
        use crate::event::Event::{Backspace, Char, Enter, Esc, Mt, Tab};

        let evnt = match evnt {
            // movement
            Mt(Mto::Left(n, dp)) => self.mto_left(c, n, dp)?,
            Mt(Mto::Right(n, dp)) => self.mto_right(c, n, dp)?,
            Mt(Mto::Up(n, dp)) => self.mto_up(c, n, dp)?,
            Mt(Mto::Down(n, dp)) => self.mto_down(c, n, dp)?,
            Mt(Mto::Home(dp)) => self.mto_home(c, dp)?,
            Mt(Mto::End) => self.mto_end(c)?,
            // Handle mode events.
            Esc => {
                self.repeat(c)?;
                self.mto_left(c, 1, DP::LineBound)?;
                c.as_mut_buffer().mode_normal()?;
                Event::Noop
            }
            // on going insert
            Char(ch, _) => {
                c.as_mut_buffer().insert_char(ch)?;
                Event::Noop
            }
            Backspace => {
                c.as_mut_buffer().backspace(1)?;
                Event::Noop
            }
            Enter => {
                c.as_mut_buffer().insert_char(NL)?;
                Event::Noop
            }
            Tab => {
                c.as_mut_buffer().insert_char('\t')?;
                Event::Noop
            }
            Delete => {
                let from = Bound::Included(c.as_mut_buffer().to_cursor());
                let to = from.clone();
                c.as_mut_buffer().remove_at(from, to)?;
                Event::Noop
            }
            evnt => evnt,
        };

        Ok(evnt)
    }

    fn repeat(&mut self, c: &mut Context) -> Result<()> {
        use crate::event::Event::{Backspace, Char, Delete, Enter, Tab};
        let last_inserts = {
            let evnts: Vec<Event> = self.last_inserts.drain(..).collect();
            let valid = evnts.iter().all(|evnt| match evnt {
                Char(_, _) | Enter | Tab | Backspace | Delete => true,
                _ => false,
            });
            if valid {
                evnts
            } else {
                vec![]
            }
        };

        for _ in 0..self.insert_repeat {
            for evnt in last_inserts.iter() {
                self.ex_i_event(c, evnt.clone())?;
            }
        }

        self.insert_repeat = 0;
        self.last_inserts = last_inserts;
        Ok(())
    }
}

impl Text {
    fn mto_left(&mut self, c: &mut Context, n: usize, dp: DP) -> Result<Event> {
        let mut cursor = c.as_buffer().to_cursor();
        cursor = match dp {
            DP::LineBound => {
                let home = c.as_buffer().line_home();
                let new_cursor = cursor.saturating_sub(n);
                Ok(if_else!(new_cursor > home, new_cursor, home))
            }
            DP::Nobound => Ok(cursor.saturating_sub(n)),
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
        let b = c.as_mut_buffer();
        b.set_cursor(b.line_home());
        match pos {
            DP::Caret => {
                b.skip_whitespace(DP::Right);
            }
            DP::Nope => (),
            _ => err_at!(Fatal, msg: format!("unreachable"))?,
        }
        Ok(Event::Noop)
    }

    fn mto_up(&mut self, c: &mut Context, n: usize, pos: DP) -> Result<Event> {
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
                    DP::Caret => self.mto_home(c, DP::Caret),
                    DP::Nope => Ok(Event::Noop),
                    _ => {
                        err_at!(Fatal, msg: format!("unreachable"))?;
                        Ok(Event::Noop)
                    }
                }
            }
        }
    }

    fn mto_down(&mut self, c: &mut Context, n: usize, pos: DP) -> Result<Event> {
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
                    DP::Caret => self.mto_home(c, DP::Caret),
                    DP::Nope => Ok(Event::Noop),
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
        let b = c.as_buffer();
        let row = b.char_to_line(b.to_cursor());
        match b.len_lines() {
            0 => Ok(Event::Noop),
            mut n_rows if n < 100 => {
                n_rows = n_rows.saturating_sub(1);
                match (((n_rows as f64) * (n as f64)) / (100 as f64)) as usize {
                    n if n < row => self.mto_up(c, row - n, DP::Caret),
                    n => self.mto_down(c, n - row, DP::Caret),
                }
            }
            n_rows => self.mto_down(c, n_rows.saturating_sub(1), DP::Caret),
        }
    }

    fn mto_cursor(&mut self, c: &mut Context, n: usize) -> Result<Event> {
        let b = c.as_mut_buffer();
        let cursor = b.to_cursor();
        b.set_cursor(limite!(cursor + n, b.len_chars()));
        Ok(Event::Noop)
    }

    // TODO: create an option of having sticky cursor.
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

    fn mto_char(&mut self, c: &mut Context, evnt: Mto) -> Result<Event> {
        self.mto_find_char = evnt.clone();
        let (mut n, ch, dp, pos) = match evnt {
            Mto::CharF(n, Some(ch), dp) => (n, ch, dp, DP::Find),
            Mto::CharT(n, Some(ch), dp) => (n, ch, dp, DP::Till),
            Mto::None => return Ok(Event::Noop),
            _ => unreachable!(),
        };

        let b = c.as_mut_buffer();
        let mut cursor = b.to_cursor();
        let home = b.line_home();
        cursor = match dp {
            DP::Right => {
                let mut iter = b.chars_at(cursor, DP::Right)?.enumerate();
                loop {
                    match iter.next() {
                        Some((_, NL)) => break cursor,
                        Some((i, c)) if c == ch && n == 0 && pos == DP::Find => {
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
            DP::Left => {
                let mut iter = b.chars_at(cursor, DP::Left)?.enumerate();
                loop {
                    match iter.next() {
                        Some((_, NL)) => break cursor,
                        Some((i, c)) if c == ch && n == 0 && pos == DP::Find => {
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

    fn mto_words(&mut self, c: &mut Context, evnt: Mto) -> Result<Event> {
        match evnt {
            Mto::Word(n, DP::Left, pos) => {
                for _ in 0..n {
                    let n = c.as_mut_buffer().skip_whitespace(DP::Left);
                    match pos {
                        DP::End if n == 0 => {
                            c.as_mut_buffer().skip_alphanumeric(DP::Left);
                            self.mto_right(c, 1, DP::Nobound)?;
                        }
                        DP::End => {
                            c.as_mut_buffer().skip_alphanumeric(DP::Left);
                            self.mto_right(c, 1, DP::Nobound)?;
                        }
                        DP::Start if n == 0 => {
                            c.as_mut_buffer().skip_alphanumeric(DP::Left);
                            c.as_mut_buffer().skip_whitespace(DP::Left);
                        }
                        DP::Start => (),
                        _ => unreachable!(),
                    }
                }
                Ok(Event::Noop)
            }
            Mto::Word(n, DP::Right, pos) => {
                for _ in 0..n {
                    let n = c.as_mut_buffer().skip_whitespace(DP::Right);
                    match pos {
                        DP::End if n == 0 => {
                            c.as_mut_buffer().skip_alphanumeric(DP::Right);
                            self.mto_left(c, 1, DP::Nobound)?;
                        }
                        DP::End => {
                            c.as_mut_buffer().skip_alphanumeric(DP::Right);
                            self.mto_left(c, 1, DP::Nobound)?;
                        }
                        DP::Start if n == 0 => {
                            c.as_mut_buffer().skip_alphanumeric(DP::Right);
                            c.as_mut_buffer().skip_whitespace(DP::Right);
                        }
                        DP::Start => (),
                        _ => unreachable!(),
                    }
                }
                Ok(Event::Noop)
            }
            _ => err_at!(Fatal, msg: format!("unreachable")),
        }
    }

    fn mto_wwords(&mut self, c: &mut Context, evnt: Mto) -> Result<Event> {
        match evnt {
            Mto::WWord(n, DP::Left, pos) => {
                for _ in 0..n {
                    let n = c.as_mut_buffer().skip_whitespace(DP::Left);
                    match pos {
                        DP::Start if n == 0 => {
                            c.as_mut_buffer().skip_non_whitespace(DP::Left);
                            self.mto_right(c, 1, DP::Nobound)?;
                        }
                        DP::Start => {
                            c.as_mut_buffer().skip_non_whitespace(DP::Left);
                            self.mto_right(c, 1, DP::Nobound)?;
                        }
                        DP::End if n == 0 => {
                            c.as_mut_buffer().skip_non_whitespace(DP::Left);
                            c.as_mut_buffer().skip_whitespace(DP::Left);
                        }
                        DP::End => (),
                        _ => unreachable!(),
                    }
                }
                Ok(Event::Noop)
            }
            Mto::WWord(n, DP::Right, pos) => {
                for _ in 0..n {
                    let n = c.as_mut_buffer().skip_whitespace(DP::Right);
                    match pos {
                        DP::End if n == 0 => {
                            c.as_mut_buffer().skip_non_whitespace(DP::Right);
                            self.mto_left(c, 1, DP::Nobound)?;
                        }
                        DP::End => {
                            c.as_mut_buffer().skip_non_whitespace(DP::Right);
                            self.mto_left(c, 1, DP::Nobound)?;
                        }
                        DP::Start if n == 0 => {
                            c.as_mut_buffer().skip_non_whitespace(DP::Right);
                            c.as_mut_buffer().skip_whitespace(DP::Right);
                        }
                        DP::Start => (),
                        _ => unreachable!(),
                    }
                }
                Ok(Event::Noop)
            }
            _ => err_at!(Fatal, msg: format!("unreachable")),
        }
    }

    fn mto_sentence(&mut self, c: &mut Context, e: Mto) -> Result<Event> {
        let is_ws = |ch: char| ch.is_whitespace();

        let b = c.as_mut_buffer();
        let mut cursor = b.to_cursor();
        let mut pch: Option<char> = None;
        cursor = match e {
            Mto::Sentence(mut n, DP::Left) => {
                let mut iter = b.chars_at(cursor, DP::Left)?.enumerate();
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
            Mto::Sentence(mut n, DP::Right) => {
                let mut iter = b.chars_at(cursor, DP::Right)?.enumerate();
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
        b.skip_whitespace(DP::Right);

        Ok(Event::Noop)
    }

    fn mto_para(&mut self, c: &mut Context, evnt: Mto) -> Result<Event> {
        let b = c.as_mut_buffer();
        let mut cursor = b.to_cursor();
        let row = b.char_to_line(cursor);
        cursor = match evnt {
            Mto::Para(mut n, DP::Left) => {
                let mut iter = b.lines_at(row, DP::Left)?.enumerate();
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
            Mto::Para(mut n, DP::Right) => {
                let mut iter = b.lines_at(row, DP::Right)?.enumerate();
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

    fn mto_bracket(&mut self, c: &mut Context, e: Mto) -> Result<Event> {
        let mut m = 0;
        let b = c.as_mut_buffer();
        let mut cursor = b.to_cursor();
        match e {
            Mto::Bracket(mut n, yin, yan, DP::Left) => {
                let mut iter = b.chars_at(cursor, DP::Left)?.enumerate();
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
            Mto::Bracket(mut n, yin, yan, DP::Right) => {
                let mut iter = b.chars_at(cursor, DP::Right)?.enumerate();
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

    fn mto_pattern(&mut self, c: &mut Context, evnt: Mto) -> Result<Event> {
        self.mto_pattern = evnt.clone();
        let (n, pattern, dp) = match evnt {
            Mto::Pattern(n, Some(pattern), dp) => Ok((n, pattern, dp)),
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
            DP::Left => {
                let item = search.rev(byte_off).skip(n).next();
                match item {
                    Some((s, _)) => Ok(s),
                    None => Ok(cursor),
                }
            }
            DP::Right => {
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
