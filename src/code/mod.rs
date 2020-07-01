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
    code::{config::Config, window_file::WindowFile},
    code::{window_less::WindowLess, window_line::WindowLine},
    colors::ColorScheme,
    event::{self, Event},
    location::Location,
    pubsub::{Notify, PubSub},
    state::{Opt, State},
    window::{Coord, Cursor, Window},
    Result,
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
    Edit(Edit),
    Prompt(Prompt),
    Command(Command),
    Less(Less),
    None,
}

impl Inner {
    fn into_edit(self) -> Edit {
        match self {
            Inner::Edit(edit) => edit,
            Inner::Prompt(val) => val.edit,
            Inner::Command(val) => val.edit,
            Inner::Less(val) => val.edit,
            Inner::None => unreachable!(),
        }
    }

    fn new_command(coord: Coord, edit: Edit) -> Command {
        let tbcline = Code::new_tbcline(coord);
        let cmdline = Code::new_cmdline(coord);
        Command {
            edit,
            tbcline,
            cmdline,
        }
    }
}

struct Edit {
    wfile: WindowFile,
    tbcline: WindowLine, // TODO: change this to `tabc`.
    stsline: WindowLine,
}

struct Prompt {
    edit: Edit,
    prompts: Vec<WindowPrompt>,
}

struct Command {
    edit: Edit,
    tbcline: WindowLine, // TODO: change this to `tabc`.
    cmdline: WindowLine,
}

struct Less {
    edit: Edit,
    less: WindowLess,
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

        let scheme = state.to_color_scheme(&config.color_scheme);
        let (mut buffers, prompts) = {
            let files = {
                let iter = opts.files.iter().map(
                    // TODO: encoding from cmd-line
                    |f| (f.clone(), format!("utf-8")),
                );
                iter.collect()
            };
            let coord = Self::to_coord_prompt(coord);
            Self::open_cmd_files(coord, &config, &scheme, files)
        };
        if buffers.len() == 0 {
            buffers.push(Buffer::empty());
        }

        let edit = {
            let stsline = Code::new_stsline(coord);
            let tbcline = Code::new_tbcline(coord);
            let wfile = {
                let buf = buffers.first().unwrap();
                Code::new_window_file(coord, config.clone(), buf, &scheme)
            };
            Edit {
                wfile,
                stsline,
                tbcline,
            }
        };

        Code {
            config_value: state.config.clone(),
            config: config.clone(),
            coord,
            subscribers: state.subscribers.clone(),
            scheme,
            buffers,
            inner: if prompts.len() > 0 {
                Inner::Prompt(Prompt { edit, prompts })
            } else {
                Inner::Edit(edit)
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

    fn new_window_file(
        mut coord: Coord,
        config: Config,
        buf: &Buffer,
        scheme: &ColorScheme,
    ) -> WindowFile {
        coord.hgt = coord.hgt.saturating_sub(1);
        (coord, config, buf, scheme).into()
    }

    #[inline]
    fn to_coord_prompt(mut coord: Coord) -> Coord {
        coord.hgt = coord.hgt.saturating_sub(1);
        coord
    }
}

impl Code {
    pub fn add_buffer(&mut self, buffer: Buffer) {
        self.buffers.insert(0, buffer)
    }

    pub fn take_buffer(&mut self, id: &str) -> Option<Buffer> {
        let mut iter = self
            .buffers
            .iter()
            .enumerate()
            .filter_map(|(i, buf)| if buf.to_id() == id { Some(i) } else { None })
            .take(1);
        match iter.next() {
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
    pub fn as_buffer(&self, id: &str) -> Option<&Buffer> {
        self.buffers
            .iter()
            .filter_map(|buf| if buf.to_id() == id { Some(buf) } else { None })
            .take(1)
            .next()
    }

    pub fn as_mut_buffer(&mut self, id: &str) -> Option<&mut Buffer> {
        self.buffers
            .iter_mut()
            .filter_map(|buf| if buf.to_id() == id { Some(buf) } else { None })
            .take(1)
            .next()
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
        files: Vec<(String, String)>,
    ) -> (Vec<Buffer>, Vec<WindowPrompt>) {
        let locs: Vec<Location> = files
            .into_iter()
            .map(|(f, e)| {
                let f: ffi::OsString = f.into();
                Location::new_disk(&f, &e)
            })
            .collect();

        let (mut buffers, mut prompts) = (vec![], vec![]);
        for loc in locs.into_iter() {
            match loc.read() {
                Ok(s) if loc.is_read_only() => {
                    debug!("opening {} in read-mode", loc);
                    let mut buf = Buffer::from_reader(s.as_bytes(), loc).unwrap();
                    buf.set_read_only(true);
                    buffers.push(buf);
                }
                Ok(s) => {
                    debug!("opening {} in write-mode", loc);
                    let mut buf = Buffer::from_reader(s.as_bytes(), loc).unwrap();
                    buf.set_read_only(config.read_only);
                    buffers.push(buf);
                }
                Err(err) => {
                    debug!("error opening {}", err);
                    let lines = vec![
                        format!("error opening {} : {}", loc, err),
                        format!("-press any key to continue-"),
                    ];
                    prompts.push((coord, lines, scheme).into());
                }
            }
        }

        (buffers, prompts)
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
            Inner::Edit(val) => val.wfile.to_cursor(),
            Inner::Prompt(val) => val.prompts[0].to_cursor(),
            Inner::Command(val) => val.cmdline.to_cursor(),
            Inner::Less(val) => val.less.to_cursor(),
            Inner::None => Default::default(),
        }
    }

    fn on_event(&mut self, evnt: Event) -> Result<Event> {
        use Event::Esc;

        let noop = Event::Noop;
        let inner = mem::replace(&mut self.inner, Default::default());
        let (inner, evnt) = {
            match (inner, evnt) {
                (Inner::Edit(edit), Event::Char(':', m)) if m.is_empty() => {
                    let mut val = Inner::new_command(self.coord, edit);
                    let evnt = val.cmdline.on_event(self, Event::Char(':', m))?;
                    (Inner::Command(val), evnt)
                }
                (Inner::Edit(mut edit), evnt) => {
                    let evnt = edit.wfile.on_event(self, evnt)?;
                    (Inner::Edit(edit), evnt)
                }
                (Inner::Prompt(mut prompt), evnt) => {
                    let evnt = prompt.prompts[0].on_event(self, evnt)?;
                    if let Some(_) = prompt.prompts[0].prompt_match() {
                        prompt.prompts.remove(0);
                    }
                    match prompt.prompts.len() {
                        0 => (Inner::Edit(prompt.edit), evnt),
                        _ => (Inner::Prompt(prompt), evnt),
                    }
                }
                (Inner::Command(cmd), Esc) => (Inner::Edit(cmd.edit), noop),
                (Inner::Command(mut cmd), evnt) => {
                    let evnt = cmd.cmdline.on_event(self, evnt)?;
                    (Inner::Command(cmd), evnt)
                }
                (Inner::Less(mut less), evnt) => {
                    let evnt = less.less.on_event(self, evnt)?;
                    (Inner::Less(less), evnt)
                }
                (Inner::None, _) => unreachable!(),
            }
        };

        let noop = Event::Noop;
        let (inner, evnt) = match evnt {
            Event::Esc => match inner {
                inner @ Inner::Edit(_) => (inner, Event::Esc),
                Inner::Prompt(val) => (Inner::Edit(val.edit), noop),
                Inner::Command(val) => (Inner::Edit(val.edit), noop),
                Inner::Less(val) => (Inner::Edit(val.edit), noop),
                Inner::None => unreachable!(),
            },
            Event::Code(event::Code::Cmd(name, args)) => {
                let mut cmd: cmd::Command = (name, args).into();
                (inner, cmd.on_command(self)?)
            }
            Event::Code(event::Code::Less(ref content)) => {
                let edit = inner.into_edit();
                let inner = Inner::Less(Less {
                    edit,
                    less: WindowLess::new(self.coord, content),
                });
                (inner, noop)
            }
            evnt => (inner, evnt),
        };

        self.inner = inner;
        Ok(evnt)
    }

    fn on_refresh(&mut self) -> Result<()> {
        let mut inner = mem::replace(&mut self.inner, Default::default());
        match &mut inner {
            Inner::Edit(edit) => {
                edit.wfile.on_refresh(self)?;
                edit.stsline.on_refresh(self)?;
                // TODO: edit.tbcline.on_refresh(self)?;
            }
            Inner::Prompt(prompt) => match prompt.prompts.first_mut() {
                Some(p) => p.on_refresh(self)?,
                None => (),
            },
            Inner::Command(cmd) => {
                cmd.cmdline.on_refresh(self)?;
                // TODO cmd.tbcline.on_refresh(self)?,
            }
            Inner::Less(less) => less.less.on_refresh(self)?,
            Inner::None => unreachable!(),
        }
        self.inner = inner;

        Ok(())
    }
}
