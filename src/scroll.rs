use std::{convert::TryInto, fmt, result};

use crate::{
    col_nu::ColNu,
    event::DP,
    view,
    window::{Coord, Cursor, WinBuffer, Window},
    Result,
};

pub fn scroll_down<W, B>(name: &str, w: &W, buf: &B, dir: DP, n: usize) -> Result<(Cursor, usize)>
where
    W: Window,
    B: WinBuffer,
{
    let coord = w.to_coord();
    let nbc_xy = buf.to_xy_cursor(None);

    let mut lines = if w.config_wrap() {
        let mut v: view::Wrap = (w, nbc_xy).try_into()?;
        v.shift_cursor(buf);
        v.to_edit_lines(buf)
    } else {
        todo!()
    };

    let iter = match (w.config_wrap(), dir) {
        (true, DP::Right) => WrapIter::new_scroll_down(name, w, buf)?,
        (true, DP::Left) => WrapIter::new_scroll_up(name, w, buf)?,
        (false, DP::Right) => todo!(),
        (false, DP::Left) => todo!(),
        _ => unreachable!(),
    };

    for line in iter.take(n) {
        match coord.hgt as usize {
            n if n > lines.len() => lines.push(line),
            _ => {
                lines.remove(0);
                lines.push(line)
            }
        }
    }

    let cursor = w.to_cursor().unwrap_or(Cursor::default());
    let bc = buf.to_char_cursor();

    Ok((cursor, bc))
}

struct WrapIter<'a, B>
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
    fn new_scroll_down<W>(name: &str, w: &W, buf: &'a B) -> Result<Self>
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
            iter.skip_while(|x| x.bc <= nbc).collect()
        };
        lines.reverse();

        Ok(WrapIter {
            name: name.to_string(),
            coord,
            buf,
            line_number,
            dir: DP::Right,

            line_idx: Some(line_idx),
            lines,
        })
    }

    fn new_scroll_up<W>(name: &str, w: &W, buf: &'a B) -> Result<Self>
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
            let iter = view::wrap_line(buf, nbc_xy.row, nu_wth, wth).into_iter();
            iter.take_while(|x| x.bc <= nbc).collect()
        };
        lines.pop();

        Ok(WrapIter {
            name: name.to_string(),
            coord,
            buf,
            line_number,
            dir: DP::Left,

            line_idx: Some(line_idx),
            lines,
        })
    }

    fn next_down(&mut self) -> Option<view::ScrLine> {
        match self.lines.pop() {
            Some(line) => Some(line),
            None => match self.line_idx {
                Some(line_idx) => {
                    self.lines = {
                        let nu = ColNu::new(line_idx, self.line_number);
                        let wth = self.coord.wth.saturating_sub(nu.to_width());
                        view::wrap_line(self.buf, line_idx, nu.to_width(), wth)
                    };
                    self.lines.reverse();
                    match self.lines.pop() {
                        Some(line) => Some(line),
                        None => {
                            self.line_idx = None;
                            None
                        }
                    }
                }
                None => None,
            },
        }
    }

    fn next_up(&mut self) -> Option<view::ScrLine> {
        match self.lines.pop() {
            Some(line) => Some(line),
            None => match self.line_idx {
                Some(line_idx) => {
                    self.lines = {
                        let nu = ColNu::new(line_idx, self.line_number);
                        let wth = self.coord.wth.saturating_sub(nu.to_width());
                        view::wrap_line(self.buf, line_idx, nu.to_width(), wth)
                    };
                    match self.lines.pop() {
                        Some(line) => Some(line),
                        None => {
                            self.line_idx = None;
                            None
                        }
                    }
                }
                None => None,
            },
        }
    }
}

impl<'a, B> Iterator for WrapIter<'a, B>
where
    B: WinBuffer,
{
    type Item = view::ScrLine;

    fn next(&mut self) -> Option<Self::Item> {
        match self.dir {
            DP::Right => self.next_down(),
            DP::Left => self.next_up(),
            _ => unreachable!(),
        }
    }
}
