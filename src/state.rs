//! Ted state management. [State] wraps all applications, manages the terminal
//! handles the event-loop. Some of the functionalities belong to the main.rs
//! but handled here, acts as the bridge between main.rs and the ted-library.

use dirs;
#[allow(unused_imports)]
use log::{debug, error, trace};
use simplelog;
use structopt::StructOpt;

use std::{
    convert::{TryFrom, TryInto},
    fs, path,
    sync::mpsc,
};

use crate::{
    app::App,
    code,
    colors::{self, ColorScheme},
    config,
    event::Event,
    pubsub::{Notify, PubSub},
    term::Terminal,
    util,
    window::Cursor,
    Error, Result,
};

#[derive(Debug, Clone, StructOpt)]
/// Command line options.
pub struct Opt {
    #[structopt(long = "app", default_value = "code")]
    pub app: String,

    #[structopt(short = "u", long = "config")]
    pub toml_file: Option<String>,

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
    /// Command line options, refer to [Opt][Opt] type.
    pub opts: Opt,
    /// Toml instance of configuration parameters. Following is a list
    /// of possible configuration sources:
    ///
    /// * Default configuration defined by Ted
    /// * $HOME/.ted.toml
    /// * `--config` command line option
    pub config_value: toml::Value,
    /// Terminal instance.
    pub tm: Terminal,
    /// List of available color schemes.
    pub schemes: Vec<ColorScheme>,
    /// Global subscribe-publish instance.
    pub subscribers: PubSub,

    // state machine for tabed-windows.
    inner: Inner,
}

enum Inner {
    Mono { tab: Tab },
    Multi { tabs: Vec<Tab>, active: usize },
}

impl Inner {
    fn subscribe(&mut self, topic: &str, tx: mpsc::Sender<Notify>) {
        match self {
            Inner::Mono { tab } => tab.subscribe(topic, tx),
            Inner::Multi { tabs, .. } => {
                tabs.iter_mut()
                    .for_each(|tab| tab.subscribe(topic, tx.clone()));
            }
        }
    }

    fn notify(&self, topic: &str, msg: Notify) -> Result<()> {
        match self {
            Inner::Mono { tab } => tab.notify(topic, msg)?,
            Inner::Multi { tabs, .. } => {
                for tab in tabs.iter() {
                    tab.notify(topic, msg.clone())?
                }
            }
        };
        Ok(())
    }

    fn on_event(&mut self, evnt: Event) -> Result<Event> {
        match self {
            Inner::Mono { tab } => tab.on_event(evnt),
            Inner::Multi { tabs, active } => tabs[*active].on_event(evnt),
        }
    }

    fn on_refresh(&mut self) -> Result<()> {
        match self {
            Inner::Mono { tab } => tab.on_refresh(),
            Inner::Multi { tabs, active } => tabs[*active].on_refresh(),
        }
    }

    fn to_cursor(&self) -> Cursor {
        match self {
            Inner::Mono { tab } => tab.to_cursor(),
            Inner::Multi { tabs, active } => tabs[*active].to_cursor(),
        }
    }
}

impl TryFrom<Opt> for State {
    type Error = Error;

    fn try_from(opts: Opt) -> Result<State> {
        // first the terminal
        let tm = Terminal::init()?;
        // then the logger
        init_logger(&opts)?;
        // then the configuration
        let cnf = config::read_config(opts.toml_file.clone(), None)?;
        let schemes = Self::load_color_schemes()?;

        Ok(State {
            opts,
            config_value: cnf,
            tm,
            schemes,
            subscribers: Default::default(),
            inner: Inner::Mono {
                tab: Default::default(),
            },
        })
    }
}

impl AsMut<Terminal> for State {
    fn as_mut(&mut self) -> &mut Terminal {
        &mut self.tm
    }
}

impl State {
    /// Create a new ted-state with command line opts.
    pub fn new(opts: Opt) -> Result<State> {
        let mut state: State = opts.clone().try_into()?;

        // TODO: if there is any ted-level pub-sub to be done, do it here.

        // now we are ready to create the app.
        let app = match opts.app.as_str() {
            "code" => {
                let app: code::Code = {
                    let coord = state.tm.to_screen_coord();
                    (&state, coord).into()
                };
                Ok(App::Code(app))
            }
            _ => err_at!(Invalid, msg: format!("invalid app {:?}", &opts.app)),
        }?;

        state.inner = Inner::Mono { tab: app.into() };

        // a new ted-state is created. make sure to call event_loop() to
        // launch the application.
        Ok(state)
    }

    fn load_color_schemes() -> Result<Vec<ColorScheme>> {
        let mut schemes = colors::pkg_color_schemes();
        schemes.extend(config::read_color_schemes()?);
        Ok(schemes)
    }
}

impl State {
    /// Subscribe a channel for a topic. Refer [pubsub::PubSub] for more detail.
    pub fn subscribe(&mut self, topic: &str, tx: mpsc::Sender<Notify>) {
        self.subscribers.subscribe(topic, tx.clone());
        self.inner.subscribe(topic, tx)
    }

    /// Notify all subscribers for `topic` with `msg`. Refer [pubsub::PubSub]
    /// for more detail.
    pub fn notify(&self, topic: &str, msg: Notify) -> Result<()> {
        self.subscribers.notify(topic, msg.clone())?;
        self.inner.notify(topic, msg)
    }

    pub fn as_config_value(&self) -> &toml::Value {
        &self.config_value
    }

    pub fn to_color_scheme(&self, name: &str) -> ColorScheme {
        for scheme in self.schemes.iter() {
            if scheme.name == name {
                return scheme.clone();
            }
        }
        self.to_color_scheme("default")
    }
}

impl State {
    /// main event-loop.
    pub fn event_loop(mut self) -> Result<String> {
        use crossterm::event::read;

        let mut stats = util::Latency::new();
        let mut r_stats = util::Latency::new();

        // initial screen refresh
        self.inner.on_refresh()?;
        err_at!(Fatal, termex!(self.inner.to_cursor()))?;

        loop {
            // new event
            let evnt: Event = util::time_it(&mut r_stats, || {
                let evnt: Event = err_at!(Fatal, read())?.into();
                Ok(evnt)
            })?;

            let is_break = util::time_it(&mut stats, || {
                hidecr!()?;
                for mut evnt in evnt {
                    let ctrl = evnt.is_control();
                    // preprocessing
                    match &evnt {
                        Event::Char('q', _) if ctrl => return Ok(true),
                        _ => (),
                    };
                    // dispatch
                    evnt = self.inner.on_event(evnt)?;
                    self.inner.on_refresh()?;
                    // post processing
                    for evnt in evnt {
                        match evnt {
                            Event::Char('q', _) => return Ok(true),
                            _ => (),
                        }
                    }
                }
                return Ok(false);
            })?;

            err_at!(Fatal, termex!(self.inner.to_cursor()))?;

            if is_break {
                break;
            }
        }

        let mut s = format!("read: {}\n", r_stats.pretty_print());
        s.push_str(&format!("  {}", stats.pretty_print()));
        Ok(s)
    }
}

#[derive(Default)]
struct Tab {
    app: App,
}

impl From<App> for Tab {
    fn from(app: App) -> Tab {
        Tab { app }
    }
}

impl Tab {
    fn subscribe(&mut self, topic: &str, tx: mpsc::Sender<Notify>) {
        self.app.subscribe(topic, tx)
    }

    fn notify(&self, topic: &str, msg: Notify) -> Result<()> {
        self.app.notify(topic, msg)
    }

    fn on_event(&mut self, evnt: Event) -> Result<Event> {
        self.app.on_event(evnt)
    }

    fn on_refresh(&mut self) -> Result<()> {
        self.app.on_refresh()
    }

    fn to_cursor(&self) -> Cursor {
        self.app.to_cursor()
    }

    fn to_title(&self, wth: u16) -> String {
        self.app.to_tab_title(wth)
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

    debug!(
        "logging initialized file:{:?} trace:{}",
        log_file, opts.trace
    );
    Ok(())
}
