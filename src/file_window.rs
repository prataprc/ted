use crossterm::{cursor, queue};
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
    window::{Coord, Cursor, Span, Window},
    Error, Result,
};

//
//  x----y-------------------------
//  |    |      .
//  |    |      .
//  |    |......z
//  |    |
//  |    |
//  +----+-------------------------
//
#[derive(Clone, Default)]
pub struct FileWindow {
    w_coord: Coord,                     // x window coord.
    bw_coord: Coord,                    // y buffer's coordinate inside window.
    bw_cursor: Cursor,                  // z cursor relative to buffer's coord.
    buf_origin: Option<(usize, usize)>, // (col, row) within buffer, from (0,0).
    config: Config,
}

impl fmt::Display for FileWindow {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "FileWindow<{}>", self.w_coord)
    }
}

impl FileWindow {
    #[inline]
    pub fn new(coord: Coord, config: Config) -> Result<FileWindow> {
        Ok(FileWindow {
            w_coord: coord.clone(),
            bw_coord: coord,
            bw_cursor: Cursor::new(0, 0),
            buf_origin: None,
            config,
        })
    }

    #[inline]
    fn move_by(mut self, col_off: i16, row_off: i16) -> Self {
        self.w_coord = self.w_coord.move_by(col_off, row_off);
        self
    }

    #[inline]
    fn resize_to(mut self, height: u16, width: u16) -> Self {
        self.w_coord = self.w_coord.resize_to(height, width);
        self
    }
}

impl FileWindow {
    #[inline]
    pub fn to_size(&self) -> (u16, u16) {
        self.w_coord.to_size()
    }

    #[inline]
    pub fn to_bw_top(&self) -> u16 {
        self.bw_coord.row
    }

    #[inline]
    pub fn to_bw_right(&self) -> u16 {
        self.bw_coord.col + self.bw_coord.wth - 1
    }

    #[inline]
    pub fn to_bw_bottom(&self) -> u16 {
        self.bw_coord.row + self.bw_coord.hgt - 1
    }

    #[inline]
    pub fn to_bw_left(&self) -> u16 {
        self.bw_coord.col
    }

    fn is_top_margin(&self) -> bool {
        match self.to_origin() {
            (_, 1) => false,
            _ => true,
        }
    }

    fn is_left_margin(&self) -> bool {
        match self.to_origin() {
            (1, _) => false,
            _ => true,
        }
    }
}

impl FileWindow {
    fn header_height(&self) -> u16 {
        if self.is_top_margin() {
            1
        } else {
            0
        }
    }

    fn header_width(&self, buffer: &Buffer) -> u16 {
        let n = if self.config.line_number {
            let row = buffer.visual_cursor().1 as isize;
            let hgt = self.w_coord.hgt as isize;
            let s = (row + hgt).to_string();
            s.chars().collect::<Vec<char>>().len() as u16
        } else {
            0
        };
        if self.is_left_margin() {
            n + 1 + 1
        } else {
            n + 1
        }
    }

    fn refresh_bw_coord(&self, buffer: &Buffer) -> Coord {
        let l_hdr = self.header_width(buffer);
        let t_hdr = self.header_height();
        let (col, row) = {
            let (col, row) = self.w_coord.to_origin();
            (col + l_hdr, row + t_hdr)
        };
        let (hgt, wth) = {
            let (hgt, wth) = self.w_coord.to_size();
            (hgt - t_hdr, wth - l_hdr)
        };
        Coord::new(col, row, hgt, wth)
    }

    fn refresh_once(&mut self, buffer: &mut Buffer) -> Result<()> {
        let (nboc, nbor) = buffer.visual_cursor();

        self.bw_coord = self.refresh_bw_coord(buffer);
        self.bw_cursor = Cursor::new(0, 0);
        self.buf_origin = Some((0, 0));
        buffer.set_cursor(0);

        trace!(
            "buf_cursor:{:?} buf_origin:X->{:?} vp_cursor:X->{:?}",
            (nboc, nbor),
            self.buf_origin,
            (self.bw_cursor.col, self.bw_cursor.row)
        );

        let mut stdout = io::stdout();
        let (mut col, mut row) = {
            let (c, r) = self.w_coord.to_origin();
            (c - 1, r - 1)
        };

        if self.is_top_margin() {
            let span = Span::new(String::from_iter(
                std::iter::repeat(self.config.top_margin_char)
                    .take(self.w_coord.to_origin().0 as usize),
            ));
            err_at!(Fatal, queue!(stdout, cursor::MoveTo(col, row), span))?;
            col += 1;
            row += 1;
        }

        let (from, to) = {
            let from = Bound::Included(0);
            let to = Bound::Included((self.bw_coord.hgt - 1) as usize);
            (from, to)
        };
        for (line_no, line) in buffer.iter_lines(from, to) {
            let mut l: String = Default::default();
            if self.is_left_margin() {
                l.push(self.config.left_margin_char);
            }
            if self.config.line_number {
                l.push_str(&line_no.to_string());
            }
            l.push_str(&line);
            err_at!(
                Fatal,
                queue!(stdout, cursor::MoveTo(col, row), Span::new(l))
            )?;
            col += 1;
            row += 1;
        }

        Ok(())
    }

    fn refresh_again(&mut self, buffer: &mut Buffer) -> Result<()> {
        let (nboc, nbor) = buffer.visual_cursor();

        let (b_o_c, b_o_r) = self.buf_origin.unwrap();
        let Cursor { col, row } = self.bw_cursor;
        let (cdiff, rdiff) = {
            // calculate the old cursor point into the buffer.
            let (old_cc, old_cr) = (
                (b_o_c as isize) + (col as isize),
                (b_o_r as isize) + (row as isize),
            );
            // new cursor point into the buffer.
            let (new_cc, new_cr) = ((nboc as isize), (nbor as isize));

            (new_cc - old_cc, new_cr - old_cr)
        };

        let (ccol, crow) = {
            let Cursor { col: c, row: r } = self.to_cursor(); // abslut (col, row)
            (((c as isize) + cdiff), ((r as isize) + rdiff))
        };

        // update the buffer-window coordinates.
        self.bw_coord = self.refresh_bw_coord(buffer);

        let t = (self.to_bw_top() + self.config.scroll_off) as isize;
        let r = self.to_bw_right() as isize;
        let b = (self.to_bw_bottom() - self.config.scroll_off) as isize;
        let l = self.to_bw_left() as isize;

        let (col, row) = self.bw_coord.to_origin();
        let (height, width) = self.bw_coord.to_size();

        let (ccol, oc): (u16, usize) = if ccol < l {
            (0, nboc)
        } else if ccol > r {
            (width - 1, nboc - (width as usize) + 1)
        } else {
            let new_col: u16 = ccol.try_into().unwrap();
            (new_col - col, b_o_c)
        };

        let (crow, or): (u16, usize) = if crow < t {
            (0, nbor)
        } else if crow > b {
            (height - 1, nbor - (height as usize) + 1)
        } else {
            let new_row: u16 = crow.try_into().unwrap();
            (new_row - row, b_o_r)
        };

        trace!(
            "buf_cursor:{:?} buf_origin:{:?}->{:?} vp_cursor:{:?}->{:?}",
            (nboc, nbor),
            self.buf_origin,
            (oc, or),
            self.bw_cursor,
            (ccol, crow)
        );

        self.buf_origin = Some((oc, or));
        self.bw_cursor = Cursor::new(ccol, crow);

        let mut stdout = io::stdout();
        let (mut col, mut row) = {
            let (c, r) = self.w_coord.to_origin();
            (c - 1, r - 1)
        };

        let (from, to) = (
            Bound::Included(or),
            Bound::Included(or + (self.bw_coord.hgt as usize) - 1),
        );
        for (line_no, line) in buffer.iter_lines(from, to) {
            let mut l: String = Default::default();
            if self.is_left_margin() {
                l.push(self.config.left_margin_char);
            }
            if self.config.line_number {
                l.push_str(&line_no.to_string());
            }
            l.push_str(&line);
            err_at!(
                Fatal,
                queue!(stdout, cursor::MoveTo(col, row), Span::new(l))
            )?;
            col += 1;
            row += 1;
        }

        Ok(())
    }
}

impl Window for FileWindow {
    #[inline]
    fn to_origin(&self) -> (u16, u16) {
        self.w_coord.to_origin()
    }

    #[inline]
    fn to_cursor(&self) -> Cursor {
        let (col, row) = self.bw_coord.to_origin();
        Cursor::new(col + self.bw_cursor.col, row + self.bw_cursor.row)
    }

    fn refresh(&mut self, buffer: &mut Buffer) -> Result<()> {
        match &self.buf_origin {
            Some(_) => self.refresh_again(buffer),
            None => self.refresh_once(buffer),
        }
    }

    fn handle_event(
        //
        &mut self,
        buffer: &mut Buffer,
        evnt: Event,
    ) -> Result<Option<Event>> {
        buffer.handle_event(evnt)
    }
}
