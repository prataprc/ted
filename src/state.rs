//! Ted state management.
//!
//! [State] wraps all applications, multi-tab-managment, manage the
//! terminal, and handle the event-loop. Some of the functionalities
//! belong to the main.rs but handled here, acts as the bridge between
//! main.rs and the ted-library.
//!
//! __Multi-tab__:
//!
//! By default ted is opened with single-tab window. But it is possible
//! to spawn multiple tab-windows one for each application. Although
//! [State] dedicates each tab-window for single application instance,
//! an application type can internally share single instance across
//! multiple tab-windows.

use dirs;
#[allow(unused_imports)]
use log::{debug, error, trace};
use simplelog;
use structopt::StructOpt;

use std::{
    cmp,
    convert::{TryFrom, TryInto},
    ffi, fs,
    iter::FromIterator,
    mem, path,
    str::FromStr,
    sync::mpsc,
    time,
};

use crate::{
    app::App,
    code,
    colors::{self, ColorScheme},
    config,
    event::Event,
    pubsub::{Notify, PubSub},
    term::{self, Terminal},
    util,
    window::{Coord, Cursor},
    Error, Result,
};

/// Command line options.
#[derive(Debug, Clone, StructOpt)]
pub struct Opt {
    #[structopt(long = "app", default_value = "code")]
    pub app: String,

    #[structopt(short = "u", long = "config")]
    pub toml_file: Option<String>,

    #[structopt(long = "log")]
    pub log_file: Option<String>,

    #[structopt(long = "nu")]
    pub nu: Option<usize>,

    #[structopt(long = "pattern")]
    pub pattern: Option<String>,

    #[structopt(short = "v", long = "verbose")]
    pub verbose: bool,

    #[structopt(long = "trace")]
    pub trace: bool,

    #[structopt(long = "stats")]
    pub stats: bool,

    #[structopt(short = "R", long = "read-only")]
    pub read_only: bool,

    #[structopt(short = "p")]
    pub tab_page: bool,

    #[structopt(long = "version")]
    pub version: bool,

    pub files: Vec<String>,
}

impl From<Opt> for Event {
    fn from(opts: Opt) -> Event {
        use crate::event::{Mto, DP};

        let mut evnt = match opts.nu {
            Some(nu) => Event::Mt(Mto::Row(nu, DP::None)),
            None => Event::Noop,
        };

        evnt.push(match opts.pattern.clone() {
            Some(patt) => Event::Mt(Mto::Pattern(1, Some(patt), DP::Right)),
            None => Event::Noop,
        });

        evnt
    }
}

/// Ted state.
pub struct State {
    /// Command line options, refer to [Opt][Opt] type.
    pub opts: Opt,
    /// State level configuration paramters.
    pub config: Config,
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
    Multi { coord: Coord, tabs: Vec<Tab> },
    None,
}

impl Default for Inner {
    fn default() -> Self {
        Inner::None
    }
}

impl Inner {
    fn subscribe(&mut self, topic: &str, tx: mpsc::Sender<Notify>) {
        match self {
            Inner::Mono { tab } => tab.subscribe(topic, tx),
            Inner::Multi { tabs, .. } => {
                tabs.iter_mut()
                    .for_each(|tab| tab.subscribe(topic, tx.clone()));
            }
            Inner::None => unreachable!(),
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
            Inner::None => unreachable!(),
        };
        Ok(())
    }

    fn on_event(&mut self, evnt: Event) -> Result<Event> {
        match self {
            Inner::Mono { tab } => tab.on_event(evnt),
            Inner::Multi { tabs, .. } => {
                for tab in tabs.iter_mut() {
                    if tab.active {
                        return tab.on_event(evnt);
                    }
                }
                let prefix = "".to_string();
                let msg = "no active tab".to_string();
                err_at!(Err(Error::Fatal(prefix, msg)))
            }
            Inner::None => unreachable!(),
        }
    }

    fn on_refresh(&mut self, state: &State) -> Result<()> {
        let scheme = state.to_color_scheme(None);
        match self {
            Inner::Mono { tab } => tab.on_refresh(),
            Inner::Multi { coord, tabs } => {
                let wth = cmp::min((coord.wth as usize) / tabs.len(), 16);
                let spans: Vec<term::Span> = {
                    let iter = tabs.iter();
                    iter.map(|tab| tab.to_tab_title(wth, &scheme)).collect()
                };
                let mut line = term::Spanline::from_iter(spans.into_iter());
                line.set_cursor(coord.to_origin_cursor().into());
                err_at!(Fatal, termqu!(line))?;
                for tab in tabs.iter_mut() {
                    if tab.active {
                        return tab.on_refresh();
                    }
                }
                Ok(())
            }
            Inner::None => unreachable!(),
        }
    }

    fn to_cursor(&self) -> Option<Cursor> {
        match self {
            Inner::Mono { tab } => tab.to_cursor(),
            Inner::Multi { tabs, .. } => {
                for tab in tabs.iter() {
                    if tab.active {
                        return tab.to_cursor();
                    }
                }
                None
            }
            Inner::None => None,
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
        let config_value = config::read_config(opts.toml_file.clone(), None)?;
        let config = {
            let config = err_at!(Invalid, config_value.clone().try_into())?;
            Config::default().mixin(config)
        };
        // then the schemes.
        let schemes = Self::load_color_schemes()?;

        Ok(State {
            opts,
            config,
            config_value,
            tm,
            schemes,
            subscribers: PubSub::default(),
            inner: Inner::default(),
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

        state.inner = {
            let mut tab: Tab = app.into();
            tab.active = true;
            Inner::Mono { tab }
        };

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
    /// Subscribe a channel for a topic. Refer [PubSub] for more detail.
    pub fn subscribe(&mut self, topic: &str, tx: mpsc::Sender<Notify>) {
        self.subscribers.subscribe(topic, tx.clone());
        self.inner.subscribe(topic, tx)
    }

    /// Notify all subscribers for `topic` with `msg`. Refer [PubSub]
    /// for more detail.
    pub fn notify(&self, topic: &str, msg: Notify) -> Result<()> {
        self.subscribers.notify(topic, msg.clone())?;
        self.inner.notify(topic, msg)
    }

    pub fn as_config_value(&self) -> &toml::Value {
        &self.config_value
    }

    pub fn to_color_scheme(&self, name: Option<String>) -> ColorScheme {
        // return the requested scheme.
        match name {
            Some(name) => {
                for scheme in self.schemes.iter() {
                    if scheme.name == name {
                        return scheme.clone();
                    }
                }
            }
            None => (),
        };

        // else fall back to configured default.
        self.to_color_scheme(Some(self.config.scheme.clone()))
    }
}

impl State {
    /// main event-loop.
    pub fn event_loop(mut self) -> Result<String> {
        use crossterm::event::read;

        let mut stats = util::Latency::new("EVENT");
        let mut r_stats = util::Latency::new("READT");

        let mut inner = mem::replace(&mut self.inner, Inner::default());

        // initial screen refresh
        inner.on_refresh(&self)?;
        if let Some(cursor) = inner.to_cursor() {
            err_at!(Fatal, termex!(cursor))?;
        }

        let mut evnts: Event = self.opts.clone().into();
        loop {
            if evnts.clone().any(|evnt| Self::is_quit(&evnt)) {
                break;
            }

            // handle event(s)
            {
                let start = time::Instant::now();

                hidecr!()?;

                evnts = self.bubble_down(&mut inner, evnts)?;
                evnts = self.bubble_up(&mut inner, evnts)?;

                inner.on_refresh(&self)?;
                if let Some(cursor) = inner.to_cursor() {
                    err_at!(Fatal, termex!(cursor))?;
                }

                stats.sample(start.elapsed());
            }

            evnts.drain();

            // new event
            let start = time::Instant::now();
            evnts = err_at!(Fatal, read())?.into();
            r_stats.sample(start.elapsed());
        }

        let mut s = format!("{}\n", r_stats.pretty_print());
        s.push_str(&format!("{}", stats.pretty_print()));
        Ok(s)
    }

    fn is_quit(evnt: &Event) -> bool {
        match evnt {
            Event::Char('q', _) => true,
            _ => false,
        }
    }

    fn bubble_down(&mut self, inner: &mut Inner, evnts: Event) -> Result<Event> {
        let mut res_evnts: Event = Event::default();
        for evnt in evnts {
            res_evnts.push(inner.on_event(evnt)?);
        }
        Ok(res_evnts)
    }

    fn bubble_up(&mut self, _inner: &mut Inner, evnts: Event) -> Result<Event> {
        Ok(evnts)
    }
}

#[derive(Default)]
struct Tab {
    active: bool,
    app: App,
}

impl From<App> for Tab {
    fn from(app: App) -> Tab {
        Tab { app, active: false }
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

    fn to_cursor(&self) -> Option<Cursor> {
        self.app.to_cursor()
    }

    fn to_tab_title(&self, wth: usize, scheme: &ColorScheme) -> term::Span {
        let mut tt = self.app.to_tab_title(wth);
        tt.active = self.active;
        tt.into_span(scheme)
    }
}

/// Display text and parameters for each tab-window.
pub struct TabTitle {
    /// Tab title name.
    pub text: String,
    /// Whether tab is modified.
    pub modified: bool,
    /// Whether tab is the active window.
    pub active: bool,
}

impl TabTitle {
    fn into_span(self, scheme: &ColorScheme) -> term::Span {
        use crate::colors::Highlight;

        let span: term::Span = self.text.clone().into();
        let style = {
            let canvas = scheme.to_style(Highlight::Canvas);
            let s_modif = scheme.to_style(Highlight::TabModified);
            let mut style = if self.active {
                canvas
            } else {
                let mut style = scheme.to_style(Highlight::Tab);
                let attrs = term::Style::to_attrs("underline").ok();
                for attr in attrs.unwrap_or(vec![]).into_iter() {
                    style.add_attr(attr);
                }
                style
            };
            if self.modified {
                style.set_fg(s_modif.fg);
            }
            style
        };
        span.using(style)
    }
}

fn init_logger(opts: &Opt) -> Result<()> {
    let home_dir = err_at!(
        Fatal,
        dirs::home_dir().ok_or(format!("can't find home-directory"))
    )?;
    let log_file: path::PathBuf = {
        let log_file = match opts.log_file.as_ref() {
            Some(log_file) => path::Path::new(log_file).to_path_buf(),
            None => path::Path::new(".ted.log").to_path_buf(),
        };
        [home_dir, log_file].iter().collect()
    };

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

config![
    (scheme, String, "default".to_string()),
    (max_tab_pages, u8, 16)
];
