use log::trace;

use std::{result, fmt, ops::{self, RangeBounds}, convert::TryInto};

#[derive(Clone, Debug, Default)]
pub struct Viewport {
    col: u16,
    row: u16,
    height: u16,
    width: u16,
    ed_origin: (usize, usize), // absolute (col, row) within buffer, (0,0)
    vp_cursor_off: (u16, u16), // (col-offset, row-offset)
    scroll_off: u16,
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
    pub fn new(col: u16, row: u16, height: u16, width: u16) -> Viewport {
        Viewport {
            col,
            row,
            height,
            width,
            ed_origin: Default::default(),
            vp_cursor_off: Default::default(),
            scroll_off: Default::default(),
        }
    }

    #[inline]
    pub fn set_scroll_off(&mut self, scroll_off: u16) -> &mut Self {
        self.scroll_off = scroll_off;
        self
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
    pub fn to_ed_origin(&self) -> (usize, usize) {
        self.ed_origin
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

    fn to_ed_cursor(&self, ed_origin: (usize, usize)) -> (usize, usize) {
        let col = ed_origin.0 + (self.vp_cursor_off.0 as usize);
        let row = ed_origin.1 + (self.vp_cursor_off.1 as usize);
        (col, row)
    }

    pub fn apply_ed_cursor(&mut self, ed_cursor: (usize, usize)) {
        let (cdiff, rdiff) = match (self.to_ed_cursor(self.ed_origin), ed_cursor) {
            ((old_c, old_r), (new_c, new_r)) => (
                (new_c as isize) - (old_c as isize),
                (new_r as isize) - (old_r as isize),
            ),
        };

        let ccol = ((self.col + self.vp_cursor_off.0) as isize) + cdiff;
        let crow = ((self.row + self.vp_cursor_off.1) as isize) + rdiff;

        let top = (self.to_top() + self.scroll_off) as isize;
        let bottom = (self.to_bottom() - self.scroll_off) as isize;

        let (vp_col, ed_col): (u16, usize) = if ccol < (self.to_left() as isize) {
            (0, ed_cursor.0)
        } else if ccol > (self.to_right() as isize) {
            (self.width - 1, ed_cursor.0 - (self.width as usize) + 1)
        } else {
            let new_col: u16 = ccol.try_into().unwrap();
            (new_col - self.col, self.ed_origin.0)
        };
        let (vp_row, ed_row): (u16, usize) = if crow < top {
            (0, ed_cursor.1)
        } else if crow > bottom {
            (self.height - 1, ed_cursor.1 - (self.height as usize) + 1)
        } else {
            let new_row: u16 = crow.try_into().unwrap();
            (new_row - self.row, self.ed_origin.1)
        };

        trace!(
            "ed_cursor:{:?} ed_origin:{:?}->{:?} vp_cursor:{:?}->{:?}",
            ed_cursor,
            self.ed_origin,
            (ed_col, ed_row),
            self.vp_cursor_off,
            (vp_col, vp_row)
        );

        self.ed_origin = (ed_col, ed_row);
        self.vp_cursor_off = (vp_col, vp_row);
    }
}

