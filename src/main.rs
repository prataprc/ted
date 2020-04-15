use crossterm::{
    self, cursor,
    event::{self as ct_event, Event as TermEvent},
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use dirs;
use log::trace;
use simplelog;
use structopt::StructOpt;

use std::{
    fs,
    io::{self, Write},
    path,
};

use kavi::{err_at, Config, Error, Viewport};

#[derive(Debug, StructOpt)]
pub struct Opt {
    #[structopt(long = "log", default_value = "")]
    log_file: String,

    #[structopt(short = "v", long = "verbose")]
    verbose: bool,

    #[structopt(long = "trace")]
    trace: bool,

    file: String,
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

    match Application::run(&opts) {
        Ok(()) => (),
        Err(err) => {
            println!("{}", err);
            std::process::exit(1);
        }
    }
}

struct Application {
    tm: Terminal,
    vp: Viewport,
}

impl Application {
    pub fn run(opts: &Opt) -> Result<(), String> {
        let config: Config = Default::default();
        let mut app = {
            let tm = Terminal::init()?;
            let vp = Viewport::new(0, 0, tm.rows, tm.cols, config).map_err(|e| e.to_string())?;
            Application { tm, vp }
        };
        app.event_loop()
    }

    fn event_loop(mut self) -> Result<(), String> {
        loop {
            let evnt: TermEvent = err_at!(Fatal, ct_event::read()).map_err(|e| e.to_string())?;
            trace!("Event-{:?}", evnt);
        }
    }
}

impl Application {
    #[inline]
    pub fn to_viewport(&self) -> Viewport {
        self.vp.clone()
    }
}

struct Terminal {
    stdout: io::Stdout,
    cols: u16,
    rows: u16,
}

impl Terminal {
    fn init() -> Result<Terminal, String> {
        let mut stdout = io::stdout();
        err_at!(
            //
            Fatal,
            terminal::enable_raw_mode()
        )
        .map_err(|e| e.to_string())?;
        err_at!(
            Fatal,
            execute!(
                stdout,
                EnterAlternateScreen,
                EnableMouseCapture,
                cursor::Hide
            )
        )
        .map_err(|e| e.to_string())?;

        let (cols, rows) = err_at!(
            //
            Fatal,
            terminal::size()
        )
        .map_err(|e| e.to_string())?;
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

fn init_logger(opts: &Opt) -> Result<(), String> {
    if opts.log_file.is_empty() {
        Ok(())
    } else {
        let log_file: path::PathBuf = [
            dirs::home_dir().ok_or(format!("can't find home-directory"))?,
            path::Path::new(&opts.log_file).to_path_buf(),
        ]
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

        let fs = fs::File::create(&log_file).map_err(|e| e.to_string())?;
        simplelog::WriteLogger::init(level_filter, config.build(), fs)
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}
