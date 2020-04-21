use crossterm::queue;
use log::trace;
use ropey::RopeSlice;

use std::{
    cmp,
    convert::TryInto,
    fmt,
    io::{self, Write},
    iter::FromIterator,
    result,
};

use crate::{
    config::Config,
    event::Event,
    window::{Context, Coord, Cursor, Span, Window},
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
    pub fn new(coord: Coord, _: &Config) -> Result<WindowEdit> {
        Ok(WindowEdit {
            coord,
            cursor: cursor!(0, 0),
            nu_wth: 0,
            old_bc: (0, 0),
            buffer_id: Default::default(),
        })
    }
}

impl WindowEdit {
    fn align_to_row(&self, context: &Context) -> (u16, u16) {
        let change = context.as_buffer(&self.buffer_id).as_change();
        let new_bc = change.to_xy_cursor();
        let (hgt, _) = self.coord.to_size();
        let Cursor { row, .. } = self.cursor;

        let row: u16 = {
            let soff = context.config.scroll_off * 2;
            let nx = if_else!(hgt < soff, (0, hgt - 1), (soff, hgt - soff - 1));
            bounded_num_op!(
                (row as isize) + (new_bc.1 as isize) - (self.old_bc.1 as isize),
                nx.0 as isize,
                nx.1 as isize
            )
            .try_into()
            .unwrap()
        };

        // nu_wth extra space "<n> ".
        let nu_wth = if context.config.line_number {
            let from = new_bc.1.saturating_sub(row as usize);
            let ls: Vec<RopeSlice> = {
                let iter = change.lines_at(from).take(hgt as usize);
                iter.collect()
            };
            (from + ls.len()).to_string().len() as u16 + 1
        } else {
            0
        };

        (row, nu_wth)
    }

    fn align_to_col(&self, context: &Context, nu_wth: u16) -> u16 {
        let new_bc = context.as_buffer(&self.buffer_id).to_xy_cursor();
        let (_, wth) = self.coord.to_size();
        let Cursor { col, .. } = self.cursor;

        let col = bounded_num_op!(
            (col as isize) + (new_bc.0 as isize) - (self.old_bc.0 as isize),
            0,
            (wth - nu_wth - 1) as isize
        );
        // trace!("atc {} {} {} {} {}", col, self.old_bc.0, new_bc.0,);
        col.try_into().unwrap()
    }

    fn refresh_nowrap(&mut self, context: &mut Context) -> Result<()> {
        use std::iter::repeat;

        let new_bc = context.as_mut_buffer(&self.buffer_id).to_xy_cursor();
        let (hgt, wth) = self.coord.to_size();
        let (cursor, nu_wth) = {
            let (crow, nu_wth) = self.align_to_row(context);
            let ccol = self.align_to_col(context, nu_wth);
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
        let (col, mut row) = {
            let (c, r) = self.coord.to_origin();
            (c - 1, r - 1)
        };

        let change = {
            let buf = context.as_buffer(&self.buffer_id);
            buf.as_change()
        };

        let do_padding = |line: ropey::RopeSlice| -> Vec<char> {
            trace!(
                "bufline char_idx:{} {} line:{:?}",
                new_bc.0,
                (cursor.col as usize),
                line.to_string()
            );
            line.chars_at(new_bc.0 - (cursor.col as usize))
                .chain(repeat(' '))
                .take((wth - nu_wth) as usize)
                .collect()
        };

        let from = new_bc.1.saturating_sub(cursor.row as usize);
        let lines = change.lines_at(from).map(do_padding);
        let mrgn_wth = nu_wth.saturating_sub(1) as usize;
        for (i, line) in lines.take(hgt as usize).enumerate() {
            let mut s: String = if_else!(
                context.config.line_number,
                format!("{:>width$} ", from + i + 1, width = mrgn_wth),
                Default::default()
            );
            let s_line = String::from_iter(line);
            // trace!("bufline col:{} row:{} line:{:?}", col, row, s_line);
            s.push_str(&s_line);
            err_at!(Fatal, queue!(stdout, span!((col, row), s: s)))?;
            row += 1;
        }
        for _ in row..hgt {
            let mut s: String = if_else!(
                context.config.line_number,
                format!("{} ", '~'),
                Default::default()
            );
            s.push_str(&{
                let iter = repeat(' ').take((wth - 2) as usize);
                String::from_iter(iter)
            });
            trace!("empline col:{} row:{} line:{:?}", col, row, s.len());
            err_at!(Fatal, queue!(stdout, span!((col, row), s: s)))?;
            row += 1;
        }
        assert!(row == hgt);

        Ok(())
    }
}

impl Window for WindowEdit {
    #[inline]
    fn to_origin(&self) -> (u16, u16) {
        self.coord.to_origin()
    }

    #[inline]
    fn to_cursor(&self) -> Cursor {
        let mut cursor = self.coord.to_top_left() + self.cursor;
        cursor.col += self.nu_wth;
        cursor
    }

    #[inline]
    fn move_by(&mut self, col_off: i16, row_off: i16, _: &Context) {
        self.coord = self.coord.clone().move_by(col_off, row_off);
    }

    #[inline]
    fn resize_to(&mut self, height: u16, width: u16, context: &Context) {
        let scroll_off = context.config.scroll_off;

        self.coord = self.coord.clone().resize_to(height, width);
        self.cursor.col = cmp::min(self.cursor.col, width - 1);
        self.cursor.row = {
            let row = cmp::min(self.cursor.row, height - 1);
            if_else!(row <= scroll_off, row, row - scroll_off)
        };
    }

    fn refresh(&mut self, context: &mut Context) -> Result<()> {
        self.refresh_nowrap(context)
    }

    fn handle_event(
        //
        &mut self,
        context: &mut Context,
        evnt: Event,
    ) -> Result<Option<Event>> {
        match evnt {
            Event::UseBuffer { buffer_id } => {
                self.buffer_id = buffer_id;
                Ok(None)
            }
            evnt => {
                let buffer = context.as_mut_buffer(&self.buffer_id);
                buffer.handle_event(evnt)
            }
        }
    }
}
