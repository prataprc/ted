use std::{cmp, fmt, result};

use crate::{
    buffer::{self, Buffer},
    col_nu::ColNu,
    event::{Scroll, DP},
    view,
    window::{Coord, Cursor, WinBuffer, Window},
    Error, Result,
};

#[derive(Clone)]
pub struct Wrap {
    name: String,
    coord: Coord,
    cursor: Cursor,
    scroll_off: u16,
    line_number: bool,
    screen_lines: Vec<view::ScrLine>,
}

impl fmt::Display for Wrap {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(
            f,
            "Wrap<{:?} {}@{} {}>",
            self.name,
            self.cursor,
            self.coord,
            self.screen_lines.len()
        )
    }
}

impl Wrap {
    fn new<W>(w: &W) -> Result<Self>
    where
        W: Window,
    {
        let cursor = {
            let e = Error::Invalid(String::default(), "no-cursor".to_string());
            err_at!(w.to_cursor().ok_or(e))?
        };
        let scroll_off = w.config_scroll_offset();
        let line_number = w.config_line_number();
        Ok(Wrap {
            name: w.to_name(),
            coord: w.to_coord(),
            cursor,
            scroll_off,
            line_number,
            screen_lines: Vec::default(),
        })
    }
}

impl Wrap {
    //pub fn scroll(&self, n: usize, scroll: Scroll, dp: DP) {
    //    match scroll {
    //        Scroll::Ones => self.scroll_up(n, scroll, dp),
    //    }
    //}

    pub fn scroll_down(&self, buf: &Buffer, n: usize, scroll: Scroll, dp: DP) -> (Cursor, usize) {
        let obc_xy = buf.to_xy_cursor(None);
        let obc = buf.to_char_cursor();
        let cursor = self.cursor;
        let last_idx = buf.to_last_line_idx();

        let lines: Vec<usize> = {
            let from = cursor.row.saturating_sub(self.coord.hgt) as usize;
            let to = cmp::min(last_idx, obc_xy.row.saturating_add(n));
            (from..to).collect()
        };

        let mut nu = ColNu::new(obc_xy.row, self.line_number);
        let mut wth = self.coord.wth.saturating_sub(nu.to_width());
        let f = |sl: ScrLine| ColNu -> ColNu::new(sl.line_idx, self.line_number);
        let screen_lines: Vec<view::ScrLine> = loop {
            let slines = view::wrap_lines(buf, lines, wth);
            let pivot = view::cursor_line(&slines, obc).unwrap_or(0);
            let from = {
                let from = pivot.saturating_sub(cursor.row as usize);
                cmp::min(last_idx, from.saturating_add(n))
            };
            let screen_lines = {
                let iter = slines.into_iter();
                iter.skip(from).take(self.coord.hgt as usize).collect();
            }
            let nu = match screen_lines.first().map(f) {
                Some(nnu) if nnu.to_width() > nu.to_width() => {
                    nnu.to_width() + nu.to_width() / 2
                }
                _ => break screen_lines,
            }
        };
        let scroll_off = self.scroll_off as usize;
        let row = match view::cursor_line(&screen_lines, obc) {
            Some(pivot) => cmp::min(pivot.saturating_sub(n), scroll_off),
            None => scroll_off,
        };
        let col = if cursor.col < screen_lines[row].n {
            cursor.col
        } else {
            screen_lines[row].n.saturating_sub(1)
        };
        let cursor = Cursor {
            col,
            row: row as u16,
        };
        (cursor, screen_lines[row].bc + (col as usize))
    }

    pub fn scroll_up(&self, buf: &Buffer, n: usize, scroll: Scroll, dp: DP) -> (Cursor, usize) {
        let obc_xy = buf.to_xy_cursor(None);
        let obc = buf.to_char_cursor();
        let wth = {
            let nu = ColNu::new(obc_xy.row, self.line_number);
            self.coord.wth.saturating_sub(nu.to_width())
        };
        let cursor = self.cursor;
        let last_idx = buf.to_last_line_idx();

        let lines: Vec<usize> = {
            let from = cursor.row.saturating_sub(self.coord.hgt) as usize;
            let to = cmp::min(last_idx, obc_xy.row.saturating_add(n));
            (from..to).collect()
        };
        let screen_lines: Vec<view::ScrLine> = {
            let slines = view::wrap_lines(buf, lines, wth);
            let pivot = view::cursor_line(&slines, obc).unwrap_or(0);
            let from = {
                let from = pivot.saturating_sub(cursor.row as usize);
                cmp::min(last_idx, from.saturating_add(n))
            };
            let iter = slines.into_iter();
            iter.skip(from).take(self.coord.hgt as usize).collect()
        };
        let scroll_off = self.scroll_off as usize;
        let row = match view::cursor_line(&screen_lines, obc) {
            Some(pivot) => cmp::min(pivot.saturating_sub(n), scroll_off),
            None => scroll_off,
        };
        let col = if cursor.col < screen_lines[row].n {
            cursor.col
        } else {
            screen_lines[row].n.saturating_sub(1)
        };
        let cursor = Cursor {
            col,
            row: row as u16,
        };
        (cursor, screen_lines[row].bc + (col as usize))
    }
}
