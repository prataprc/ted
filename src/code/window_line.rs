use crossterm::{cursor as term_cursor, queue};
#[allow(unused_imports)]
use log::trace;

use std::{fmt, iter::FromIterator, mem, result};

use crate::{
    buffer::{self, Buffer},
    code,
    colors::{ColorScheme, Highlight},
    event::Event,
    term::{Span, Spanline},
    window::{Coord, Cursor, Render, Window},
    Error, Result,
};

pub struct WindowLine {
    name: String,
    coord: Coord,
    inner: Inner,
}

enum Inner {
    Status { spans: Vec<Span> },
    Tab { spans: Vec<Span> },
    None,
}

impl Default for Inner {
    fn default() -> Inner {
        Inner::None
    }
}

impl fmt::Display for Inner {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        match self {
            Inner::Status { .. } => write!(f, "status"),
            Inner::Tab { .. } => write!(f, "tab"),
            Inner::None => write!(f, "none"),
        }
    }
}

impl fmt::Display for WindowLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "WindowLine<{},{},{}>", self.name, self.coord, self.inner)
    }
}

impl WindowLine {
    #[inline]
    pub fn new_status(coord: Coord, _: &code::Code) -> WindowLine {
        let spans = vec![];
        WindowLine {
            name: "stsline".to_string(),
            coord,
            inner: Inner::Status { spans },
        }
    }

    #[inline]
    pub fn new_tab(coord: Coord, _: &code::Code) -> WindowLine {
        let spans = vec![];
        WindowLine {
            name: "tbcline".to_string(),
            coord,
            inner: Inner::Tab { spans },
        }
    }
}

impl Window for WindowLine {
    type App = code::Code;

    #[inline]
    fn to_name(&self) -> String {
        "window-line".to_string()
    }

    #[inline]
    fn to_coord(&self) -> Coord {
        self.coord
    }

    #[inline]
    fn to_cursor(&self) -> Cursor {
        match self.inner {
            Inner::Status { .. } => unreachable!(),
            Inner::Tab { .. } => unreachable!(),
            Inner::None => unreachable!(),
        }
    }

    #[inline]
    fn config_line_number(&self) -> bool {
        false
    }

    #[inline]
    fn config_scroll_offset(&self) -> u16 {
        0
    }

    fn on_event(&mut self, _app: &mut code::Code, evnt: Event) -> Result<Event> {
        match &mut self.inner {
            Inner::Status { .. } => Ok(evnt),
            Inner::Tab { .. } => Ok(evnt),
            Inner::None => Ok(evnt),
        }
    }

    fn on_refresh(&mut self, app: &mut code::Code) -> Result<()> {
        use std::iter::repeat;

        let scheme = app.to_color_scheme(None);

        let (col, row) = self.coord.to_origin_cursor();
        err_at!(Fatal, termqu!(term_cursor::MoveTo(col, row)))?;

        let mut inner = mem::replace(&mut self.inner, Default::default());
        match &mut inner {
            Inner::Status { spans } => {
                for span in spans.iter() {
                    err_at!(Fatal, termqu!(span))?;
                }
                let padding = {
                    let n: usize = spans.iter().map(|span| span.to_width()).sum();
                    let iter = repeat(' ').take((self.coord.wth as usize) - n);
                    let padding: Span = String::from_iter(iter).into();
                    padding.using(scheme.to_style(Highlight::StatusLine))
                };
                err_at!(Fatal, termqu!(padding))?;
            }
            Inner::Tab { spans } => {
                for span in spans.iter() {
                    err_at!(Fatal, termqu!(span))?;
                }
                let padding = {
                    let n: usize = spans.iter().map(|span| span.to_width()).sum();
                    let iter = repeat(' ').take((self.coord.wth as usize) - n);
                    let padding: Span = String::from_iter(iter).into();
                    padding.using(scheme.to_style(Highlight::TabLine))
                };
                err_at!(Fatal, termqu!(padding))?;
            }
            Inner::None => (),
        };
        self.inner = inner;

        Ok(())
    }
}

impl Render for WindowLine {
    type Buf = Buffer;

    #[inline]
    fn as_color_scheme(&self) -> &ColorScheme {
        unreachable!()
    }

    #[inline]
    fn to_span_line(&self, buf: &Self::Buf, a: usize, z: usize) -> Result<Spanline> {
        buffer::to_span_line(buf, a, z)
    }
}
