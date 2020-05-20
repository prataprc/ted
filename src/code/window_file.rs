use crossterm::queue;

use std::{
    ffi, fmt,
    io::{self, Write},
    iter::FromIterator,
    result,
};

use crate::{
    buffer::Buffer,
    code::App,
    code::{new_window_line, window_edit::WindowEdit, window_line::WindowLine},
    event::{Event, Ted},
    window::{Coord, Cursor, Span},
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
    stsline: WindowLine,
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
        WindowFile {
            coord,
            we: WindowEdit::new(coord.clone()),
            stsline: new_window_line("stsline", coord),
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

    #[inline]
    fn to_origin(&self) -> (u16, u16) {
        self.coord.to_origin()
    }

    fn status_line(&self, app: &App) -> Result<Span> {
        let alt: ffi::OsString = "--display-error--".into();
        let (hgt, wth) = self.coord.to_size();
        let b = app.as_buffer(&self.we.to_buffer_id());

        let l_name = {
            let loc = b.to_location();
            loc.to_long_string().unwrap_or(alt.clone())
        };
        let s_name = {
            let loc = b.to_location();
            loc.to_short_string().unwrap_or(alt.clone())
        };
        let fstt = {
            let mut ss = vec![];
            if b.is_read_only() {
                ss.push("read-only")
            } else if b.is_modified() {
                ss.push("modified")
            }
            ss.join(", ")
        };
        let ft = b.to_file_type();

        let long_ver = format!("{:?} {} [{}]", l_name, fstt, ft);
        let shrt_ver = format!("{:?} {} [{}]", s_name, fstt, ft);
        let n = long_ver.chars().collect::<Vec<char>>().len();

        let (col, mut row) = self.coord.to_origin_cursor();
        row += hgt - 1;
        Ok(span!(
            (col, row),
            "{}",
            if_else!(n > (wth as usize), shrt_ver, long_ver)
        ))
    }

    fn do_refresh(&mut self, app: &App) -> Result<()> {
        use std::iter::repeat;

        let Cursor { col, row } = self.coord.to_top_left();
        let (hgt, _) = self.coord.to_size();
        let mut stdout = io::stdout();

        if self.is_top_margin() {
            let iter = repeat(app.as_ref().top_margin_char);
            let span = span!(
                (col, row),
                st: String::from_iter(iter.take(self.coord.wth as usize))
            );
            err_at!(Fatal, queue!(stdout, span))?;
        }
        if self.is_left_margin() {
            let st = app.as_ref().left_margin_char.to_string();
            for _i in 0..hgt {
                let string = st.clone();
                err_at!(Fatal, queue!(stdout, span!((col, row), st: sting)))?;
            }
        }

        Ok(())
    }
}

impl WindowFile {
    #[inline]
    pub fn to_cursor(&self) -> Cursor {
        self.we.to_cursor()
    }

    pub fn on_event(&mut self, app: &mut App, mut evnt: Event) -> Result<Event> {
        use crate::event::Event::Td;

        match self.we.on_event(app, evnt)? {
            Td(Ted::StatusFile { .. }) => {
                let span = self.status_line(app);
                app.notify("code", Notify::Status(span));
                Ok(Event::Noop)
            }
            evnt => Ok(evnt),
        }
    }

    pub fn on_refresh(&mut self, app: &mut App) -> Result<()> {
        self.do_refresh(app)?;
        self.we.on_refresh(app)?
    }
}
