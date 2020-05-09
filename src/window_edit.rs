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
    nu_wth: u16,
    old_bc: buffer::Cursor,
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
            old_bc: (0, 0).into(),
            buffer_id: Default::default(),
        }
    }
}

impl WindowEdit {
    fn to_nu_width(&self, cr: &Cursor, co: &Coord, s: &State) -> Result<u16> {
        if s.as_ref().line_number {
            let buf = s.as_buffer(&self.buffer_id);
            let new_bc = buf.to_xy_cursor();
            let n = new_bc.row + ((co.hgt - cr.row) as usize);
            let ls: Vec<RopeSlice> = {
                let iter = buf.lines_at(new_bc.row, DP::Right)?.take(n);
                iter.collect()
            };
            Ok((1 + new_bc.row + ls.len()).to_string().len() as u16)
        } else {
            Ok(0)
        }
    }

    fn refresh_nowrap(&mut self, s: &mut State) -> Result<()> {
        use std::iter::repeat;

        let new_bc = s.as_mut_buffer(&self.buffer_id).to_xy_cursor();
        let (hgt, wth) = self.coord.to_size();
        let (cursor, nu_wth) = {
            let cursor = self.cursor.move_to(
                self.coord.clone(),
                self.old_bc.clone(),
                s.as_buffer(&self.buffer_id).to_xy_cursor(),
                s.as_ref().scroll_off,
            );
            let nu_wth = self.to_nu_width(&cursor, &self.coord, s)?;
            (cursor.adjust_nu(nu_wth), nu_wth)
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
            line.chars_at(new_bc.col - (cursor.col as usize))
                .chain(repeat(' '))
                .take((wth - nu_wth) as usize)
                .collect()
        };

        let from = new_bc.row.saturating_sub(cursor.row as usize);
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
