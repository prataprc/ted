use crossterm::{queue, style::Color};
use unicode_width::UnicodeWidthChar;

use std::{
    convert::TryFrom,
    fmt,
    io::{self, Write},
    result,
};

use crate::{
    buffer::Buffer,
    event::{self, Event, DP},
    state::Context,
    window::{Coord, Cursor, Span},
    Error, Result,
};

#[derive(Clone, Default)]
pub struct WindowPrompt {
    coord: Coord,
    prompt_lines: Vec<Span>,
    prompt_cursor: Cursor,

    buffer: Option<Buffer>,
}

impl TryFrom<WindowPrompt> for Event {
    type Error = Error;

    fn try_from(w: WindowPrompt) -> Result<Event> {
        Ok(Event::Prompt(
            w.buffer
                .as_ref()
                .map(|b| b.to_string())
                .unwrap_or("".to_string()),
        ))
    }
}

impl fmt::Display for WindowPrompt {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "WindowPrompt<{}>", self.coord)
    }
}

impl From<Vec<Span>> for WindowPrompt {
    fn from(spans: Vec<Span>) -> WindowPrompt {
        WindowPrompt::new(spans)
    }
}

impl TryFrom<event::OpenFile> for WindowPrompt {
    type Error = Error;

    fn try_from(of: event::OpenFile) -> Result<WindowPrompt> {
        let fg = Color::AnsiValue(9);
        let bg = Color::AnsiValue(15);
        let spans = match of {
            event::OpenFile::ReadOnly(_, file) => vec![
                span!(fg: fg, bg: bg, "-- Read only file {:?}", file),
                span!(fg: fg, bg: bg, "-- Press c or space to continue --"),
            ],
            event::OpenFile::NotFound(file) => vec![
                span!(fg: fg, bg: bg, "-- File not found {:?}", file),
                span!(fg: fg, bg: bg, "-- Press c or space to continue --"),
            ],
            event::OpenFile::NoPermission(file) => vec![
                span!(fg: fg, bg: bg, "-- Permission denied {:?}", file),
                span!(fg: fg, bg: bg, "-- Press c or space to continue --"),
            ],
            event::OpenFile::ReadWrite(_, file) => {
                err_at!(FailConvert, msg: format!("{:?}", file))?;
                unreachable!()
            }
        };
        Ok(spans.into())
    }
}

impl WindowPrompt {
    #[inline]
    pub fn new(prompt_lines: Vec<Span>) -> WindowPrompt {
        let mut w = WindowPrompt {
            coord: Default::default(),
            prompt_lines,
            prompt_cursor: Default::default(),

            buffer: Some(Buffer::empty()),
        };
        w.buffer.as_mut().map(|b| b.mode_insert().unwrap());
        w
    }

    pub fn set_coord(&mut self, coord: Coord) -> &mut Self {
        self.coord = coord;
        self
    }

    pub fn set_cursor(&mut self, cursor: Cursor) -> &mut Self {
        self.prompt_cursor = cursor;
        self
    }
}

impl WindowPrompt {
    #[inline]
    pub fn to_cursor(&self) -> Cursor {
        let n: usize = match &self.buffer {
            Some(buffer) => {
                let iter = buffer.chars_at(0, DP::Right).unwrap();
                iter.map(|ch| ch.width().unwrap()).sum()
            }
            None => 0,
        };
        let (col, row) = {
            let Cursor { col, row } = self.prompt_cursor;
            ((col + n as u16), row)
        };
        Cursor::new(col, row)
    }

    pub fn on_event(&mut self, c: &mut Context, evnt: Event) -> Result<Event> {
        match evnt {
            Event::Esc => Ok(Event::Noop),
            evnt => {
                c.buffer = self.buffer.take();
                let evnt = Buffer::on_event(c, evnt)?;
                self.buffer = c.buffer.take();
                Ok(evnt)
            }
        }
    }

    pub fn on_refresh(&mut self, _: &mut Context) -> Result<()> {
        let mut stdout = io::stdout();

        let span = {
            let mut span: Span = self
                .buffer
                .as_ref()
                .map(|b| b.to_string())
                .unwrap_or("".to_string())
                .into();
            span.set_cursor(self.prompt_cursor);
            span
        };
        err_at!(Fatal, queue!(stdout, span))?;
        Ok(())
    }
}
