use crossterm::{cursor as term_cursor, queue};

use std::{
    fmt,
    io::{self, Write},
    iter::FromIterator,
    result,
};

use crate::{
    buffer::{self, Buffer},
    code::App,
    color_scheme::Highlight,
    event::Event,
    location::Location,
    window::{Coord, Cursor, Span},
    Error, Result,
};

#[derive(Clone)]
pub struct WindowLine {
    name: String,
    coord: Coord,
    inner: Inner,
}

#[derive(Clone)]
enum Inner {
    Cmd {
        cursor: Cursor,
        obc_xy: buffer::Cursor,
        buffer: Buffer,
    },
    Status {
        spans: Vec<Span>,
    },
    Tab {
        spans: Vec<Span>,
    },
}

impl fmt::Display for Inner {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        match self {
            Inner::Cmd { .. } => write!(f, "cmd"),
            Inner::Status { .. } => write!(f, "status"),
            Inner::Tab { .. } => write!(f, "tab"),
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
    pub fn new_cmd(coord: Coord) -> WindowLine {
        use crate::code::view::NoWrap;

        let name = "cmdline".to_string();
        let buffer = {
            let loc = Location::new_ted(&name);
            Buffer::from_reader(vec![].as_slice(), loc).unwrap()
        };
        let cursor = NoWrap::initial_cursor(false /*line_number*/);
        let obc_xy = (0, 0).into();
        WindowLine {
            name,
            coord,
            inner: Inner::Cmd {
                cursor,
                obc_xy,
                buffer,
            },
        }
    }

    #[inline]
    pub fn new_status(coord: Coord) -> WindowLine {
        let spans = vec![];
        WindowLine {
            name: "stsline".to_string(),
            coord,
            inner: Inner::Status { spans },
        }
    }

    #[inline]
    pub fn new_tab(coord: Coord) -> WindowLine {
        let spans = vec![];
        WindowLine {
            name: "tbcline".to_string(),
            coord,
            inner: Inner::Tab { spans },
        }
    }
}

impl WindowLine {
    #[inline]
    pub fn to_cursor(&self) -> Cursor {
        match self.inner {
            Inner::Cmd { cursor, .. } => self.coord.to_top_left() + cursor,
            Inner::Status { .. } => unreachable!(),
            Inner::Tab { .. } => unreachable!(),
        }
    }

    pub fn on_event(&mut self, _app: &mut App, evnt: Event) -> Result<Event> {
        match &mut self.inner {
            Inner::Cmd { buffer, .. } => match evnt {
                Event::Esc => Ok(Event::Esc),
                evnt => buffer.on_event(evnt),
            },
            Inner::Status { .. } => Ok(evnt),
            Inner::Tab { .. } => Ok(evnt),
        }
    }

    pub fn on_refresh(&mut self, app: &mut App) -> Result<()> {
        use crate::code::view::NoWrap;
        use std::iter::repeat;

        let mut stdout = io::stdout();

        let (col, row) = self.coord.to_origin_cursor();
        err_at!(Fatal, queue!(stdout, term_cursor::MoveTo(col, row)))?;
        match &mut self.inner {
            Inner::Cmd {
                buffer,
                obc_xy,
                cursor,
            } => {
                let (name, coord) = (&self.name, self.coord);
                let mut v = NoWrap::new(name, coord, *cursor, *obc_xy);
                v.set_scroll_off(0).set_line_number(false);
                *cursor = v.render(buffer, app.as_color_scheme())?;
            }
            Inner::Status { spans } => {
                for span in spans.iter() {
                    err_at!(Fatal, queue!(stdout, span))?;
                }
                let padding = {
                    let (_, wth) = self.coord.to_size();
                    let n: usize = spans.iter().map(|span| span.to_width()).sum();
                    let iter = repeat(' ').take((wth as usize) - n);
                    let padding: Span = String::from_iter(iter).into();
                    let scheme = app.as_color_scheme();
                    padding.using(scheme.to_style(Highlight::StatusLine))
                };
                err_at!(Fatal, queue!(stdout, padding))?;
            }
            Inner::Tab { spans } => {
                for span in spans.iter() {
                    err_at!(Fatal, queue!(stdout, span))?;
                }
                let padding = {
                    let (_, wth) = self.coord.to_size();
                    let n: usize = spans.iter().map(|span| span.to_width()).sum();
                    let iter = repeat(' ').take((wth as usize) - n);
                    let padding: Span = String::from_iter(iter).into();
                    let scheme = app.as_color_scheme();
                    padding.using(scheme.to_style(Highlight::TabLine))
                };
                err_at!(Fatal, queue!(stdout, padding))?;
            }
        };

        Ok(())
    }
}
