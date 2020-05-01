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
    ffi, fs,
    io::{self, Write},
    mem, path,
    time::SystemTime,
};

use ted::{
    err_at,
    location::Location,
    on_win_event, on_win_refresh, stats,
    window::{Coord, Cursor, State},
    window_file::WindowFile,
    Config, Error, Event, Result,
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
}

impl Application {
    fn run(opts: Opt) -> Result<()> {
        let config: Config = Default::default();
        let app = {
            let tm = Terminal::init()?;
            let s = {
                let coord = Coord::new(1, 1, tm.rows, tm.cols);
                State::new(config, WindowFile::new(coord))
            };
            Application { tm, s }
        };

        let evnt = if opts.files.len() == 0 {
            Event::NewBuffer
        } else {
            let mut flocs = vec![];
            for f in opts.files.clone().into_iter() {
                let f: ffi::OsString = f.into();
                flocs.push(Location::new_disk(&f));
            }
            Event::OpenFiles { flocs }
        };

        app.event_loop(evnt)
    }

    fn event_loop(mut self, mut evnt: Event) -> Result<()> {
        let mut stats = stats::Latency::new();

        let mut s = mem::replace(&mut self.s, Default::default());
        let mut start = SystemTime::now();

        // TODO: later statistics can be moved to a different release stream
        // and or controlled by command line option.
        let res = loop {
            // hide cursor, handle event and refresh window
            match evnt {
                Event::Noop => Event::Noop,
                evnt => {
                    err_at!(Fatal, queue!(self.tm.stdout, cursor::Hide))?;
                    s = on_win_event!(s, evnt);
                    s = on_win_refresh!(s);
                    mem::replace(&mut s.event, Default::default())
                }
            };

            // show-cursor
            let Cursor { col, row } = s.to_window_cursor();
            err_at!(Fatal, queue!(self.tm.stdout, cursor::MoveTo(col, row)))?;
            err_at!(Fatal, queue!(self.tm.stdout, cursor::Show))?;
            err_at!(Fatal, self.tm.stdout.flush())?;

            stats.sample(start.elapsed().unwrap());

            // new event
            evnt = {
                let tevnt: TermEvent = err_at!(Fatal, ct_event::read())?;
                trace!("Event-{:?} Cursor:({},{})", tevnt, col, row);
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

    //fn do_open_files(mut self, mut flocs: Vec<event::OpenFile>) -> Result<Self> {
    //    let inner = mem::replace(&mut self.inner, Default::default());
    //    self.inner = match inner {
    //        Inner::Usual { window } => {
    //            let mut fds = vec![];
    //            loop {
    //                let floc = flocs.remove(0);
    //                let pw: Result<WindowPrompt> = floc.clone().try_into();
    //                match pw {
    //                    Err(_) => {
    //                        fds.push(floc);
    //                        break Inner::Usual { window };
    //                    }
    //                    Ok(pw) => {
    //                        flocs.insert(0, floc);
    //                        break Inner::OpenFiles {
    //                            pw,
    //                            window,
    //                            flocs,
    //                            fds,
    //                        };
    //                    }
    //                }
    //            }
    //        }
    //        val @ Inner::OpenFiles { .. } => val,
    //        Inner::None => err_at!(Fatal, msg: format!("unreachable"))?,
    //    };

    //    Ok(self)
    //}
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
