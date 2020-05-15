use crossterm::{
    self, cursor as term_cursor, event as ct_event,
    event::{DisableMouseCapture, EnableMouseCapture, Event as TermEvent},
    execute, queue,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use log::{trace, warn};

use std::{
    io::{self, Write},
    time::SystemTime,
};

use crate::{
    app_code::Code, buffer::Buffer, config::Config, event::Event, stats, window::Cursor, Error,
    Result,
};

// Application state
pub struct State {
    pub tm: Terminal,
    pub config: Config,
    pub buffers: Vec<Buffer>,
}

impl AsRef<Config> for State {
    fn as_ref(&self) -> &Config {
        &self.config
    }
}

impl State {
    pub fn new(config: Config) -> Result<State> {
        let tm = Terminal::init()?;
        Ok(State {
            tm,
            config,
            buffers: Default::default(),
        })
    }

    pub fn event_loop(mut self, mut app: App, mut evnt: Event) -> Result<()> {
        let mut stdout = io::stdout();
        let mut stats = stats::Latency::new();

        let mut start = SystemTime::now();

        // TODO: later statistics can be moved to a different release stream
        // and or controlled by command line option.
        let res = loop {
            // hide cursor, handle event and refresh window
            let _evnt = match evnt {
                Event::Noop => Event::Noop,
                evnt => {
                    err_at!(Fatal, queue!(stdout, term_cursor::Hide))?;
                    let evnt = app.on_event(&mut self, evnt)?;
                    app.on_refresh(&mut self)?;
                    evnt
                }
            };
            // show-cursor
            let Cursor { col, row } = app.to_cursor();
            err_at!(Fatal, queue!(stdout, term_cursor::MoveTo(col, row)))?;
            err_at!(Fatal, queue!(stdout, term_cursor::Show))?;
            err_at!(Fatal, stdout.flush())?;

            stats.sample(start.elapsed().unwrap());
            // new event
            evnt = {
                let tevnt: TermEvent = err_at!(Fatal, ct_event::read())?;
                trace!("{:?} Cursor:({},{})", tevnt, col, row);
                match tevnt.clone().into() {
                    Event::Char('q', m) if m.is_empty() => break Ok(()),
                    evnt => evnt,
                }
            };
            start = SystemTime::now();
        };

        stats.pretty_print("");

        res
    }

    pub fn take_buffer(&mut self, id: &str) -> Option<Buffer> {
        let i = {
            let mut iter = self.buffers.iter().enumerate();
            loop {
                match iter.next() {
                    Some((i, b)) if b.to_id() == id => break Some(i),
                    None => break None,
                    _ => (),
                }
            }
        };
        match i {
            Some(i) => Some(self.buffers.remove(i)),
            None => None,
        }
    }

    pub fn add_buffer(&mut self, buffer: Buffer) {
        self.buffers.insert(0, buffer)
    }
}

impl State {
    pub fn as_buffer(&self, id: &str) -> &Buffer {
        for b in self.buffers.iter() {
            if b.to_id() == id {
                return b;
            }
        }
        unreachable!()
    }

    pub fn as_mut_buffer(&mut self, id: &str) -> &mut Buffer {
        for b in self.buffers.iter_mut() {
            if b.to_id() == id {
                return b;
            }
        }
        unreachable!()
    }

    pub fn to_buffer_num(&self, id: String) -> Option<usize> {
        for b in self.buffers.iter() {
            if b.to_id() == id {
                return Some(b.to_num());
            }
        }
        None
    }
}

pub enum App {
    Code(Code),
    None,
}

impl Default for App {
    fn default() -> App {
        App::None
    }
}

impl App {
    fn to_cursor(&self) -> Cursor {
        match self {
            App::Code(code) => code.to_cursor(),
            App::None => {
                warn!("application not selected !!");
                Default::default()
            }
        }
    }

    fn on_event(&mut self, s: &mut State, evnt: Event) -> Result<Event> {
        match self {
            App::Code(code) => code.on_event(s, evnt),
            App::None => {
                warn!("application not selected !!");
                Ok(Event::Noop)
            }
        }
    }

    fn on_refresh(&mut self, s: &mut State) -> Result<()> {
        match self {
            App::Code(code) => code.on_refresh(s),
            App::None => {
                warn!("application not selected !!");
                Ok(())
            }
        }
    }
}

pub struct Terminal {
    stdout: io::Stdout,
    pub cols: u16,
    pub rows: u16,
}

impl Terminal {
    fn init() -> Result<Terminal> {
        let mut stdout = io::stdout();
        err_at!(Fatal, terminal::enable_raw_mode())?;
        err_at!(
            Fatal,
            execute!(
                stdout,
                EnterAlternateScreen,
                EnableMouseCapture,
                term_cursor::Hide
            )
        )?;

        let (cols, rows) = err_at!(Fatal, terminal::size())?;
        Ok(Terminal { stdout, cols, rows })
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        execute!(
            self.stdout,
            LeaveAlternateScreen,
            DisableMouseCapture,
            term_cursor::Show
        )
        .unwrap();
        terminal::disable_raw_mode().unwrap();
    }
}
