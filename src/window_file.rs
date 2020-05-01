use crossterm::queue;

use std::{
    fmt,
    io::{self, Write},
    iter::FromIterator,
    mem, result,
};

use crate::{
    buffer::Buffer,
    event::Event,
    window::{Coord, Cursor, Span, State},
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
    pub fn new(coord: Coord) -> WindowFile {
        let we = WindowEdit::new(coord.clone());
        WindowFile {
            coord,
            we,
            we_hgt: 0,
            we_wth: 0,
        }
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

    fn do_refresh(&mut self, s: State) -> Result<State> {
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

        Ok(s)
    }
}

impl WindowFile {
    #[inline]
    fn to_origin(&self) -> (u16, u16) {
        self.coord.to_origin()
    }

    #[inline]
    pub fn to_cursor(&self) -> Cursor {
        self.we.to_cursor()
    }

    pub fn on_refresh(&mut self, mut s: State) -> Result<State> {
        s = self.do_refresh(s)?;
        self.we.on_refresh(s)
    }

    pub fn on_event(&mut self, mut s: State) -> Result<State> {
        let evnt = mem::replace(&mut s.event, Default::default());
        s.event = match evnt {
            Event::NewBuffer => {
                let (buffer_id, buffer) = {
                    let mut b = Buffer::empty()?;
                    b.as_mut_context().set_location(Default::default());
                    (b.to_id(), b)
                };
                s.buffers.push(buffer);
                Event::UseBuffer { buffer_id }
            }
            evnt => evnt,
        };
        self.we.on_event(s)
    }
}
