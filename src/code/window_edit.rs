#[allow(unused_imports)]
use log::{debug, trace};

use std::{cmp, convert::TryInto, fmt, result};

use crate::{
    app::Application,
    buffer::{self, Buffer},
    code::{self},
    col_nu::ColNu,
    colors::ColorScheme,
    event::{self, Event, Scroll, DP},
    keymap::Keymap,
    syntax::{self, Syntax},
    term::Spanline,
    view,
    window::{Coord, Cursor, Render, WinBuffer, Window},
    Error, Result,
};

pub struct WindowEdit {
    coord: Coord,
    cursor: Cursor,
    obc_xy: buffer::Cursor,
    curr_buf_id: String,
    altn_buf_id: Option<String>,
    syn: syntax::Syn,
    scheme: ColorScheme,
    keymap: Keymap,
    old_screen: Option<Vec<view::ScrLine>>,
    // configuration.
    scroll_off: u16,
    line_number: bool,
}

impl fmt::Display for WindowEdit {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(
            f,
            "WindowEdit<{}@{} {}>",
            self.cursor, self.coord, self.obc_xy,
        )
    }
}

impl<'a, 'b> From<(&'a code::Code, &'b Buffer, Coord)> for WindowEdit {
    fn from((app, buf, coord): (&'a code::Code, &'b Buffer, Coord)) -> Self {
        let cursor = if app.config.wrap {
            view::Wrap::initial_cursor(app.config.line_number)
        } else {
            view::NoWrap::initial_cursor(app.config.line_number)
        };

        let scheme = app.to_color_scheme(None);
        let w = WindowEdit {
            coord,
            cursor,
            obc_xy: (0, 0).into(),
            curr_buf_id: buf.to_id(),
            altn_buf_id: None,
            syn: syntax::detect(buf, &scheme).unwrap(),
            scheme,
            keymap: Keymap::new_edit(),
            old_screen: None,
            // configuration
            scroll_off: app.as_ref().scroll_off,
            line_number: app.as_ref().line_number,
        };
        debug!("{} {} {}", w, w.scroll_off, w.line_number);
        w
    }
}

impl WindowEdit {
    pub fn set_buffer(&mut self, buf: &Buffer) -> &mut Self {
        self.altn_buf_id = Some(self.curr_buf_id.clone());
        self.curr_buf_id = buf.to_id();
        self
    }

    pub fn flip_buffer(&mut self) -> &mut Self {
        match self.altn_buf_id.take() {
            Some(altn_buf_id) => {
                let curr_buf_id = self.curr_buf_id.clone();
                self.curr_buf_id = altn_buf_id;
                self.altn_buf_id = Some(curr_buf_id);
            }
            None => (),
        }
        self
    }

    pub fn to_event_prefix(&self) -> Event {
        self.keymap.to_event_prefix()
    }
}

impl WindowEdit {
    #[inline]
    pub fn to_buffer_id(&self) -> String {
        self.curr_buf_id.clone()
    }

    #[inline]
    pub fn to_text_type(&self) -> String {
        self.syn.as_name().to_string()
    }

    // return the width of editable screen's width.
    fn to_edit_width(&self) -> (u16, u16) {
        let nu_wth = ColNu::new(self.obc_xy.row, self.line_number).to_width();
        (nu_wth, self.coord.wth.saturating_sub(nu_wth))
    }

    // return the number of characters to move left ro reach screen-home.
    fn to_cursor_col(&self) -> u16 {
        let nu_wth = ColNu::new(self.obc_xy.row, self.line_number).to_width();
        self.cursor.col - nu_wth
    }

    fn mto_screen_home(&self, buf: &Buffer, dp: DP) -> Result<usize> {
        use crate::buffer::mto_left;

        let cursor = {
            let c = self.to_cursor_col() as usize;
            mto_left(buf, c, DP::None)?
        };
        let cursor = match dp {
            DP::TextCol => {
                let xy = buf.to_xy_cursor(Some(cursor));
                let line = buf.line(xy.row);
                cursor + buffer::skip_whitespace(&line, xy.col, DP::Right)?
            }
            dp => err_at!(Fatal, msg: format!("invalid direction: {}", dp))?,
        };
        Ok(cursor)
    }

    fn mto_screen_end(&self, buf: &Buffer, mut n: usize, dp: DP) -> Result<usize> {
        use crate::text;

        let edit_wth = self.to_edit_width().1 as usize;
        let mut col = (self.obc_xy.col / edit_wth) * edit_wth;
        let mut row = self.obc_xy.row;

        let mut to_cursor = || -> Result<usize> {
            for line in buf.lines_at(self.obc_xy.row, DP::Right)? {
                let m = text::visual_line_n(&line.to_string());
                let ends: Vec<usize> = {
                    let iter = (0..).map(|i| i * edit_wth).skip_while(|c| c < &col);
                    iter.take_while(|c| c <= &m).collect()
                };
                if ends.len() < n {
                    n -= ends.len();
                    row += 1;
                    col = 0;
                } else {
                    let item = ends.into_iter().skip(n).next();
                    let end = item.unwrap_or(m).saturating_sub(1);
                    let cursor = buf.line_to_char(row) + cmp::min(end, m);
                    return Ok(cursor);
                }
            }
            buffer::mto_end(buf)
        };

        let cursor = match dp {
            DP::TextCol => {
                let cursor = to_cursor()?;
                let xy = buf.to_xy_cursor(Some(cursor));
                let n = {
                    let home = &buf.line(xy.row);
                    buffer::skip_whitespace(home, xy.col, DP::Right)?
                };
                cursor + n
            }
            DP::None => to_cursor()?,
            dp => err_at!(Fatal, msg: format!("invalid direction: {}", dp))?,
        };
        Ok(cursor)
    }

    fn mto_screen_middle(&self, buf: &Buffer) -> Result<usize> {
        use crate::buffer::{mto_left, mto_right};

        let edit_wth = self.to_edit_width().1 / 2;
        let cursor = match self.to_cursor_col() {
            c if edit_wth < c => {
                let n = c.saturating_sub(edit_wth) as usize;
                mto_left(buf, n, DP::LineBound)?
            }
            c => {
                let n = edit_wth.saturating_sub(c) as usize;
                mto_right(buf, n, DP::LineBound)?
            }
        };
        Ok(cursor)
    }

    fn mto_screen_up(&self, app: &mut code::Code, buf: &Buffer, n: usize, dp: DP) -> Result<usize> {
        let cursor = if app.as_ref().wrap {
            let (nu_wth, edit_wth) = self.to_edit_width();
            let edit_col = self.to_cursor_col() as usize;
            let cursor = buf.to_char_cursor();

            let mut slines = {
                let bc_xy = buf.to_xy_cursor(None);
                let (from, to) = (bc_xy.row.saturating_sub(n), bc_xy.row);
                view::wrap_lines(buf, (from..=to).collect(), nu_wth, edit_wth)
            };
            slines.reverse();
            let item = {
                let mut iter = slines.into_iter().skip_while(|sl| sl.bc > cursor);
                iter.next();
                iter.skip(n.saturating_sub(1)).next().clone()
            };
            item.map(|sl| sl.bc + edit_col).unwrap_or(edit_col)
        } else {
            buffer::mto_up(buf, n, dp)?
        };
        Ok(cursor)
    }

    fn mto_screen_down(
        &self,
        app: &mut code::Code,
        buf: &Buffer,
        n: usize,
        dp: DP,
    ) -> Result<usize> {
        let cursor = if app.as_ref().wrap {
            let (nu_wth, edit_wth) = self.to_edit_width();
            let scr_col = self.to_cursor_col() as usize;
            let cursor = buf.to_char_cursor();

            let slines = {
                let bc_xy = buf.to_xy_cursor(None);
                let last_line = buf.to_last_line_idx();
                let from = bc_xy.row;
                let to = cmp::min(last_line, bc_xy.row + n);
                view::wrap_lines(buf, (from..=to).collect(), nu_wth, edit_wth)
            };
            let item = {
                let iter = slines.into_iter().skip_while(|sl| sl.bc <= cursor);
                iter.skip(n.saturating_sub(1)).next().clone()
            };
            item.map(|sl| sl.bc + scr_col).unwrap_or(scr_col)
        } else {
            buffer::mto_up(buf, n, dp)?
        };
        Ok(cursor)
    }

    fn mto_win_high(&self, app: &mut code::Code, buf: &Buffer, n: usize) -> Result<usize> {
        use crate::text;

        let screen_lines = if app.as_ref().wrap {
            let mut v: view::Wrap = (&*self, self.obc_xy).try_into()?;
            v.shift_cursor(buf);
            v.to_edit_lines(buf)
        } else {
            let mut v: view::NoWrap = (&*self, self.obc_xy).try_into()?;
            v.shift_cursor(buf);
            v.to_edit_lines(buf)
        };
        let screen_lines: Vec<view::ScrLine> = {
            let iter = screen_lines.into_iter();
            iter.take_while(|sl| !sl.colk.is_empty()).collect()
        };
        let bc = match screen_lines.len() {
            0 => buf.to_char_cursor(),
            m if n < m => {
                let soff = self.scroll_off as usize;
                let off = limit!(n, soff, m.saturating_sub(soff + 1));
                screen_lines[off].bc
            }
            _ => screen_lines.last().unwrap().bc,
        };
        let bc = {
            let xy = buf.to_xy_cursor(Some(bc));
            let line = buf.line(xy.row);
            let line = text::visual_line(&line);
            bc + buffer::skip_whitespace(line, xy.col, DP::Right)?
        };
        Ok(bc)
    }

    fn mto_win_middle(&self, app: &mut code::Code, buf: &Buffer) -> Result<usize> {
        use crate::text;

        let screen_lines = if app.as_ref().wrap {
            let mut v: view::Wrap = (&*self, self.obc_xy).try_into()?;
            v.shift_cursor(buf);
            v.to_edit_lines(buf)
        } else {
            let mut v: view::NoWrap = (&*self, self.obc_xy).try_into()?;
            v.shift_cursor(buf);
            v.to_edit_lines(buf)
        };
        let screen_lines: Vec<view::ScrLine> = {
            let iter = screen_lines.into_iter();
            iter.take_while(|sl| !sl.colk.is_empty()).collect()
        };
        let bc = match screen_lines.len() {
            0 => buf.to_char_cursor(),
            m => screen_lines[m / 2].bc,
        };
        let bc = {
            let xy = buf.to_xy_cursor(Some(bc));
            let line = buf.line(xy.row);
            let line = text::visual_line(&line);
            bc + buffer::skip_whitespace(line, xy.col, DP::Right)?
        };
        Ok(bc)
    }

    fn mto_win_low(&self, app: &mut code::Code, buf: &Buffer, n: usize) -> Result<usize> {
        use crate::text;

        let screen_lines = if app.as_ref().wrap {
            let mut v: view::Wrap = (&*self, self.obc_xy).try_into()?;
            v.shift_cursor(buf);
            v.to_edit_lines(buf)
        } else {
            let mut v: view::NoWrap = (&*self, self.obc_xy).try_into()?;
            v.shift_cursor(buf);
            v.to_edit_lines(buf)
        };
        let screen_lines: Vec<view::ScrLine> = {
            let iter = screen_lines.into_iter();
            iter.take_while(|sl| !sl.colk.is_empty()).collect()
        };
        let bc = match screen_lines.len() {
            0 => buf.to_char_cursor(),
            m => {
                let off = m.saturating_sub(n + 1);
                let soff = self.scroll_off as usize;
                let off = limit!(off, soff, m.saturating_sub(soff + 1));
                screen_lines[off].bc
            }
        };
        let bc = {
            let xy = buf.to_xy_cursor(Some(bc));
            let line = buf.line(xy.row);
            let line = text::visual_line(&line);
            bc + buffer::skip_whitespace(line, xy.col, DP::Right)?
        };
        Ok(bc)
    }

    //fn mto_win_scroll(
    //    &self,
    //    app: &mut code::Code,
    //    buf: &Buffer,
    //    n: usize,
    //    scroll: Scroll,
    //    dp: DP,
    //) -> Result<usize> {
    //    let max_row = self.coord.hgt
    //    match (scroll, dp) {
    //        (Scroll::Ones, DP::Left) => {
    //            self.cursor = match self.cursor + 1 {
    //                cursor if cursor >
    //            }
    //        }
    //        (Scroll::Ones, DP::Right) =>
    //        (Scroll::Lines, DP::Left) =>
    //        (Scroll::Lines, DP::Right) =>
    //        (Scroll::Pages, DP::Left) =>
    //        (Scroll::Pages, DP::Right) =>
    //    }
    //}
}

impl Window for WindowEdit {
    type App = code::Code;

    #[inline]
    fn to_name(&self) -> String {
        "window-edit".to_string()
    }

    #[inline]
    fn to_coord(&self) -> Coord {
        self.coord
    }

    #[inline]
    fn to_cursor(&self) -> Option<Cursor> {
        Some(self.coord.to_top_left() + self.cursor)
    }

    #[inline]
    fn config_line_number(&self) -> bool {
        self.line_number
    }

    #[inline]
    fn config_scroll_offset(&self) -> u16 {
        self.scroll_off
    }

    fn on_event(&mut self, app: &mut code::Code, evnt: Event) -> Result<Event> {
        use crate::{event::Mto, pubsub::Notify};

        let (evnt, buf) = match app.take_buffer(&self.curr_buf_id) {
            Some(mut buf) => match self.keymap.fold(&mut buf, evnt)? {
                Event::Mt(Mto::ScreenHome(dp)) => {
                    let cursor = self.mto_screen_home(&buf, dp)?;
                    buf.set_cursor(cursor).clear_sticky_col();
                    (Event::Noop, Some(buf))
                }
                Event::Mt(Mto::ScreenEnd(n, dp)) => {
                    let cursor = self.mto_screen_end(&buf, n, dp)?;
                    buf.set_cursor(cursor).clear_sticky_col();
                    (Event::Noop, Some(buf))
                }
                Event::Mt(Mto::ScreenMiddle) => {
                    let cursor = self.mto_screen_middle(&buf)?;
                    buf.set_cursor(cursor).clear_sticky_col();
                    (Event::Noop, Some(buf))
                }
                Event::Mt(Mto::ScreenUp(n, dp)) => {
                    let cursor = self.mto_screen_up(app, &buf, n, dp)?;
                    buf.set_cursor(cursor).clear_sticky_col();
                    (Event::Noop, Some(buf))
                }
                Event::Mt(Mto::ScreenDown(n, dp)) => {
                    let cursor = self.mto_screen_down(app, &buf, n, dp)?;
                    buf.set_cursor(cursor).clear_sticky_col();
                    (Event::Noop, Some(buf))
                }
                Event::Mt(Mto::WinH(n)) => {
                    let cursor = {
                        let n = n.saturating_sub(1);
                        self.mto_win_high(app, &buf, n)?
                    };
                    buf.set_cursor(cursor).clear_sticky_col();
                    (Event::Noop, Some(buf))
                }
                Event::Mt(Mto::WinM) => {
                    let cursor = self.mto_win_middle(app, &buf)?;
                    buf.set_cursor(cursor).clear_sticky_col();
                    (Event::Noop, Some(buf))
                }
                Event::Mt(Mto::WinL(n)) => {
                    let cursor = {
                        let n = n.saturating_sub(1);
                        self.mto_win_low(app, &buf, n)?
                    };
                    buf.set_cursor(cursor).clear_sticky_col();
                    (Event::Noop, Some(buf))
                }
                //Event::Mt(Mto::WinScroll(n, scroll, dp)) => {
                //    let cursor = self.mto_win_scroll(app, &buf, n, scroll, dp)?;
                //    buf.set_cursor(cursor).clear_sticky_col();
                //    (Event::Noop, Some(buf))
                //}
                Event::Appn(event::Appn::StatusCursor) => {
                    let msg = vec![self.syn.to_status_cursor()?];
                    app.notify("code", Notify::Status(msg))?;
                    (Event::Noop, Some(buf))
                }
                evnt => {
                    let evnt = buf.on_event(evnt)?;
                    let evnt = self.syn.on_edit(&mut buf, evnt)?;
                    (evnt, Some(buf))
                }
            },
            None => (evnt, None),
        };

        buf.map(|buf| app.add_buffer(buf));
        Ok(evnt)
    }

    fn on_refresh(&mut self, app: &mut code::Code) -> Result<()> {
        let err = {
            let s = format!("buffer {}", self.curr_buf_id);
            Error::Invalid(String::new(), s)
        };
        self.cursor = if app.as_ref().wrap {
            let mut v: view::Wrap = (&*self, self.obc_xy).try_into()?;
            let buf = err_at!(app.as_buffer(&self.curr_buf_id).ok_or(err))?;
            v.shift_cursor(buf);
            let old_screen = self.old_screen.replace(v.to_edit_lines(buf));
            v.render(buf, self, old_screen)?
        } else {
            let mut v: view::NoWrap = (&*self, self.obc_xy).try_into()?;
            let buf = err_at!(app.as_buffer(&self.curr_buf_id).ok_or(err))?;
            v.shift_cursor(buf);
            let old_screen = self.old_screen.replace(v.to_edit_lines(buf));
            v.render(buf, self, old_screen)?
        };
        self.obc_xy = {
            let err = {
                let s = format!("buffer {}", self.curr_buf_id);
                Error::Invalid(String::new(), s)
            };
            let buf = err_at!(app.as_buffer(&self.curr_buf_id).ok_or(err))?;
            buf.to_xy_cursor(None)
        };
        debug!("obc_xy:{}", self.obc_xy);

        Ok(())
    }
}

impl Render for WindowEdit {
    type Buf = Buffer;
    #[inline]
    fn as_color_scheme(&self) -> &ColorScheme {
        &self.scheme
    }

    fn to_span_line(&self, buf: &Self::Buf, a: usize, z: usize) -> Result<Spanline> {
        self.syn.to_span_line(buf, a, z)
    }
}
