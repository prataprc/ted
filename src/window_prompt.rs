use crossterm::{cursor, queue, style::Color};
use unicode_width::UnicodeWidthChar;

use std::{
    convert::TryFrom,
    fmt,
    io::{self, Write},
    mem, result,
};

use crate::{
    event::{self, Event},
    window::{Coord, Cursor, Span, State},
    Error, Result,
};

#[derive(Clone, Default)]
pub struct WindowPrompt {
    coord: Coord,
    prompt_lines: Vec<Span>,
    prompt_cursor: Cursor,
    rendered: bool,

    input: String,
}

impl fmt::Display for WindowPrompt {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "WindowPrompt<{}>", self.coord)
    }
}

impl From<Vec<Span>> for WindowPrompt {
    fn from(spans: Vec<Span>) -> WindowPrompt {
        WindowPrompt::new(spans).ok().unwrap()
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
    pub fn new(prompt_lines: Vec<Span>) -> Result<WindowPrompt> {
        Ok(WindowPrompt {
            coord: Default::default(),
            prompt_lines,
            prompt_cursor: Default::default(),
            rendered: false,

            input: Default::default(),
        })
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
    pub fn on_refresh(&mut self, s: State) -> Result<State> {
        let mut stdout = io::stdout();

        if !self.rendered {
            let mut stdout = io::stdout();
            for span in self.prompt_lines.iter() {
                err_at!(Fatal, queue!(stdout, span))?;
            }
        } else {
            let span = {
                let mut span = Span::new(&self.input.clone());
                span.set_cursor(self.prompt_cursor);
                span
            };
            err_at!(Fatal, queue!(stdout, span))?;
            let n: usize = self.input.chars().map(|ch| ch.width().unwrap()).sum();
            let Cursor { mut col, row } = self.prompt_cursor;
            col += n as u16;
            err_at!(Fatal, queue!(stdout, cursor::MoveTo(col, row)))?;
        }

        Ok(s)
    }

    pub fn on_event(&mut self, mut s: State) -> Result<State> {
        s.event = match mem::replace(&mut s.event, Default::default()) {
            Event::Backspace => {
                self.input.pop();
                Event::Noop
            }
            Event::Enter => Event::PromptReply {
                input: self.input.clone(),
            },
            Event::Char(ch, _m) => {
                self.input.push(ch);
                Event::Noop
            }
            _ => Event::Noop,
        };
        Ok(s)
    }
}
