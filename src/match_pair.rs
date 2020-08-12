use lazy_static::lazy_static;

use crate::{buffer::Buffer, window::WinBuffer};

lazy_static! {
    static ref PIVOTS: Vec<usize> = {
        let mut pivots: Vec<usize> = Vec::default();
        for ch in CurlyBracket::pivot_chars() {
            pivots[ch as u32 as usize] = MatchPair::Curly as u32 as usize;
        }
        for ch in Paranthesis::pivot_chars() {
            pivots[ch as u32 as usize] = MatchPair::Paran as u32 as usize;
        }
        for ch in SquarBracket::pivot_chars() {
            pivots[ch as u32 as usize] = MatchPair::Squar as u32 as usize;
        }
        pivots
    };
}

pub fn match_under_cursor(buf: &Buffer) -> Option<usize> {
    use crate::event::DP;

    let cursor = buf.to_char_cursor();
    match buf.chars_at(cursor, DP::Right).ok()?.next()? {
        '{' | '}' => CurlyBracket::match_under_cursor(buf),
        '(' | ')' => Paranthesis::match_under_cursor(buf),
        '[' | ']' => SquarBracket::match_under_cursor(buf),
        _ => None,
    }
}

pub fn unmatch_before(buf: &Buffer, ch: char, n: usize) -> Option<usize> {
    match ch {
        '{' => CurlyBracket::unmatch_before(buf, n),
        '(' => Paranthesis::unmatch_before(buf, n),
        _ => None,
    }
}

pub fn unmatch_after(buf: &Buffer, ch: char, n: usize) -> Option<usize> {
    match ch {
        '}' => CurlyBracket::unmatch_after(buf, n),
        ')' => Paranthesis::unmatch_after(buf, n),
        _ => None,
    }
}

enum MatchPair {
    Curly = 0,
    Paran,
    Squar,
}

struct CurlyBracket;

impl CurlyBracket {
    fn pivot_chars() -> Vec<char> {
        vec!['{', '}']
    }

    fn match_under_cursor(buf: &Buffer) -> Option<usize> {
        use crate::event::DP;

        let mut iter = buf.chars_at(buf.to_char_cursor(), DP::Right).ok()?;
        match iter.next() {
            Some('{') => Self::match_fwd(buf),
            Some('}') => Self::match_rev(buf),
            _ => None,
        }
    }

    fn unmatch_before(buf: &Buffer, mut n: usize) -> Option<usize> {
        use crate::event::DP;

        let cursor = buf.to_char_cursor();
        let mut iter = buf.chars_at(cursor, DP::Left).ok()?.enumerate();
        let mut stack = 0_usize;
        loop {
            match iter.next() {
                Some((_, '}')) => stack = stack.saturating_add(1),
                Some((i, '{')) if stack == 0 && n == 0 => break Some(cursor.saturating_sub(i + 1)),
                Some((_, '{')) if stack == 0 => n = n.saturating_sub(1),
                Some((_, '{')) => stack = stack.saturating_sub(1),
                Some((_, _)) => (),
                None => break None,
            }
        }
    }

    fn unmatch_after(buf: &Buffer, mut n: usize) -> Option<usize> {
        use crate::event::DP;

        let cursor = buf.to_char_cursor();
        let mut iter = buf.chars_at(cursor, DP::Right).ok()?.enumerate();
        let mut stack = 0_usize;
        loop {
            match iter.next() {
                Some((_, '{')) => stack = stack.saturating_add(1),
                Some((i, '}')) if stack == 0 && n == 0 => break Some(cursor.saturating_add(i)),
                Some((_, '}')) if stack == 0 => n = n.saturating_sub(1),
                Some((_, '}')) => stack = stack.saturating_sub(1),
                Some((_, _)) => (),
                None => break None,
            }
        }
    }

    fn match_rev(buf: &Buffer) -> Option<usize> {
        use crate::event::DP;

        let cursor = buf.to_char_cursor();
        let mut iter = buf.chars_at(cursor, DP::Left).ok()?.enumerate();
        let mut stack = 0_usize;
        loop {
            match iter.next() {
                Some((i, '{')) if stack == 0 => break Some(cursor.saturating_sub(i + 1)),
                Some((_, '{')) => stack = stack.saturating_sub(1),
                Some((_, '}')) => stack = stack.saturating_add(1),
                Some((_, _)) => (),
                None => break None,
            }
        }
    }

    fn match_fwd(buf: &Buffer) -> Option<usize> {
        use crate::event::DP;

        let cursor = buf.to_char_cursor();
        let mut iter = buf.chars_at(cursor, DP::Right).ok()?.enumerate();
        let mut stack = 0_usize;
        loop {
            match iter.next() {
                Some((i, '}')) if stack == 1 => break Some(cursor.saturating_add(i)),
                Some((_, '}')) => stack = stack.saturating_sub(1),
                Some((_, '{')) => stack = stack.saturating_add(1),
                Some((_, _)) => (),
                None => break None,
            }
        }
    }
}

struct Paranthesis;

impl Paranthesis {
    fn pivot_chars() -> Vec<char> {
        vec!['(', ')']
    }

    fn match_under_cursor(buf: &Buffer) -> Option<usize> {
        use crate::event::DP;

        let mut iter = buf.chars_at(buf.to_char_cursor(), DP::Right).ok()?;
        match iter.next() {
            Some('(') => Self::match_fwd(buf),
            Some(')') => Self::match_rev(buf),
            _ => None,
        }
    }

    fn unmatch_before(buf: &Buffer, mut n: usize) -> Option<usize> {
        use crate::event::DP;

        let cursor = buf.to_char_cursor();
        let mut iter = buf.chars_at(cursor, DP::Left).ok()?.enumerate();
        let mut stack = 0_usize;
        loop {
            match iter.next() {
                Some((_, ')')) => stack = stack.saturating_add(1),
                Some((i, '(')) if stack == 0 && n == 0 => break Some(cursor.saturating_sub(i + 1)),
                Some((_, '(')) if stack == 0 => n = n.saturating_sub(1),
                Some((_, '(')) => stack = stack.saturating_sub(1),
                Some((_, _)) => (),
                None => break None,
            }
        }
    }

    fn unmatch_after(buf: &Buffer, mut n: usize) -> Option<usize> {
        use crate::event::DP;

        let cursor = buf.to_char_cursor();
        let mut iter = buf.chars_at(cursor, DP::Right).ok()?.enumerate();
        let mut stack = 0_usize;
        loop {
            match iter.next() {
                Some((_, '(')) => stack = stack.saturating_add(1),
                Some((i, ')')) if stack == 0 && n == 0 => break Some(cursor.saturating_add(i)),
                Some((_, ')')) if stack == 0 => n = n.saturating_sub(1),
                Some((_, ')')) => stack = stack.saturating_sub(1),
                Some((_, _)) => (),
                None => break None,
            }
        }
    }

    fn match_rev(buf: &Buffer) -> Option<usize> {
        use crate::event::DP;

        let cursor = buf.to_char_cursor();
        let mut iter = buf.chars_at(cursor, DP::Left).ok()?.enumerate();
        let mut stack = 0_usize;
        loop {
            match iter.next() {
                Some((i, '(')) if stack == 0 => break Some(cursor.saturating_sub(i + 1)),
                Some((_, '(')) => stack = stack.saturating_sub(1),
                Some((_, ')')) => stack = stack.saturating_add(1),
                Some((_, _)) => (),
                None => break None,
            }
        }
    }

    fn match_fwd(buf: &Buffer) -> Option<usize> {
        use crate::event::DP;

        let cursor = buf.to_char_cursor();
        let mut iter = buf.chars_at(cursor, DP::Right).ok()?.enumerate();
        let mut stack = 0_usize;
        loop {
            match iter.next() {
                Some((i, ')')) if stack == 0 => break Some(cursor.saturating_add(i)),
                Some((_, ')')) => stack = stack.saturating_sub(1),
                Some((_, '(')) => stack = stack.saturating_add(1),
                Some((_, _)) => (),
                None => break None,
            }
        }
    }
}

struct SquarBracket;

impl SquarBracket {
    fn pivot_chars() -> Vec<char> {
        vec!['[', ']']
    }

    fn match_under_cursor(buf: &Buffer) -> Option<usize> {
        use crate::event::DP;

        let mut iter = buf.chars_at(buf.to_char_cursor(), DP::Right).ok()?;
        match iter.next() {
            Some('[') => Self::match_fwd(buf),
            Some(']') => Self::match_rev(buf),
            _ => None,
        }
    }

    fn match_rev(buf: &Buffer) -> Option<usize> {
        use crate::event::DP;

        let cursor = buf.to_char_cursor();
        let mut iter = buf.chars_at(cursor, DP::Left).ok()?.enumerate();
        let mut stack = 0_usize;
        loop {
            match iter.next() {
                Some((i, '[')) if stack == 0 => break Some(cursor.saturating_sub(i + 1)),
                Some((_, '[')) => stack = stack.saturating_sub(1),
                Some((_, ']')) => stack = stack.saturating_add(1),
                Some((_, _)) => (),
                None => break None,
            }
        }
    }

    fn match_fwd(buf: &Buffer) -> Option<usize> {
        use crate::event::DP;

        let cursor = buf.to_char_cursor();
        let mut iter = buf.chars_at(cursor, DP::Right).ok()?.enumerate();
        let mut stack = 0_usize;
        loop {
            match iter.next() {
                Some((i, ']')) if stack == 0 => break Some(cursor.saturating_add(i)),
                Some((_, ']')) => stack = stack.saturating_sub(1),
                Some((_, '[')) => stack = stack.saturating_add(1),
                Some((_, _)) => (),
                None => break None,
            }
        }
    }
}
