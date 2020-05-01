#![feature(backtrace)]
#![feature(box_syntax)]

use crossterm::{
    self, cursor,
    event::{self as ct_event, Event as TermEvent},
    event::{DisableMouseCapture, EnableMouseCapture},
    execute, queue,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use dirs;
use log::trace;
use simplelog;
use structopt::StructOpt;

use std::{
    convert::TryInto,
    fs,
    io::{self, Write},
    mem, path,
    time::SystemTime,
};

use ted::{
    err_at, event, stats,
    window::{Coord, Cursor, State},
    window_file::WindowFile,
    window_prompt::WindowPrompt,
    Buffer, Config, Error, Event, Result, Window,
};

#[derive(Debug, StructOpt)]
pub struct Opt {
    #[structopt(long = "log", default_value = "")]
    log_file: String,

    #[structopt(short = "v", long = "verbose")]
    verbose: bool,

    #[structopt(long = "trace")]
    trace: bool,

    #[structopt(long = "stats")]
    stats: bool,

    files: Vec<String>,
}

fn main() {
    let opts = Opt::from_args();

    match init_logger(&opts) {
        Ok(()) => (),
        Err(err) => {
            println!("{}", err);
            std::process::exit(1);
        }
    }

    std::panic::set_hook(box |panic_info| {
        let mut strng = format!(
            "panic occured: {:?}",
            panic_info.payload().downcast_ref::<String>().unwrap()
        );
        strng.push_str(&format!("{}", std::backtrace::Backtrace::capture()));
        fs::write("ted-panic.out", strng.as_bytes()).unwrap();
    });

    match Application::run(opts) {
        Ok(_) => (),
        Err(err) => println!("{}", err),
    }
}

struct Application {
    tm: Terminal,
    s: State,
    inner: Inner,
}

enum Inner {
    OpenFiles {
        pw: WindowPrompt,
        window: Box<dyn Window>,
        flocs: Vec<event::OpenFile>,
        fds: Vec<event::OpenFile>,
    },
    Usual {
        window: Box<dyn Window>,
    },
    None,
}

impl Default for Inner {
    fn default() -> Inner {
        Inner::None
    }
}

impl Inner {
    fn as_mut_window(&mut self) -> &mut Box<(dyn ted::window::Window + 'static)> {
        match self {
            Inner::Usual { window } => window,
            Inner::OpenFiles { window, .. } => window,
            _ => todo!(),
        }
    }
}

impl Application {
    fn to_window_cursor(&self) -> Cursor {
        match &self.inner {
            Inner::Usual { window } => window.to_cursor(),
            Inner::OpenFiles { window, .. } => window.to_cursor(),
            _ => todo!(),
        }
    }

    fn run(opts: Opt) -> Result<()> {
        let config: Config = Default::default();
        let app = {
            let tm = Terminal::init()?;
            let coord = Coord::new(1, 1, tm.rows, tm.cols);
            let window = err_at!(Fatal, WindowFile::new(coord))?;
            Application {
                tm,
                s: State::new(config),
                inner: Inner::Usual {
                    window: Box::new(window),
                },
            }
        };

        let evnt = if opts.files.len() == 0 {
            Event::NewBuffer
        } else {
            let flocs: Vec<event::OpenFile> =
                opts.files.clone().into_iter().map(Into::into).collect();
            Event::OpenFiles { flocs }
        };

        app.event_loop(evnt)
    }

    fn event_loop(mut self, mut evnt: Event) -> Result<()> {
        let mut stats_a = stats::Latency::new();
        let mut stats_z = stats::Latency::new();

        // TODO: later statistics can be moved to a different release stream
        // and or controlled by command line option.
        let res = loop {
            let start = SystemTime::now();
            // app-handle bubble up event.
            self = match self.handle_up(evnt)? {
                Some(app) => app,
                None => break Ok(()),
            };

            err_at!(Fatal, self.dispatch_refresh())?;

            // show-cursor
            let Cursor { col, row } = self.to_window_cursor();
            err_at!(Fatal, queue!(self.tm.stdout, cursor::MoveTo(col, row)))?;
            err_at!(Fatal, queue!(self.tm.stdout, cursor::Show))?;
            err_at!(Fatal, self.tm.stdout.flush())?;
            stats_a.sample(start.elapsed().unwrap());

            // new event
            evnt = {
                let tevnt: TermEvent = err_at!(Fatal, ct_event::read())?;
                trace!("Event-{:?} Cursor:({},{})", tevnt, col, row);
                tevnt.clone().into()
            };

            let start = SystemTime::now();
            // hide-cursor
            err_at!(Fatal, queue!(self.tm.stdout, cursor::Hide))?;
            // app-handle bubble down event
            self = self.handle_down(evnt.clone())?;
            // event handling
            evnt = self.dispatch_event(evnt)?;
            stats_z.sample(start.elapsed().unwrap());
        };

        stats_a.pretty_print("");
        stats_z.pretty_print("");

        res
    }

    fn dispatch_event(&mut self, mut evnt: Event) -> Result<Event> {
        let mut inner = mem::replace(&mut self.inner, Default::default());
        evnt = inner.as_mut_window().on_event(&mut self.s, evnt)?;
        self.inner = inner;

        Ok(evnt)
    }

    fn dispatch_refresh(&mut self) -> Result<()> {
        let mut inner = mem::replace(&mut self.inner, Default::default());
        let res = inner.as_mut_window().refresh(&mut self.s);
        self.inner = inner;
        res
    }

    fn handle_up(mut self, evnt: Event) -> Result<Option<Self>> {
        match evnt {
            Event::NewBuffer => {
                let (buffer_id, buffer) = {
                    let mut b = Buffer::empty()?;
                    b.as_mut_context().set_location(Default::default());
                    (b.to_id(), b)
                };

                self.s.buffers.push(buffer);

                let mut inner = mem::replace(&mut self.inner, Default::default());
                inner
                    .as_mut_window()
                    .on_event(&mut self.s, Event::UseBuffer { buffer_id })?;
                self.inner = inner;

                Ok(Some(self))
            }
            Event::OpenFiles { flocs } if flocs.len() > 0 => {
                //
                self.handle_open_files(flocs).map(|x| Some(x))
            }
            Event::Char('q', m) if m.is_empty() => Ok(None),
            _ => Ok(Some(self)),
        }
    }

    fn handle_down(self, _evnt: Event) -> Result<Self> {
        Ok(self)
    }

    fn handle_open_files(mut self, mut flocs: Vec<event::OpenFile>) -> Result<Self> {
        let inner = mem::replace(&mut self.inner, Default::default());
        self.inner = match inner {
            Inner::Usual { window } => {
                let mut fds = vec![];
                loop {
                    let floc = flocs.remove(0);
                    let pw: Result<WindowPrompt> = floc.clone().try_into();
                    match pw {
                        Err(_) => {
                            fds.push(floc);
                            break Inner::Usual { window };
                        }
                        Ok(pw) => {
                            flocs.insert(0, floc);
                            break Inner::OpenFiles {
                                pw,
                                window,
                                flocs,
                                fds,
                            };
                        }
                    }
                }
            }
            val @ Inner::OpenFiles { .. } => val,
            Inner::None => err_at!(Fatal, msg: format!("unreachable"))?,
        };

        Ok(self)
    }
}

struct Terminal {
    stdout: io::Stdout,
    cols: u16,
    rows: u16,
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
                cursor::Hide
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
            cursor::Show
        )
        .unwrap();
        terminal::disable_raw_mode().unwrap();
    }
}

fn init_logger(opts: &Opt) -> Result<()> {
    let home_dir = err_at!(
        Fatal,
        dirs::home_dir().ok_or(format!("can't find home-directory"))
    )?;
    let log_file: path::PathBuf = if opts.log_file.is_empty() {
        [home_dir, path::Path::new(".ted.log").to_path_buf()]
    } else {
        [home_dir, path::Path::new(&opts.log_file).to_path_buf()]
    }
    .iter()
    .collect();

    let level_filter = if opts.trace {
        simplelog::LevelFilter::Trace
    } else if opts.verbose {
        simplelog::LevelFilter::Debug
    } else {
        simplelog::LevelFilter::Info
    };

    let mut config = simplelog::ConfigBuilder::new();
    config
        .set_location_level(simplelog::LevelFilter::Error)
        .set_target_level(simplelog::LevelFilter::Off)
        .set_thread_mode(simplelog::ThreadLogMode::Both)
        .set_thread_level(simplelog::LevelFilter::Error)
        .set_time_to_local(true)
        .set_time_format("%Y-%m-%dT%H-%M-%S%.3f".to_string());

    let fs = err_at!(Fatal, fs::File::create(&log_file))?;
    err_at!(
        Fatal,
        simplelog::WriteLogger::init(level_filter, config.build(), fs)
    )?;

    Ok(())
}
