use crate::{
    buffer::Buffer,
    window::{Coord, Cursor},
    Result,
};

enum VShift {
    Left(usize),
    Right(usize),
    Skip,
    Done,
}

pub struct WrapView {
    pub lines: Vec<Line>,
}

impl WrapView {
    pub fn new(line_idx: usize, coord: Coord, buf: &Buffer) -> Result<WrapView> {
        let mut lines = vec![];
        let iter = (line_idx..).take(coord.hgt as usize).enumerate();
        for (row, line_idx) in iter {
            assert!(row < 1_000); // TODO: avoid magic number
            Line::new_line(line_idx, row as u16, coord.wth, buf)
                //
                .map(|line| lines.push(line));
        }
        Ok(WrapView { lines })
    }

    pub fn align(&mut self, bc: usize, cursor: Cursor) {
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

pub struct Line {
    pub nu: usize,
    pub rows: Vec<Row>,
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
                    assert!(r < 1_000); // TODO avoid magic number
                    (row + (r as u16), bc + (r * (wth as usize)), wth, wth)
                })
                .collect()
        };

        if (len_chars % (wth as usize)) > 0 {
            let r = rows.len();
            let w = len_chars % (wth as usize);
            assert!(w <= (wth as usize));
            assert!(r < 1_000); // TODO avoid magic number
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

pub struct Row {
    pub cells: Vec<Cell>,
}

impl Row {
    fn new_row(row: u16, bc: usize, ln: u16, wth: u16) -> Row {
        use std::iter::repeat;

        let mut bcs: Vec<Option<usize>> = {
            let bc_end = bc + (ln as usize);
            let iter = (bc..bc_end).into_iter().map(|bc| Some(bc));
            iter.collect()
        };
        assert!(bcs.len() < 10_000); // TODO avoid magic number
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

pub struct Cell {
    pub bc: Option<usize>,
    pub col: u16,
    pub row: u16,
}
