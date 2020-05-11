use crossterm::queue;
use log::trace;
use ropey::RopeSlice;

use std::{
    fmt,
    io::{self, Write},
    iter::FromIterator,
    result,
};

use crate::{
    buffer::{self, Buffer},
    event::{Event, Ted, DP},
    window::{Context, Coord, Cursor, Span, State},
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
    fn wshift_after(&self, s: &State) -> usize {
        let buf = s.as_buffer(&self.buffer_id);
        let obc_xy = self.obc_xy;
        let nbc_xy = buf.to_xy_cursor();

        let mut n = 0;
        for row in obc_xy.row..=nbc_xy.row {
            n += match row {
                row if row == obc_xy.row => {
                    let m = self.coord.to_cells(buf.line(row).len_chars());
                    m - obc_xy.col
                }
                row if row == nbc_xy.row => nbc_xy.col,
                row => self.coord.to_cells(buf.line(row).len_chars()),
            }
        }
        n
    }

    fn wshift_before(&self, s: &State) -> usize {
        let buf = s.as_buffer(&self.buffer_id);
        let obc_xy = self.obc_xy;
        let nbc_xy = buf.to_xy_cursor();

        let mut n = 0;
        for row in (nbc_xy.row..=obc_xy.row).rev() {
            n += match row {
                row if row == obc_xy.row => obc_xy.col,
                row if row == nbc_xy.row => {
                    let m = self.coord.to_cells(buf.line(row).len_chars());
                    m - nbc_xy.col
                }
                row => self.coord.to_cells(buf.line(row).len_chars()),
            }
        }
        n
    }

    fn wshift(&self, s: &State) -> Cursor {
        use std::cmp::Ordering::{Equal, Greater, Less};
        use crate::event::DP::{Left, Right};

        let scroll_off = s.as_ref().scroll_off; // accounting for scroll-offset.
        // gather variables.
        let nbc_xy = s.as_buffer(&self.buffer_id).to_xy_cursor();
        // create possible cursor positions.
        let mut cursors: Vec<Cursor> = if nbc_xy < self.obc_xy {
            let mut iter = self.cursor.prev_cursors(coord).into_iter().rev();
            iter.skip(scroll_off * coord.wth).rev().collect()
        } else {
            let mut iter = self.cursor.next_cursors(coord).into_iter().rev();
            iter.skip(scroll_off * coord.wth).rev().collect()
        };
        // compute the number of cells to drain and its direction.
        let same_row = nbc_xy.row == self.obc_xy.row;
        let (m, dp) = match nbc_xy.cmp(&self.obc_xy) {
            Equal => return Ok(self.cursor),
            Greater if same_row => (nbc_xy.col - self.obc_xy.col, Right),
            Greater => (self.wshift_after(), Right)
            Less if same_row => (self.obc_xy.col - nbc_xy.col, Left),
            Less => (self.wshift_before(), Left),
        };
        let (n, m) = (cursors.len(), cursors.len() - m);
        cursors.drain(n..m);
        // compute cursor.
        let cursor = match (cursors.pop(), dp) {
            (Some(cursor), _) => Ok(cursor),
            (None, DP::Left) => Cursor {
                row: scroll_off,
                col: nbc_xy.col % coord.wth,
            },
            (None, DP::Right) => Cursor {
                row: coord.hgt - scroll_off - 1,
                col: nbc_xy.col % coord.wth,
            }
        };

        Ok(cursor)
    }

    fn wrefresh(&self, new_cursor: Cursor, s: &mut State) -> Result<()> {
        use std::iter::repeat;

        let (nbc_xy, nu_wth) = {
            let buf = s.as_mut_buffer(&self.buffer_id);
            let nbc_xy = buf.to_xy_cursor();
            let nu = buf.char_to_line(buf.to_cursor());
            (nbc_xy, cmp::min(nu.to_string().len() + 1, 3))
        };

        let coord = self.coord.shrink_width(nu_wth);
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

        let wv = {
            let line_idx = nbc_xy.row - new_cursor.row
            WrapView::new(line_idx, coord, buf);
        };
        'a: for line in wv.lines.iter() {
            let nus = line.nu.to_string();
            for (r, rline) in line.rows.enumerate() {
                let s = match r {
                    0 => nus.clone(),
                    _ => String::from_iter(iter::repeat(' ').take(nus.len())),
                };
                let s = format!("{:>width$} ", s, width = nu_wth);
                err_at!(Fatal, queue!(stdout, span!((col, row), st: s)))?;

                let bcs = rline.cells.filter_map(|c| c.bc);
                let s = match bcs.first(), bcs.last() {
                    (Some(fbc), Some(ebc)) => {
                        let iter = buf.chars_at(n, DP::Right);
                        let chs: Vec<Char> = iter.take(ebc - fbc + 1).collect();
                        String::from_iter(chs)
                    }
                    _ => "".to_string()
                }
                err_at!(Fatal, queue!(stdout, span!(st: s)))?;

                row += 1;
                if row >= max_row {
                    break 'a;
                }
            }
        }

        Ok(())
    }

    fn nshift(&self, s: &State) -> Cursor {
        let scroll_off = c.scroll_off; // accounting for scroll-offset.

        let coord = self.coord;

        let (diff_col, diff_row) = self.obc_xy.diff(&self.nbc_xy);
        let (hgt, wth) = coord.to_size();
        let Cursor { row, col } = self.cursor;

        let (r_min, r_max): (isize, isize) = if hgt < (scroll_off * 2) {
            (0, (hgt - 1) as isize)
        } else {
            (scroll_off as isize, (hgt - scroll_off - 1) as isize)
        };
        let new_row: u16 = limit!((row as isize) + diff_row, r_min, r_max)
            .try_into()
            .unwrap();

        let new_col: u16 = limite!((col as isize) + diff_col, 0, wth as isize)
            .try_into()
            .unwrap();

        Cursor {
            col: new_col,
            row: new_row,
        }
    }

    fn nrefresh(&mut self, new_cursor: Cursor, s: &mut State) -> Result<()> {
        use std::iter::repeat;

        let (nbc_xy, nu_wth) = {
            let buf = s.as_mut_buffer(&self.buffer_id);
            let nbc_xy = buf.to_xy_cursor();
            let nu = buf.char_to_line(buf.to_cursor());
            (nbc_xy, cmp::min(nu.to_string().len() + 1, 3))
        };

        let coord = self.coord.shrink_width(nu_wth);
        let (hgt, wth) = self.coord.to_size();

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
        let (col, mut row) = self.coord.to_origin_cursor();

        let do_padding = |line: ropey::RopeSlice| -> Vec<char> {
            // trace!("l {} {} {:?}", nbc_xy.0, new_cursor.col, line.to_string());
            line.chars_at(nbc_xy.col - (new_cursor.col as usize))
                .chain(repeat(' '))
                .take((wth - nu_wth) as usize)
                .collect()
        };

        let from = nbc_xy.row.saturating_sub(new_cursor.row as usize);
        let lines = buf.lines_at(from, DP::Right)?.map(do_padding);
        let mrgn_wth = nu_wth.saturating_sub(1) as usize;
        for (i, line) in lines.take(hgt as usize).enumerate() {
            let mut st: String = if_else!(
                s.as_ref().line_number,
                format!("{:>width$} ", from + i + 1, width = mrgn_wth),
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
        let mut cursor = self.coord.to_top_left() + self.cursor;
        cursor.col += self.nu_wth;
        cursor
    }

    pub fn on_refresh(&mut self, s: &mut State) -> Result<()> {
        self.cursor = if s.as_ref().wrap {
            let new_cursor = self.wshift(s);
            self.wrefresh(new_cursor, s)?;
            new_cursor
        } else {
            let new_cursor = self.nshift(s);
            self.nrefresh(new_cursor, s)?;
            new_cursor
        };
        self.obc_xy = s.as_mut_buffer().to_xy_cursor();

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
