use crossterm::queue;
use log::trace;

use std::{
    convert::TryInto,
    fmt,
    io::{self, Write},
    iter::FromIterator,
    ops::Bound,
    result,
};

use crate::{
    buffer::Buffer,
    config::Config,
    event::Event,
    window::{Context, Coord, Cursor, Span, Window},
    Error, Result,
};

#[derive(Clone, Default)]
pub struct WindowEdit {
    coord: Coord,
    cursor: Cursor,
    buf_origin: Option<(usize, usize)>,

    buffer_id: String,
    config: Config,
}

impl fmt::Display for WindowEdit {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "WindowEdit<{}>", self.coord)
    }
}

impl WindowEdit {
    #[inline]
    pub fn new(coord: Coord, config: Config) -> Result<WindowEdit> {
        Ok(WindowEdit {
            coord,
            cursor: cursor!(0, 0),
            buf_origin: None,

            buffer_id: Default::default(),
            config,
        })
    }

    #[inline]
    fn move_by(mut self, col_off: i16, row_off: i16) -> Self {
        self.coord = self.coord.move_by(col_off, row_off);
        self.cursor = {
            //
            cursor!(self.cursor.col - col_off, self.cursor.row - row_off)
        };
        self.buf_origin = match self.buf_origin {
            Some((col, row)) => Some((col + col_off, row_off)),
            None => None,
        };
        self
    }

    #[inline]
    fn resize_to(mut self, height: u16, width: u16) -> Self {
        self.coord = self.coord.resize_to(height, width);
        self.cursor = {
            //
            cursor!(self.cursor.col - col_off, self.cursor.row - row_off)
        };
        self.buf_origin = match self.buf_origin {
            Some((col, row)) => Some((col + col_off, row_off)),
            None => None,
        };
        self
    }
}

impl WindowEdit {
    #[inline]
    pub fn to_top(&self) -> u16 {
        self.coord.row
    }

    #[inline]
    pub fn to_right(&self) -> u16 {
        self.coord.col + self.coord.wth - 1
    }

    #[inline]
    pub fn to_bottom(&self) -> u16 {
        self.coord.row + self.coord.hgt - 1
    }

    #[inline]
    pub fn to_left(&self) -> u16 {
        self.coord.col
    }
}

impl WindowEdit {
    fn iter_lines(
        &self,
        from: Bound<usize>,
        to: Bound<usize>,
        context: &Context,
    ) -> impl Iterator<Item = (usize, String)> + 'a {
        match context.as_buffer(&self.buffer_id) {
            Some(buffer) => buffer.iter_lines(from, to),
            None => todo!(),
        }
    }

    fn visual_cursor(&self, context: &Context) -> (usize, usize) {
        match context.as_buffer(&self.buffer_id) {
            Some(buffer) => buffer.visual_cursor(),
            None => todo!(),
        }
    }

    fn refresh_once(&mut self, buffer: &mut Buffer) -> Result<()> {
        buffer.set_cursor(0);

        let (buf_cur_col, buf_cur_row) = buffer.visual_cursor();
        self.cursor = cursor!(0, 0);
        self.buf_origin = Some((0, 0));

        trace!(
            "{} buf_cursor:{:?} buf_origin:X->{:?} cursor:X->{}",
            self.coord,
            (buf_cur_col, buf_cur_row),
            self.buf_origin,
            self.cursor,
        );

        let mut stdout = io::stdout();
        let (mut col, mut row) = self.coord.to_top_left();

        let (from, to) = (
            Bound::Included(0),
            Bound::Included((self.coord.hgt - 1) as usize),
        );
        for (line_no, line) in buffer.iter_lines(from, to) {
            trace!("{} {:?}", line_no, line);
            err_at!(Fatal, queue!(stdout, span!((col, row), s: line)))?;
            row += 1;
        }

        Ok(())
    }

    fn refresh_again(&mut self, buffer: &mut Buffer) -> Result<()> {
        let (buf_org_col, buf_org_row) = self.buf_origin.unwrap();
        let (buf_cur_col, buf_cur_col) = buffer.visual_cursor();

        let (cdiff, rdiff) = {
            let Cursor { col, row } = self.cursor;
            // calculate the old cursor point into the buffer.
            let (old_cc, old_cr) = (
                (buf_org_col as isize) + (col as isize),
                (buf_org_row as isize) + (row as isize),
            );
            // new cursor point into the buffer.
            let (ncc, ncr) = ((buf_cur_col as isize), (buf_cur_row as isize));
            (ncc - old_cc, ncr - old_cr)
        };

        let (ccol, crow) = {
            let Cursor { col: c, row: r } = self.to_cursor();
            (((c as isize) + cdiff), ((r as isize) + rdiff))
        };

        let (new_buf_origin, new_cursor) = {
            let t = (self.to_bw_top() + self.config.scroll_off) as isize;
            let r = self.to_bw_right() as isize;
            let b = (self.to_bw_bottom() - self.config.scroll_off) as isize;
            let l = self.to_bw_left() as isize;

            let (col, row) = self.coord.to_origin();
            let (height, width) = self.coord.to_size();

            let (ccol, oc): (u16, usize) = if ccol < l {
                (0, buf_cur_col)
            } else if ccol > r {
                (width - 1, buf_cur_col - (width as usize) + 1)
            } else {
                let new_col: u16 = ccol.try_into().unwrap();
                (new_col - col, buf_org_col)
            };

            let (crow, or): (u16, usize) = if crow < t {
                (0, buf_cur_row)
            } else if crow > b {
                (height - 1, buf_cur_row - (height as usize) + 1)
            } else {
                let new_row: u16 = crow.try_into().unwrap();
                (new_row - row, buf_org_row)
            };
            ((oc, or), (ccol, crow))
        };

        trace!(
            "buf_cursor:{:?} buf_origin:{:?}->{:?} vp_cursor:{}->{}",
            (buf_cur_col, buf_cur_row),
            self.buf_origin,
            buf_origin,
            self.cursor,
            new_cursor
        );

        self.buf_origin = Some(new_buf_origin);
        self.cursor = Cursor::new(new_cursor.col, new_cursor.row);

        let mut stdout = io::stdout();
        let (mut col, mut row) = self.coord.to_top_left();

        let (from, to) = (
            Bound::Included(self.buf_origin.row),
            Bound::Included(self.buf_origin.row + (self.coord.hgt as usize) - 1),
        );
        for (line_no, line) in buffer.iter_lines(from, to) {
            trace!("{} {:?}", line_no, line);
            err_at!(Fatal, queue!(stdout, span!((col, row), s: line)))?;
            row += 1;
        }

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
        self.coord.to_top_left() + self.cursor
    }

    fn refresh(&mut self, context: &mut Context) -> Result<()> {
        match context.as_mut_buffer(&self.buffer_id) {
            Some(buffer) => match &self.buf_origin {
                Some(_) => self.refresh_again(buffer),
                None => self.refresh_once(buffer),
            },
            None => todo!(),
        }
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
            evnt => match context.as_mut_buffer(&self.buffer_id) {
                Some(buffer) => buffer.handle_event(evnt),
                None => todo!(),
            },
        }
    }
}
