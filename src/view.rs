use crossterm::queue;
use log::{debug, trace};

use std::{cmp, fmt, result};

use crate::{
    buffer::{self},
    col_nu::{ColKind, ColNu},
    colors::Highlight,
    window::{Coord, Cursor, Render, WinBuffer, Window},
    Error, Result,
};

/// Type to position and render a buffer in wrap mode. Takes the following
/// as input.
///
/// * coord, terminal viewport to render, origin starts from (1, 1).
/// * cursor, cursor within the viewport, origin starts from (0, 0).
/// * obc_xy, old-buffer-cursor starts from (0, 0).
///
/// Can be configured for:
///
/// * scroll_offset, that sets the top and bottom limit for cursor movement.
/// * line_number, whether to render the line number.
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

impl<'a, W> From<(&'a W, buffer::Cursor)> for Wrap
where
    W: Window,
{
    fn from((w, obc_xy): (&'a W, buffer::Cursor)) -> Wrap {
        let line_number = w.config_line_number();
        Wrap {
            name: w.to_name(),
            coord: w.to_coord(),
            cursor: w.to_cursor(),
            obc_xy,
            nu: ColNu::new(obc_xy.row, line_number),
            scroll_off: w.config_scroll_offset(),
            line_number,
        }
    }
}

impl Wrap {
    /// Initial cursor position on the top-left of the view-port accounting for
    /// line-numbering.
    pub fn initial_cursor(line_number: bool) -> Cursor {
        let nbc_xy: buffer::Cursor = Default::default();
        let col = ColNu::new(nbc_xy.row, line_number).to_width();
        Cursor { row: 0, col }
    }

    pub fn render<R>(mut self, buf: &R::Buf, r: &R, scroll: bool) -> Result<Cursor>
    where
        R: Render,
        <R as Render>::Buf: WinBuffer,
    {
        let nu_wth = self.nu.to_width();
        let nbc_xy = buf.to_xy_cursor();
        self.discount_nu(nu_wth);
        let screen_lines = {
            let (coord, cursor, screen_lines) = self.shift_cursor(buf, scroll);
            self.coord = coord;
            self.cursor = cursor;
            screen_lines
        };
        self.nu = ColNu::new(nbc_xy.row, self.line_number);
        self.nu.set_color_scheme(r.as_color_scheme());
        Ok(self.refresh(buf, r, screen_lines)?.account_nu(nu_wth))
    }

    fn shift_cursor<B>(&self, buf: &B, scroll: bool) -> (Coord, Cursor, Vec<ScrLine>)
    where
        B: WinBuffer,
    {
        let (cursor, screen_lines) = {
            let view: WrapView = self.clone().into();
            view.to_view_rows(buf, scroll)
        };
        let nbc_xy = buf.to_xy_cursor();
        let coord = {
            let old_nu = ColNu::new(self.obc_xy.row, self.line_number);
            let nu = ColNu::new(nbc_xy.row, self.line_number);
            self.coord.resize_to(
                self.coord.hgt,
                self.coord.wth + old_nu.to_width() - nu.to_width(),
            )
        };

        debug!(
            "SHIFT {}->{} {}@{} screen_lines:{}",
            self.obc_xy,
            nbc_xy,
            cursor,
            coord,
            screen_lines.len()
        );

        (coord, cursor, screen_lines)
    }

    fn refresh<R>(self, buf: &R::Buf, r: &R, screen_lines: Vec<ScrLine>) -> Result<Cursor>
    where
        R: Render,
        <R as Render>::Buf: WinBuffer,
    {
        debug!("WRAP-REFRESH {}", self);

        let s_canvas = {
            let scheme = r.as_color_scheme();
            scheme.to_style(Highlight::Canvas)
        };
        let full_coord = self.outer_coord();
        let (col, row) = full_coord.to_origin_cursor();

        let rows = row..(row + full_coord.hgt);
        for (row, sline) in rows.zip(screen_lines.into_iter()) {
            let nu_span = {
                let mut nu_span = self.nu.to_span(sline.colk);
                nu_span.set_cursor(Cursor { col, row });
                nu_span
            };
            let mut line_span = {
                let (a, z) = (sline.bc, sline.bc + (sline.n as usize));
                r.to_span_line(buf, a, z)?
            };
            let padding = {
                let n = sline.n.saturating_sub(line_span.trim_newline() as u16);
                self.coord.wth.saturating_sub(n)
            };
            line_span.right_padding(padding, s_canvas.clone());
            err_at!(Fatal, termqu!(nu_span, line_span))?;

            trace!(
                "  to_span_line row:{} {} w:{} {} {:?}",
                row,
                sline,
                line_span.to_width(),
                line_span,
                nu_span.cursor
            );
        }

        Ok(self.cursor)
    }

    fn discount_nu(&mut self, nu_wth: u16) {
        if self.line_number {
            self.coord = self
                .coord
                .resize_to(self.coord.hgt, self.coord.wth - nu_wth);
            self.cursor = self.cursor.move_by(-(nu_wth as i16), 0);
        }
    }

    fn outer_coord(&self) -> Coord {
        self.coord
            .resize_to(self.coord.hgt, self.coord.wth + self.nu.to_width())
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

impl<'a, W> From<(&'a W, buffer::Cursor)> for NoWrap
where
    W: Window,
{
    fn from((w, obc_xy): (&'a W, buffer::Cursor)) -> NoWrap {
        let line_number = w.config_line_number();
        NoWrap {
            name: w.to_name(),
            coord: w.to_coord(),
            cursor: w.to_cursor(),
            obc_xy,
            nu: ColNu::new(obc_xy.row, line_number),
            scroll_off: w.config_scroll_offset(),
            line_number,
        }
    }
}

impl NoWrap {
    /// Initial cursor position on the top-left of the view-port accounting for
    /// line-numbering.
    pub fn initial_cursor(line_number: bool) -> Cursor {
        let nbc_xy: buffer::Cursor = Default::default();
        let col = ColNu::new(nbc_xy.row, line_number).to_width();
        Cursor { row: 0, col }
    }

    pub fn render<R>(mut self, buf: &R::Buf, r: &R, scroll: bool) -> Result<Cursor>
    where
        R: Render,
        <R as Render>::Buf: WinBuffer,
    {
        let nu_wth = self.nu.to_width();
        let nbc_xy = buf.to_xy_cursor();
        self.discount_nu(nu_wth);
        let screen_lines = {
            let (coord, cursor, screen_lines) = self.shift_cursor(buf, scroll);
            self.coord = coord;
            self.cursor = cursor;
            screen_lines
        };
        self.nu = ColNu::new(nbc_xy.row, self.line_number);
        self.nu.set_color_scheme(r.as_color_scheme());
        Ok(self.refresh(buf, r, screen_lines)?.account_nu(nu_wth))
    }

    fn shift_cursor<B>(&self, buf: &B, scroll: bool) -> (Coord, Cursor, Vec<ScrLine>)
    where
        B: WinBuffer,
    {
        use std::iter::repeat;

        let nbc_xy = buf.to_xy_cursor();
        let r_min = cmp::max(0, self.scroll_off) as isize;
        let r_max = {
            let r_max = self.coord.hgt.saturating_sub(1);
            cmp::min(r_max, r_max.saturating_sub(self.scroll_off)) as isize
        };

        let coord = {
            let Cursor { row, col: _ } = self.cursor;
            let (_, diff_row) = self.obc_xy.diff(&nbc_xy);

            let row = (row as isize) + diff_row;
            if row < r_min || row > r_max {
                let nu = ColNu::new(nbc_xy.row, self.line_number);
                let coord = self.coord.resize_to(
                    self.coord.hgt,
                    self.coord.wth + self.nu.to_width() - nu.to_width(),
                );
                coord
            } else {
                self.coord
            }
        };

        let cursor = if scroll {
            self.cursor
        } else {
            let Cursor { row, col } = self.cursor;
            let (diff_col, diff_row) = self.obc_xy.diff(&nbc_xy);
            let row = limit!((row as isize) + diff_row, r_min, r_max);
            let col = limite!((col as isize) + diff_col, 0, coord.wth as isize);
            Cursor {
                col: col as u16,
                row: row as u16,
            }
        };

        let screen_lines = {
            let col = nbc_xy.col.saturating_sub(cursor.col as usize);
            let wth = coord.wth;
            let lines: Vec<usize> = {
                let from = nbc_xy.row.saturating_sub(cursor.row as usize);
                let to = cmp::max(buf.n_lines(), from + (coord.hgt as usize));
                (from..=to).collect()
            };
            let mut screen_lines = nowrap_lines(buf, lines, col, wth);
            let empty_lines: Vec<ScrLine> = repeat(ScrLine::new_empty())
                .take((coord.hgt as usize).saturating_sub(screen_lines.len()))
                .collect();
            screen_lines.extend(empty_lines);
            screen_lines
        };

        debug!(
            "SHIFT {}@{} screen_lines:{}",
            cursor,
            coord,
            screen_lines.len()
        );
        (coord, cursor, screen_lines)
    }

    fn refresh<R>(self, buf: &R::Buf, r: &R, screen_lines: Vec<ScrLine>) -> Result<Cursor>
    where
        R: Render,
        <R as Render>::Buf: WinBuffer,
    {
        debug!("NOWRAP-REFRESH {}", self);

        let s_canvas = {
            let scheme = r.as_color_scheme();
            scheme.to_style(Highlight::Canvas)
        };
        let full_coord = self.outer_coord();
        let (col, row) = full_coord.to_origin_cursor();

        let rows = row..(row + full_coord.hgt);
        for (row, sline) in rows.zip(screen_lines.into_iter()) {
            let nu_span = {
                let mut span = self.nu.to_span(sline.colk);
                span.set_cursor(Cursor { col, row });
                span
            };
            let mut line_span = {
                let (a, z) = (sline.bc, sline.bc + (sline.n as usize));
                r.to_span_line(buf, a, z)?
            };
            let padding = {
                let n = sline.n.saturating_sub(line_span.trim_newline() as u16);
                self.coord.wth.saturating_sub(n)
            };
            line_span.right_padding(padding, s_canvas.clone());
            err_at!(Fatal, termqu!(nu_span, line_span))?;
        }

        Ok(self.cursor)
    }

    fn discount_nu(&mut self, nu_wth: u16) {
        if self.line_number {
            self.coord = self
                .coord
                .resize_to(self.coord.hgt, self.coord.wth - nu_wth);
            self.cursor = self.cursor.move_by(-(nu_wth as i16), 0);
        }
    }

    fn outer_coord(&self) -> Coord {
        self.coord
            .resize_to(self.coord.hgt, self.coord.wth + self.nu.to_width())
    }
}

struct WrapView {
    coord: Coord, // full coordinate including line-number
    cursor: Cursor,
    obc_xy: buffer::Cursor,
    scroll_off: u16,
    line_number: bool,
}

impl From<Wrap> for WrapView {
    fn from(w: Wrap) -> Self {
        WrapView {
            coord: w.coord,
            cursor: w.cursor,
            obc_xy: w.obc_xy,
            scroll_off: w.scroll_off,
            line_number: w.line_number,
        }
    }
}

impl WrapView {
    fn to_view_rows<B>(&self, buf: &B, scroll: bool) -> (Cursor, Vec<ScrLine>)
    where
        B: WinBuffer,
    {
        use std::iter::repeat;

        let obc_xy = self.obc_xy;
        let nbc_xy = buf.to_xy_cursor();
        let hgt = self.coord.hgt as usize;
        let nbc = buf.line_to_char(nbc_xy.row) + nbc_xy.col;

        let lines: Vec<usize> = if obc_xy <= nbc_xy {
            let from = obc_xy.row.saturating_sub(hgt);
            let to = cmp::min(buf.n_lines(), nbc_xy.row + hgt);
            (from..=to).collect()
        } else {
            let from = nbc_xy.row.saturating_sub(hgt);
            let to = cmp::min(buf.n_lines(), obc_xy.row + hgt);
            (from..=to).collect()
        };

        let screen_lines = wrap_lines(buf, lines, self.coord.wth);

        let cursor = if scroll {
            self.cursor
        } else {
            let arow = self.scroll_off;
            let zrow = self.coord.hgt.saturating_sub(self.scroll_off + 1);
            match self.to_cursor(buf, screen_lines.clone()) {
                Cursor { col, row } if row < arow => Cursor { col, row: arow },
                Cursor { col, row } if row > zrow => Cursor { col, row: zrow },
                cursor => cursor,
            }
        };

        let pivot = {
            let item = {
                let iter = screen_lines.iter().enumerate();
                iter.take_while(|(_, sline)| sline.bc <= nbc).last().clone()
            };
            item.map(|(i, _)| i).unwrap_or(0)
        };
        debug!("pivot:{} cursor:{} nbc:{}", pivot, cursor, nbc);
        // debug!("screen_lines: {:?}", screen_lines);
        let mut screen_lines = match screen_lines.len() {
            0 => vec![],
            n => {
                let from = pivot.saturating_sub(cursor.row as usize);
                let to = cmp::min(from + hgt, n);
                screen_lines[from..to].to_vec()
            }
        };
        let empty_lines: Vec<ScrLine> = repeat(ScrLine::new_empty())
            .take(hgt.saturating_sub(screen_lines.len()))
            .collect();
        screen_lines.extend(empty_lines);

        assert_eq!(screen_lines.len(), hgt as usize);

        (cursor, screen_lines)
    }

    // viewport is editable window,
    fn to_cursor<B>(&self, buf: &B, screen_lines: Vec<ScrLine>) -> Cursor
    where
        B: WinBuffer,
    {
        let obc_xy = self.obc_xy;
        let nbc_xy = buf.to_xy_cursor();
        let obc = buf.line_to_char(obc_xy.row) + obc_xy.col;
        let nbc = buf.line_to_char(nbc_xy.row) + nbc_xy.col;

        if obc_xy <= nbc_xy {
            let rows: Vec<ScrLine> = screen_lines
                .into_iter()
                .skip_while(|sline| sline.bc <= obc)
                .take_while(|sline| sline.bc <= nbc)
                .collect();
            let row = {
                let mut row = self.cursor.row as usize;
                row = row.saturating_add(rows.len());
                cmp::min(self.coord.hgt.saturating_sub(1) as usize, row) as u16
            };
            let col = (nbc_xy.col % (self.coord.wth as usize)) as u16;
            // debug!("<< rows:{:?} row:{} col:{}", rows, row, col);
            Cursor { col, row }
        } else {
            let rows: Vec<ScrLine> = screen_lines
                .into_iter()
                .skip_while(|sline| sline.bc <= nbc)
                .take_while(|sline| sline.bc <= obc)
                .collect();
            let row = {
                let mut row = self.cursor.row as usize;
                row = row.saturating_sub(rows.len());
                cmp::min(self.coord.hgt.saturating_sub(1) as usize, row) as u16
            };
            let col = (nbc_xy.col % (self.coord.wth as usize)) as u16;
            // debug!(">> rows:{:?} row:{} col:{}", rows, row, col);
            Cursor { col, row }
        }
    }
}

fn nowrap_lines<B>(buf: &B, lines: Vec<usize>, col: usize, wth: u16) -> Vec<ScrLine>
where
    B: WinBuffer,
{
    lines
        .into_iter()
        .map(|line_idx| nowrap_line(buf, line_idx, col, wth))
        .collect()
}

fn nowrap_line<B>(buf: &B, line_idx: usize, col: usize, wth: u16) -> ScrLine
where
    B: WinBuffer,
{
    use crate::text::Format;

    let bc = buf.line_to_char(line_idx);
    let n = {
        let n = Format::trim_newline(&buf.line(line_idx)).0.chars().count();
        cmp::max(wth as usize, n.saturating_sub(col)) as u16
    };
    ScrLine::new_nu(line_idx, bc + col, n)
}

fn wrap_lines<B>(buf: &B, lines: Vec<usize>, wth: u16) -> Vec<ScrLine>
where
    B: WinBuffer,
{
    use std::convert;

    lines
        .into_iter()
        .map(|line_idx| wrap_line(buf, line_idx, wth).into_iter())
        .flat_map(convert::identity)
        .collect()
}

fn wrap_line<B>(buf: &B, line_idx: usize, wth: u16) -> Vec<ScrLine>
where
    B: WinBuffer,
{
    use crate::text::Format;
    use std::iter::repeat;

    let bc = buf.line_to_char(line_idx);
    let w = wth as usize;
    let (m, n) = {
        let line = buf.line(line_idx);
        let m = line.chars().count();
        let n = Format::trim_newline(&line).0.chars().count();
        (m, n)
    };
    //debug!(
    //    "... {} {} {}",
    //    line_idx,
    //    n,
    //    buf.line(line_idx).chars().count()
    //);
    match n {
        0 if line_idx == 0 || m > 0 => vec![ScrLine::new_nu(line_idx, bc, 0)],
        0 if m == 0 => vec![],
        n => {
            let mut ns: Vec<u16> = repeat(wth).take(n / w).collect();
            match n % w {
                0 => (),
                r => ns.push(r as u16),
            }
            let mut slines = vec![ScrLine::new_nu(line_idx, bc, ns.remove(0))];
            for (i, n) in ns.into_iter().enumerate().into_iter() {
                slines.push(ScrLine::new_wrap(line_idx, bc + ((i + 1) * w), n))
            }
            slines
        }
    }
}

fn col_kinds(nu: usize, n: usize) -> Vec<ColKind> {
    use std::iter::repeat;

    let mut list = vec![ColKind::Nu(nu)];
    let tail: Vec<ColKind> = {
        let iter = repeat(ColKind::Wrap).take(n.saturating_sub(1));
        iter.collect()
    };
    list.extend(tail);
    list
}

#[derive(Clone)]
struct ScrLine {
    colk: ColKind,
    line_idx: usize,
    bc: usize,
    n: u16,
}

impl fmt::Display for ScrLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "SL<{},{},{}>", self.colk, self.bc, self.n)
    }
}

impl fmt::Debug for ScrLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        <Self as fmt::Display>::fmt(self, f)
    }
}

impl ScrLine {
    fn new_nu(line_idx: usize, bc: usize, n: u16) -> Self {
        ScrLine {
            colk: ColKind::Nu(line_idx + 1),
            line_idx,
            bc,
            n,
        }
    }

    fn new_wrap(line_idx: usize, bc: usize, n: u16) -> Self {
        ScrLine {
            colk: ColKind::Wrap,
            line_idx,
            bc,
            n,
        }
    }

    fn new_empty() -> Self {
        ScrLine {
            colk: ColKind::Empty,
            line_idx: Default::default(),
            bc: Default::default(),
            n: Default::default(),
        }
    }
}
