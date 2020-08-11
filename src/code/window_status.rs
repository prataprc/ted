#[allow(unused_imports)]
use log::trace;

use std::{fmt, iter::FromIterator, result};

use crate::{
    code,
    event::Event,
    term::{Span, Spanline},
    window::{Coord, Cursor, Window},
    Error, Result,
};

pub struct WindowStatus {
    coord: Coord,
    spans: Vec<Span>,
}

impl fmt::Display for WindowStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "WindowStatus<{},{}>", self.coord, self.spans.len())
    }
}

impl WindowStatus {
    pub fn new(coord: Coord) -> Self {
        WindowStatus {
            coord,
            spans: Vec::default(),
        }
    }
}

impl Window for WindowStatus {
    type App = code::Code;

    #[inline]
    fn to_name(&self) -> String {
        "window-status".to_string()
    }

    #[inline]
    fn to_coord(&self) -> Coord {
        self.coord
    }

    #[inline]
    fn to_cursor(&self) -> Option<Cursor> {
        None
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
        Ok(evnt)
    }

    fn on_refresh(&mut self, _app: &mut code::Code) -> Result<()> {
        let mut line = Spanline::from_iter(self.spans.drain(..));
        let padding = self.coord.wth.saturating_sub(line.to_width() as u16);

        line.set_cursor(self.coord.to_origin_cursor().into())
            .right_padding(padding);

        err_at!(Fatal, termqu!(line))?;
        Ok(())
    }
}
