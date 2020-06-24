//! Ted state management. [State] wraps all applications, manages the terminal
//! handles the event-loop. Some of the functionalities belong to the main.rs
//! but handled here, acts as the bridge between main.rs and the ted-library.

use dirs;
use log::trace;
use simplelog;
use structopt::StructOpt;

use std::{
    fs,
    io::Write,
    path,
    sync::mpsc,
    time::{Duration, SystemTime},
};

use crate::{
    app::Application,
    code,
    event::Event,
    pubsub::{Notify, PubSub},
    term::Terminal,
    window::Coord,
    Error, Result,
};

#[derive(Debug, Clone, StructOpt)]
/// Command line options.
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

    #[structopt(short = "R", long = "read-only")]
    pub read_only: bool,

    #[structopt(long = "version")]
    pub version: bool,

    pub files: Vec<String>,
}

/// Application state
pub struct State {
    pub tm: Terminal,
    pub config: toml::Value,
    pub app: code::Code,
    pub subscribers: PubSub,
}

impl AsMut<Terminal> for State {
    fn as_mut(&mut self) -> &mut Terminal {
        &mut self.tm
    }
}

impl State {
    /// Create a new ted-state with command line opts.
    pub fn new(opts: Opt) -> Result<State> {
        use crate::config;

        let tm = Terminal::init()?;

        // then the logger
        init_logger(&opts)?;
        // then the configuration
        let cnf = config::read_config(&opts.toml_file, None)?;
        // if there is any ted-level pub-sub to be done, do it here.
        let subscribers: PubSub = Default::default();

        // now we are ready to create the app.
        let app = match opts.app.as_str() {
            "code" => {
                let mut app = {
                    let aconfig = config::to_app_config(&cnf, "code");
                    let coord = Coord::new(1, 1, tm.rows, tm.cols);
                    code::Code::new(aconfig, coord, opts.clone())?
                };
                for (topic, chans) in subscribers.to_subscribers().into_iter() {
                    for chan in chans.into_iter() {
                        app.subscribe(&topic, chan)
                    }
                }
                Ok(app)
            }
            _ => err_at!(Invalid, msg: format!("invalid app {:?}", &opts.app)),
        }?;

        // a new ted-state is created. make sure to call event_loop() to
        // launch the application.
        Ok(State {
            tm,
            config: cnf,
            app,
            subscribers: Default::default(),
        })
    }
}

impl State {
    /// Subscribe a channel for a topic. Refer [pubsub::PubSub] for more detail.
    pub fn subscribe(&mut self, topic: &str, tx: mpsc::Sender<Notify>) {
        self.subscribers.subscribe(topic, tx.clone());
        self.app.subscribe(topic, tx);
    }

    /// Notify all subscribers for `topic` with `msg`. Refer [pubsub::PubSub]
    /// for more detail.
    pub fn notify(&self, topic: &str, msg: Notify) -> Result<()> {
        match self.subscribers.notify(topic, msg.clone()) {
            Ok(_) => Ok(()),
            Err(Error::NoTopic) => self.app.notify(topic, msg.clone()),
            Err(err) => Err(err),
        }
    }

    pub fn as_config(&self) -> &toml::Value {
        &self.config
    }
}

impl State {
    /// main event-loop.
    pub fn event_loop(mut self) -> Result<String> {
        let mut stats = Latency::new();

        // initial screen refresh
        self.app.on_refresh()?;
        err_at!(Fatal, term_ce!(self, self.app.to_cursor()))?;

        'a: loop {
            // new event
            let evnt: Event = err_at!(Fatal, crossterm::event::read())?.into();
            trace!("{} {}", evnt, self.app.to_cursor());

            let start = SystemTime::now();

            for mut evnt in evnt {
                // preprocessing
                match &evnt {
                    Event::Char('q', _) if evnt.is_control() => break 'a,
                    _ => (),
                };
                // dispatch
                evnt = {
                    let evnt = self.app.on_event(evnt)?;
                    self.app.on_refresh()?;
                    evnt
                };
                // post processing
                for evnt in evnt {
                    match evnt {
                        Event::Char('q', _) => break 'a,
                        _ => (),
                    }
                }
            }
            err_at!(Fatal, term_ce!(self, self.app.to_cursor()))?;

            stats.sample(start.elapsed().unwrap());
        }

        Ok(stats.pretty_print())
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

    fn pretty_print(&self) -> String {
        let mean = self.mean();
        let mut outs = format!(
            //
            "duration (min, avg, max): {:?}",
            (self.min, mean, self.max)
        );
        for (dur, n) in self.percentiles().into_iter() {
            if n > 0 {
                outs.push_str(&format!("  {} percentile = {}", dur, n));
            }
        }

        outs
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

    trace!(
        "logging initialized file:{:?} trace:{}",
        log_file,
        opts.trace
    );
    Ok(())
}
