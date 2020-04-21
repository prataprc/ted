use crossterm::queue;
use log::trace;
use ropey::Rope;

use std::{
    cmp,
    convert::TryInto,
    fmt,
    io::{self, Write},
    iter::FromIterator,
    ops::Bound,
    result,
};

use crate::{
    config::Config,
    event::Event,
    window::{Context, Coord, Cursor, Span, Window},
    Error, Result,
};

#[derive(Clone, Default)]
pub struct WindowEdit {
    coord: Coord,
    cursor: Cursor,
    old_bc: (usize, usize),

    buffer_id: String,
    config: Config,
}

impl fmt::Display for WindowEdit {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "WindowEdit<{}>", self.coord)
    }
}

impl WindowEdit {
    #[inline]
    pub fn new(coord: Coord, config: Config) -> Result<WindowEdit> {
        Ok(WindowEdit {
            coord,
            cursor: cursor!(0, 0),
            old_bc: (0, 0),

            buffer_id: Default::default(),
            config,
        })
    }

    #[inline]
    pub fn to_size(&self) -> (u16, u16) {
        self.coord.to_size()
    }
}

impl WindowEdit {
    pub fn to_lines<'a>(
        &self,
        from: Bound<usize>,
        to: Bound<usize>,
        context: &'a Context,
    ) -> impl Iterator<Item = (usize, String)> + 'a {
        let buffer = context.as_buffer(&self.buffer_id);
        buffer.to_lines(from, to)
    }

    pub fn visual_cursor(&self, context: &Context) -> (usize, usize) {
        let buffer = context.as_buffer(&self.buffer_id);
        buffer.visual_cursor()
    }

    fn align_up(&self, context: &Context) -> u16 {
        let soff = self.config.scroll_off;
        let change = {
            let buf = context.as_buffer(&self.buffer_id);
            buf.as_change()
        };
        let r: &Rope = change.as_ref();

        let new_bc = change.to_xy_cursor();
        assert!(self.old_bc.1 >= new_bc.1);

        let (hgt, _) = self.coord.to_size();
        let Cursor { mut row, .. } = self.cursor;
        let mut lines = r.lines_at(self.old_bc.1);
        loop {
            match lines.prev() {
                Some(_) if row == 0 && hgt <= soff => break 0,
                Some(_) if row == 0 => break soff,
                Some(_) => row -= 1,
                None => break 0,
            }
        }
    }

    fn align_down(&self, context: &Context) -> u16 {
        let soff = self.config.scroll_off;
        let change = {
            let buf = context.as_buffer(&self.buffer_id);
            buf.as_change()
        };
        let r: &Rope = change.as_ref();

        let new_bc = change.to_xy_cursor();
        assert!(new_bc.1 >= self.old_bc.1);

        let (hgt, _) = self.coord.to_size();
        let Cursor { mut row, .. } = self.cursor;
        let mut lines = r.lines_at(self.old_bc.1);
        loop {
            match lines.next() {
                Some(_) if (row + 1) == hgt && hgt <= soff => break row,
                Some(_) if (row + 1) == hgt => break row - soff,
                Some(_) => row += 1,
                None => break row,
            }
        }
    }

    fn align_left(&self, context: &Context) -> u16 {
        let change = {
            let buf = context.as_buffer(&self.buffer_id);
            buf.as_change()
        };

        let new_bc = change.to_xy_cursor();
        assert!(self.old_bc.0 >= new_bc.0);

        self.cursor.col.saturating_sub(
            (self.old_bc.0 - new_bc.0)
                .try_into()
                .ok()
                .unwrap_or(self.cursor.col),
        )
    }

    fn align_right(&self, context: &Context) -> u16 {
        let change = {
            let buf = context.as_buffer(&self.buffer_id);
            buf.as_change()
        };

        let new_bc = change.to_xy_cursor();
        assert!(new_bc.0 >= self.old_bc.0);

        let (_, width) = self.to_size();
        cmp::min(
            self.cursor.col
                + (new_bc.0 - self.old_bc.0)
                    .try_into()
                    .ok()
                    .unwrap_or(u16::MAX),
            width,
        )
    }

    fn refresh_nowrap(&mut self, context: &mut Context) -> Result<()> {
        use std::iter::repeat;

        let new_bc = {
            let buffer = context.as_mut_buffer(&self.buffer_id);
            buffer.to_xy_cursor()
        };
        let cursor = {
            let ccol = match (self.old_bc, new_bc) {
                ((oc, _), (nc, _)) if oc <= nc => self.align_right(context),
                _ => self.align_left(context),
            };
            let crow = match (self.old_bc, new_bc) {
                ((_, or), (_, nr)) if or <= nr => self.align_down(context),
                _ => self.align_up(context),
            };
            cursor!(ccol, crow)
        };

        trace!(
            "{} bc:{:?}->{:?} vc:{}->{}",
            self.coord,
            self.old_bc,
            new_bc,
            self.cursor,
            cursor
        );

        let mut stdout = io::stdout();
        let (col, mut row) = self.coord.to_origin();

        let change = {
            let buf = context.as_buffer(&self.buffer_id);
            buf.as_change()
        };

        let lines = change
            .lines_at(new_bc.1 - (cursor.row as usize))
            .map(|line| {
                line.chars_at(new_bc.0 - (cursor.col as usize))
                    .chain(repeat(' '))
                    .take(self.coord.wth as usize)
                    .collect::<Vec<char>>()
            });
        for line in lines
            .take(self.coord.hgt as usize)
            .collect::<Vec<Vec<char>>>()
        {
            // trace!("{:?}", line);
            let s = String::from_iter(line);
            err_at!(Fatal, queue!(stdout, span!((col, row), s: s)))?;
            row += 1;
        }

        Ok(())
    }
}

impl Window for WindowEdit {
    #[inline]
    fn to_origin(&self) -> (u16, u16) {
        self.coord.to_origin()
    }

    #[inline]
    fn to_cursor(&self) -> Cursor {
        self.coord.to_top_left() + self.cursor
    }

    #[inline]
    fn move_by(&mut self, col_off: i16, row_off: i16, _: &Context) {
        self.coord = self.coord.clone().move_by(col_off, row_off);
    }

    #[inline]
    fn resize_to(&mut self, height: u16, width: u16, _: &Context) {
        let scroll_off = self.config.scroll_off;

        self.coord = self.coord.clone().resize_to(height, width);
        self.cursor.col = cmp::min(self.cursor.col, width - 1);
        self.cursor.row = {
            let row = cmp::min(self.cursor.row, height - 1);
            if_else!(row <= scroll_off, row, row - scroll_off)
        };
    }

    fn refresh(&mut self, context: &mut Context) -> Result<()> {
        self.refresh_nowrap(context)
    }

    fn handle_event(
        //
        &mut self,
        context: &mut Context,
        evnt: Event,
    ) -> Result<Option<Event>> {
        match evnt {
            Event::UseBuffer { buffer_id } => {
                self.buffer_id = buffer_id;
                Ok(None)
            }
            evnt => {
                let buffer = context.as_mut_buffer(&self.buffer_id);
                buffer.handle_event(evnt)
            }
        }
    }
}
