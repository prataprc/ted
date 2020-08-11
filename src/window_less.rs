#[allow(unused_imports)]
use log::{debug, trace};

use std::{convert::TryInto, fmt, io, mem, result};

use crate::{
    buffer::Buffer,
    colors::ColorScheme,
    event::Event,
    keymap::Keymap,
    location::Location,
    syntax, view,
    window::{Coord, Cursor},
    Result,
};

#[derive(Clone)]
pub struct WindowLess {
    coord: Coord,
    cursor: Cursor,
    buffer: Buffer,
    syn: syntax::Syn,
    scheme: ColorScheme,
    keymap: Keymap,
    old_screen: Option<Vec<view::ScrLine>>,
    // configuration
    wrap: bool,
}

impl Eq for WindowLess {}

impl PartialEq for WindowLess {
    fn eq(&self, other: &Self) -> bool {
        let mut ok = self.coord == other.coord;
        ok = ok && self.buffer.to_string() == other.buffer.to_string();
        ok && self.wrap == other.wrap
    }
}

impl fmt::Display for WindowLess {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "WindowLess<{}>", self.coord)
    }
}

impl WindowLess {
    pub fn new(name: &str, content: &str, coord: Coord, scheme: ColorScheme) -> Result<Self> {
        let buf = {
            let read_only = true;
            let loc = Location::new_ted(&name, io::empty(), read_only)?;
            Buffer::from_reader(content.as_bytes(), loc)?
        };
        let mut w = WindowLess {
            coord,
            cursor: Cursor::default(),
            buffer: buf,
            syn: (name, content, scheme.clone()).try_into()?,
            scheme,
            keymap: Keymap::new_less(),
            old_screen: None,
            wrap: false,
        };
        debug!("{}", w);
        w.buffer.mode_normal();
        Ok(w)
    }

    pub fn set_wrap(&mut self, wrap: bool) -> &mut Self {
        self.wrap = wrap;
        self
    }
}

impl WindowLess {
    #[inline]
    pub fn to_name(&self) -> String {
        "window-less".to_string()
    }

    #[inline]
    pub fn to_coord(&self) -> Coord {
        self.coord
    }

    #[inline]
    pub fn to_cursor(&self) -> Option<Cursor> {
        None
    }

    pub fn on_event(&mut self, evnt: Event) -> Result<Event> {
        match evnt {
            evnt @ Event::Esc => Ok(evnt),
            evnt => {
                let mut km = mem::replace(&mut self.keymap, Keymap::default());
                let evnt = km.fold(&self.buffer, evnt)?;
                let evnt = self.buffer.on_event(evnt)?;
                self.keymap = km;
                Ok(evnt)
            }
        }
    }

    pub fn on_refresh(&mut self) -> Result<()> {
        //use crate::Error;
        //use crossterm::queue;
        //use std::io::{self, Write};

        //let (col, row_iter) = {
        //    let (col, _) = self.coord.to_origin_cursor();
        //    let start = self.coord.hgt.saturating_sub(
        //        self.span_lines.len() as u16
        //    );
        //    (col, start..self.coord.hgt)
        //};
        //for (row, line) in row_iter.zip(self.span_lines.iter_mut()) {
        //    line.set_cursor(Cursor { col, row });
        //    err_at!(Fatal, termqu!(line))?;
        //}
        //let span: Span = self.buffer.to_string().into();
        //err_at!(Fatal, termqu!(span))?;
        Ok(())
    }
}
