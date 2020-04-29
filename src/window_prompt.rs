use crossterm::{cursor, queue, style::Color};
use unicode_width::UnicodeWidthChar;

use std::{
    convert::TryFrom,
    fmt,
    io::{self, Write},
    result,
};

use crate::{
    event::{self, Event},
    window::{Context, Coord, Cursor, Span, Window},
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

impl Window for WindowPrompt {
    #[inline]
    fn to_origin(&self) -> (u16, u16) {
        self.coord.to_origin()
    }

    #[inline]
    fn to_cursor(&self) -> Cursor {
        self.prompt_cursor
    }

    #[inline]
    fn move_by(&mut self, col_off: i16, row_off: i16, _: &Context) {
        self.coord = self.coord.clone().move_by(col_off, row_off);
    }

    #[inline]
    fn resize_to(&mut self, height: u16, width: u16, _: &Context) {
        self.coord = self.coord.clone().resize_to(height, width);
    }

    fn refresh(&mut self, _context: &mut Context) -> Result<()> {
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

        Ok(())
    }

    fn handle_event(
        //
        &mut self,
        _context: &mut Context,
        evnt: Event,
    ) -> Result<Option<Event>> {
        match evnt {
            Event::Backspace => {
                self.input.pop();
                Ok(None)
            }
            Event::Enter => Ok(Some(Event::PromptReply {
                input: self.input.clone(),
            })),
            Event::Char(ch, _m) => {
                self.input.push(ch);
                Ok(None)
            }
            _ => Ok(Some(evnt)),
        }
    }
}
