#[allow(unused_imports)]
use log::{debug, trace};

use std::{cmp, convert::TryInto, fmt, result};

use crate::{
    app::Application,
    buffer::{self, Buffer},
    code::{self},
    colors::ColorScheme,
    event::{self, Event, Scroll, DP},
    keymap::Keymap,
    scroll,
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
    wrap: bool,
    scroll_off: u16,
    line_number: bool,
    scroll: Option<usize>,
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
            wrap: app.as_ref().wrap,
            scroll_off: app.as_ref().scroll_off,
            line_number: app.as_ref().line_number,
            scroll: None,
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
    fn to_edit_lines(&self, buf: &Buffer) -> Result<Vec<view::ScrLine>> {
        if self.wrap {
            let mut v: view::Wrap = (&*self, self.obc_xy).try_into()?;
            v.shift_cursor(buf)?;
            Ok(v.to_edit_lines(buf))
        } else {
            let mut v: view::NoWrap = (&*self, self.obc_xy).try_into()?;
            v.shift_cursor(buf)?;
            Ok(v.to_edit_lines(buf))
        }
    }

    fn to_slide_width(&self, buf: &Buffer) -> Result<usize> {
        let (_, nu_wth) = {
            let lines = self.to_edit_lines(buf)?;
            view::to_nu_width(&lines, self.line_number)
        };
        Ok((self.coord.wth.saturating_sub(nu_wth) / 2) as usize)
    }

    fn mto_screen_home(&self, buf: &Buffer, dp: DP) -> Result<usize> {
        let lines = self.to_edit_lines(buf)?;
        let nbc = match view::cursor_line(&lines, buf.to_char_cursor()) {
            Some(off) => lines[off].bc,
            None => buf.to_char_cursor(),
        };

        let nbc = match dp {
            DP::TextCol => {
                let xy = buf.to_xy_cursor(Some(nbc));
                let line = buf.line(xy.row);
                nbc + buffer::skip_whitespace(&line, xy.col, DP::Right)?
            }
            DP::None => nbc,
            dp => err_at!(Fatal, msg: format!("invalid direction: {}", dp))?,
        };
        Ok(nbc)
    }

    fn mto_screen_middle(&self, buf: &Buffer) -> Result<usize> {
        let lines = self.to_edit_lines(buf)?;
        let (_, nu_wth) = view::to_nu_width(&lines, self.line_number);
        let middle = self.coord.wth.saturating_sub(nu_wth) / 2;

        let nbc = match view::cursor_line(&lines, buf.to_char_cursor()) {
            Some(off) if lines[off].n <= middle => {
                let eol = lines[off].n.saturating_sub(1) as usize;
                lines[off].bc + eol
            }
            Some(off) => lines[off].bc + (middle as usize),
            None => buf.to_char_cursor(),
        };
        Ok(nbc)
    }

    fn mto_screen_end(&self, buf: &Buffer, n: usize) -> Result<usize> {
        let name = "mto_screen_end";
        let item = if self.wrap {
            let iter = scroll::WrapIter::new_scroll_down(name, self, buf)?;
            iter.skip(n.saturating_sub(1)).next()
        } else {
            let iter = scroll::NowrapIter::new_scroll_down(name, self, buf)?;
            iter.skip(n.saturating_sub(1)).next()
        };
        let nbc = match item {
            Some(sl) => sl.bc + (sl.n.saturating_sub(1) as usize),
            None => buffer::last_char_idx(buf),
        };
        Ok(nbc)
    }

    fn mto_screen_up(&self, buf: &Buffer, n: usize) -> Result<usize> {
        let (_, nu_wth) = {
            let lines = self.to_edit_lines(buf)?;
            view::to_nu_width(&lines, self.line_number)
        };

        let name = "mto_screen_up";
        let item = if self.wrap {
            let iter = scroll::WrapIter::new_scroll_up(name, self, buf)?;
            iter.skip(n.saturating_sub(1)).next()
        } else {
            let iter = scroll::NowrapIter::new_scroll_up(name, self, buf)?;
            iter.skip(n.saturating_sub(1)).next()
        };
        let col = self.cursor.col.saturating_sub(nu_wth);
        let nbc = match item {
            Some(sl) if col < sl.n => sl.bc + (col as usize),
            Some(sl) => sl.bc + (sl.n.saturating_sub(1) as usize),
            None => 0,
        };
        Ok(nbc)
    }

    fn mto_screen_down(&self, buf: &Buffer, n: usize) -> Result<usize> {
        let (_, nu_wth) = {
            let lines = self.to_edit_lines(buf)?;
            view::to_nu_width(&lines, self.line_number)
        };

        let name = "mto_screen_down";
        let item = if self.wrap {
            let iter = scroll::WrapIter::new_scroll_down(name, self, buf)?;
            iter.skip(n).next()
        } else {
            let iter = scroll::NowrapIter::new_scroll_down(name, self, buf)?;
            iter.skip(n).next()
        };
        let col = self.cursor.col.saturating_sub(nu_wth);
        let nbc = match item {
            Some(sl) if col < sl.n => sl.bc + (col as usize),
            Some(sl) => sl.bc + (sl.n.saturating_sub(1) as usize),
            None => 0,
        };
        Ok(nbc)
    }

    fn mto_win_high(&self, buf: &Buffer, n: usize) -> Result<usize> {
        let lines: Vec<view::ScrLine> = {
            let lines = self.to_edit_lines(buf)?;
            let iter = lines.into_iter().filter(|line| !line.colk.is_empty());
            iter.collect()
        };
        let nbc = match lines.len() {
            0 => buf.to_char_cursor(),
            _ => {
                let sl = match lines.iter().skip(n.saturating_sub(1)).next() {
                    Some(sl) => sl,
                    None => lines.last().unwrap(),
                };
                let xy = buf.to_xy_cursor(Some(sl.bc));
                let line = buf.line(xy.row);
                sl.bc + buffer::skip_whitespace(&line, xy.col, DP::Right)?
            }
        };
        Ok(nbc)
    }

    fn mto_win_middle(&self, buf: &Buffer) -> Result<usize> {
        let lines = self.to_edit_lines(buf)?;
        let nbc = {
            let bc = match lines.len() {
                0 => buf.to_char_cursor(),
                m => lines[m / 2].bc,
            };
            let xy = buf.to_xy_cursor(Some(bc));
            let line = buf.line(xy.row);
            bc + buffer::skip_whitespace(&line, xy.col, DP::Right)?
        };
        Ok(nbc)
    }

    fn mto_win_low(&self, buf: &Buffer, n: usize) -> Result<usize> {
        let lines: Vec<view::ScrLine> = {
            let lines = self.to_edit_lines(buf)?;
            let iter = lines.into_iter().filter(|line| !line.colk.is_empty());
            iter.collect()
        };
        let nbc = match lines.len() {
            0 => buf.to_char_cursor(),
            m => {
                let off = if m < (self.scroll_off as usize) {
                    m.saturating_sub(n + 1)
                } else {
                    cmp::max(m.saturating_sub(n + 1), self.scroll_off as usize)
                };
                let xy = buf.to_xy_cursor(Some(lines[off].bc));
                let line = buf.line(xy.row);
                lines[off].bc + buffer::skip_whitespace(&line, xy.col, DP::Right)?
            }
        };
        Ok(nbc)
    }

    fn mto_win_scroll(
        &mut self,
        buf: &mut Buffer,
        mto: event::Mto,
    ) -> Result<(usize, Option<Cursor>)> {
        use crate::scroll::{scroll_down, scroll_left, scroll_right, scroll_up};

        let name = format!("mto_win_scroll-{}", mto);
        let (n, scrll, dp) = match mto {
            event::Mto::WinScroll(n, scrll, dp) => (n, scrll, dp),
            mto => err_at!(Fatal, msg: format!("{}", mto))?,
        };

        match (scrll, dp) {
            (Scroll::Ones, DP::Left) => {
                let (cursor, nbc) = scroll_up(&name, self, buf, n)?;
                Ok((nbc, Some(cursor)))
            }
            (Scroll::Ones, DP::Right) => {
                let (cursor, nbc) = scroll_down(&name, self, buf, n)?;
                Ok((nbc, Some(cursor)))
            }
            (Scroll::Lines, DP::Left) if n == 1 => {
                let n = self.scroll.unwrap_or((self.coord.hgt / 2) as usize);
                let (cursor, nbc) = scroll_up(&name, self, buf, n)?;
                Ok((nbc, Some(cursor)))
            }
            (Scroll::Lines, DP::Left) => {
                let (cursor, nbc) = scroll_up(&name, self, buf, n)?;
                Ok((nbc, Some(cursor)))
            }
            (Scroll::Lines, DP::Right) if n == 1 => {
                let n = self.scroll.unwrap_or((self.coord.hgt / 2) as usize);
                let (cursor, nbc) = scroll_down(&name, self, buf, n)?;
                Ok((nbc, Some(cursor)))
            }
            (Scroll::Lines, DP::Right) => {
                let (cursor, nbc) = scroll_down(&name, self, buf, n)?;
                Ok((nbc, Some(cursor)))
            }
            (Scroll::Pages, DP::Left) => {
                let n_page = self.coord.hgt.saturating_sub(2) as usize;
                let (cursor, nbc) = scroll_up(&name, self, buf, n_page * n)?;
                Ok((nbc, Some(cursor)))
            }
            (Scroll::Pages, DP::Right) => {
                let n_page = self.coord.hgt.saturating_sub(2) as usize;
                let (cursor, nbc) = scroll_down(&name, self, buf, n_page * n)?;
                Ok((nbc, Some(cursor)))
            }
            (Scroll::TextUp, pos) => {
                let lines = self.to_edit_lines(buf)?;
                let (_, nu_wth) = view::to_nu_width(&lines, self.line_number);

                let (cursor, nbc) = if n == 0 {
                    let nbc = lines[self.cursor.row as usize].bc;
                    let (row, col) = (self.scroll_off, nu_wth);
                    (Cursor { row, col }, nbc)
                } else {
                    buf.set_cursor(0);
                    let (row, col) = (0, nu_wth);
                    self.cursor = Cursor { row, col };

                    let n = n.saturating_sub((self.scroll_off as usize) + 1);
                    let (mut cursor, nbc) = scroll_down(&name, self, buf, n)?;
                    cursor.row = self.scroll_off;
                    (cursor, nbc)
                };

                buf.set_cursor(nbc);
                self.obc_xy = buf.to_xy_cursor(Some(nbc));
                self.cursor = cursor;

                let nbc = match pos {
                    DP::TextCol => {
                        let xy = buf.to_xy_cursor(Some(nbc));
                        let line = buf.line(xy.row);
                        nbc + buffer::skip_whitespace(&line, xy.col, DP::Right)?
                    }
                    _ => nbc,
                };
                Ok((nbc, None))
            }
            (Scroll::TextCenter, pos) => {
                let lines = self.to_edit_lines(buf)?;
                let (_, nu_wth) = view::to_nu_width(&lines, self.line_number);

                let (cursor, nbc) = if n == 0 {
                    let nbc = lines[self.cursor.row as usize].bc;
                    let (row, col) = (self.coord.hgt / 2, nu_wth);
                    (Cursor { row, col }, nbc)
                } else {
                    buf.set_cursor(0);
                    let (row, col) = (0, nu_wth);
                    self.cursor = Cursor { row, col };

                    let n = n.saturating_sub((self.scroll_off as usize) + 1);
                    let (mut cursor, nbc) = scroll_down(&name, self, buf, n)?;
                    cursor.row = self.coord.hgt / 2;
                    (cursor, nbc)
                };

                buf.set_cursor(nbc);
                self.obc_xy = buf.to_xy_cursor(Some(nbc));
                self.cursor = cursor;
                let nbc = match pos {
                    DP::TextCol => {
                        let xy = buf.to_xy_cursor(Some(nbc));
                        let line = buf.line(xy.row);
                        nbc + buffer::skip_whitespace(&line, xy.col, DP::Right)?
                    }
                    _ => nbc,
                };
                Ok((nbc, None))
            }
            (Scroll::TextBottom, pos) => {
                let lines = self.to_edit_lines(buf)?;
                let (_, nu_wth) = view::to_nu_width(&lines, self.line_number);

                let (cursor, nbc) = if n == 0 {
                    let nbc = lines[self.cursor.row as usize].bc;
                    let (row, col) = (self.coord.hgt.saturating_sub(1), nu_wth);
                    (Cursor { row, col }, nbc)
                } else {
                    buf.set_cursor(0);
                    let (row, col) = (0, nu_wth);
                    self.cursor = Cursor { row, col };

                    let n = n.saturating_sub((self.scroll_off as usize) + 1);
                    let (mut cursor, nbc) = scroll_down(&name, self, buf, n)?;
                    cursor.row = self.coord.hgt.saturating_sub(1);
                    (cursor, nbc)
                };

                buf.set_cursor(nbc);
                self.obc_xy = buf.to_xy_cursor(Some(nbc));
                self.cursor = cursor;
                let nbc = match pos {
                    DP::TextCol => {
                        let xy = buf.to_xy_cursor(Some(nbc));
                        let line = buf.line(xy.row);
                        nbc + buffer::skip_whitespace(&line, xy.col, DP::Right)?
                    }
                    _ => nbc,
                };
                Ok((nbc, None))
            }
            (Scroll::Chars, DP::Right) if !self.wrap => {
                let (cursor, nbc) = scroll_right(&name, self, buf, n)?;
                Ok((nbc, Some(cursor)))
            }
            (Scroll::Chars, DP::Left) if !self.wrap => {
                let (cursor, nbc) = scroll_left(&name, self, buf, n)?;
                Ok((nbc, Some(cursor)))
            }
            (Scroll::Slide, DP::Right) if !self.wrap => {
                let n = self.to_slide_width(buf)?;
                let (cursor, nbc) = scroll_right(&name, self, buf, n)?;
                Ok((nbc, Some(cursor)))
            }
            (Scroll::Slide, DP::Left) if !self.wrap => {
                let n = self.to_slide_width(buf)?;
                let (cursor, nbc) = scroll_left(&name, self, buf, n)?;
                Ok((nbc, Some(cursor)))
            }
            (Scroll::Align, DP::Right) if !self.wrap => {
                let n = self.coord.wth.saturating_sub(self.cursor.col + 1);
                let (cursor, nbc) = scroll_right(&name, self, buf, n as usize)?;
                Ok((nbc, Some(cursor)))
            }
            (Scroll::Align, DP::Left) if !self.wrap => {
                let (_, nu_wth) = {
                    let lines = self.to_edit_lines(buf)?;
                    view::to_nu_width(&lines, self.line_number)
                };
                let n = self.cursor.col.saturating_sub(nu_wth) as usize;
                let (cursor, nbc) = scroll_right(&name, self, buf, n)?;
                Ok((nbc, Some(cursor)))
            }
            (Scroll::Chars, _) => Ok((buf.to_char_cursor(), None)),
            (Scroll::Slide, _) => Ok((buf.to_char_cursor(), None)),
            (Scroll::Align, _) => Ok((buf.to_char_cursor(), None)),
            (Scroll::Cursor, DP::Left) => todo!(),
            (Scroll::Cursor, DP::Right) => todo!(),
            (scrll, dp) => err_at!(Fatal, msg: format!("{} {}", scrll, dp))?,
        }
    }
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
    fn config_wrap(&self) -> bool {
        self.wrap
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
                    let nbc = self.mto_screen_home(&buf, dp)?;
                    buf.set_cursor(nbc).clear_sticky_col();
                    (Event::Noop, Some(buf))
                }
                Event::Mt(Mto::ScreenEnd(n, _dp)) => {
                    let nbc = self.mto_screen_end(&buf, n)?;
                    buf.set_cursor(nbc).clear_sticky_col();
                    (Event::Noop, Some(buf))
                }
                Event::Mt(Mto::ScreenMiddle) => {
                    let nbc = self.mto_screen_middle(&buf)?;
                    buf.set_cursor(nbc).clear_sticky_col();
                    (Event::Noop, Some(buf))
                }
                Event::Mt(Mto::ScreenUp(n, _dp)) => {
                    let nbc = self.mto_screen_up(&buf, n)?;
                    buf.set_cursor(nbc).clear_sticky_col();
                    (Event::Noop, Some(buf))
                }
                Event::Mt(Mto::ScreenDown(n, _dp)) => {
                    let nbc = self.mto_screen_down(&buf, n)?;
                    buf.set_cursor(nbc).clear_sticky_col();
                    (Event::Noop, Some(buf))
                }
                Event::Mt(Mto::WinH(n)) => {
                    let nbc = self.mto_win_high(&buf, n.saturating_sub(1))?;
                    buf.set_cursor(nbc).clear_sticky_col();
                    (Event::Noop, Some(buf))
                }
                Event::Mt(Mto::WinM) => {
                    let nbc = self.mto_win_middle(&buf)?;
                    buf.set_cursor(nbc).clear_sticky_col();
                    (Event::Noop, Some(buf))
                }
                Event::Mt(Mto::WinL(n)) => {
                    let nbc = self.mto_win_low(&buf, n.saturating_sub(1))?;
                    buf.set_cursor(nbc).clear_sticky_col();
                    (Event::Noop, Some(buf))
                }
                Event::Mt(mto @ Mto::WinScroll(_, _, _)) => {
                    let (nbc, cursor) = self.mto_win_scroll(&mut buf, mto)?;
                    self.cursor = match cursor {
                        Some(cursor) => {
                            self.obc_xy = buf.to_xy_cursor(Some(nbc));
                            cursor
                        }
                        None => self.cursor,
                    };
                    buf.set_cursor(nbc).clear_sticky_col();
                    (Event::Noop, Some(buf))
                }
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
        let buf = err_at!(app.as_buffer(&self.curr_buf_id).ok_or(err))?;
        self.cursor = if self.wrap {
            let mut v: view::Wrap = (&*self, self.obc_xy).try_into()?;
            v.shift_cursor(buf)?;
            let old_screen = self.old_screen.replace(v.to_edit_lines(buf));
            v.render(buf, self, old_screen)?
        } else {
            let mut v: view::NoWrap = (&*self, self.obc_xy).try_into()?;
            v.shift_cursor(buf)?;
            let old_screen = self.old_screen.replace(v.to_edit_lines(buf));
            v.render(buf, self, old_screen)?
        };
        self.obc_xy = buf.to_xy_cursor(None);
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
