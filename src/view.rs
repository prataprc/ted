use crossterm::queue;
use log::trace;

use std::{
    cmp,
    io::{self, Write},
    iter::FromIterator,
};

use crate::{
    buffer::{self, Buffer, NL},
    event::DP,
    window::{Coord, Cursor, Span, State},
    wrap_view::WrapView,
    Error, Result,
};

#[derive(Clone, Copy)]
pub struct Wrap {
    coord: Coord,   // full coordinate
    cursor: Cursor, // within full coordinate
    obc_xy: buffer::Cursor,
}

impl Wrap {
    pub fn new(coord: Coord, cursor: Cursor, obc_xy: buffer::Cursor) -> Wrap {
        let mut v = Wrap {
            coord,
            cursor,
            obc_xy,
        };

        let nu_wth = compute_nu_width(obc_xy.row);
        v.coord = {
            let (hgt, wth) = v.coord.to_size();
            v.coord.resize_to(hgt, wth - nu_wth)
        };
        v.cursor = v.cursor.move_by(-((nu_wth + 1) as i16), 0);

        v
    }

    pub fn render(self, s: &State, buf: &Buffer) -> Result<Cursor> {
        let (v, nu_wth) = self.shift(s, buf);
        let cursor = v.cursor;
        v.refresh(nu_wth, s, buf)?;
        Ok(cursor)
    }

    fn shift_after(&self, buf: &Buffer) -> usize {
        let nbc_xy = buf.to_xy_cursor();

        let mut n = 0;
        for row in self.obc_xy.row..=nbc_xy.row {
            n += match row {
                row if row == self.obc_xy.row => {
                    let m = self.coord.to_cells(buf.line_len(row));
                    m.saturating_sub(self.obc_xy.col)
                }
                row if row == nbc_xy.row => nbc_xy.col,
                row => self.coord.to_cells(buf.line_len(row)),
            }
        }
        n
    }

    fn shift_before(&self, buf: &Buffer) -> usize {
        let nbc_xy = buf.to_xy_cursor();

        let mut n = 0;
        for row in (nbc_xy.row..=self.obc_xy.row).rev() {
            n += match row {
                row if row == self.obc_xy.row => self.obc_xy.col,
                row if row == nbc_xy.row => {
                    let m = self.coord.to_cells(buf.line_len(row));
                    m.saturating_sub(nbc_xy.col)
                }
                row => self.coord.to_cells(buf.line_len(row)),
            }
        }
        n
    }

    fn shift(mut self, s: &State, buf: &Buffer) -> (Self, u16) {
        use crate::event::DP::{Left, Right};
        use std::cmp::Ordering::{Equal, Greater, Less};

        self = {
            let nu_wth = compute_nu_width(self.obc_xy.row);
            let (hgt, wth) = self.coord.to_size();
            let coord = self.coord.resize_to(hgt, wth - nu_wth);
            let cursor = self.cursor.move_by(-((nu_wth + 1) as i16), 0);
            Wrap {
                coord,
                cursor,
                obc_xy: self.obc_xy,
            }
        };

        let scroll_off = s.as_ref().scroll_off; // accounting for scroll-offset.
        let nbc_xy = buf.to_xy_cursor();

        // create possible cursor positions.
        let mut cursors: Vec<Cursor> = if nbc_xy < self.obc_xy {
            let iter = self.cursor.prev_cursors(self.coord).into_iter().rev();
            iter.skip((scroll_off * self.coord.wth) as usize)
                .rev()
                .collect()
        } else {
            let iter = self.cursor.next_cursors(self.coord).into_iter().rev();
            iter.skip((scroll_off * self.coord.wth) as usize)
                .rev()
                .collect()
        };
        // compute the number of cells to drain and its direction.
        let same_row = nbc_xy.row == self.obc_xy.row;
        let (m, dp) = match nbc_xy.cmp(&self.obc_xy) {
            Equal => return (self, compute_nu_width(self.obc_xy.row)),
            Greater if same_row => (nbc_xy.col - self.obc_xy.col, Right),
            Greater => (self.shift_after(buf), Right),
            Less if same_row => (self.obc_xy.col - nbc_xy.col, Left),
            Less => (self.shift_before(buf), Left),
        };
        cursors.drain(..cmp::min(m, cursors.len()));
        // compute cursor.
        let new_coord = {
            let old_nu_wth = compute_nu_width(self.obc_xy.row);
            let new_nu_wth = compute_nu_width(nbc_xy.row);
            let (hgt, wth) = self.coord.to_size();
            self.coord.resize_to(hgt, wth + old_nu_wth - new_nu_wth)
        };
        let (mut v, nu_wth) = match (cursors.pop(), dp) {
            (Some(cursor), _) => {
                self.cursor = cursor;
                (self, compute_nu_width(self.obc_xy.row))
            }
            (None, DP::Left) => {
                self.coord = new_coord;
                self.cursor = Cursor {
                    row: scroll_off,
                    col: (nbc_xy.col % (self.coord.wth as usize)) as u16,
                };
                (self, compute_nu_width(nbc_xy.row))
            }
            (None, DP::Right) => {
                self.coord = new_coord;
                self.cursor = Cursor {
                    row: self.coord.hgt.saturating_sub(scroll_off + 1),
                    col: (nbc_xy.col % (self.coord.wth as usize)) as u16,
                };
                (self, compute_nu_width(nbc_xy.row))
            }
            _ => unreachable!(),
        };

        v = {
            let (hgt, wth) = v.coord.to_size();
            let coord = v.coord.resize_to(hgt, wth + nu_wth);
            let cursor = v.cursor.move_by((nu_wth + 1) as i16, 0);
            Wrap {
                coord,
                cursor,
                obc_xy: v.obc_xy,
            }
        };

        trace!("wrap-shift new_cursor:{}, nu_wth:{}", v.cursor, nu_wth);
        (v, nu_wth)
    }

    fn refresh(self, nu_wth: u16, s: &State, buf: &Buffer) -> Result<()> {
        use std::iter::repeat;

        let nbc_xy = buf.to_xy_cursor();
        let line_idx = nbc_xy.row.saturating_sub(self.cursor.row as usize);
        trace!(
            "nu:{} {} {}@{} line_idx:{}",
            nu_wth,
            nbc_xy,
            self.cursor,
            self.coord,
            line_idx,
        );

        let mut stdout = io::stdout();

        let (col, mut row) = self.coord.to_origin_cursor();
        let max_row = row + self.coord.hgt;
        let line_number = s.as_ref().line_number;
        let (edit_coord, edit_cursor) = {
            let (hgt, wth) = self.coord.to_size();
            let coord = self.coord.resize_to(hgt, wth - nu_wth);
            let cursor = self.cursor.move_by(-((nu_wth + 1) as i16), 0);
            (coord, cursor)
        };

        let mut wv = WrapView::new(line_idx, edit_coord, buf)?;
        wv.align(buf.to_cursor(), edit_cursor);

        let s_nu_blank = String::from_iter(repeat(' ').take(nu_wth as usize));
        'a: for line in wv.lines.iter() {
            let s_nu = line.nu.to_string();
            for (r, rline) in line.rows.iter().enumerate() {
                let s = match r {
                    0 => s_nu.clone(),
                    _ => s_nu_blank.clone(),
                };
                let s = if line_number {
                    format!("{:>width$} ", s, width = (nu_wth as usize))
                } else {
                    "".to_string()
                };
                err_at!(Fatal, queue!(stdout, span!((col, row), st: s)))?;

                let bcs: Vec<usize> = {
                    let iter = rline.cells.iter().filter_map(|c| c.bc);
                    iter.collect()
                };
                let s = match (bcs.first(), bcs.last()) {
                    (Some(fbc), Some(ebc)) => {
                        let iter = buf.chars_at(*fbc, DP::Right)?;
                        let chs: Vec<char> = iter.take(*ebc - *fbc + 1).collect();
                        String::from_iter(chs)
                    }
                    _ => "".to_string(),
                };
                err_at!(Fatal, queue!(stdout, span!(st: s)))?;

                row += 1;
                if row >= max_row {
                    break 'a;
                }
            }
        }
        empty_lines(tail_line(row, self.coord, buf)?, self.coord, s)
    }
}

#[derive(Clone, Copy)]
pub struct NoWrap {
    coord: Coord,   // full coordinate
    cursor: Cursor, // within full coordinate
    obc_xy: buffer::Cursor,
}

impl NoWrap {
    pub fn new(coord: Coord, cursor: Cursor, obc_xy: buffer::Cursor) -> NoWrap {
        let mut v = NoWrap {
            coord,
            cursor,
            obc_xy,
        };

        let nu_wth = compute_nu_width(obc_xy.row);
        v.coord = {
            let (hgt, wth) = v.coord.to_size();
            v.coord.resize_to(hgt, wth - nu_wth)
        };
        v.cursor = v.cursor.move_by(-((nu_wth + 1) as i16), 0);

        v
    }

    pub fn render(self, s: &State, buf: &Buffer) -> Result<Cursor> {
        let (v, nu_wth) = self.shift(s, buf);
        let cursor = v.cursor;
        v.refresh(nu_wth, s, buf)?;
        Ok(cursor)
    }

    fn shift(self, s: &State, buf: &Buffer) -> (Self, u16) {
        let scroll_off = s.as_ref().scroll_off; // accounting for scroll-offset.
        let nbc_xy = buf.to_xy_cursor();

        let (diff_col, diff_row) = self.obc_xy.diff(&nbc_xy);
        let (hgt, wth) = self.coord.to_size();
        let Cursor { row, col } = self.cursor;

        let (r_min, r_max): (isize, isize) = if hgt < (scroll_off * 2) {
            (0, (hgt.saturating_sub(1) as isize))
        } else {
            (
                scroll_off as isize,
                (hgt.saturating_sub(scroll_off + 1) as isize),
            )
        };

        let (coord, nu_wth) = {
            let row = (row as isize) + diff_row;
            if row < r_min || row > r_max {
                let coord = {
                    let old_nu_wth = compute_nu_width(self.obc_xy.row);
                    let new_nu_wth = compute_nu_width(nbc_xy.row);
                    self.coord.resize_to(hgt, wth + old_nu_wth - new_nu_wth)
                };
                (coord, compute_nu_width(nbc_xy.row))
            } else {
                (self.coord, compute_nu_width(self.obc_xy.row))
            }
        };

        let new_row: u16 = {
            let row = limit!((row as isize) + diff_row, r_min, r_max);
            assert!(row < (hgt as isize));
            row as u16
        };
        let new_col: u16 = {
            let col = limite!((col as isize) + diff_col, 0, wth as isize);
            assert!(col < (wth as isize));
            col as u16
        };

        let cursor = Cursor {
            col: new_col,
            row: new_row,
        };

        let v = {
            let (hgt, wth) = coord.to_size();
            let coord = coord.resize_to(hgt, wth + nu_wth);
            let cursor = cursor.move_by((nu_wth + 1) as i16, 0);
            NoWrap {
                coord,
                cursor,
                obc_xy: self.obc_xy,
            }
        };

        (v, nu_wth)
    }

    fn refresh(self, nu_wth: u16, s: &State, buf: &Buffer) -> Result<()> {
        use std::iter::repeat;

        let nbc_xy = buf.to_xy_cursor();
        trace!("nu:{} {} {}@{}", nu_wth, nbc_xy, self.cursor, self.coord);

        let mut stdout = io::stdout();

        let (col, mut row) = self.coord.to_origin_cursor();
        let (hgt, wth) = self.coord.to_size();
        let edit_cursor = self.cursor.move_by(-(nu_wth as i16 + 1), 0);

        let do_padding = |line: ropey::RopeSlice| -> Vec<char> {
            // trace!("l {} {} {:?}", nbc_xy.0, new_cursor.col, line.to_string());
            line.chars_at(nbc_xy.col - (edit_cursor.col as usize))
                .chain(repeat(' '))
                .take((wth - nu_wth) as usize)
                .collect()
        };

        let from = nbc_xy.row.saturating_sub(edit_cursor.row as usize);
        let lines = buf.lines_at(from, DP::Right)?.map(do_padding);
        for (i, line) in lines.take(hgt as usize).enumerate() {
            let mut st: String = if_else!(
                s.as_ref().line_number,
                format!("{:>width$} ", from + i + 1, width = (nu_wth as usize)),
                Default::default()
            );
            let s_line = String::from_iter(line);
            // trace!("bufline col:{} row:{} line:{:?}", col, row, s_line);
            st.push_str(&s_line);
            err_at!(Fatal, queue!(stdout, span!((col, row), st: st)))?;
            row += 1;
        }

        empty_lines(tail_line(row, self.coord, buf)?, self.coord, s)
    }
}

fn empty_lines(mut row: u16, coord: Coord, s: &State) -> Result<()> {
    use std::iter::repeat;

    let mut stdout = io::stdout();
    let (col, _) = coord.to_origin_cursor();
    let (hgt, wth) = coord.to_size();

    if row < hgt {
        trace!("empty lines {} {}", row, hgt);
        for _ in row..hgt {
            let mut st: String = if_else!(
                s.as_ref().line_number,
                format!("{} ", '~'),
                Default::default()
            );
            st.push_str(&{
                let iter = repeat(' ').take((wth - 2) as usize);
                String::from_iter(iter)
            });
            // trace!("empline col:{} row:{} line:{:?}", col, row, st.len());
            err_at!(Fatal, queue!(stdout, span!((col, row), st: st)))?;
            row += 1;
        }
    }
    assert!(row == hgt);

    Ok(())
}

fn tail_line(row: u16, coord: Coord, buf: &Buffer) -> Result<u16> {
    let n = buf.len_chars();
    let (col, _) = coord.to_origin_cursor();
    let ok1 = n == 0;
    let ok2 = (row == coord.hgt - 1) && buf.char(n - 1) == NL;

    let mut stdout = io::stdout();

    if ok1 || ok2 {
        let line_idx = if ok1 { 1 } else { buf.char_to_line(n - 1) + 1 };
        let nu_wth = compute_nu_width(line_idx);
        let st = format!("{:>width$} ", line_idx, width = (nu_wth as usize));
        err_at!(Fatal, queue!(stdout, span!((col, row), st: st)))?;
        Ok(row + 1)
    } else {
        Ok(row)
    }
}

fn compute_nu_width(line_idx: usize) -> u16 {
    use crate::buffer::MAX_LINES;

    assert!(line_idx < MAX_LINES);
    (cmp::max(line_idx.to_string().len(), 2) + 1) as u16
}
