#[allow(unused_imports)]
use log::{debug, trace};

use std::{fmt, result};

use crate::{
    app::Application,
    buffer::{self, Buffer},
    code::{self, keymap::Keymap},
    col_nu::ColNu,
    colors::ColorScheme,
    event::{self, Event},
    syntax::{self, Syntax},
    term::Spanline,
    window::{Coord, Cursor, Render, WinBuffer, Window},
    Error, Result,
};

pub struct WindowEdit {
    coord: Coord,
    cursor: Cursor,
    obc_xy: buffer::Cursor,
    curr_buf_id: String,
    altn_buf_id: Option<String>,
    syn: syntax::Type,
    scheme: ColorScheme,
    keymap: Keymap,
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

impl WindowEdit {
    #[inline]
    pub fn new(coord: Coord, buf: &Buffer, app: &code::Code) -> WindowEdit {
        use crate::view::{NoWrap, Wrap};

        let cursor = if app.config.wrap {
            Wrap::initial_cursor(app.config.line_number)
        } else {
            NoWrap::initial_cursor(app.config.line_number)
        };

        let scheme = app.to_color_scheme(None);
        let we = WindowEdit {
            coord,
            cursor,
            obc_xy: (0, 0).into(),
            curr_buf_id: buf.to_id(),
            altn_buf_id: None,
            syn: syntax::detect(buf, &scheme).unwrap(),
            scheme,
            keymap: Keymap::new_edit(),
            // configuration
            scroll_off: app.as_ref().scroll_off,
            line_number: app.as_ref().line_number,
        };
        debug!("{} {} {}", we, we.scroll_off, we.line_number);
        we
    }

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
    fn to_screen_width(&self) -> u16 {
        let nu_wth = ColNu::new(self.obc_xy.row, self.line_number).to_width();
        self.coord.wth - nu_wth
    }

    // return the number of characters to move left ro reach screen-home.
    fn to_cursor_col(&self) -> u16 {
        let nu_wth = ColNu::new(self.obc_xy.row, self.line_number).to_width();
        self.cursor.col - nu_wth
    }

    fn mto_screen_end(&self, buf: &mut Buffer, mut n: usize) -> Result<()> {
        use crate::{event::DP, text::Format};
        use std::cmp;

        let cursor = {
            let w = self.to_screen_width() as usize;
            let mut col = (self.obc_xy.col / w) * w;
            let mut row = self.obc_xy.row;

            let mut iter = buf.lines_at(self.obc_xy.row, DP::Right)?;
            loop {
                match iter.next() {
                    Some(line) => {
                        let m = {
                            let s = line.to_string();
                            Format::trim_newline(&s).0.chars().count()
                        };
                        let ends: Vec<usize> = {
                            let iter = (0..).map(|i| i * w);
                            let iter = iter.skip_while(|c| c < &col);
                            iter.take_while(|c| c <= &m).collect()
                        };
                        if ends.len() <= n {
                            n -= ends.len();
                            row += 1;
                            col = 0;
                        } else {
                            let cursor = {
                                let item = ends.into_iter().skip(n).next();
                                let end = item.unwrap_or(m).saturating_sub(1);
                                buf.line_to_char(row) + cmp::min(end, m)
                            };
                            break cursor;
                        }
                    }
                    None => {
                        let line_idx = buf.n_lines().saturating_sub(1);
                        let n = {
                            let s = buf.line(line_idx);
                            Format::trim_newline(&s).0.chars().count()
                        };
                        break buf.line_to_char(line_idx) + n.saturating_sub(1);
                    }
                }
            }
        };

        buf.set_cursor(cursor);
        Ok(())
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
    fn to_cursor(&self) -> Cursor {
        self.coord.to_top_left() + self.cursor
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
        use crate::{
            buffer::mto_left,
            event::{Mto, DP},
            pubsub::Notify,
        };

        let (evnt, buf) = match app.take_buffer(&self.curr_buf_id) {
            Some(mut buf) => match self.keymap.fold(app, &mut buf, evnt)? {
                Event::Code(event::Code::StatusCursor) => {
                    let msg = vec![self.syn.to_status_cursor()?];
                    app.notify("code", Notify::Status(msg))?;
                    (Event::Noop, Some(buf))
                }
                Event::Mt(Mto::ScreenHome(DP::TextCol)) => {
                    let c = self.to_cursor_col() as usize;
                    let evnt = mto_left(&mut buf, c, DP::None)?;
                    buf.skip_whitespace(DP::Right);
                    (evnt, Some(buf))
                }
                Event::Mt(Mto::ScreenHome(DP::None)) => {
                    let c = self.to_cursor_col() as usize;
                    let evnt = mto_left(&mut buf, c, DP::None)?;
                    (evnt, Some(buf))
                }
                Event::Mt(Mto::ScreenEnd(n, DP::TextCol)) => {
                    self.mto_screen_end(&mut buf, n)?;
                    buf.skip_whitespace(DP::Left);
                    (Event::Noop, Some(buf))
                }
                Event::Mt(Mto::ScreenEnd(n, DP::None)) => {
                    self.mto_screen_end(&mut buf, n)?;
                    (Event::Noop, Some(buf))
                }
                Event::Mt(Mto::ScreenMiddle(_)) => {
                    let m = self.to_screen_width() / 2;
                    let evnt = match self.to_cursor_col() {
                        c if m < c => {
                            let n = c.saturating_sub(m) as usize;
                            buffer::mto_right(&mut buf, n, DP::None)?
                        }
                        c => {
                            let n = m.saturating_sub(c) as usize;
                            buffer::mto_right(&mut buf, n, DP::None)?
                        }
                    };
                    (evnt, Some(buf))
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
        use crate::view::{NoWrap, Wrap};

        let err = Error::Invalid(format!("buffer {}", self.curr_buf_id));
        self.cursor = if app.as_ref().wrap {
            let v: Wrap = (&*self, self.obc_xy).into();
            let buf = err_at!(app.as_buffer(&self.curr_buf_id).ok_or(err))?;
            v.render(buf, self)?
        } else {
            let v: NoWrap = (&*self, self.obc_xy).into();
            let buf = err_at!(app.as_buffer(&self.curr_buf_id).ok_or(err))?;
            v.render(buf, self)?
        };
        self.obc_xy = {
            let err = Error::Invalid(format!("buffer {}", self.curr_buf_id));
            err_at!(app.as_buffer(&self.curr_buf_id).ok_or(err))?.to_xy_cursor()
        };

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
