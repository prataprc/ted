use crossterm::queue;

use std::{fmt, iter::FromIterator, result};

use crate::{
    app::Application,
    buffer::Buffer,
    code::window_edit::WindowEdit,
    code::{config::Config, Code},
    colors::ColorScheme,
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
}

impl fmt::Display for WindowFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "WindowFile<{}>", self.coord)
    }
}

impl From<(Coord, Config, &Buffer, &ColorScheme)> for WindowFile {
    fn from((coord, config, buf, scheme): (Coord, Config, &Buffer, &ColorScheme)) -> WindowFile {
        WindowFile {
            coord,
            we: WindowEdit::new(coord, config, buf, scheme),
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
        let alt = format!("--display-error--");
        let b = {
            let id = self.we.to_buffer_id();
            app.as_buffer(&id)
                .ok_or(Error::Invalid(format!("buffer {}", id)))
        }?;

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
        let ft = self.we.to_text_type();

        let span: Span = {
            let long_ver = format!("{:?} {} [{}]", l_name, fstt, ft);
            let shrt_ver = format!("{:?} {} [{}]", s_name, fstt, ft);
            let n = long_ver.chars().collect::<Vec<char>>().len();
            let st = if_else!(n > (self.coord.wth as usize), shrt_ver, long_ver);
            st.into()
        };

        Ok(span)
    }

    fn do_refresh(&mut self, app: &Code) -> Result<()> {
        use std::iter::repeat;

        if self.is_top_margin() {
            let iter = repeat(app.as_ref().top_margin_char);
            let span = {
                let st = String::from_iter(iter.take(self.coord.wth as usize));
                let mut span: Span = st.into();
                span.set_cursor(self.coord.to_top_left());
                span
            };
            err_at!(Fatal, termqu!(span))?;
        }
        if self.is_left_margin() {
            let st = app.as_ref().left_margin_char.to_string();
            for _i in 0..self.coord.hgt {
                let mut span: Span = st.clone().into();
                span.set_cursor(self.coord.to_top_left());
                err_at!(Fatal, termqu!(span))?;
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
