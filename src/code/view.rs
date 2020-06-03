use crossterm::queue;
use log::trace;

use std::{
    convert::TryInto,
    fmt,
    io::{self, Write},
    iter::FromIterator,
    result,
};

use crate::{
    buffer::{self, Buffer},
    code::col_nu::{ColKind, ColNu},
    color_scheme::{ColorScheme, Highlight},
    event::DP,
    window::{Coord, Cursor, Span},
    Error, Result,
};

#[derive(Clone)]
pub struct Wrap {
    name: String,
    coord: Coord,
    cursor: Cursor,
    obc_xy: buffer::Cursor,
    nu: ColNu,
    scroll_off: u16,
    line_number: bool,
}

impl fmt::Display for Wrap {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(
            f,
            "Wrap<{:?} {} {} {}@{}>",
            self.name, self.nu, self.obc_xy, self.cursor, self.coord,
        )
    }
}

impl Wrap {
    pub fn initial_cursor(line_number: bool) -> Cursor {
        let nbc_xy: buffer::Cursor = Default::default();
        let col = ColNu::new(nbc_xy.row, line_number).to_width();
        Cursor { row: 0, col }
    }

    // create a wrap view using previous cursor's nu_width.
    pub fn new(name: &str, coord: Coord, cursor: Cursor, obc_xy: buffer::Cursor) -> Wrap {
        Wrap {
            name: name.to_string(),
            coord,
            cursor,
            obc_xy,
            nu: ColNu::new(obc_xy.row, false),
            scroll_off: 0,
            line_number: false,
        }
    }

    pub fn set_scroll_off(&mut self, scroll_off: u16) -> &mut Self {
        self.scroll_off = scroll_off;
        self
    }

    pub fn set_line_number(&mut self, line_number: bool) -> &mut Self {
        self.line_number = line_number;
        self.nu = ColNu::new(self.obc_xy.row, line_number);
        self
    }

    pub fn render(mut self, buf: &Buffer, cs: &ColorScheme) -> Result<Cursor> {
        let nu_wth = self.nu.to_width();
        self.discount_nu(nu_wth);
        self = self.shift_cursor(buf)?;
        Ok(self.refresh(buf, cs)?.account_nu(nu_wth))
    }

    fn shift_cursor(&self, buf: &Buffer) -> Result<Self> {
        let view = {
            let mut view = WrapView::new(self.coord, self.cursor, self.obc_xy);
            view.set_scroll_off(self.scroll_off)
                .set_line_number(self.line_number);
            view.into_new_view(buf)?
        };

        trace!("SHIFT {}->{}@{}", self, view.cursor, view.coord);

        Ok(Wrap {
            name: self.name.clone(),
            coord: view.coord,
            cursor: view.cursor,
            obc_xy: view.bc_xy,
            nu: ColNu::new(view.bc_xy.row, self.line_number),
            scroll_off: self.scroll_off,
            line_number: self.line_number,
        })
    }

    fn refresh(self, buf: &Buffer, scheme: &ColorScheme) -> Result<Cursor> {
        use crate::event::DP::Right;
        use std::iter::repeat;

        let nbc_xy = buf.to_xy_cursor();
        let line_idx = nbc_xy.row.saturating_sub(self.cursor.row as usize);
        trace!("REFRESH {} nbc_xy:{} line_idx:{}", self, nbc_xy, line_idx);

        let mut stdout = io::stdout();

        let full_coord = self.outer_coord();
        let (col, row) = full_coord.to_origin_cursor();
        let max_row = row + full_coord.hgt;

        let view_rows = {
            let view = WrapView::new(self.coord, self.cursor, self.obc_xy);
            view.to_view_rows(buf)?
        };

        for (r, (line_nu, bc_caret, n)) in view_rows.iter().enumerate() {
            let nu_span = {
                let mut nu_span = match line_nu {
                    Some(num) => self.nu.to_span(ColKind::Nu(*num), scheme),
                    None => self.nu.to_span(ColKind::Wrap, scheme),
                };
                nu_span.set_cursor(Cursor {
                    col,
                    row: row + (r as u16),
                });
                nu_span
            };

            let chars = {
                let iter = buf.chars_at(*bc_caret, Right)?.chain(repeat(' '));
                iter.take(bc_caret + (*n as usize))
            };
            let line_span = span!(st: String::from_iter(chars));
            // trace!("    text {:?}", line_span.to_content());
            err_at!(Fatal, queue!(stdout, nu_span, line_span))?;
        }
        let row: u16 = {
            let row = (row as usize) + view_rows.len();
            err_at!(FailConvert, row.try_into())?
        };
        empty_lines(
            tail_line(col, row, max_row - 1, &self.nu, buf, scheme)?,
            max_row - 1,
            full_coord,
            &self.nu,
            scheme,
        )?;

        Ok(self.cursor)
    }

    fn discount_nu(&mut self, nu_wth: u16) {
        if self.line_number {
            self.coord = {
                let (hgt, wth) = self.coord.to_size();
                self.coord.resize_to(hgt, wth - nu_wth)
            };
            self.cursor = self.cursor.move_by(-(nu_wth as i16), 0);
        }
    }

    fn outer_coord(&self) -> Coord {
        let (hgt, wth) = self.coord.to_size();
        self.coord.resize_to(hgt, wth + self.nu.to_width())
    }
}

#[derive(Clone)]
pub struct NoWrap {
    name: String,
    coord: Coord,   // full coordinate
    cursor: Cursor, // within full coordinate
    obc_xy: buffer::Cursor,
    nu: ColNu,
    scroll_off: u16,
    line_number: bool,
}

impl fmt::Display for NoWrap {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(
            f,
            "NoWrap<{:?} {} {} {}@{}>",
            self.name, self.nu, self.obc_xy, self.cursor, self.coord,
        )
    }
}

impl NoWrap {
    pub fn initial_cursor(line_number: bool) -> Cursor {
        let nbc_xy: buffer::Cursor = Default::default();
        let col = ColNu::new(nbc_xy.row, line_number).to_width();
        Cursor { row: 0, col }
    }

    pub fn new(name: &str, coord: Coord, cursor: Cursor, obc_xy: buffer::Cursor) -> NoWrap {
        NoWrap {
            name: name.to_string(),
            coord,
            cursor,
            obc_xy,
            nu: ColNu::new(obc_xy.row, false),
            scroll_off: 0,
            line_number: false,
        }
    }

    pub fn set_scroll_off(&mut self, scroll_off: u16) -> &mut Self {
        self.scroll_off = scroll_off;
        self
    }

    pub fn set_line_number(&mut self, line_number: bool) -> &mut Self {
        self.line_number = line_number;
        self.nu = ColNu::new(self.obc_xy.row, line_number);
        self
    }

    pub fn render(mut self, buf: &Buffer, cs: &ColorScheme) -> Result<Cursor> {
        let nu_wth = self.nu.to_width();
        self.discount_nu(nu_wth);
        self = self.shift_cursor(buf)?;
        Ok(self.refresh(buf, cs)?.account_nu(nu_wth))
    }

    fn shift_cursor(self, buf: &Buffer) -> Result<Self> {
        let scroll_off = self.scroll_off; // accounting for scroll-offset.

        let (r_min, r_max) = if self.coord.hgt < (scroll_off * 2) {
            (0, (self.coord.hgt.saturating_sub(1) as isize))
        } else {
            (
                scroll_off as isize,
                (self.coord.hgt.saturating_sub(scroll_off + 1) as isize),
            )
        };

        let nbc_xy = buf.to_xy_cursor();
        let (diff_col, diff_row) = self.obc_xy.diff(&nbc_xy);
        let Cursor { row, col } = self.cursor;

        let (coord, nu) = {
            let row = (row as isize) + diff_row;
            if row < r_min || row > r_max {
                let nu = ColNu::new(nbc_xy.row, self.line_number);
                let coord = self.coord.resize_to(
                    self.coord.hgt,
                    self.coord.wth + self.nu.to_width() - nu.to_width(),
                );
                (coord, nu)
            } else {
                (self.coord, self.nu.clone())
            }
        };

        let new_row: u16 = {
            let row = limit!((row as isize) + diff_row, r_min, r_max);
            assert!(row < (coord.hgt as isize), "assert {} {}", row, coord.hgt);
            row as u16
        };
        let new_col: u16 = {
            let col = limite!((col as isize) + diff_col, 0, coord.wth as isize);
            assert!(col < (coord.wth as isize), "assert {} {}", col, coord.wth);
            col as u16
        };
        let cursor = Cursor {
            col: new_col,
            row: new_row,
        };

        trace!("SHIFT {}->{}@{}", self, cursor, coord);
        Ok(NoWrap {
            name: self.name,
            coord,
            cursor,
            obc_xy: self.obc_xy,
            nu,
            scroll_off: self.scroll_off,
            line_number: self.line_number,
        })
    }

    fn refresh(self, buf: &Buffer, scheme: &ColorScheme) -> Result<Cursor> {
        use std::iter::repeat;

        let nbc_xy = buf.to_xy_cursor();
        trace!("REFRESH {} nbc_xy:{}", self, nbc_xy,);

        let mut stdout = io::stdout();

        let full_coord = self.outer_coord();
        let (col, mut row) = full_coord.to_origin_cursor();
        let max_row = row + full_coord.hgt;
        let (hgt, wth) = self.coord.to_size();
        let nu_wth = self.nu.to_width();

        let do_padding = |line: ropey::RopeSlice| -> Vec<char> {
            line.chars_at(nbc_xy.col.saturating_sub(self.cursor.col as usize))
                .chain(repeat(' '))
                .take((wth - nu_wth) as usize)
                .collect()
        };

        let from = nbc_xy.row.saturating_sub(self.cursor.row as usize);
        let lines = buf.lines_at(from, DP::Right)?.map(do_padding);
        for (i, line) in lines.take(hgt as usize).enumerate() {
            let nu_span = {
                let mut span = self.nu.to_span(ColKind::Nu(from + i + 1), scheme);
                span.set_cursor(Cursor { col, row });
                span
            };
            let line_span = span!(st: String::from_iter(line));
            err_at!(Fatal, queue!(stdout, nu_span, line_span))?;
            row += 1;
        }

        empty_lines(
            tail_line(col, row, max_row - 1, &self.nu, buf, scheme)?,
            max_row - 1,
            full_coord,
            &self.nu,
            scheme,
        )?;

        Ok(self.cursor)
    }

    fn outer_coord(&self) -> Coord {
        let (hgt, wth) = self.coord.to_size();
        self.coord.resize_to(hgt, wth + self.nu.to_width())
    }

    fn discount_nu(&mut self, nu_wth: u16) {
        if self.line_number {
            self.coord = {
                let (hgt, wth) = self.coord.to_size();
                self.coord.resize_to(hgt, wth - nu_wth)
            };
            self.cursor = self.cursor.move_by(-(nu_wth as i16), 0);
        }
    }
}

fn empty_lines(
    //
    mut row: u16,
    max_row: u16,
    full_coord: Coord,
    nu: &ColNu,
    scheme: &ColorScheme,
) -> Result<()> {
    use std::iter::repeat;

    let mut stdout = io::stdout();
    let (col, _) = full_coord.to_origin_cursor();
    let (_, wth) = full_coord.to_size();

    if row <= max_row {
        trace!("EMPTY LINES {}..={}", row, max_row);
        for _ in row..=max_row {
            let mut nu_span = nu.to_span(ColKind::Empty, scheme);
            nu_span.set_cursor(Cursor { col, row });
            let line_span = {
                let iter = repeat(' ').take((wth - nu.to_width()) as usize);
                let span: Span = String::from_iter(iter).into();
                span.using(scheme.to_style(Highlight::Canvas))
            };
            err_at!(Fatal, queue!(stdout, nu_span, line_span))?;
            row += 1;
        }
    }
    assert!(row == (max_row + 1), "assert {} {}", row, max_row);

    Ok(())
}

fn tail_line(
    //
    col: u16,
    row: u16,
    max_row: u16,
    nu: &ColNu,
    buf: &Buffer,
    scheme: &ColorScheme,
) -> Result<u16> {
    let n = buf.n_chars();
    let ok1 = n == 0;
    let ok2 = (row <= max_row) && buf.is_trailing_newline();

    let n = if ok1 { 1 } else { buf.char_to_line(n - 1) + 1 };
    let mut stdout = io::stdout();

    let new_row = if ok1 || ok2 {
        let mut span = nu.to_span(ColKind::Nu(n), scheme);
        span.set_cursor(Cursor { col, row });
        err_at!(Fatal, queue!(stdout, span))?;
        row + 1
    } else {
        row
    };

    trace!(
        "TRAIL {}->{}-of-{} nu:{} trail:{},{}",
        row,
        new_row,
        max_row,
        n,
        ok1,
        ok2
    );
    Ok(new_row)
}

struct WrapView {
    coord: Coord, // full coordinate including line-number
    cursor: Cursor,
    bc_xy: buffer::Cursor,
    scroll_off: u16,
    line_number: bool,
}

impl WrapView {
    fn new(coord: Coord, cursor: Cursor, bc_xy: buffer::Cursor) -> WrapView {
        WrapView {
            coord,
            bc_xy,
            cursor,
            scroll_off: Default::default(),
            line_number: false,
        }
    }

    fn set_scroll_off(&mut self, scroll_off: u16) -> &mut Self {
        self.scroll_off = scroll_off;
        self
    }

    fn set_line_number(&mut self, line_number: bool) -> &mut Self {
        self.line_number = line_number;
        self
    }

    fn into_new_view(mut self, buf: &Buffer) -> Result<Self> {
        let nbc_xy = buf.to_xy_cursor();

        match self.to_cursor(buf, self.to_view_rows(buf)?)? {
            Some(cursor) => {
                self.cursor = cursor;
                self.bc_xy = nbc_xy;
                Ok(self)
            }
            None => {
                let coord = {
                    let old_nu = ColNu::new(self.bc_xy.row, self.line_number);
                    let nu = ColNu::new(nbc_xy.row, self.line_number);
                    let (hgt, wth) = self.coord.to_size();
                    self.coord
                        .resize_to(hgt, wth + old_nu.to_width() - nu.to_width())
                };
                let cursor = {
                    let (hgt, wth) = coord.to_size();
                    let row = if nbc_xy <= self.bc_xy {
                        self.scroll_off + 1
                    } else {
                        hgt.saturating_sub(self.scroll_off + 1)
                    };
                    let col = {
                        let col = nbc_xy.col % (wth as usize);
                        err_at!(FailConvert, col.try_into())?
                    };
                    Cursor { row, col }
                };

                self.coord = coord;
                self.cursor = cursor;
                self.bc_xy = nbc_xy;
                Ok(self)
            }
        }
    }
}

impl WrapView {
    // return (Option<line_nu>, buffer_cursor, len)
    fn to_view_rows(&self, buf: &Buffer) -> Result<Vec<(Option<usize>, usize, u16)>> {
        use crate::event::DP::Right;

        let (coord, cursor, bc_xy) = (self.coord, self.cursor, self.bc_xy);
        let (hgt, wth) = coord.to_size();
        let tops = cursor.row as usize;
        let bots = hgt.saturating_sub(cursor.row) as usize;

        // (Option<line_nu>, buffer-cursor, line-len)
        let mut rows: Vec<(Option<usize>, usize, u16)> = {
            let (mut bc, line_idx) = {
                let line_idx = bc_xy.row.saturating_sub(tops);
                (buf.line_to_char(line_idx), line_idx)
            };
            let mut top_rows = vec![];
            let mut line_nu = Some(line_idx);
            let iter = buf.lines_at(line_idx, Right)?.take(tops);
            for (i, line) in iter.enumerate() {
                let mut n = line.len_chars();
                while n > (wth as usize) {
                    top_rows.push((line_nu.take(), bc, wth));
                    bc += wth as usize;
                    n -= wth as usize;
                }
                if n > 0 {
                    top_rows.push((line_nu.take(), bc, n as u16));
                }
                line_nu = Some(line_idx + i);
            }
            {
                top_rows.reverse();
                top_rows = top_rows.into_iter().take(tops).collect();
                top_rows.reverse();
                top_rows
            }
        };

        rows.extend::<Vec<(Option<usize>, usize, u16)>>({
            let (mut bc, line_idx) = (buf.line_to_char(bc_xy.row), bc_xy.row);
            let mut bot_rows: Vec<(Option<usize>, usize, u16)> = vec![];
            let mut line_nu = Some(line_idx);
            let iter = buf.lines_at(line_idx, Right)?.take(bots);
            for (i, line) in iter.enumerate() {
                let mut n = line.len_chars();
                while n > (wth as usize) {
                    bot_rows.push((line_nu.take(), bc, wth));
                    bc += wth as usize;
                    n -= wth as usize;
                }
                if n > 0 {
                    bot_rows.push((line_nu.take(), bc, n as u16));
                }
                line_nu = Some(line_idx + i);
            }
            bot_rows.into_iter().take(bots).collect()
        });

        assert_eq!(rows.len(), self.coord.hgt as usize);
        Ok(rows)
    }

    fn to_cursor(
        //
        &self,
        buf: &Buffer,
        mut rs: Vec<(Option<usize>, usize, u16)>,
    ) -> Result<Option<Cursor>> {
        let rows = {
            // crop the rows for scroll offset.
            let so = self.scroll_off as usize;
            match (so * 2, rs.len()) {
                (m, n) if m < n => rs,
                (_, n) => rs.drain(so..(n - so)).collect(),
            }
        };

        let nbc_xy = buf.to_xy_cursor();
        let wth = {
            let (_, wth) = self.coord.to_size();
            wth as usize
        };

        let nbc_caret = {
            let col = if (nbc_xy.col % wth) == 0 {
                (nbc_xy.col / wth).saturating_sub(1) * wth
            } else {
                (nbc_xy.col / wth) * wth
            };
            buf.line_to_char(nbc_xy.row) + col
        };

        for (row, (_, bc_caret, _)) in rows.into_iter().enumerate() {
            if bc_caret == nbc_caret {
                let col = {
                    let col = nbc_xy.col % (wth as usize);
                    err_at!(FailConvert, col.try_into())?
                };
                let row = err_at!(FailConvert, row.try_into())?;
                return Ok(Some(Cursor { col, row }));
            }
        }
        return Ok(None);
    }
}
