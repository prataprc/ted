use crossterm::{
    cursor as term_cursor, event as ct_event,
    event::{DisableMouseCapture, EnableMouseCapture, Event as TermEvent},
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
    path,
    sync::mpsc,
    time::{Duration, SystemTime},
};

use crate::{
    code,
    event::Event,
    pubsub::PubSub,
    window::{Coord, Cursor, Notify},
    Error, Result,
};

#[derive(Debug, Clone, StructOpt)]
pub struct Opt {
    #[structopt(long = "app", default_value = "code")]
    pub app: String,

    #[structopt(short = "u", long = "config", default_value = "")]
    pub toml_file: String,

    #[structopt(long = "log", default_value = "")]
    pub log_file: String,

    #[structopt(short = "v", long = "verbose")]
    pub verbose: bool,

    #[structopt(long = "trace")]
    pub trace: bool,

    #[structopt(long = "stats")]
    pub stats: bool,

    pub files: Vec<String>,
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

// Application state
pub struct State {
    pub tm: Terminal,
    pub app: code::App,
    pub subscribers: PubSub,
}

impl State {
    pub fn new(opts: Opt) -> Result<State> {
        use std::str::from_utf8;

        init_logger(&opts)?;

        let config: toml::Value = if opts.toml_file.len() > 0 {
            let toml_file: ffi::OsString = opts.toml_file.clone().into();
            let toml_file = {
                let p = err_at!(IOError, fs::canonicalize(&toml_file))?;
                p.into_os_string()
            };
            let bytes = err_at!(IOError, fs::read(toml_file))?;
            let s = err_at!(FailConvert, from_utf8(&bytes))?;
            err_at!(FailConvert, s.parse())?
        } else {
            toml::Value::Table(Default::default())
        };

        let tm = Terminal::init()?;
        let app_config: toml::Value = match config.get(&opts.app) {
            Some(value) => value.clone(),
            None => toml::Value::Table(Default::default()),
        };
        let subscribers: PubSub = Default::default();
        let app = match opts.app.as_str() {
            "code" => {
                let coord = Coord::new(1, 1, tm.rows, tm.cols);
                let mut app = code::App::new(app_config, coord, opts.clone())?;
                for (topic, tx) in subscribers.to_subscribers().into_iter() {
                    app.subscribe(&topic, tx)
                }
                Ok(app)
            }
            _ => err_at!(Invalid, msg: format!("invalid app {:?}", &opts.app)),
        }?;
        Ok(State {
            tm,
            app,
            subscribers: Default::default(),
        })
    }

    pub fn subscribe(&mut self, topic: &str, tx: mpsc::Sender<Notify>) {
        self.subscribers.subscribe(topic, tx.clone());
        self.app.subscribe(topic, tx);
    }

    pub fn notify(&self, topic: &str, msg: Notify) -> Result<()> {
        match self.subscribers.notify(topic, msg.clone()) {
            Ok(_) => Ok(()),
            Err(Error::NoTopic) => self.app.notify(topic, msg.clone()),
            Err(err) => Err(err),
        }
    }
}

impl State {
    pub fn event_loop(mut self) -> Result<()> {
        let mut stdout = io::stdout();
        let mut stats = Latency::new();

        let res = 'a: loop {
            // new event
            let evnt: Event = {
                let tevnt: TermEvent = err_at!(Fatal, ct_event::read())?;
                trace!("{:?} {}", tevnt, self.app.to_cursor());
                tevnt.clone().into()
            };

            let start = SystemTime::now();

            // hide cursor, handle event and refresh window
            err_at!(Fatal, queue!(stdout, term_cursor::Hide))?;
            for evnt in evnt {
                let evnt = {
                    let evnt = self.app.on_event(evnt)?;
                    self.app.on_refresh()?;
                    evnt
                };
                for evnt in evnt {
                    match evnt {
                        Event::Char('q', m) if m.is_empty() => break 'a Ok(()),
                        _ => (),
                    }
                }
            }
            // show-cursor
            let Cursor { col, row } = self.app.to_cursor();
            err_at!(Fatal, queue!(stdout, term_cursor::MoveTo(col, row)))?;
            err_at!(Fatal, queue!(stdout, term_cursor::Show))?;
            err_at!(Fatal, stdout.flush())?;

            stats.sample(start.elapsed().unwrap());
        };

        stats.pretty_print("");
        res
    }
}

#[derive(Clone, Default, Debug)]
struct Latency {
    samples: usize,
    min: Duration,
    max: Duration,
    total: Duration,
    durations: Vec<usize>,
}

impl Latency {
    fn new() -> Latency {
        let mut stats: Latency = Default::default();
        stats.durations = Vec::with_capacity(256);
        stats.durations.resize(256, 0);
        stats
    }

    fn sample(&mut self, duration: Duration) {
        self.samples += 1;
        self.total += duration;
        if self.min == Duration::from_nanos(0) || self.min > duration {
            self.min = duration
        }
        if self.max == Duration::from_nanos(0) || self.max < duration {
            self.max = duration
        }
        let off: usize = (duration.as_nanos() / 10_000_000) as usize;
        self.durations[off] += 1;
    }

    #[allow(dead_code)]
    fn samples(&self) -> usize {
        self.samples
    }

    #[allow(dead_code)]
    fn min(&self) -> Duration {
        self.min
    }

    #[allow(dead_code)]
    fn max(&self) -> Duration {
        self.max
    }

    fn mean(&self) -> Duration {
        self.total / (self.samples as u32)
    }

    fn percentiles(&self) -> Vec<(u8, usize)> {
        let mut percentiles: Vec<(u8, usize)> = vec![];
        let (mut acc, mut prev_perc) = (0_f64, 90_u8);
        let iter = self
            .durations
            .iter()
            .enumerate()
            .filter(|(_, &item)| item > 0);
        for (duration, samples) in iter {
            acc += *samples as f64;
            let perc = ((acc / (self.samples as f64)) * 100_f64) as u8;
            if perc >= prev_perc {
                percentiles.push((perc, duration));
                prev_perc = perc;
            }
        }
        percentiles
    }

    fn pretty_print(&self, prefix: &str) {
        let mean = self.mean();
        println!(
            "{}duration (min, avg, max): {:?}",
            prefix,
            (self.min, mean, self.max)
        );
        for (duration, n) in self.percentiles().into_iter() {
            if n > 0 {
                println!("{}  {} percentile = {}", prefix, duration, n);
            }
        }
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

    let mut lcnf = simplelog::ConfigBuilder::new();
    lcnf.set_location_level(simplelog::LevelFilter::Error)
        .set_target_level(simplelog::LevelFilter::Off)
        .set_thread_mode(simplelog::ThreadLogMode::Both)
        .set_thread_level(simplelog::LevelFilter::Error)
        .set_time_to_local(true)
        .set_time_format("%Y-%m-%dT%H-%M-%S%.3f".to_string());

    let fs = err_at!(Fatal, fs::File::create(&log_file))?;
    err_at!(
        Fatal,
        simplelog::WriteLogger::init(level_filter, lcnf.build(), fs)
    )?;

    Ok(())
}
