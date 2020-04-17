use crossterm::{cursor, queue};
use unicode_width::UnicodeWidthChar;

use std::{
    fmt,
    io::{self, Write},
    result,
};

use crate::{
    event::Event,
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

impl WindowPrompt {
    #[inline]
    pub fn new(
        coord: Coord,
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

    pub fn set_prompt(&mut self, prompt_lines: Vec<Span>, prompt_cursor: Cursor) {
        self.prompt_lines = prompt_lines;
        self.prompt_cursor = prompt_cursor;
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

    fn refresh(&mut self, _context: &mut Context) -> Result<()> {
        let mut stdout = io::stdout();

        if !self.rendered {
            let mut stdout = io::stdout();
            for span in self.prompt_lines.iter() {
                err_at!(Fatal, queue!(stdout, span))?;
            }
        } else {
            let span = Span::new(self.input.clone(), self.prompt_cursor);
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
            Event::Enter => Ok(Some(Event::PromptAns {
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
