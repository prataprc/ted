use crossterm::queue;
use log::trace;
use unicode_width::UnicodeWidthChar;

use std::{
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

macro_rules! align_window_buffer {
    ($coord:expr, $ncursor:expr, $bcursor:expr, $borigin:expr, $soff:expr) => {{
        let (t, r, b, l) = $coord.to_trbl($soff);
        let (col, row) = $coord.to_origin();
        let (height, width) = $coord.to_size();

        let (ccol, oc): (u16, usize) = if $ncursor.0 < (l as isize) {
            (0, $bcursor.0)
        } else if $ncursor.0 > (r as isize) {
            (width - 1, $bcursor.0 - (width as usize) + 1)
        } else {
            (($ncursor.0 as u16) - col + 1, $borigin.0)
        };

        let (crow, or): (u16, usize) = if $ncursor.1 < (t as isize) {
            (0, $bcursor.1)
        } else if $ncursor.1 > (b as isize) {
            (height - 1, $bcursor.1 - (height as usize) + 1)
        } else {
            (($ncursor.1 as u16) - row + 1, $borigin.1)
        };
        (cursor!(ccol, crow), (oc, or)) // new_cursor, buf_origin
    }};
}

macro_rules! fix_line {
    ($width:expr, $line:expr) => {{
        use std::iter::repeat;

        let items: Vec<(char, usize)> = {
            let mut w = $width as isize;
            let iter = $line.chars().map(|ch| (ch, ch.width().unwrap_or(0)));
            iter.take_while(|(_, n)| {
                w -= (*n as isize);
                w > 0
            })
            .collect::<Vec<(char, usize)>>()
        };

        let (chars, ns): (Vec<char>, Vec<usize>) = items.into_iter().unzip();
        let mut line = String::from_iter(chars.into_iter());
        let n: usize = ns.into_iter().sum();

        let spaces = repeat(' ').take(($width as usize) - n).into_iter();
        line.push_str(&String::from_iter(spaces.into_iter()));

        line
    }};
}

#[derive(Clone, Default)]
pub struct WindowEdit {
    coord: Coord,
    cursor: Cursor,
    buf_origin: (usize, usize),

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
            buf_origin: (0, 0),

            buffer_id: Default::default(),
            config,
        })
    }

    #[inline]
    pub fn to_size(&self) -> (u16, u16) {
        self.coord.to_size()
    }
}

impl WindowEdit {
    pub fn to_lines<'a>(
        &self,
        from: Bound<usize>,
        to: Bound<usize>,
        context: &'a Context,
    ) -> impl Iterator<Item = (usize, String)> + 'a {
        let buffer = context.as_buffer(&self.buffer_id);
        buffer.to_lines(from, to)
    }

    pub fn visual_cursor(&self, context: &Context) -> (usize, usize) {
        let buffer = context.as_buffer(&self.buffer_id);
        buffer.visual_cursor()
    }

    fn refresh_once(&mut self, buffer: &mut Buffer) -> Result<()> {
        buffer.set_cursor(0);

        let (buf_csr_col, buf_csr_row) = buffer.visual_cursor();
        self.cursor = cursor!(0, 0);
        self.buf_origin = (0, 0);

        trace!(
            "{} buf_cursor:{:?} buf_origin:X->{:?} cursor:X->{}",
            self.coord,
            (buf_csr_col, buf_csr_row),
            self.buf_origin,
            self.cursor,
        );

        let mut stdout = io::stdout();
        let Cursor { col, mut row } = self.coord.to_top_left();

        let (from, to) = (
            Bound::Included(0),
            Bound::Included((self.coord.hgt - 1) as usize),
        );
        for (_line_no, line) in buffer.to_lines(from, to) {
            // trace!("{} {:?}", line_no, line);
            let line = fix_line!(self.coord.wth, line);
            err_at!(Fatal, queue!(stdout, span!((col, row), s: line)))?;
            row += 1;
        }

        Ok(())
    }

    fn refresh_again(&mut self, buffer: &mut Buffer) -> Result<()> {
        let (buf_org_col, buf_org_row) = self.buf_origin;
        let (buf_csr_col, buf_csr_row) = buffer.visual_cursor();

        let (buf_diff_col, buf_diff_row) = {
            let Cursor { col, row } = self.cursor;
            // calculate the old cursor point into the buffer.
            let (old_cc, old_cr) = (
                (buf_org_col as isize) + (col as isize),
                (buf_org_row as isize) + (row as isize),
            );
            // new cursor point into the buffer.
            let (ncc, ncr) = ((buf_csr_col as isize), (buf_csr_row as isize));
            (ncc - old_cc, ncr - old_cr)
        };

        let (ccol, crow) = {
            let Cursor { col: c, row: r } = self.to_cursor();
            (((c as isize) + buf_diff_col), ((r as isize) + buf_diff_row))
        };

        let (new_cursor, new_buf_origin) = align_window_buffer!(
            self.coord,
            (ccol, crow),
            (buf_csr_col, buf_csr_row),
            (buf_org_col, buf_org_row),
            self.config.scroll_off
        );

        trace!(
            "{} buf_cursor:{:?} buf_origin:{:?}->{:?} vp_cursor:{}->{}",
            self.coord,
            (buf_csr_col, buf_csr_row),
            self.buf_origin,
            new_buf_origin,
            self.cursor,
            new_cursor
        );

        let (from, to) = (
            Bound::Included(new_buf_origin.1),
            Bound::Included(new_buf_origin.1 + (self.coord.hgt as usize) - 1),
        );

        self.buf_origin = new_buf_origin;
        self.cursor = cursor!(new_cursor.col, new_cursor.row);

        let mut stdout = io::stdout();
        let Cursor { col, mut row } = self.coord.to_top_left();

        for (_line_no, line) in buffer.to_lines(from, to) {
            // trace!("{} {:?}", line_no, line);
            let line = fix_line!(self.coord.wth, line);
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

    #[inline]
    fn move_by(&mut self, col_off: i16, row_off: i16, _: &Context) {
        self.coord = self.coord.clone().move_by(col_off, row_off);
    }

    #[inline]
    fn resize_to(&mut self, height: u16, width: u16, _: &Context) {
        self.coord = self.coord.clone().resize_to(height, width);
    }

    fn refresh(&mut self, context: &mut Context) -> Result<()> {
        let buffer = context.as_mut_buffer(&self.buffer_id);
        self.refresh_again(buffer)
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
