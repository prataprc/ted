use crossterm::queue;

use std::{
    fmt,
    io::{self, Write},
    iter::FromIterator,
    result,
};

use crate::{
    event::Event,
    window::{Coord, Cursor, Span, State, Window},
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
    pub fn new(coord: Coord) -> Result<WindowFile> {
        let we = WindowEdit::new(coord.clone())?;
        Ok(WindowFile {
            coord,
            we,
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

    fn do_refresh(&mut self, s: &mut State) -> Result<()> {
        use std::iter::repeat;

        let Cursor { col, row } = self.coord.to_top_left();
        let (hgt, _) = self.coord.to_size();
        let mut stdout = io::stdout();

        if self.is_top_margin() {
            let iter = repeat(s.config.top_margin_char);
            let span = span!(
                (col, row),
                st: String::from_iter(iter.take(self.coord.wth as usize))
            );
            err_at!(Fatal, queue!(stdout, span))?;
        }
        if self.is_left_margin() {
            let st = s.config.left_margin_char.to_string();
            for _i in 0..hgt {
                err_at!(Fatal, queue!(stdout, span!((col, row), st: st)))?;
            }
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
    fn move_by(&mut self, _: &State, col_off: i16, row_off: i16) {
        self.coord = self.coord.clone().move_by(col_off, row_off);
    }

    #[inline]
    fn resize_to(&mut self, _: &State, height: u16, width: u16) {
        self.coord = self.coord.clone().resize_to(height, width);
    }

    fn refresh(&mut self, s: &mut State) -> Result<()> {
        self.do_refresh(s)?;
        self.we.refresh(s)
    }

    fn on_event(&mut self, s: &mut State, evnt: Event) -> Result<Event> {
        self.we.on_event(s, evnt)
    }
}
