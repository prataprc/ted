use crossterm::queue;
#[allow(unused_imports)]
use log::{debug, trace, warn};

use std::{cmp, convert::TryFrom, fmt, result};

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
    scroll_off: u16,
    line_number: bool,
    screen_lines: Vec<ScrLine>,
}

impl fmt::Display for Wrap {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(
            f,
            "Wrap<{:?} {} {}@{} {}>",
            self.name,
            self.obc_xy,
            self.cursor,
            self.coord,
            self.screen_lines.len()
        )
    }
}

impl<'a, W> TryFrom<(&'a W, buffer::Cursor)> for Wrap
where
    W: Window,
{
    type Error = Error;

    fn try_from((w, obc_xy): (&'a W, buffer::Cursor)) -> Result<Wrap> {
        let cursor = {
            let e = Error::Invalid(String::default(), "no-cursor".to_string());
            err_at!(w.to_cursor().ok_or(e))?
        };
        let scroll_off = w.config_scroll_offset();
        let line_number = w.config_line_number();
        Ok(Wrap {
            name: w.to_name(),
            coord: w.to_coord(),
            cursor,
            obc_xy,
            scroll_off,
            line_number,
            screen_lines: Vec::default(),
        })
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

    /// return the pre-computed screen-lines that exactly matches the window
    /// coordinates and fetch with text content, from `buf`, for each line.
    #[inline]
    pub fn to_screen_lines<B>(&self, buf: &B) -> Vec<ScrLine>
    where
        B: WinBuffer,
    {
        use crate::text;

        let mut screen_lines = self.screen_lines.clone();
        for sl in screen_lines.iter_mut() {
            let txt = buf.slice(sl.bc..(sl.bc + (sl.n as usize)));
            sl.text = Some(text::visual_line(&txt).to_string());
        }
        screen_lines
    }

    /// Update cursor and screen-lines for this wrap-view instance.
    /// If `scroll` is true, on screen cursor position remains
    /// the same, buffer is aligned with the screen/window.
    pub fn shift_cursor<B>(&mut self, buf: &B)
    where
        B: WinBuffer,
    {
        let (cursor, screen_lines) = {
            let view: WrapView = self.clone().into();
            view.to_screen_lines(buf)
        };
        let nbc_xy = buf.to_xy_cursor(None);

        debug!(
            "SHIFT {}->{} {}@{} screen_lines:{}",
            self.obc_xy,
            nbc_xy,
            cursor,
            self.coord,
            screen_lines.len()
        );

        // update this wrap-view.
        self.cursor = cursor;
        self.screen_lines = screen_lines;
    }

    /// consume this wrap-view and render the screen content.
    pub fn render<R>(self, buf: &R::Buf, r: &R, old_screen: Option<Vec<ScrLine>>) -> Result<Cursor>
    where
        R: Render,
        <R as Render>::Buf: WinBuffer,
    {
        self.refresh(buf, r, old_screen)
    }

    fn refresh<R>(self, buf: &R::Buf, r: &R, old_screen: Option<Vec<ScrLine>>) -> Result<Cursor>
    where
        R: Render,
        <R as Render>::Buf: WinBuffer,
    {
        debug!("WRAP-REFRESH {}", self);

        let nbc_xy = buf.to_xy_cursor(None);
        let canvas = {
            let scheme = r.as_color_scheme();
            scheme.to_style(Highlight::Canvas)
        };
        let (col, row) = self.coord.to_origin_cursor();
        let mut nu = ColNu::new(nbc_xy.row, self.line_number);
        nu.set_color_scheme(r.as_color_scheme());

        let rows = row..(row + self.coord.hgt);
        let iter = self.to_screen_lines(buf).into_iter().enumerate();
        for (row, (i, sline)) in rows.zip(iter) {
            match old_screen.as_ref() {
                Some(old_screen) if sline == old_screen[i] => continue,
                _ => (),
            }

            let nu_span = {
                let mut span = nu.to_span(sline.colk);
                span.set_cursor(Cursor { col, row });
                span
            };
            let mut line_span = {
                let (a, z) = (sline.bc, sline.bc + (sline.n as usize));
                r.to_span_line(buf, a, z)?
            };
            let padding = {
                let n = sline.n.saturating_sub(line_span.trim_newline() as u16);
                self.coord.wth.saturating_sub(nu.to_width() + n)
            };
            line_span.right_padding(padding);
            line_span.optimize_spans(canvas.clone());
            match &canvas.bg {
                Some(bg) => err_at!(Fatal, termbg!(bg.clone()))?,
                None => (),
            };
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
}

#[derive(Clone)]
pub struct NoWrap {
    name: String,
    coord: Coord,   // full coordinate
    cursor: Cursor, // within full coordinate
    obc_xy: buffer::Cursor,
    scroll_off: u16,
    line_number: bool,
    screen_lines: Vec<ScrLine>,
}

impl fmt::Display for NoWrap {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(
            f,
            "NoWrap<{:?} {} {}@{}>",
            self.name, self.obc_xy, self.cursor, self.coord,
        )
    }
}

impl<'a, W> TryFrom<(&'a W, buffer::Cursor)> for NoWrap
where
    W: Window,
{
    type Error = Error;

    fn try_from((w, obc_xy): (&'a W, buffer::Cursor)) -> Result<NoWrap> {
        let cursor = {
            let e = Error::Invalid(String::default(), "no-cursor".to_string());
            err_at!(w.to_cursor().ok_or(e))?
        };
        let line_number = w.config_line_number();
        let scroll_off = w.config_scroll_offset();
        Ok(NoWrap {
            name: w.to_name(),
            coord: w.to_coord(),
            cursor,
            obc_xy,
            scroll_off,
            line_number,
            screen_lines: Vec::default(),
        })
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

    #[inline]
    pub fn to_screen_lines<B>(&self, buf: &B) -> Vec<ScrLine>
    where
        B: WinBuffer,
    {
        use crate::text;

        let mut screen_lines = self.screen_lines.clone();
        for sl in screen_lines.iter_mut() {
            let txt = buf.slice(sl.bc..(sl.bc + (sl.n as usize)));
            sl.text = Some(text::visual_line(&txt).to_string());
        }
        screen_lines
    }

    /// Update cursor, coordinate and screen-lines for this wrap-view
    /// instance. if `scroll` is true, on screen cursor position remains
    /// the same, buffer is aligned with the screen/window.
    pub fn shift_cursor<B>(&mut self, buf: &B)
    where
        B: WinBuffer,
    {
        let nbc_xy = buf.to_xy_cursor(None);
        let nu = ColNu::new(nbc_xy.row, self.line_number);
        let (diff_col, diff_row) = self.obc_xy.diff(&nbc_xy);

        let cursor = Cursor {
            row: self.cursor.add_row(diff_row, self.coord, self.scroll_off),
            col: {
                let Cursor { col, .. } = self.cursor;
                let (min, max) = (nu.to_width() as isize, self.coord.wth as isize);
                limit!((col as isize) + diff_col, min, max) as u16
            },
        };

        let col = {
            let col = cursor.col.saturating_sub(nu.to_width());
            nbc_xy.col.saturating_sub(col as usize)
        };
        let wth = self.coord.wth.saturating_sub(nu.to_width());
        let screen_lines = {
            let lines: Vec<usize> = {
                let from = nbc_xy.row.saturating_sub(cursor.row as usize);
                let to = {
                    let to = from + (self.coord.hgt as usize);
                    cmp::max(buf.to_last_line_idx(), to)
                };
                (from..=to).collect()
            };
            let screen_lines = nowrap_lines(buf, lines, col, wth);
            padd_lines(screen_lines, self.coord)
        };

        debug!(
            "SHIFT {}->{} {}@{} screen_lines:{}",
            self.obc_xy,
            nbc_xy,
            cursor,
            self.coord,
            screen_lines.len()
        );

        // update this wrap-view.
        self.cursor = cursor;
        self.screen_lines = screen_lines;
    }

    pub fn render<R>(self, buf: &R::Buf, r: &R, old_screen: Option<Vec<ScrLine>>) -> Result<Cursor>
    where
        R: Render,
        <R as Render>::Buf: WinBuffer,
    {
        self.refresh(buf, r, old_screen)
    }

    fn refresh<R>(self, buf: &R::Buf, r: &R, old_screen: Option<Vec<ScrLine>>) -> Result<Cursor>
    where
        R: Render,
        <R as Render>::Buf: WinBuffer,
    {
        debug!("NOWRAP-REFRESH {}", self);

        let nbc_xy = buf.to_xy_cursor(None);
        let canvas = {
            let scheme = r.as_color_scheme();
            scheme.to_style(Highlight::Canvas)
        };
        let (col, row) = self.coord.to_origin_cursor();
        let mut nu = ColNu::new(nbc_xy.row, self.line_number);
        nu.set_color_scheme(r.as_color_scheme());

        let rows = row..(row + self.coord.hgt);
        let iter = self.to_screen_lines(buf).into_iter().enumerate();
        for (row, (i, sline)) in rows.zip(iter) {
            match old_screen.as_ref() {
                Some(old_screen) if sline == old_screen[i] => continue,
                _ => (),
            }

            let nu_span = {
                let mut span = nu.to_span(sline.colk);
                span.set_cursor(Cursor { col, row });
                span
            };
            let mut line_span = {
                let (a, z) = (sline.bc, sline.bc + (sline.n as usize));
                r.to_span_line(buf, a, z)?
            };
            let padding = {
                let n = sline.n.saturating_sub(line_span.trim_newline() as u16);
                self.coord.wth.saturating_sub(nu.to_width() + n)
            };
            line_span.right_padding(padding);
            line_span.optimize_spans(canvas.clone());
            match &canvas.bg {
                Some(bg) => err_at!(Fatal, termbg!(bg.clone()))?,
                None => (),
            };
            err_at!(Fatal, termqu!(nu_span, line_span))?;
        }

        Ok(self.cursor)
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
    fn to_screen_lines<B>(&self, buf: &B) -> (Cursor, Vec<ScrLine>)
    where
        B: WinBuffer,
    {
        let nbc_xy = buf.to_xy_cursor(None);
        let hgt = self.coord.hgt as usize;
        let wth = {
            let nu = ColNu::new(nbc_xy.row, self.line_number);
            self.coord.wth.saturating_sub(nu.to_width())
        };

        // compute the approximate range of lines top-to-bottom.
        let lines: Vec<usize> = {
            let (from, to) = if self.obc_xy <= nbc_xy {
                (self.obc_xy.row.saturating_sub(hgt), nbc_xy.row)
            } else {
                (nbc_xy.row.saturating_sub(hgt), self.obc_xy.row)
            };
            let to = cmp::min(buf.to_last_line_idx(), to.saturating_add(hgt));
            (from..to).collect()
        };

        let screen_lines = wrap_lines(buf, lines, wth);
        let pivot = cursor_line(&screen_lines, buf.to_char_cursor());

        let cursor = self
            .to_cursor(buf, &screen_lines)
            .saturate_row(self.coord, self.scroll_off);

        debug!(
            "pivot:{} cursor:{} nbc:{}",
            pivot,
            cursor,
            buf.to_char_cursor()
        );
        // debug!("screen_lines: {:?}", screen_lines);
        let screen_lines = {
            let ls = crop_lines(&screen_lines, pivot, self.coord, cursor);
            padd_lines(ls, self.coord)
        };
        assert_eq!(screen_lines.len(), hgt as usize);

        (cursor, screen_lines)
    }

    // viewport is editable window,
    fn to_cursor<B>(&self, buf: &B, screen_lines: &[ScrLine]) -> Cursor
    where
        B: WinBuffer,
    {
        let obc_xy = self.obc_xy;
        let nbc_xy = buf.to_xy_cursor(None);
        let obc = buf.line_to_char(obc_xy.row) + obc_xy.col;
        let nbc = buf.line_to_char(nbc_xy.row) + nbc_xy.col;

        let row = self.cursor.row as usize;
        let row = if obc_xy <= nbc_xy {
            let rows: Vec<&ScrLine> = screen_lines
                .iter()
                .skip_while(|sline| sline.bc <= obc)
                .take_while(|sline| sline.bc <= nbc)
                .collect();
            row.saturating_add(rows.len())
        } else {
            let rows: Vec<&ScrLine> = screen_lines
                .iter()
                .skip_while(|sline| sline.bc <= nbc)
                .take_while(|sline| sline.bc <= obc)
                .collect();
            row.saturating_sub(rows.len())
        };
        let row = cmp::min(row, self.coord.hgt.saturating_sub(1) as usize) as u16;

        // debug!("<< rows:{:?} row:{} col:{}", rows, row, col);
        let col = {
            let nu = ColNu::new(nbc_xy.row, self.line_number);
            let wth = self.coord.wth.saturating_sub(nu.to_width());
            let col = nbc_xy.col % (wth as usize);
            col.saturating_add(nu.to_width() as usize) as u16
        };
        Cursor { col, row }
    }
}

pub fn cursor_line(screen_lines: &[ScrLine], bc: usize) -> usize {
    let item = {
        let iter = screen_lines.iter().enumerate();
        iter.take_while(|(_, sline)| sline.bc <= bc).last().clone()
    };
    item.map(|(i, _)| i).unwrap_or(0)
}

pub fn crop_lines(
    screen_lines: &[ScrLine],
    pivot: usize,
    coord: Coord,
    cursor: Cursor,
) -> Vec<ScrLine> {
    match screen_lines.len() {
        0 => vec![],
        n => {
            let from = pivot.saturating_sub(cursor.row as usize);
            let to = cmp::min(from + (coord.hgt as usize), n);
            screen_lines[from..to].to_vec()
        }
    }
}

pub fn padd_lines(mut screen_lines: Vec<ScrLine>, coord: Coord) -> Vec<ScrLine> {
    use std::iter::repeat;

    let empty_line = ScrLine::new_empty();
    let n = (coord.hgt as usize).saturating_sub(screen_lines.len());
    let empty_lines: Vec<ScrLine> = repeat(empty_line).take(n).collect();
    screen_lines.extend(empty_lines);
    screen_lines
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
    use crate::text;

    let bc = buf.line_to_char(line_idx);
    let n = {
        let n = text::visual_line_n(&buf.line(line_idx));
        cmp::max(wth as usize, n.saturating_sub(col)) as u16
    };
    ScrLine::new_nu(line_idx, bc + col, n)
}

pub fn wrap_lines<B>(buf: &B, lines: Vec<usize>, wth: u16) -> Vec<ScrLine>
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
    use crate::text;
    use std::iter::repeat;

    let bc = buf.line_to_char(line_idx);
    let w = wth as usize;
    let (m, n) = {
        let line = buf.line(line_idx);
        (line.chars().count(), text::visual_line_n(&line))
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

#[derive(Clone, Eq, PartialEq)]
pub struct ScrLine {
    pub colk: ColKind,
    pub line_idx: usize,
    pub bc: usize,
    pub n: u16,
    pub text: Option<String>,
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
            text: None,
        }
    }

    fn new_wrap(line_idx: usize, bc: usize, n: u16) -> Self {
        ScrLine {
            colk: ColKind::Wrap,
            line_idx,
            bc,
            n,
            text: None,
        }
    }

    fn new_empty() -> Self {
        ScrLine {
            colk: ColKind::Empty,
            line_idx: usize::default(),
            bc: usize::default(),
            n: u16::default(),
            text: None,
        }
    }
}
