use crossterm::queue;
use log::trace;

use std::{
    cmp, fmt,
    io::{self, Write},
    iter::FromIterator,
    result,
};

use crate::{
    buffer::{self, Buffer, NL},
    code::col_nu::ColNu,
    event::DP,
    window::{Coord, Cursor, Span},
    Error, Result,
};

#[derive(Clone, Copy)]
pub struct Wrap {
    coord: Coord,
    cursor: Cursor,
    obc_xy: buffer::Cursor,
    nu: ColNu,
    scroll_off: u16,
    line_number: bool,
}

impl fmt::Display for Wrap {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "Wrap<{},{},{}>", self.coord, self.cursor, self.nu)
    }
}

impl Wrap {
    pub fn initial_cursor(line_number: bool) -> Cursor {
        let nbc_xy: buffer::Cursor = Default::default();
        let col = if line_number {
            ColNu::new(nbc_xy.row).to_width()
        } else {
            0
        };
        Cursor { row: 0, col }
    }

    // create a wrap view using previous cursor's nu_width.
    pub fn new(coord: Coord, cursor: Cursor, obc_xy: buffer::Cursor) -> Wrap {
        Wrap {
            coord,
            cursor,
            obc_xy,
            nu: ColNu::new(obc_xy.row),
            scroll_off: 0,
            line_number: false,
        }
    }

    pub fn set_scroll_off(&mut self, scroll_off: u16) -> &mut Self {
        self.scroll_off = scroll_off;
        self
    }

    pub fn set_line_number(&mut self, line_number: bool) -> &mut Self {
        self.line_number = line_number;
        self
    }

    pub fn render(mut self, buf: &Buffer) -> Result<Cursor> {
        if self.line_number {
            self.exclude_nu(self.nu.to_width())
        }
        self = self.shift(buf);
        let cursor = self.cursor;
        self.refresh(buf)?;
        Ok(cursor)
    }

    // number of cells to move forward.
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

    // number of cells to move backward.
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

    fn shift(mut self, buf: &Buffer) -> Self {
        use crate::event::DP::{Left, Right};
        use std::cmp::Ordering::{Equal, Greater, Less};

        let scroll_off = self.scroll_off; // accounting for scroll-offset.
        let nbc_xy = buf.to_xy_cursor();

        // create possible cursor positions.
        let mut cursors: Vec<Cursor> = {
            let iter = if nbc_xy < self.obc_xy {
                self.cursor.prev_cursors(self.coord).into_iter().rev()
            } else {
                self.cursor.next_cursors(self.coord).into_iter().rev()
            };
            iter.skip((scroll_off * self.coord.wth) as usize)
                .rev()
                .collect()
        };
        // compute the number of cells to drain and its direction.
        let same_row = nbc_xy.row == self.obc_xy.row;
        let (m, dp) = match nbc_xy.cmp(&self.obc_xy) {
            Equal => return self,
            Greater if same_row => (nbc_xy.col - self.obc_xy.col, Right),
            Greater => (self.shift_after(buf), Right),
            Less if same_row => (self.obc_xy.col - nbc_xy.col, Left),
            Less => (self.shift_before(buf), Left),
        };
        cursors.drain(..cmp::min(m, cursors.len()));
        // compute cursor.
        self = match (cursors.pop(), dp) {
            (Some(cursor), _) => {
                trace!("{}->{}", self.cursor, cursor);
                self.cursor = cursor;
                self
            }
            (None, DP::Left) => self.into_resized(nbc_xy, scroll_off, Left),
            (None, DP::Right) => self.into_resized(nbc_xy, scroll_off, Right),
            _ => unreachable!(),
        };

        self
    }

    fn refresh(self, buf: &Buffer) -> Result<()> {
        let nbc_xy = buf.to_xy_cursor();
        let line_idx = nbc_xy.row.saturating_sub(self.cursor.row as usize);
        trace!(
            "wrap-refresh {} {} {}@{} line_idx:{}",
            self.nu,
            nbc_xy,
            self.cursor,
            self.coord,
            line_idx,
        );

        let mut stdout = io::stdout();

        let full_coord = self.outer_coord();
        let (col, mut row) = full_coord.to_origin_cursor();
        let max_row = row + full_coord.hgt;
        let line_number = self.line_number;

        let mut wv = WrapView::new(line_idx, self.coord, buf)?;
        wv.align(buf.to_cursor(), self.cursor);

        'a: for line in wv.lines.iter() {
            for (r, rline) in line.rows.iter().enumerate() {
                let mut nu_span = match r {
                    0 if line_number => self.nu.to_span(Some(line.nu)),
                    _ if line_number => self.nu.to_span(None),
                    _ => span!(st: "".to_string()),
                };
                nu_span.set_cursor(Cursor { col, row });

                let bcs: Vec<usize> = {
                    let iter = rline.cells.iter().filter_map(|c| c.bc);
                    iter.collect()
                };
                let line_span = match (bcs.first(), bcs.last()) {
                    (Some(fbc), Some(ebc)) => {
                        let iter = buf.chars_at(*fbc, DP::Right)?;
                        let chs: Vec<char> = iter.take(*ebc - *fbc + 1).collect();
                        span!(st: String::from_iter(chs))
                    }
                    _ => span!(st: "".to_string()),
                };
                err_at!(Fatal, queue!(stdout, nu_span, line_span))?;

                row += 1;
                if row >= max_row {
                    break 'a;
                }
            }
        }
        empty_lines(
            tail_line(row, max_row, &self.nu, buf)?,
            max_row,
            full_coord,
            self.line_number,
        )
    }

    fn exclude_nu(&mut self, nu_wth: u16) {
        self.coord = {
            let (hgt, wth) = self.coord.to_size();
            self.coord.resize_to(hgt, wth - nu_wth)
        };
        self.cursor = self.cursor.move_by(-(nu_wth as i16), 0);
    }

    fn into_resized(self, nbc_xy: buffer::Cursor, so: u16, dp: DP) -> Self {
        let nu = ColNu::new(nbc_xy.row);
        let old_wth = ColNu::new(self.obc_xy.row).to_width();
        let new_wth = nu.to_width();
        let coord = {
            let (hgt, wth) = self.coord.to_size();
            self.coord.resize_to(hgt, wth + old_wth - new_wth)
        };
        let cursor = match dp {
            DP::Left => Cursor {
                row: so,
                col: (nbc_xy.col % (coord.wth as usize)) as u16,
            },
            DP::Right => Cursor {
                row: coord.hgt.saturating_sub(so + 1),
                col: (nbc_xy.col % (coord.wth as usize)) as u16,
            },
            _ => unreachable!(),
        };

        trace!("{}->{} {}->{}", self.coord, coord, self.cursor, cursor);
        Wrap {
            coord: coord,
            cursor: cursor,
            obc_xy: self.obc_xy,
            nu,
            scroll_off: self.scroll_off,
            line_number: self.line_number,
        }
    }

    fn outer_coord(&self) -> Coord {
        let (hgt, wth) = self.coord.to_size();
        self.coord.resize_to(hgt, wth + self.nu.to_width())
    }
}

#[derive(Clone, Copy)]
pub struct NoWrap {
    coord: Coord,   // full coordinate
    cursor: Cursor, // within full coordinate
    obc_xy: buffer::Cursor,
    nu: ColNu,
    scroll_off: u16,
    line_number: bool,
}

impl fmt::Display for NoWrap {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "NoWrap<{},{},{}>", self.coord, self.cursor, self.nu)
    }
}

impl NoWrap {
    pub fn initial_cursor(line_number: bool) -> Cursor {
        let nbc_xy: buffer::Cursor = Default::default();
        let col = if line_number {
            ColNu::new(nbc_xy.row).to_width()
        } else {
            0
        };
        Cursor { row: 0, col }
    }

    pub fn new(coord: Coord, cursor: Cursor, obc_xy: buffer::Cursor) -> NoWrap {
        NoWrap {
            coord,
            cursor,
            obc_xy,
            nu: ColNu::new(obc_xy.row),
            scroll_off: 0,
            line_number: false,
        }
    }

    pub fn set_scroll_off(&mut self, scroll_off: u16) -> &mut Self {
        self.scroll_off = scroll_off;
        self
    }

    pub fn set_line_number(&mut self, line_number: bool) -> &mut Self {
        self.line_number = line_number;
        self
    }

    pub fn render(mut self, buf: &Buffer) -> Result<Cursor> {
        if self.line_number {
            self.exclude_nu(self.nu.to_width())
        }
        self = self.shift(buf);
        let cursor = self.cursor;
        self.refresh(buf)?;
        Ok(cursor)
    }

    fn shift(self, buf: &Buffer) -> Self {
        let scroll_off = self.scroll_off; // accounting for scroll-offset.

        let (r_min, r_max) = if self.coord.hgt < (scroll_off * 2) {
            (0, (self.coord.hgt.saturating_sub(1) as isize))
        } else {
            (
                scroll_off as isize,
                (self.coord.hgt.saturating_sub(scroll_off + 1) as isize),
            )
        };

        let nbc_xy = buf.to_xy_cursor();
        let (diff_col, diff_row) = self.obc_xy.diff(&nbc_xy);
        let Cursor { row, col } = self.cursor;

        let (coord, nu) = {
            let row = (row as isize) + diff_row;
            if row < r_min || row > r_max {
                let nu = ColNu::new(nbc_xy.row);
                let coord = self.coord.resize_to(
                    self.coord.hgt,
                    self.coord.wth + self.nu.to_width() - nu.to_width(),
                );
                (coord, nu)
            } else {
                (self.coord, self.nu)
            }
        };

        let new_row: u16 = {
            let row = limit!((row as isize) + diff_row, r_min, r_max);
            assert!(row < (coord.hgt as isize), "assert {} {}", row, coord.hgt);
            row as u16
        };
        let new_col: u16 = {
            let col = limite!((col as isize) + diff_col, 0, coord.wth as isize);
            assert!(col < (coord.wth as isize), "assert {} {}", col, coord.wth);
            col as u16
        };
        let cursor = Cursor {
            col: new_col,
            row: new_row,
        };

        trace!("{}->{} {}->{}", self.coord, coord, self.cursor, cursor);
        NoWrap {
            coord,
            cursor,
            obc_xy: self.obc_xy,
            nu,
            scroll_off: self.scroll_off,
            line_number: self.line_number,
        }
    }

    fn refresh(self, buf: &Buffer) -> Result<()> {
        use std::iter::repeat;

        let nbc_xy = buf.to_xy_cursor();
        trace!(
            "nowrap-refresh {} {} {}@{}",
            self.nu,
            nbc_xy,
            self.cursor,
            self.coord
        );

        let mut stdout = io::stdout();

        let full_coord = self.outer_coord();
        let (col, mut row) = full_coord.to_origin_cursor();
        let (hgt, wth) = self.coord.to_size();
        let nu_wth = self.nu.to_width();

        let do_padding = |line: ropey::RopeSlice| -> Vec<char> {
            line.chars_at(nbc_xy.col - (self.cursor.col as usize))
                .chain(repeat(' '))
                .take((wth - nu_wth) as usize)
                .collect()
        };

        let from = nbc_xy.row.saturating_sub(self.cursor.row as usize);
        let lines = buf.lines_at(from, DP::Right)?.map(do_padding);
        for (i, line) in lines.take(hgt as usize).enumerate() {
            let mut nu_span = self.nu.to_span(Some(from + i + 1));
            nu_span.set_cursor(Cursor { col, row });
            let line_span = span!(st: String::from_iter(line));
            err_at!(Fatal, queue!(stdout, nu_span, line_span))?;
            row += 1;
        }

        empty_lines(
            tail_line(row, row + hgt, &self.nu, buf)?,
            row + hgt,
            full_coord,
            self.line_number,
        )
    }

    fn outer_coord(&self) -> Coord {
        let (hgt, wth) = self.coord.to_size();
        self.coord.resize_to(hgt, wth + self.nu.to_width())
    }

    fn exclude_nu(&mut self, nu_wth: u16) {
        self.coord = {
            let (hgt, wth) = self.coord.to_size();
            self.coord.resize_to(hgt, wth - nu_wth)
        };
        self.cursor = self.cursor.move_by(-(nu_wth as i16), 0);
    }
}

fn empty_lines(mut row: u16, max_row: u16, coord: Coord, nu: bool) -> Result<()> {
    use std::iter::repeat;

    let mut stdout = io::stdout();
    let (col, _) = coord.to_origin_cursor();
    let (_, wth) = coord.to_size();

    if row < max_row {
        trace!("empty lines {}..{}", row, max_row);
        for _ in row..max_row {
            let mut st: String = if_else!(
                //
                nu,
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
    assert!(row == max_row, "assert {} {}", row, max_row);

    Ok(())
}

fn tail_line(row: u16, max_row: u16, nu: &ColNu, buf: &Buffer) -> Result<u16> {
    let n = buf.n_chars();
    let ok1 = n == 0;
    let ok2 = max_row > 0 && ((row + 1) == max_row) && buf.is_trailing_newline();

    let mut stdout = io::stdout();

    if ok1 || ok2 {
        let span = {
            let line_idx = if ok1 { 1 } else { buf.char_to_line(n - 1) + 1 };
            nu.to_span(Some(line_idx))
        };
        err_at!(Fatal, queue!(stdout, span))?;
        Ok(row + 1)
    } else {
        Ok(row)
    }
}

enum VShift {
    Left(usize),
    Right(usize),
    Skip,
    Done,
}

struct WrapView {
    lines: Vec<Line>,
}

impl WrapView {
    fn new(line_idx: usize, coord: Coord, buf: &Buffer) -> Result<WrapView> {
        let mut lines = vec![];
        let iter = (line_idx..).take(coord.hgt as usize).enumerate();
        for (row, line_idx) in iter {
            assert!(row < 1_000, "assert {}", row); // TODO: avoid magic number
            Line::new_line(line_idx, row as u16, coord.wth, buf)
                //
                .map(|line| lines.push(line));
        }
        Ok(WrapView { lines })
    }

    fn align(&mut self, bc: usize, cursor: Cursor) {
        loop {
            match self.do_align(bc, cursor) {
                VShift::Left(_) => {
                    let line = self.lines.remove(0);
                    line.drop_row().map(|line| self.lines.push(line));
                }
                VShift::Right(_) => unreachable!(),
                VShift::Skip => (),
                VShift::Done => break,
            }
        }
    }

    fn do_align(&self, bc: usize, cursor: Cursor) -> VShift {
        for line in self.lines.iter() {
            match line.align(bc, cursor) {
                VShift::Left(n) => return VShift::Left(n),
                VShift::Right(_) => unreachable!(),
                VShift::Skip => (),
                VShift::Done => return VShift::Done,
            }
        }
        VShift::Done
    }
}

struct Line {
    nu: usize,
    rows: Vec<Row>,
}

impl Line {
    fn new_line(line_idx: usize, row: u16, wth: u16, buf: &Buffer) -> Option<Self> {
        use std::iter::repeat;

        let len_chars = match buf.n_lines() {
            rows if line_idx == rows => Some(0),
            rows if line_idx < rows => Some(buf.line_len(line_idx)),
            _ => None,
        }?;
        let bc = buf.char_to_line(buf.to_cursor());

        let mut rows: Vec<(u16, usize, u16, u16)> = {
            let iter = repeat(wth).take(len_chars / (wth as usize));
            iter.enumerate()
                .map(|(r, wth)| {
                    assert!(r < 1_000, "assert {}", r); // TODO no magi number
                    (row + (r as u16), bc + (r * (wth as usize)), wth, wth)
                })
                .collect()
        };

        if (len_chars % (wth as usize)) > 0 {
            let r = rows.len();
            let w = len_chars % (wth as usize);
            assert!(w <= (wth as usize), "assert {} {}", w, wth);
            assert!(r < 1_000, "assert {}", r); // TODO avoid magic number
            let row = row + (r as u16);
            rows.push((row, bc + (r * (wth as usize)), w as u16, wth))
        }

        let rows: Vec<Row> = {
            let i1 = rows.into_iter();
            let i2 = i1.map(|(row, bc, ln, wth)| Row::new_row(row, bc, ln, wth));
            i2.collect()
        };

        Some(Line {
            nu: line_idx + 1,
            rows,
        })
    }

    fn align(&self, bc: usize, cursor: Cursor) -> VShift {
        for row in self.rows.iter() {
            match row.align(bc, cursor) {
                shift @ VShift::Left(_) => return shift,
                shift @ VShift::Right(_) => return shift,
                VShift::Skip => (),
                VShift::Done => return VShift::Done,
            }
        }
        VShift::Done
    }

    fn drop_row(mut self) -> Option<Self> {
        match self.rows.len() {
            0 => None,
            1 => None,
            _ => {
                self.rows.remove(0);
                self.rows.iter_mut().for_each(|r| r.pull_row());
                Some(self)
            }
        }
    }
}

struct Row {
    cells: Vec<Cell>,
}

impl Row {
    fn new_row(row: u16, bc: usize, ln: u16, wth: u16) -> Row {
        use std::iter::repeat;

        let mut bcs: Vec<Option<usize>> = {
            let bc_end = bc + (ln as usize);
            let iter = (bc..bc_end).into_iter().map(|bc| Some(bc));
            iter.collect()
        };
        assert!(bcs.len() < 10_000, "assert {}", bcs.len()); // TODO no magic
        bcs.extend(&{
            let n = wth.saturating_sub(bcs.len() as u16) as usize;
            let pad: Vec<Option<usize>> = repeat(None).take(n).collect();
            pad
        });

        let cells = {
            let iter = bcs.into_iter().zip((0..wth).into_iter());
            iter.map(|(bc, col)| Cell { bc, col, row }).collect()
        };
        Row { cells }
    }

    fn align(&self, bc: usize, cursor: Cursor) -> VShift {
        use std::cmp::Ordering::{Equal, Greater, Less};

        let mut cells: Vec<&Cell> = self
            .cells
            .iter()
            .take_while(|cell| {
                let ok = cell.row < cursor.row;
                ok || (cell.row == cursor.row) && (cell.col <= cursor.col)
            })
            .collect();

        cells = {
            let iter = cells.into_iter().rev().skip_while(|c| c.bc.is_none());
            iter.collect()
        };

        match cells.first() {
            Some(Cell {
                bc: Some(cell_bc), ..
            }) => match cell_bc.cmp(&bc) {
                Equal => VShift::Done,
                Less => VShift::Left(bc - cell_bc),
                Greater => VShift::Right(cell_bc - bc),
            },
            _ => VShift::Skip,
        }
    }

    fn pull_row(&mut self) {
        for cell in self.cells.iter_mut() {
            cell.row = cell.row.saturating_sub(1)
        }
    }
}

struct Cell {
    bc: Option<usize>,
    col: u16,
    row: u16,
}
