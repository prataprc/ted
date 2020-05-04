use crossterm::queue;
use log::trace;
use ropey::RopeSlice;

use std::{
    convert::TryInto,
    fmt,
    io::{self, Write},
    iter::FromIterator,
    result,
};

use crate::{
    buffer::Buffer,
    event::{Event, DP},
    window::{Context, Coord, Cursor, Span, State},
    Error, Result,
};

#[derive(Clone, Default)]
pub struct WindowEdit {
    coord: Coord,
    cursor: Cursor,
    nu_wth: u16,
    old_bc: (usize, usize),
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
            nu_wth: 0,
            old_bc: (0, 0),
            buffer_id: Default::default(),
        }
    }
}

impl WindowEdit {
    fn align_to_row(&self, s: &State) -> Result<(u16, u16)> {
        let buf = s.as_buffer(&self.buffer_id);
        let new_bc = buf.to_xy_cursor();
        let (hgt, _) = self.coord.to_size();
        let Cursor { row, .. } = self.cursor;

        let row: u16 = {
            let soff = s.as_ref().scroll_off * 2;
            let nx = if_else!(hgt < soff, (0, hgt - 1), (soff, hgt - soff - 1));
            limit!(
                (row as isize) + (new_bc.1 as isize) - (self.old_bc.1 as isize),
                nx.0 as isize,
                nx.1 as isize
            )
            .try_into()
            .unwrap()
        };

        // nu_wth extra space "<n> ".
        let nu_wth = if s.as_ref().line_number {
            let from = new_bc.1.saturating_sub(row as usize);
            let ls: Vec<RopeSlice> = {
                let iter = buf.lines_at(from, DP::Right)?.take(hgt as usize);
                iter.collect()
            };
            (from + ls.len()).to_string().len() as u16 + 1
        } else {
            0
        };

        Ok((row, nu_wth))
    }

    fn align_to_col(&self, s: &State, nu_wth: u16) -> u16 {
        let new_bc = s.as_buffer(&self.buffer_id).to_xy_cursor();
        let (_, wth) = self.coord.to_size();
        let Cursor { col, .. } = self.cursor;

        let col = limite!(
            (col as isize) + (new_bc.0 as isize) - (self.old_bc.0 as isize),
            0,
            (wth - nu_wth) as isize
        );
        // trace!("atc {} {} {} {} {}", col, self.old_bc.0, new_bc.0,);
        col.try_into().unwrap()
    }

    fn refresh_nowrap(&mut self, s: &mut State) -> Result<()> {
        use std::iter::repeat;

        let new_bc = s.as_mut_buffer(&self.buffer_id).to_xy_cursor();
        let (hgt, wth) = self.coord.to_size();
        let (cursor, nu_wth) = {
            let (crow, nu_wth) = self.align_to_row(&s)?;
            let ccol = self.align_to_col(&s, nu_wth);
            (cursor!(ccol, crow), nu_wth)
        };

        trace!(
            "{} bc:{:?}->{:?} vc:{}->{} nu_wth:{}",
            self.coord,
            self.old_bc,
            new_bc,
            self.cursor,
            cursor,
            nu_wth
        );
        self.cursor = cursor;
        self.nu_wth = nu_wth;
        self.old_bc = new_bc;

        let mut stdout = io::stdout();
        let (col, mut row) = self.coord.to_origin_cursor();

        let buf = s.as_buffer(&self.buffer_id);

        let do_padding = |line: ropey::RopeSlice| -> Vec<char> {
            // trace!("l {} {} {:?}", new_bc.0, cursor.col, line.to_string());
            line.chars_at(new_bc.0 - (cursor.col as usize))
                .chain(repeat(' '))
                .take((wth - nu_wth) as usize)
                .collect()
        };

        let from = new_bc.1.saturating_sub(cursor.row as usize);
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
        self.refresh_nowrap(s)
    }

    pub fn on_event(&mut self, s: &mut State, evnt: Event) -> Result<Event> {
        match evnt {
            Event::Td(Ted::UseBuffer { buffer_id }) => {
                self.buffer_id = buffer_id;
                Ok(Event::Noop)
            }
            mut evnt => match s.take_buffer(&self.buffer_id) {
                Some(buffer) => {
                    let (buffer, evnt) = {
                        let mut c = Context::new(s, buffer);
                        evnt = Buffer::on_event(&mut c, evnt)?;
                        (c.buffer, evnt)
                    };
                    s.add_buffer(buffer);
                    Ok(evnt)
                }
                None => Ok(evnt),
            },
        }
    }
}
