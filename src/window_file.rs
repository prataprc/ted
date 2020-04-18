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
    config::Config,
    cursor,
    event::Event,
    window::{Context, Coord, Cursor, Span, Window},
    window_edit::WindowEdit;
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
pub struct WindowFile {
    coord: Coord, // x window coord.
    we: WindowEdit,
    nu_begin: Option<usize>,
    config: Config,
}

impl fmt::Display for WindowFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "WindowFile<{}>", self.coord)
    }
}

impl WindowFile {
    #[inline]
    pub fn new(coord: Coord, config: Config) -> Result<WindowFile> {
        let we = WindowEdit::new(coord.clone, config.clone())?;
        Ok(WindowFile {
            coord,
            we,
            nu_begin: None,
            config,
        })
    }

    #[inline]
    fn move_by(mut self, col_off: i16, row_off: i16) -> Self {
        self.coord = self.coord.move_by(col_off, row_off);
        self.nu_begin = None;
        self
    }

    #[inline]
    fn resize_to(mut self, height: u16, width: u16) -> Self {
        self.coord = self.coord.resize_to(height, width);
        self.nu_begin = None;
        self
    }
}

impl WindowFile {
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

    fn do_refresh(&mut self, context: &mut Context) -> Result<()> {
        use std::iter::repeat;

        let (_, mut line_no) = self.we.visual_cursor();
        line_no += 1;

        trace!("{}", self.coord);

        let mut stdout = io::stdout();
        let (mut col, mut row) = self.coord.to_top_left();
        let (mut height, width) = self.coord.to_size();

        if self.is_top_margin() {
            let ch = self.config.top_margin_char;
            let span = span!(
                (col, row),
                s: String::from_iter(repeat(ch).take(self.coord.wth))
            );
            err_at!(Fatal, queue!(stdout, span))?;
            row += 1;
            height -= 1;
            self.we.move_by(0, 1).resize_to(height, width)
        }

        let (from, to) = (
            Bound::Included(self.line_no),
            Bound::Included(self.line_no + (self.coord.hgt as usize) - 1),
        );

        let (n, m) = {
            let iter = self.we.iter_lines(from, to, context).take(height);
            let lines: Vec<(usize, String)> = iter.collect();
            (lines.len(), (height as usize) - lines.len())
        };

        {
            let xc = (line_no + n).to_string().chars().collect::Vec<char>().len();
            self.we.move_by(xc, 0).resize_to(col-xc-1);
        };

        self.we = WindowEdit::new(coord, 

        trace!("left header {} {}", n, m);
        for _i in 0..n {
            let mut l: String = Default::default();
            if self.is_left_margin() {
                l.push(self.config.left_margin_char);
            }
            if self.config.line_number {
                l.push_str(&format!("{} ", line_no));
            }
            err_at!(Fatal, queue!(stdout, span!((col, row), s: l)))?;
            row += 1;
        }
        for _i in 0..m {
            let mut l: String = Default::default();
            if self.is_left_margin() {
                l.push(self.config.left_margin_char);
            }
            if self.config.line_number {
                l.push('~');
            }
            err_at!(Fatal, queue!(stdout, span!((col, row), s: l)))?;
            row += 1;
        }
    }
}

impl Window for WindowFile {
    #[inline]
    fn to_origin(&self) -> (u16, u16) {
        self.w_coord.to_origin()
    }

    #[inline]
    fn to_cursor(&self) -> Cursor {
        self.bw_coord.to_top_left() + self.bw_cursor
    }

    fn refresh(&mut self, context: &mut Context) -> Result<()> {
        let (_, mut line_no) = self.we.visual_cursor();
        line_no += 1;

        match self.nu_begin {
            Some(nu_begin) if nu_begin == line_no => Ok(()),
            _ => self.do_refresh(context),
        }
    }

    fn handle_event(
        //
        &mut self,
        context: &mut Context,
        evnt: Event,
    ) -> Result<Option<Event>> {
        self.we.handle_event(context, evnt)
    }
}
