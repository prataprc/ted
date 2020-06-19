use crossterm::{execute, queue};
use log::trace;

use std::{
    fmt,
    io::{self, Write},
    result,
};

use crate::{window::Cursor, Error, Result};

/// Captures the screen and cleans up on exit.
pub struct Terminal {
    /// number of colums on the screen
    pub cols: u16,
    /// number of rows on the screen
    pub rows: u16,
}

impl From<(u16, u16)> for Terminal {
    fn from((cols, rows): (u16, u16)) -> Terminal {
        Terminal { cols, rows }
    }
}

impl Terminal {
    pub fn init() -> Result<Terminal> {
        use crossterm::cursor::Hide;
        use crossterm::event::EnableMouseCapture;
        use crossterm::terminal::{enable_raw_mode, size, EnterAlternateScreen};

        let tm: Terminal = err_at!(Fatal, size())?.into();

        err_at!(Fatal, enable_raw_mode())?;
        err_at!(
            Fatal,
            execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture, Hide)
        )?;
        trace!(
            "{} color_count:{}",
            tm,
            crossterm::style::available_color_count()
        );

        Ok(tm)
    }
}

impl fmt::Display for Terminal {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "Terminal<{},{}>", self.cols, self.rows)
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        use crossterm::cursor::Show;
        use crossterm::event::DisableMouseCapture;
        use crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};

        execute!(
            io::stdout(),
            LeaveAlternateScreen,
            DisableMouseCapture,
            Show
        )
        .unwrap();
        disable_raw_mode().unwrap();
    }
}

#[inline]
pub fn flush(cursor: Cursor) -> Result<()> {
    use crossterm::cursor::{MoveTo, Show};

    let mut stdout = io::stdout();
    let Cursor { col, row } = cursor;
    err_at!(Fatal, queue!(stdout, MoveTo(col, row), Show))?;
    err_at!(Fatal, stdout.flush())?;
    Ok(())
}

#[inline]
pub fn hide_cursor() -> Result<()> {
    use crossterm::cursor::Hide;

    let mut stdout = io::stdout();
    err_at!(Fatal, queue!(stdout, Hide))?;
    Ok(())
}
