use crossterm::queue;
use log::trace;
use ropey;

use std::{
    cmp, fmt,
    io::{self, Write},
    iter::FromIterator,
    result,
};

use crate::{
    buffer::{self, Buffer},
    event::{Event, Ted, DP},
    window::{Context, Coord, Cursor, Span, State},
    wrap_view::WrapView,
    Error, Result,
};

#[derive(Clone, Default)]
pub struct WindowEdit {
    coord: Coord,
    cursor: Cursor,
    obc_xy: buffer::Cursor,
    buffer_id: String,
}

impl fmt::Display for WindowEdit {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "WindowEdit<{}>", self.coord)
    }
}

impl WindowEdit {
    #[inline]
    pub fn new(coord: Coord) -> WindowEdit {
        WindowEdit {
            coord,
            cursor: cursor!(0, 0),
            obc_xy: (0, 0).into(),
            buffer_id: Default::default(),
        }
    }
}

impl WindowEdit {
    fn wshift_after(&self, coord: Coord, s: &State) -> usize {
        let buf = s.as_buffer(&self.buffer_id);
        let nbc_xy = buf.to_xy_cursor();

        let mut n = 0;
        for row in self.obc_xy.row..=nbc_xy.row {
            n += match row {
                row if row == self.obc_xy.row => {
                    let m = coord.to_cells(buf.line_len(row));
                    m.saturating_sub(self.obc_xy.col)
                }
                row if row == nbc_xy.row => nbc_xy.col,
                row => coord.to_cells(buf.line_len(row)),
            }
        }
        n
    }

    fn wshift_before(&self, coord: Coord, s: &State) -> usize {
        let buf = s.as_buffer(&self.buffer_id);
        let nbc_xy = buf.to_xy_cursor();

        let mut n = 0;
        for row in (nbc_xy.row..=self.obc_xy.row).rev() {
            n += match row {
                row if row == self.obc_xy.row => self.obc_xy.col,
                row if row == nbc_xy.row => {
                    let m = coord.to_cells(buf.line_len(row));
                    m.saturating_sub(nbc_xy.col)
                }
                row => coord.to_cells(buf.line_len(row)),
            }
        }
        n
    }

    fn wshift(&self, mut coord: Coord, s: &State) -> (Cursor, u16) {
        use crate::event::DP::{Left, Right};
        use std::cmp::Ordering::{Equal, Greater, Less};

        let scroll_off = s.as_ref().scroll_off; // accounting for scroll-offset.
        let nbc_xy = s.as_buffer(&self.buffer_id).to_xy_cursor();
        coord = coord.shrink_width(compute_nu_width(self.obc_xy.row));

        // create possible cursor positions.
        let mut cursors: Vec<Cursor> = if nbc_xy < self.obc_xy {
            let iter = self.cursor.prev_cursors(coord).into_iter().rev();
            iter.skip((scroll_off * coord.wth) as usize).rev().collect()
        } else {
            let iter = self.cursor.next_cursors(coord).into_iter().rev();
            iter.skip((scroll_off * coord.wth) as usize).rev().collect()
        };
        // compute the number of cells to drain and its direction.
        let same_row = nbc_xy.row == self.obc_xy.row;
        let (m, dp) = match nbc_xy.cmp(&self.obc_xy) {
            Equal => return (self.cursor, compute_nu_width(self.obc_xy.row)),
            Greater if same_row => (nbc_xy.col - self.obc_xy.col, Right),
            Greater => (self.wshift_after(coord, s), Right),
            Less if same_row => (self.obc_xy.col - nbc_xy.col, Left),
            Less => (self.wshift_before(coord, s), Left),
        };
        cursors.drain(..cmp::min(m, cursors.len()));
        // compute cursor.
        coord = coord.shrink_width(compute_nu_width(nbc_xy.row));
        match (cursors.pop(), dp) {
            (Some(cursor), _) => (cursor, compute_nu_width(self.obc_xy.row)),
            (None, DP::Left) => {
                let cursor = Cursor {
                    row: scroll_off,
                    col: (nbc_xy.col % (coord.wth as usize)) as u16,
                };
                (cursor, compute_nu_width(nbc_xy.row))
            }
            (None, DP::Right) => {
                let cursor = Cursor {
                    row: coord.hgt.saturating_sub(scroll_off + 1),
                    col: (nbc_xy.col % (coord.wth as usize)) as u16,
                };
                (cursor, compute_nu_width(nbc_xy.row))
            }
            _ => unreachable!(),
        }
    }

    fn wrefresh(
        //
        &self,
        new_cursor: Cursor,
        coord: Coord,
        nu_wth: u16,
        s: &mut State,
    ) -> Result<()> {
        use std::iter::repeat;

        let s_nu_blank = String::from_iter(repeat(' ').take(nu_wth as usize));
        let nbc_xy = s.as_mut_buffer(&self.buffer_id).to_xy_cursor();
        let (hgt, wth) = coord.to_size();
        trace!(
            "nu:{} {} bc:{}->{} vc:{}->{}",
            nu_wth,
            coord,
            self.obc_xy,
            nbc_xy,
            self.cursor,
            new_cursor,
        );

        let buf = s.as_buffer(&self.buffer_id);
        let mut stdout = io::stdout();
        let (col, mut row) = coord.to_origin_cursor();
        let max_row = row + coord.hgt;
        let line_number = s.as_ref().line_number;

        let wv = {
            let line_idx = nbc_xy.row.saturating_sub(new_cursor.row as usize);
            WrapView::new(line_idx, coord, buf)
        };
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

        for _ in row..hgt {
            let mut st: String = if_else!(
                //
                line_number,
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
        assert!(row == hgt);

        Ok(())
    }

    fn nshift(&self, coord: Coord, s: &State) -> (Cursor, u16) {
        let scroll_off = s.as_ref().scroll_off; // accounting for scroll-offset.
        let nbc_xy = s.as_buffer(&self.buffer_id).to_xy_cursor();

        let (diff_col, diff_row) = self.obc_xy.diff(&nbc_xy);
        let (hgt, wth) = coord.to_size();
        let Cursor { row, col } = self.cursor;

        let (r_min, r_max): (isize, isize) = if hgt < (scroll_off * 2) {
            (0, (hgt.saturating_sub(1) as isize))
        } else {
            (
                scroll_off as isize,
                (hgt.saturating_sub(scroll_off + 1) as isize),
            )
        };

        let nu_wth = {
            let row = (row as isize) + diff_row;
            if row < r_min || row > r_max {
                compute_nu_width(nbc_xy.row)
            } else {
                compute_nu_width(self.obc_xy.row)
            }
        };

        let new_row: u16 = {
            let row = limit!((row as isize) + diff_row, r_min, r_max);
            assert!(row < (coord.hgt as isize));
            row as u16
        };
        let new_col: u16 = {
            let col = limite!((col as isize) + diff_col, 0, wth as isize);
            assert!(col < (coord.wth as isize));
            col as u16
        };

        let cursor = Cursor {
            col: new_col,
            row: new_row,
        };
        (cursor, nu_wth)
    }

    fn nrefresh(
        &mut self,
        new_cursor: Cursor,
        coord: Coord,
        nu_wth: u16,
        s: &mut State,
    ) -> Result<()> {
        use std::iter::repeat;

        let nbc_xy = s.as_mut_buffer(&self.buffer_id).to_xy_cursor();
        let (hgt, wth) = coord.to_size();
        trace!(
            "nu:{} {} bc:{}->{} vc:{}->{}",
            nu_wth,
            coord,
            self.obc_xy,
            nbc_xy,
            self.cursor,
            new_cursor,
        );

        let buf = s.as_buffer(&self.buffer_id);
        let mut stdout = io::stdout();
        let (col, mut row) = coord.to_origin_cursor();

        let do_padding = |line: ropey::RopeSlice| -> Vec<char> {
            // trace!("l {} {} {:?}", nbc_xy.0, new_cursor.col, line.to_string());
            line.chars_at(nbc_xy.col - (new_cursor.col as usize))
                .chain(repeat(' '))
                .take((wth - nu_wth) as usize)
                .collect()
        };

        let from = nbc_xy.row.saturating_sub(new_cursor.row as usize);
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
        assert!(row == hgt);

        Ok(())
    }
}

impl WindowEdit {
    #[inline]
    pub fn to_buffer_id(&self) -> String {
        self.buffer_id.clone()
    }

    #[inline]
    pub fn to_cursor(&self) -> Cursor {
        self.coord.to_top_left() + self.cursor
    }

    pub fn on_refresh(&mut self, s: &mut State) -> Result<()> {
        self.cursor = if s.as_ref().wrap {
            let (mut new_cursor, nu_wth) = self.wshift(self.coord, s);
            let coord = self.coord.shrink_width(nu_wth);
            new_cursor = new_cursor.add_nu_wth(nu_wth);
            self.wrefresh(new_cursor, coord, nu_wth, s)?;
            new_cursor
        } else {
            let (mut new_cursor, nu_wth) = self.nshift(self.coord, s);
            let coord = self.coord.shrink_width(nu_wth);
            new_cursor = new_cursor.add_nu_wth(nu_wth);
            self.nrefresh(new_cursor, coord, nu_wth, s)?;
            new_cursor
        };
        self.obc_xy = s.as_mut_buffer(&self.buffer_id).to_xy_cursor();

        Ok(())
    }

    pub fn on_event(&mut self, s: &mut State, evnt: Event) -> Result<Event> {
        match evnt {
            Event::Td(Ted::UseBuffer { buffer_id }) => {
                self.buffer_id = buffer_id;
                Ok(Event::Noop)
            }
            mut evnt => match s.take_buffer(&self.buffer_id) {
                Some(buf) => {
                    let (buf, evnt) = {
                        let mut c = Context::new(s, buf);
                        evnt = Buffer::on_event(&mut c, evnt)?;
                        (c.buffer, evnt)
                    };
                    s.add_buffer(buf);
                    Ok(evnt)
                }
                None => Ok(evnt),
            },
        }
    }
}

fn compute_nu_width(row: usize) -> u16 {
    use crate::buffer::MAX_LINES;

    assert!(row < MAX_LINES);
    (cmp::min(row.to_string().len(), 2) + 1) as u16
}
