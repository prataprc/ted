use crossterm::{cursor, queue};
use log::trace;

use std::{
    convert::TryInto,
    fmt,
    io::{self, Write},
    iter::FromIterator,
    ops::Bound,
    result,
};

use crate::{
    buffer::Buffer,
    config::Config,
    event::Event,
    window::{Coord, Cursor, Span, Window},
    Error, Result,
};

#[derive(Clone, Default)]
pub struct WindowPrompt {
    coord: Coord,
    prompt_lines: Vec<Span>,
    prompt_cursor: Cursor,
    rendered: false,

    input: String,
}

impl fmt::Display for WindowPrompt {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "WindowPrompt<{}>", self.coord)
    }
}

impl WindowPrompt {
    #[inline]
    pub fn new(
        cursor: Coord,
        prompt_lines: Vec<Span>,
        prompt_cursor: Cursor,
    ) -> Result<WindowPrompt> {
        Ok(WindowPrompt {
            coord,
            prompt_lines,
            prompt_cursor,
            rendered: false,

            input: Default::default(),
        })
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

    fn refresh(&mut self, buffer: &mut Buffer) -> Result<()> {
        if rendered == false {
            let mut stdout = io::stdout();
            for span in self.spans.iter() {
                err_at!(Fatal, queue!(stdout, span))?
            }
        } else {
            let span = Span::new(self.input, self.prompt_cursor);
            err_at!(Fatal, queue!(stdout, span))?;
            let n: usize =self.input.chars().map(|ch| ch.width()).sum();
            let Cursor { col, row } = self.prompt_cursor;
            err_at!(Fatal, queue!(stdout, cursor::MoveTo(col + n, row)))?;
        }
    }

    fn handle_event(
        //
        &mut self,
        buffer: &mut Buffer,
        evnt: Event,
    ) -> Result<Option<Event>> {
        match evnt {
            Event::Backspace => {
                self.input.pop();
                Ok(None)
            }
            Event::Enter => Ok(Event::PromptAns{ input: self.input }),
            Event::Char(ch, m) => {
                self.input.push(ch);
                Ok(None)
            }
        }
    }
}
