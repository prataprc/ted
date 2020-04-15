use log::trace;

use std::{
    convert::TryInto,
    fmt, io,
    ops::{self, RangeBounds},
    result,
};

use crate::{Buffer, Config, Event, Result};

#[derive(Clone, Default)]
pub struct Viewport {
    col: u16,
    row: u16,
    height: u16,
    width: u16,
    vp_cursor_off: (u16, u16), // (col-offset, row-offset)

    buffer: Buffer,
    // absolute (col, row) within buffer inrelation to view-port
    // origin, starts from (0,0).
    buf_origin: (usize, usize),

    config: Config,
}

impl fmt::Display for Viewport {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(
            f,
            "Viewport<col:{} row:{} height:{} width:{}>",
            self.col, self.row, self.height, self.width
        )
    }
}

impl Viewport {
    #[inline]
    pub fn new(col: u16, row: u16, height: u16, width: u16, config: Config) -> Result<Viewport> {
        let bytes: Vec<u8> = vec![];
        Ok(Viewport {
            col,
            row,
            height,
            width,
            vp_cursor_off: (0, 0),

            buffer: Buffer::from_reader(bytes.as_slice(), config.clone())?,
            buf_origin: (0, 0),

            config,
        })
    }

    pub fn clear(&mut self) -> Result<()> {
        let bytes: Vec<u8> = vec![];
        self.buffer = Buffer::from_reader(bytes.as_slice(), self.config.clone())?;
        self.buf_origin = (0, 0);

        self.vp_cursor_off = (0, 0);

        Ok(())
    }

    pub fn load<R>(&mut self, data: R) -> Result<()>
    where
        R: io::Read,
    {
        self.buffer = Buffer::from_reader(data, self.config.clone())?;
        self.buf_origin = (0, 0);

        self.vp_cursor_off = (0, 0);

        Ok(())
    }

    #[inline]
    pub fn move_by(mut self, col_off: i16, row_off: i16) -> Self {
        self.col = ((self.col as i16) + col_off) as u16;
        self.row = ((self.row as i16) + row_off) as u16;
        self
    }

    #[inline]
    pub fn resize_to(mut self, height: u16, width: u16) -> Self {
        self.height = height;
        self.width = width;
        self
    }
}

impl Viewport {
    #[inline]
    pub fn col_range(&self) -> impl ops::RangeBounds<u16> {
        self.col..(self.col + self.width)
    }

    #[inline]
    pub fn row_range(&self) -> impl ops::RangeBounds<u16> {
        self.row..(self.row + self.height)
    }

    #[inline]
    pub fn contain_cell(&self, col: u16, row: u16) -> bool {
        self.col_range().contains(&col) && self.row_range().contains(&row)
    }

    #[inline]
    pub fn to_origin(&self) -> (u16, u16) {
        (self.col, self.row)
    }

    #[inline]
    pub fn to_buf_origin(&self) -> (usize, usize) {
        self.buf_origin
    }

    #[inline]
    pub fn to_size(&self) -> (u16, u16) {
        (self.height, self.width)
    }

    #[inline]
    pub fn to_top(&self) -> u16 {
        self.row
    }

    #[inline]
    pub fn to_right(&self) -> u16 {
        self.col + self.width - 1
    }

    #[inline]
    pub fn to_bottom(&self) -> u16 {
        self.row + self.height - 1
    }

    #[inline]
    pub fn to_left(&self) -> u16 {
        self.col
    }

    pub fn to_cursor_off(&self) -> (u16, u16) {
        self.vp_cursor_off
    }

    pub fn to_cursor(&self) -> (u16, u16) {
        let (col, row) = self.to_origin();
        let (coff, roff) = self.to_cursor_off();
        (col + coff, row + roff)
    }

    fn to_ed_cursor(&self, buf_origin: (usize, usize)) -> (usize, usize) {
        let col = buf_origin.0 + (self.vp_cursor_off.0 as usize);
        let row = buf_origin.1 + (self.vp_cursor_off.1 as usize);
        (col, row)
    }
}

impl Viewport {
    fn handle_event(&mut self, evnt: Event) -> Result<Option<Event>> {
        let cursor = self.buffer.handle_event(evnt)?;

        let (cdiff, rdiff) = match self.to_ed_cursor(self.buf_origin) {
            (old_c, old_r) => (
                (cursor.col_at as isize) - (old_c as isize),
                (cursor.row_at as isize) - (old_r as isize),
            ),
        };

        let ccol = ((self.col + self.vp_cursor_off.0) as isize) + cdiff;
        let crow = ((self.row + self.vp_cursor_off.1) as isize) + rdiff;

        let top = (self.to_top() + self.config.scroll_off) as isize;
        let bottom = (self.to_bottom() - self.config.scroll_off) as isize;

        let (vp_col, ed_col): (u16, usize) = if ccol < (self.to_left() as isize) {
            (0, cursor.col_at)
        } else if ccol > (self.to_right() as isize) {
            (self.width - 1, cursor.col_at - (self.width as usize) + 1)
        } else {
            let new_col: u16 = ccol.try_into().unwrap();
            (new_col - self.col, self.buf_origin.0)
        };
        let (vp_row, ed_row): (u16, usize) = if crow < top {
            (0, cursor.row_at)
        } else if crow > bottom {
            (self.height - 1, cursor.row_at - (self.height as usize) + 1)
        } else {
            let new_row: u16 = crow.try_into().unwrap();
            (new_row - self.row, self.buf_origin.1)
        };

        trace!(
            "buf_cursor:{:?} buf_origin:{:?}->{:?} vp_cursor:{:?}->{:?}",
            (cursor.col_at, cursor.row_at),
            self.buf_origin,
            (ed_col, ed_row),
            self.vp_cursor_off,
            (vp_col, vp_row)
        );

        self.buf_origin = (ed_col, ed_row);
        self.vp_cursor_off = (vp_col, vp_row);

        Ok(cursor.evnt)
    }
}
