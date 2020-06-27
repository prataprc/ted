mod cmd;
mod cmd_set;
//mod cmd_edit;
//mod cmd_file;
//mod cmd_write;

mod col_nu;
mod config;
mod keymap;
mod keymap_cmd;
mod keymap_edit;
mod keymap_less;
mod keymap_prompt;
mod view;
mod window_edit;
mod window_file;
mod window_less;
mod window_line;
mod window_prompt;

#[allow(unused_imports)]
use log::{debug, error, trace};
use toml;

use std::{ffi, mem, sync::mpsc};

use crate::{
    app::Application,
    buffer::Buffer,
    code::window_prompt::WindowPrompt,
    code::{cmd::Command, config::Config, window_file::WindowFile},
    code::{window_less::WindowLess, window_line::WindowLine},
    colors::{ColorScheme, Highlight},
    event::{self, Event},
    location::Location,
    pubsub::{Notify, PubSub},
    state::{Opt, State},
    term::{Span, Spanline},
    window::{Coord, Cursor, Window},
    Error, Result,
};

pub struct Code {
    config_value: toml::Value,
    config: Config,
    coord: Coord,
    subscribers: PubSub,
    scheme: ColorScheme,
    buffers: Vec<Buffer>,

    inner: Inner,
}

enum Inner {
    Edit {
        wfile: WindowFile,
        tbcline: WindowLine, // TODO: change this to `tabc`.
        stsline: WindowLine,
    },
    Prompt {
        edit: Box<Inner>,
        prompts: Vec<WindowPrompt>,
    },
    Command {
        edit: Box<Inner>,
        tbcline: WindowLine, // TODO: change this to `tabc`.
        cmdline: WindowLine,
    },
    Less {
        edit: Box<Inner>,
        less: WindowLess,
    },
    None,
}

impl Default for Inner {
    fn default() -> Inner {
        Inner::None
    }
}

impl AsRef<Config> for Code {
    fn as_ref(&self) -> &Config {
        &self.config
    }
}

impl AsMut<Config> for Code {
    fn as_mut(&mut self) -> &mut Config {
        &mut self.config
    }
}

impl<'a> From<(Opt, &'a State, Coord)> for Code {
    fn from((opts, state, coord): (Opt, &'a State, Coord)) -> Code {
        use crate::config as mod_config;

        let config: Config = {
            let value = mod_config::to_section(state.config.clone(), "code");
            let cnf: Config = Default::default();
            cnf.mixin(value.try_into().unwrap())
        };

        debug!(
            "starting app `code` coord:{} config...\n{}",
            coord,
            toml::to_string(&config).unwrap(),
        );

        let scheme = state.to_color_scheme(&config.color_scheme),
        let (buffers, prompts) = {
            let files = opts.files.clone();
            open_cmd_files(Self::to_coord_prompt(coord), &config, &scheme, files)
        };
        if buffers.len() == 0 {
            buffers.push(Buffer::empty());
        }

        let edit = {
            let stsline = Code::new_stsline(coord);
            let tbcline = Code::new_tbcline(coord);
            let wfile = {
                let buf = buffers.first().unwrap();
                Code::new_window_file(coord, config.clone(), buf)
            };
            Inner::Edit { wfile, stsline, wfile }
        };

        Code {
            config_value: state.config.clone(),
            config: config.clone(),
            coord,
            subscribers: state.subscribers.clone(),
            scheme,
            buffers: Default::default(),
            inner: if prompts.len() > 0 {
                Inner::Prompt { prompts, edit: Box::new(edit)}
            } else {
                edit
            },
        }
    }
}

impl Code {
    fn new_stsline(mut coord: Coord) -> WindowLine {
        coord.row = coord.hgt;
        coord.hgt = 1;
        WindowLine::new_status(coord)
    }

    fn new_cmdline(mut coord: Coord) -> WindowLine {
        coord.row = coord.hgt;
        coord.hgt = 1;
        WindowLine::new_cmd(coord)
    }

    fn new_tbcline(mut coord: Coord) -> WindowLine {
        coord.row = coord.hgt.saturating_sub(1);
        coord.hgt = 1;
        WindowLine::new_tab(coord)
    }

    fn new_window_file(mut coo: Coord, cnf: Config, buf: &Buffer) -> WindowFile {
        coo.hgt = coo.hgt.saturating_sub(1);
        (coo, buf, config).into()
    }

    #[inline]
    fn to_coord_prompt(mut coord: Coord) -> Coord {
        coord.hgt = coord.hgt.saturating_sub(1)
        coord
    }
}

impl Code {
    pub fn add_buffer(&mut self, buffer: Buffer) {
        self.buffers.insert(0, buffer)
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

    #[inline]
    pub fn post(&mut self, _msg: Notify) -> Result<()> {
        //match msg {
        //    Notify::Status(sl)) -> self.stsline.set(sl),
        //    Notify::TabComplete(sl) -> self.tbcline.set(sl),
        //}
        Ok(())
    }
}

impl Code {
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

    fn as_color_scheme(&self) -> &ColorScheme {
        &self.scheme
    }
}

impl Code {
    fn open_cmd_files(
        coord: Coord,
        config: &Config,
        scheme: &ColorScheme,
        files: Vec<String>,
    ) -> Result<(Vec<Buffer>, Vec<WindowPrompt>)> {
        let locs: Vec<Location> = {
            let files: Vec<ffi::OsString> = files.into_iter().map(Into);
            files.map(|f| Location::new_disk(&f)).collect()
        };
        let mut efiles = vec![];
        let mut buffers = vec![];
        let mut prompts = vec![];
        for loc in locs.into_iter() {
            let items = loc.to_rw_file().map(|f| ("rw", Some(f))).unwrap_or(
                loc.to_r_file()
                    .map(|f| ("r", Some(f)))
                    .unwrap_or(("err", None)),
            );
            let buf = match items {
                ("rw", Some(f)) => {
                    debug!("opening {} in write-mode", loc);
                    Buffer::from_reader(f, loc.clone()).unwrap(),
                },
                ("r", Some(f)) => {
                    debug!("opening {} in read-mode", loc);
                    Buffer::from_reader(f, loc.clone()).unwrap(),
                },
                ("err", None) => {
                    debug!("error opening {}", loc);
                    let lines = vec![
                        format!("error opening {:?}", loc.to_long_string()),
                        format!("-press any key to continue-")
                    ];
                    prompts.push((coord, lines, scheme).into())
                },
                _ => unreachable!(),
            };
            buffers.push(buf);
            match items {
                ("r", _) => buf.set_read_only(true),
                ("rw", _) => buf.set_read_only(config.read_only),
                _ => (),
            }
        }

        Ok((buffers, prompts))
    }
}

impl Application for Code {
    fn subscribe(&mut self, topic: &str, tx: mpsc::Sender<Notify>) {
        self.subscribers.subscribe(topic, tx);
    }

    fn notify(&self, topic: &str, msg: Notify) -> Result<()> {
        self.subscribers.notify(topic, msg)
    }

    fn to_cursor(&self) -> Cursor {
        match &self.inner {
            Inner::Edit { .. } => self.wfile.as_ref().unwrap().to_cursor(),
            Inner::Prompt { prompts, .. } => prompts[0].to_cursor(),
            Inner::Command { cmdline } => cmdline.to_cursor(),
            Inner::Less { less } => less.to_cursor(),
            Inner::None => Default::default(),
        }
    }

    fn on_event(&mut self, evnt: Event) -> Result<Event> {
        let inner = mem::replace(&mut self.inner, Default::default());
        let (inner, mut evnt) = inner.on_event(self, evnt)?;
        self.inner = inner;

        loop {
            match evnt {
                Event::Code(event::Code::Cmd(name, args)) => {
                    let mut cmd: Command = (name, args).into();
                    evnt = cmd.on_command(self)?;
                }
                Event::Code(event::Code::Less(ref content)) => {
                    let less = WindowLess::new(self.coord, content);
                    self.inner = Inner::Less { less };
                }
                Event::Code(event::Code::Edit) => {
                    let stsline = Code::new_stsline(self.coord);
                    self.inner = Inner::Edit { stsline };
                }
                evnt => break Ok(evnt),
            }
        }
    }

    fn on_refresh(&mut self) -> Result<()> {
        self.wfile = match self.wfile.take() {
            Some(mut wfile) => {
                wfile.on_refresh(self)?;
                Some(wfile)
            }
            None => unreachable!(),
        };

        let inner = mem::replace(&mut self.inner, Default::default());
        self.inner = inner.on_refresh(self)?;

        //let mut wline = mem::replace(&mut self.tbcline, Default::default());
        //wline.on_refresh(self)?;
        //self.tbcline = wline;

        Ok(())
    }
}

impl Inner {
    fn on_event(self, app: &mut Code, evnt: Event) -> Result<(Inner, Event)> {
        match self {
            Inner::Edit { stsline } => match evnt {
                Event::Char(':', m) if m.is_empty() => {
                    let mut cmdline = Code::new_cmdline(app.coord);
                    cmdline.on_event(app, Event::Char(':', m))?;
                    Ok((Inner::Command { cmdline }, Event::Noop))
                }
                evnt => match app.wfile.take() {
                    Some(mut wfile) => {
                        let evnt = wfile.on_event(app, evnt)?;
                        app.wfile = Some(wfile);
                        Ok((Inner::Edit { stsline }, evnt))
                    }
                    None => unreachable!(),
                },
            },
            Inner::Prompt {
                mut prompts,
                stsline,
            } => {
                let evnt = prompts[0].on_event(app, evnt)?;
                if prompts[0].prompt_match().is_some() {
                    prompts.remove(0);
                }
                Ok(match prompts.len() {
                    0 => (Inner::Edit { stsline }, evnt),
                    _ => (Inner::Prompt { prompts, stsline }, evnt),
                })
            }
            Inner::Command { mut cmdline } => {
                let evnt = cmdline.on_event(app, evnt)?;
                let (inner, evnt) = match evnt {
                    Event::Esc => {
                        let stsline = Code::new_stsline(app.coord);
                        (Inner::Edit { stsline }, Event::Noop)
                    }
                    evnt @ Event::Code(event::Code::Cmd(_, _)) => {
                        let stsline = Code::new_stsline(app.coord);
                        (Inner::Edit { stsline }, evnt)
                    }
                    evnt => (Inner::Command { cmdline }, evnt),
                };
                Ok((inner, evnt))
            }
            Inner::Less { mut less } => {
                let evnt = less.on_event(app, evnt)?;
                Ok((Inner::Less { less }, evnt))
            }
            Inner::None => Ok((Inner::None, evnt)),
        }
    }

    fn on_refresh(mut self, app: &mut Code) -> Result<Inner> {
        match &mut self {
            Inner::Edit { stsline } => stsline.on_refresh(app)?,
            Inner::Prompt { prompts, stsline } => {
                prompts[0].on_refresh(app)?;
                stsline.on_refresh(app)?;
            }
            Inner::Command { cmdline } => cmdline.on_refresh(app)?,
            Inner::Less { less } => less.on_refresh(app)?,
            Inner::None => (),
        }
        Ok(self)
    }
}
