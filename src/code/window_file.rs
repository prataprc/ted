use crossterm::queue;

use std::{
    ffi, fmt,
    io::{self, Write},
    iter::FromIterator,
    result,
};

use crate::{
    app::Application,
    buffer::Buffer,
    code::window_edit::WindowEdit,
    code::{config::Config, Code},
    event::Event,
    term::Span,
    window::{Coord, Cursor, Window},
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
pub struct WindowFile {
    coord: Coord, // x window coord.
    we: WindowEdit,
    // stsline: Option<WindowLine>, TODO: needed for split windows.
}

impl fmt::Display for WindowFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "WindowFile<{}>", self.coord)
    }
}

impl WindowFile {
    #[inline]
    pub fn new(app: &Code, coord: Coord, buf: &Buffer, config: &Config) -> WindowFile {
        WindowFile {
            coord,
            we: WindowEdit::new(app, coord.clone(), buf, config),
            // stsline: None,
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

    fn status_file(&self, app: &Code) -> Result<Span> {
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
        let ft = self.we.to_file_type();

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

    fn do_refresh(&mut self, app: &Code) -> Result<()> {
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
                err_at!(Fatal, queue!(stdout, span!((col, row), st: string)))?;
            }
        }

        Ok(())
    }
}

impl Window for WindowFile {
    type App = Code;

    #[inline]
    fn to_cursor(&self) -> Cursor {
        self.we.to_cursor()
    }

    fn on_event(&mut self, app: &mut Code, evnt: Event) -> Result<Event> {
        use crate::{event::Code::StatusFile, pubsub::Notify};

        match self.we.on_event(app, evnt)? {
            Event::Code(StatusFile { .. }) => {
                let span = self.status_file(app)?;
                app.notify("code", Notify::Status(vec![span]))?;
                Ok(Event::Noop)
            }
            evnt => Ok(evnt),
        }
    }

    fn on_refresh(&mut self, app: &mut Code) -> Result<()> {
        self.do_refresh(app)?;
        self.we.on_refresh(app)
    }
}
