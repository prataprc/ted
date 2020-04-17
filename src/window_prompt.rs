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
    w_coord: Coord,                     // x window coord.
    prompt_lines: Span,
    prompt_cursor: Cursor,
    input: String,
}

impl fmt::Display for WindowPrompt {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "WindowPrompt<{}>", self.w_coord)
    }
}

impl WindowPrompt {
    #[inline]
    pub fn new(cursor: Coord, prompt_lines: Vec<Span>, prompt_cursor: Cursor) -> Result<WindowPrompt> {
        Ok(WindowPrompt {
            w_coord: coord,
            prompt_lines,
            prompt_cursor,
            input: Default::default(),
        })
    }
}

impl Window for WindowPrompt {
    #[inline]
    fn to_origin(&self) -> (u16, u16) {
        self.w_coord.to_origin()
    }

    #[inline]
    fn to_cursor(&self) -> Cursor {
        self.prompt_cursor
    }

    fn refresh(&mut self, buffer: &mut Buffer) -> Result<()> {
        Ok(())
    }

    fn handle_event(
        &mut self,
        buffer: &mut Buffer,
        evnt: Event,
    ) -> Result<Option<Event>> {
        match evnt {
            Event::Backspace => self.input.pop(),
            Event::Enter => self.input.pop(),
            Event::Char(ch, m) => self.input.push(ch),
        }
    }
}

