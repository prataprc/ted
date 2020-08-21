#[allow(unused_imports)]
use log::{debug, trace};

use std::{cmp, convert::TryInto, fmt, result};

use crate::{
    col_nu::ColNu,
    event::DP,
    view,
    window::{Coord, Cursor, WinBuffer, Window},
    Result,
};

pub fn scroll_down<W, B>(name: &str, w: &W, buf: &B, n: usize) -> Result<(Cursor, usize)>
where
    W: Window,
    B: WinBuffer,
{
    let coord = w.to_coord();
    let mut cursor = w.to_cursor().unwrap_or(Cursor::default());

    let nbc_xy = buf.to_xy_cursor(None);
    let mut nbc = buf.to_char_cursor();

    let scroll_off = w.config_scroll_offset();
    let line_number = w.config_line_number();

    let lines = match w.config_wrap() {
        true => {
            let mut v: view::Wrap = (w, nbc_xy).try_into()?;
            v.shift_cursor(buf)?;
            v.to_edit_lines(buf)
        }
        false => {
            let mut v: view::NoWrap = (w, nbc_xy).try_into()?;
            v.shift_cursor(buf)?;
            v.to_edit_lines(buf)
        }
    };

    // lines till cursor-line (exclusive).
    let mut lines = {
        let off = view::cursor_line(&lines, nbc).unwrap_or(cursor.row.into());
        lines[..off].to_vec()
    };

    // lines from cursor-line (inclusive), till screen-end.
    let m = (coord.hgt as usize).saturating_sub(lines.len());
    let iter: Box<dyn Iterator<Item = view::ScrLine>> = match w.config_wrap() {
        true => {
            let mut iter = WrapIter::new_scroll_down(name, w, buf)?;
            lines.extend(iter.take_lines(m));
            Box::new(iter)
        }
        false => {
            let mut iter = NowrapIter::new_scroll_down(name, w, buf)?;
            lines.extend(iter.take_lines(m));
            Box::new(iter)
        }
    };
    let (_, nu_wth) = view::to_nu_width(&lines, line_number);

    let ocol = cursor.col.saturating_sub(nu_wth);
    let max_wth = coord.wth.saturating_sub(nu_wth);

    for line in iter.take(n) {
        lines.push(line.clone());
        lines.remove(0);

        let row = cursor.row.saturating_sub(1);
        cursor.row = if_else!(row >= scroll_off, row, scroll_off);
        cursor.col = cmp::min(cmp::min(ocol, line.n), max_wth);
        nbc = lines[cursor.row as usize].bc + (cursor.col as usize);
    }
    // adjust for new nu_wth
    cursor.col = {
        let (_, nu_wth) = view::to_nu_width(&lines, line_number);
        cmp::min(cursor.col.saturating_add(nu_wth), coord.wth)
    };

    Ok((cursor, nbc))
}

pub fn scroll_up<W, B>(name: &str, w: &W, buf: &B, n: usize) -> Result<(Cursor, usize)>
where
    W: Window,
    B: WinBuffer,
{
    let coord = w.to_coord();
    let mut cursor = w.to_cursor().unwrap_or(Cursor::default());

    let nbc_xy = buf.to_xy_cursor(None);
    let mut nbc = buf.to_char_cursor();

    let scroll_off = w.config_scroll_offset();
    let line_number = w.config_line_number();

    let lines = match w.config_wrap() {
        true => {
            let mut v: view::Wrap = (w, nbc_xy).try_into()?;
            v.shift_cursor(buf)?;
            v.to_edit_lines(buf)
        }
        false => {
            let mut v: view::NoWrap = (w, nbc_xy).try_into()?;
            v.shift_cursor(buf)?;
            v.to_edit_lines(buf)
        }
    };

    // lines till cursor-line (exclusive).
    let mut lines = {
        let off = view::cursor_line(&lines, nbc).unwrap_or(cursor.row.into());
        lines[off..].to_vec()
    };

    // lines from cursor-line (inclusive), till screen-end.
    let m = (coord.hgt as usize).saturating_sub(lines.len());
    let iter: Box<dyn Iterator<Item = view::ScrLine>> = match w.config_wrap() {
        true => {
            let mut iter = WrapIter::new_scroll_up(name, w, buf)?;
            iter.take_lines(m)
                .into_iter()
                .for_each(|l| lines.insert(0, l));
            Box::new(iter)
        }
        false => {
            let mut iter = NowrapIter::new_scroll_up(name, w, buf)?;
            iter.take_lines(m)
                .into_iter()
                .for_each(|l| lines.insert(0, l));
            Box::new(iter)
        }
    };
    let (_, nu_wth) = view::to_nu_width(&lines, line_number);

    let ocol = cursor.col.saturating_sub(nu_wth);
    let max_wth = coord.wth.saturating_sub(nu_wth);

    let max_row = coord.hgt.saturating_sub(scroll_off + 1);
    for line in iter.take(n) {
        lines.insert(0, line.clone());
        lines.pop();

        let row = cursor.row.saturating_add(1);
        cursor.row = if_else!(row <= max_row, row, max_row);
        cursor.col = cmp::min(cmp::min(ocol, line.n), max_wth);
        nbc = lines[cursor.row as usize].bc + (cursor.col as usize);
    }
    // adjust for new nu_wth
    let (_, nu_wth) = view::to_nu_width(&lines, line_number);
    cursor.col = cmp::min(cursor.col.saturating_add(nu_wth), coord.wth);

    Ok((cursor, nbc))
}

pub struct WrapIter<'a, B>
where
    B: WinBuffer,
{
    name: String,
    coord: Coord,
    buf: &'a B,
    line_number: bool,
    dir: DP,

    line_idx: Option<usize>,
    lines: Vec<view::ScrLine>,
}

impl<'a, B> fmt::Display for WrapIter<'a, B>
where
    B: WinBuffer,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(
            f,
            "WrapIter<{:?} {} {} {:?}>",
            self.name, self.coord, self.dir, self.line_idx,
        )
    }
}

impl<'a, B> WrapIter<'a, B>
where
    B: WinBuffer,
{
    pub fn new_scroll_down<W>(name: &str, w: &W, buf: &'a B) -> Result<Self>
    where
        W: Window,
    {
        let coord = w.to_coord();
        let line_number = w.config_line_number();

        let nbc = buf.to_char_cursor();
        let nbc_xy = buf.to_xy_cursor(None);

        let line_idx = nbc_xy.row;
        let mut lines: Vec<view::ScrLine> = {
            let nu_wth = ColNu::new(line_idx, line_number).to_width();
            let wth = coord.wth.saturating_sub(nu_wth);
            let iter = view::wrap_line(buf, line_idx, nu_wth, wth).into_iter();
            iter.skip_while(|x| (x.bc + (x.n as usize)) <= nbc)
                .collect()
        };
        lines.reverse();

        Ok(WrapIter {
            name: name.to_string(),
            coord,
            buf,
            line_number,
            dir: DP::Right,

            line_idx: incr_line_idx(buf, Some(line_idx)),
            lines,
        })
    }

    pub fn new_scroll_up<W>(name: &str, w: &W, buf: &'a B) -> Result<Self>
    where
        W: Window,
    {
        let coord = w.to_coord();
        let line_number = w.config_line_number();

        let nbc = buf.to_char_cursor();
        let nbc_xy = buf.to_xy_cursor(None);

        let line_idx = nbc_xy.row;
        let lines: Vec<view::ScrLine> = {
            let nu_wth = ColNu::new(line_idx, line_number).to_width();
            let wth = coord.wth.saturating_sub(nu_wth);
            let iter = view::wrap_line(buf, line_idx, nu_wth, wth).into_iter();
            iter.take_while(|x| (x.bc + (x.n as usize)) < nbc).collect()
        };

        Ok(WrapIter {
            name: name.to_string(),
            coord,
            buf,
            line_number,
            dir: DP::Left,

            line_idx: decr_line_idx(Some(line_idx)),
            lines,
        })
    }

    fn next_down(&mut self) -> Option<view::ScrLine> {
        match self.lines.pop() {
            Some(line) => {
                debug!("WrapIter-down {}", line);
                Some(line)
            }
            None => {
                let line_idx = self.line_idx?;

                self.lines = {
                    let nu = ColNu::new(line_idx, self.line_number);
                    let wth = self.coord.wth.saturating_sub(nu.to_width());
                    view::wrap_line(self.buf, line_idx, nu.to_width(), wth)
                };
                self.lines.reverse();
                self.line_idx = incr_line_idx(self.buf, self.line_idx);
                let line = self.lines.pop()?;
                debug!("WrapIter-down {}", line);
                Some(line)
            }
        }
    }

    fn next_up(&mut self) -> Option<view::ScrLine> {
        match self.lines.pop() {
            Some(line) => {
                debug!("WrapIter-up {}", line);
                Some(line)
            }
            None => {
                let line_idx = self.line_idx?;

                self.lines = {
                    let nu = ColNu::new(line_idx, self.line_number);
                    let wth = self.coord.wth.saturating_sub(nu.to_width());
                    view::wrap_line(self.buf, line_idx, nu.to_width(), wth)
                };
                self.line_idx = decr_line_idx(self.line_idx);
                let line = self.lines.pop()?;
                debug!("WrapIter-up {}", line);
                Some(line)
            }
        }
    }

    fn take_lines(&mut self, mut n: usize) -> Vec<view::ScrLine> {
        let mut lines = vec![];
        while n > 0 {
            match self.next() {
                Some(line) => lines.push(line),
                None => n = 0,
            }
            n = n.saturating_sub(1);
        }
        lines
    }
}

impl<'a, B> Iterator for WrapIter<'a, B>
where
    B: WinBuffer,
{
    type Item = view::ScrLine;

    fn next(&mut self) -> Option<Self::Item> {
        match self.dir {
            DP::Left => self.next_up(),
            DP::Right => self.next_down(),
            _ => unreachable!(),
        }
    }
}

pub struct NowrapIter<'a, B>
where
    B: WinBuffer,
{
    name: String,
    coord: Coord,
    cursor: Cursor,
    buf: &'a B,
    dir: DP,
    line_number: bool,

    line_idx: Option<usize>,
}

impl<'a, B> fmt::Display for NowrapIter<'a, B>
where
    B: WinBuffer,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(
            f,
            "NowrapIter<{:?} {} {} {:?}>",
            self.name, self.coord, self.dir, self.line_idx,
        )
    }
}

impl<'a, B> NowrapIter<'a, B>
where
    B: WinBuffer,
{
    pub fn new_scroll_down<W>(name: &str, w: &W, buf: &'a B) -> Result<Self>
    where
        W: Window,
    {
        let line_idx = buf.to_xy_cursor(None).row;
        Ok(NowrapIter {
            name: name.to_string(),
            coord: w.to_coord(),
            cursor: w.to_cursor().unwrap_or(Cursor::default()),
            buf,
            dir: DP::Right,
            line_number: w.config_line_number(),

            line_idx: Some(line_idx),
        })
    }

    pub fn new_scroll_up<W>(name: &str, w: &W, buf: &'a B) -> Result<Self>
    where
        W: Window,
    {
        let line_idx = buf.to_xy_cursor(None).row;
        Ok(NowrapIter {
            name: name.to_string(),
            coord: w.to_coord(),
            cursor: w.to_cursor().unwrap_or(Cursor::default()),
            buf,
            dir: DP::Left,
            line_number: w.config_line_number(),

            line_idx: decr_line_idx(Some(line_idx)),
        })
    }

    fn next_down(&mut self) -> Option<view::ScrLine> {
        let line_idx = self.line_idx?;

        let nu_wth = ColNu::new(line_idx, self.line_number).to_width();
        let col = {
            let col = self.cursor.col.saturating_sub(nu_wth) as usize;
            self.buf.to_xy_cursor(None).col.saturating_sub(col)
        };
        let line = {
            let wth = self.coord.wth.saturating_sub(nu_wth);
            view::nowrap_line(self.buf, line_idx, col, nu_wth, wth)
        };
        self.line_idx = incr_line_idx(self.buf, Some(line_idx));
        debug!("NowrapIter-down {}", line);
        Some(line)
    }

    fn next_up(&mut self) -> Option<view::ScrLine> {
        let line_idx = self.line_idx?;

        let nu_wth = ColNu::new(line_idx, self.line_number).to_width();
        let col = {
            let col = self.cursor.col.saturating_sub(nu_wth) as usize;
            self.buf.to_xy_cursor(None).col.saturating_sub(col)
        };
        let line = {
            let nu = ColNu::new(line_idx, self.line_number);
            let wth = self.coord.wth.saturating_sub(nu.to_width());
            view::nowrap_line(self.buf, line_idx, col, nu_wth, wth)
        };
        self.line_idx = decr_line_idx(Some(line_idx));
        debug!("NowrapIter-down {}", line);
        Some(line)
    }

    fn take_lines(&mut self, mut n: usize) -> Vec<view::ScrLine> {
        let mut lines = vec![];
        while n > 0 {
            match self.next() {
                Some(line) => lines.push(line),
                None => n = 0,
            }
            n = n.saturating_sub(1);
        }
        lines
    }
}

impl<'a, B> Iterator for NowrapIter<'a, B>
where
    B: WinBuffer,
{
    type Item = view::ScrLine;

    fn next(&mut self) -> Option<Self::Item> {
        match self.dir {
            DP::Left => self.next_up(),
            DP::Right => self.next_down(),
            _ => unreachable!(),
        }
    }
}

fn incr_line_idx<B>(buf: &B, line_idx: Option<usize>) -> Option<usize>
where
    B: WinBuffer,
{
    let last_line_idx = buf.to_last_line_idx();
    match line_idx {
        Some(line_idx) if line_idx >= last_line_idx => None,
        Some(line_idx) => Some(line_idx.saturating_add(1)),
        None => None,
    }
}

fn decr_line_idx(line_idx: Option<usize>) -> Option<usize> {
    match line_idx {
        Some(0) => None,
        Some(line_idx) => Some(line_idx.saturating_sub(1)),
        None => None,
    }
}
