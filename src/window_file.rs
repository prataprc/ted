use crossterm::queue;
use log::trace;

use std::{
    cmp, fmt,
    io::{self, Write},
    iter::FromIterator,
    ops::Bound,
    result,
};

use crate::{
    config::Config,
    event::Event,
    window::{Context, Coord, Cursor, Span, Window},
    window_edit::WindowEdit,
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
    config: Config,
    // cached parameters.
    we_hgt: i16,
    we_wth: i16,
}

impl fmt::Display for WindowFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "WindowFile<{}>", self.coord)
    }
}

impl WindowFile {
    #[inline]
    pub fn new(coord: Coord, config: Config) -> Result<WindowFile> {
        let we = WindowEdit::new(coord.clone(), config.clone())?;
        Ok(WindowFile {
            coord,
            we,
            config,
            we_hgt: 0,
            we_wth: 0,
        })
    }
}

impl WindowFile {
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

    fn do_refresh(
        &mut self,
        line_no: usize,
        lines: Vec<(usize, String)>,
        _: &mut Context,
    ) -> Result<()> {
        use std::iter::repeat;

        let mut stdout = io::stdout();

        let Cursor { col, mut row } = self.coord.to_top_left();
        let (height, _) = self.coord.to_size();

        if self.is_top_margin() {
            let ch = self.config.top_margin_char;
            let span = span!(
                (col, row),
                s: String::from_iter(repeat(ch).take(self.coord.wth as usize))
            );
            err_at!(Fatal, queue!(stdout, span))?;
        }

        let n = cmp::max(lines.len(), 1);
        let m = (height as usize) - n;

        trace!(
            "line_no:{} n_lines:{} n:{} m:{}",
            line_no,
            lines.len(),
            n,
            m
        );

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

        Ok(())
    }
}

impl Window for WindowFile {
    #[inline]
    fn to_origin(&self) -> (u16, u16) {
        self.coord.to_origin()
    }

    #[inline]
    fn to_cursor(&self) -> Cursor {
        self.we.to_cursor()
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
        let (height, _) = self.coord.to_size();

        let (_, mut line_no) = self.we.visual_cursor(context);
        line_no += 1;

        let (from, to) = (
            Bound::Included(line_no),
            Bound::Included(line_no + (height as usize) - 1),
        );

        let iter = self.we.to_lines(from, to, context).take(height as usize);
        let lines: Vec<(usize, String)> = iter.collect();
        let n_lines = lines.len();

        self.do_refresh(line_no, lines, context)?;

        {
            let (we_hgt, h): (isize, isize) = if self.is_top_margin() {
                (1, -1)
            } else {
                (0, 0)
            };
            let (mut we_wth, mut w): (isize, isize) = if self.is_left_margin() {
                (1, -1)
            } else {
                (0, 0)
            };
            if self.config.line_number {
                let n = format!("{} ", line_no + n_lines).len() as isize;
                we_wth += n;
                w -= n;
            }
            let (mut we_height, mut we_width) = self.coord.to_size();
            we_height = ((we_height as isize) + h) as u16;
            we_width = ((we_width as isize) + w) as u16;

            trace!(
                "{} move_by:({},{}), resize_to:({},{})",
                self.coord,
                we_wth,
                we_hgt,
                we_height,
                we_width,
            );

            self.we.move_by(-self.we_wth, -self.we_hgt, context);
            self.we.move_by(we_wth as i16, we_hgt as i16, context);
            self.we_wth = we_wth as i16;
            self.we_hgt = we_hgt as i16;
            self.we.resize_to(we_height, we_width, context);
        }
        self.we.refresh(context)
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
