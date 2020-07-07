mod cmd;
mod cmd_set;
//mod cmd_edit;
//mod cmd_file;
//mod cmd_write;

mod config;
mod keymap;
mod keymap_cmd;
mod keymap_edit;
mod keymap_less;
mod window_cmd;
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
    code::{config::Config, window_cmd::WindowCmd, window_file::WindowFile},
    code::{window_less::WindowLess, window_line::WindowLine},
    colors::ColorScheme,
    event::{self, Event},
    location::Location,
    pubsub::{Notify, PubSub},
    state::State,
    window::{Coord, Cursor, Window},
    Result,
};

pub struct Code {
    config_value: toml::Value,
    config: Config,
    coord: Coord,
    subscribers: PubSub,
    schemes: Vec<ColorScheme>,
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
    wcmd: WindowCmd,
}

struct Less {
    edit: Edit,
    wless: WindowLess,
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

impl<'a> From<(&'a State, Coord)> for Code {
    fn from((state, coord): (&'a State, Coord)) -> Code {
        use crate::config as mod_config;

        let config: Config = {
            let value = {
                let toml_value = state.config_value.clone();
                mod_config::to_section(toml_value, "code")
            };
            let cnf: Config = Default::default();
            cnf.mixin(value.try_into().unwrap())
        };

        debug!(
            "starting app `code` coord:{} config...\n{}",
            coord,
            toml::to_string(&config).unwrap(),
        );

        let mut app = Code {
            config_value: state.config_value.clone(),
            config: config.clone(),
            coord,
            schemes: state.schemes.clone(),
            subscribers: state.subscribers.clone(),
            buffers: Default::default(),
            inner: Default::default(),
        };

        let (mut buffers, prompts) = {
            let files = {
                let iter = state.opts.files.iter().map(
                    // TODO: encoding from cmd-line
                    |f| (f.clone(), format!("utf-8")),
                );
                iter.collect()
            };
            let coord = Self::to_coord_prompt(coord);
            app.open_cmd_files(coord, files)
        };
        if buffers.len() == 0 {
            buffers.push(Buffer::empty());
        }
        let edit = {
            let stsline = app.new_window_line("status-line", coord);
            let tbcline = app.new_window_line("tab-compl-line", coord);
            let wfile = {
                let buf = buffers.first().unwrap();
                app.new_window_file(coord, buf)
            };
            Edit {
                wfile,
                stsline,
                tbcline,
            }
        };

        app.buffers = buffers;
        app.inner = if prompts.len() > 0 {
            Inner::Prompt(Prompt { edit, prompts })
        } else {
            Inner::Edit(edit)
        };
        app
    }
}

impl Code {
    fn new_window_file(&self, mut coord: Coord, buf: &Buffer) -> WindowFile {
        coord.hgt = coord.hgt.saturating_sub(1);
        WindowFile::new(coord, buf, self)
    }

    fn new_window_prompt(&self, coord: Coord, ls: Vec<String>) -> WindowPrompt {
        WindowPrompt::new(coord, ls, self)
    }

    fn new_window_line(&self, what: &str, mut coord: Coord) -> WindowLine {
        match what {
            "status-line" => {
                coord.row = coord.hgt;
                coord.hgt = 1;
                WindowLine::new_status(coord, self)
            }
            "tab-compl-line" => {
                coord.row = coord.hgt.saturating_sub(1);
                coord.hgt = 1;
                WindowLine::new_tab(coord, self)
            }
            _ => unreachable!(),
        }
    }

    fn new_window_cmd(&self, mut coord: Coord, edit: Edit) -> Command {
        let tbcline = self.new_window_line("tab-compl-line", coord);
        let wcmd = {
            coord.row = coord.hgt;
            coord.hgt = 1;
            WindowCmd::new(coord, self)
        };
        Command {
            edit,
            tbcline,
            wcmd,
        }
    }

    fn new_window_less(&self, coord: Coord, content: &str) -> WindowLess {
        WindowLess::new(coord, content, self)
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

    pub fn as_config(&self) -> &Config {
        self.as_ref()
    }

    pub fn to_color_scheme(&self, name: Option<&str>) -> ColorScheme {
        let name = name.unwrap_or(self.config.color_scheme.as_str());
        for scheme in self.schemes.iter() {
            if scheme.name == name {
                return scheme.clone();
            }
        }
        self.to_color_scheme(Some("default"))
    }
}

impl Code {
    fn open_cmd_files(
        &self,
        coord: Coord,
        files: Vec<(String, String)>,
    ) -> (Vec<Buffer>, Vec<WindowPrompt>) {
        let mut locs: Vec<Location> = vec![];
        for (f, e) in files.into_iter() {
            let f: ffi::OsString = f.into();
            locs.push(Location::new_disk(&f, &e).unwrap())
        }

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
                    buf.set_read_only(self.config.read_only);
                    buffers.push(buf);
                }
                Err(err) => {
                    debug!("error opening {}", err);
                    let lines = vec![
                        format!("error opening {} : {}", loc, err),
                        format!("-press any key to continue-"),
                    ];
                    prompts.push(self.new_window_prompt(coord, lines));
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
            Inner::Command(val) => val.wcmd.to_cursor(),
            Inner::Less(val) => val.wless.to_cursor(),
            Inner::None => Default::default(),
        }
    }

    fn on_event(&mut self, evnt: Event) -> Result<Event> {
        let inner = mem::replace(&mut self.inner, Default::default());
        let (mut inner, evnt) = match (inner, evnt) {
            (Inner::Edit(edit), Event::Char(':', m)) if m.is_empty() => {
                let mut val = self.new_window_cmd(self.coord, edit);
                let evnt = val.wcmd.on_event(self, Event::Char(':', m))?;
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
            (Inner::Command(mut cmd), evnt) => {
                let evnt = cmd.wcmd.on_event(self, evnt)?;
                (Inner::Command(cmd), evnt)
            }
            (Inner::Less(mut less), evnt) => {
                let evnt = less.wless.on_event(self, evnt)?;
                (Inner::Less(less), evnt)
            }
            (Inner::None, _) => unreachable!(),
        };

        let mut new_evnt: Event = Default::default();
        for evnt in evnt.into_iter() {
            match evnt {
                Event::Code(event::Code::Less(ref content)) => {
                    let edit = inner.into_edit();
                    let wless = self.new_window_less(self.coord, content);
                    inner = Inner::Less(Less { edit, wless });
                }
                Event::Esc => {
                    inner = Inner::Edit(inner.into_edit());
                }
                evnt => new_evnt.push(evnt),
            }
        }

        self.inner = inner;
        Ok(new_evnt)
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
                cmd.wcmd.on_refresh(self)?;
            }
            Inner::Less(less) => less.wless.on_refresh(self)?,
            Inner::None => unreachable!(),
        }
        self.inner = inner;

        Ok(())
    }
}
