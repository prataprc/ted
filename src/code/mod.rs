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

use std::{convert::TryFrom, ffi, mem, sync::mpsc};

use crate::{
    app::Application,
    buffer::Buffer,
    code::config::Config,
    code::window_cmd::WindowCmd,
    code::window_file::WindowFile,
    code::window_less::WindowLess,
    code::window_line::WindowLine,
    code::window_prompt::WindowPrompt,
    colors::ColorScheme,
    event::{self, Event},
    location::Location,
    mark,
    pubsub::{self, PubSub},
    state::{self, State},
    window::{Coord, Cursor, Window},
    Result,
};

pub struct Code {
    // full set of configuration paramter from [State][State].
    config_value: toml::Value,
    // application only configuration.
    config: Config,
    // screen coordinate for this application.
    coord: Coord,
    // application local subscribe-publish instance, also includes
    // global subscribers.
    subscribers: PubSub,
    // a copy of all available color schemes.
    schemes: Vec<ColorScheme>,
    // buffer-list
    buffers: Vec<Buffer>,
    // list of global marks,
    marks: mark::Marks,

    // application state machine
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
        let config = {
            let value = {
                let toml_value = state.config_value.clone();
                crate::config::to_section(toml_value, "code")
            };
            Config::default().mixin(value.try_into().unwrap())
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
            buffers: Vec::default(),
            marks: mark::new_marks(),
            inner: Inner::default(),
        };

        let (buffers, prompts) = {
            let files = {
                let iter = state.opts.files.iter();
                iter.map(|f| (f.clone(), format!("utf-8"))).collect()
            };
            match app.open_cmd_files(files) {
                (bufs, ps) if bufs.len() == 0 => (vec![Buffer::empty()], ps),
                (bufs, ps) => (bufs, ps),
            }
        };

        let edit = {
            let buffer = buffers.first().unwrap();
            Edit {
                wfile: (&app, buffer, app.to_coord_wfile()).into(),
                stsline: ("stsline", app.to_coord_stsline()).into(),
                tbcline: ("tbcline", app.to_coord_tbcline()).into(),
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
    #[inline]
    fn to_coord_wfile(&self) -> Coord {
        let mut coord = self.coord;
        coord.hgt = coord.hgt.saturating_sub(1);
        coord
    }

    #[inline]
    fn to_coord_wcmd(&self) -> Coord {
        let mut coord = self.coord;
        coord.row = coord.hgt;
        coord.hgt = 1;
        coord
    }

    #[inline]
    fn to_coord_prompt(&self) -> Coord {
        let mut coord = self.coord;
        coord.hgt = coord.hgt.saturating_sub(1);
        coord
    }

    #[inline]
    fn to_coord_stsline(&self) -> Coord {
        let mut coord = self.coord;
        coord.row = coord.hgt;
        coord.hgt = 1;
        coord
    }

    #[inline]
    fn to_coord_tbcline(&self) -> Coord {
        let mut coord = self.coord;
        coord.row = coord.hgt.saturating_sub(1);
        coord.hgt = 1;
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
    fn open_cmd_files(&self, files: Vec<(String, String)>) -> (Vec<Buffer>, Vec<WindowPrompt>) {
        let coord = self.to_coord_prompt();
        let (mut buffers, mut prompts) = (vec![], vec![]);

        let mut locs: Vec<Location> = vec![];
        for (f, e) in files.into_iter() {
            let f: ffi::OsString = f.into();
            match Location::new_disk(&f, &e) {
                Ok(loc) => locs.push(loc),
                Err(err) => {
                    let lines = vec![
                        format!("error opening {:?} : {}", f, err.to_error()),
                        format!("-press any key to continue-"),
                    ];
                    prompts.push((self, coord, lines).into());
                }
            }
        }

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
                    let lines = vec![
                        format!("error opening {} : {}", loc, err.to_error()),
                        format!("-press any key to continue-"),
                    ];
                    prompts.push((self, coord, lines).into());
                }
            }
        }

        (buffers, prompts)
    }
}

impl Application for Code {
    fn subscribe(&mut self, topic: &str, tx: mpsc::Sender<pubsub::Notify>) {
        self.subscribers.subscribe(topic, tx);
    }

    fn notify(&self, topic: &str, msg: pubsub::Notify) -> Result<()> {
        self.subscribers.notify(topic, msg)
    }

    fn to_cursor(&self) -> Cursor {
        match &self.inner {
            Inner::Edit(val) => val.wfile.to_cursor(),
            Inner::Prompt(val) => val.prompts[0].to_cursor(),
            Inner::Command(val) => val.wcmd.to_cursor(),
            Inner::Less(val) => val.wless.to_cursor(),
            Inner::None => Cursor::default(),
        }
    }

    fn on_event(&mut self, evnt: Event) -> Result<Event> {
        use crate::event::Mto;

        let inner = mem::replace(&mut self.inner, Inner::default());
        let (mut inner, evnt) = match (inner, evnt) {
            (Inner::Edit(edit), Event::Mark(mrk)) => {
                mark::set_mark(&mut self.marks, mrk);
                (Inner::Edit(edit), Event::Noop)
            }
            (Inner::Edit(edit), Event::Mt(Mto::Jump('`', _mindex))) => {
                // TODO: use `:buffer` command to load the buffer.
                // TODO: set the cursor, clear-sticky-col
                (Inner::Edit(edit), Event::Noop)
            }
            (Inner::Edit(edit), Event::Mt(Mto::Jump('\'', _mindex))) => {
                // TODO: use `:buffer` command to load the buffer.
                // TODO: set the cursor, clear-sticky-col
                // TODO: mto_line_home
                (Inner::Edit(edit), Event::Noop)
            }
            (Inner::Edit(edit), Event::Mt(Mto::Jump(typ, mindex))) => {
                (Inner::Edit(edit), Event::Mt(Mto::Jump(typ, mindex)))
            }
            (Inner::Edit(edit), Event::Char(':', m)) if m.is_empty() => {
                let prefix = edit.wfile.to_event_prefix();
                let mut val = Command {
                    edit,
                    tbcline: ("tbcline", self.to_coord_tbcline()).into(),
                    wcmd: TryFrom::try_from((&*self, self.to_coord_wcmd()))?,
                };
                let evnt = val.wcmd.on_event(self, prefix)?;
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

        let mut new_evnt: Event = Event::default();
        for evnt in evnt.into_iter() {
            inner = match evnt {
                Event::Code(event::Code::Less(ref content)) => {
                    let edit = inner.into_edit();
                    let wless = {
                        let coord = self.coord;
                        TryFrom::try_from((&*self, content.as_str(), coord))?
                    };
                    Inner::Less(Less { edit, wless })
                }
                Event::Esc => Inner::Edit(inner.into_edit()),
                evnt => {
                    new_evnt.push(evnt);
                    inner
                }
            }
        }

        self.inner = inner;
        Ok(new_evnt)
    }

    fn on_refresh(&mut self) -> Result<()> {
        let mut inner = mem::replace(&mut self.inner, Inner::default());
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

    fn to_tab_title(&self, wth: usize) -> state::TabTitle {
        use crate::text;
        use std::iter::FromIterator;

        let edit = match &self.inner {
            Inner::Edit(edit) => edit,
            Inner::Prompt(val) => &val.edit,
            Inner::Command(val) => &val.edit,
            Inner::Less(val) => &val.edit,
            Inner::None => unreachable!(),
        };
        let active = false;
        match self.as_buffer(&edit.wfile.to_buffer_id()) {
            Some(buf) => {
                let text = match buf.to_location().to_tab_title(wth) {
                    Ok(text) => text,
                    Err(_) => {
                        let iter = text::take_width(buf.to_id().chars().rev(), wth);
                        String::from_iter(iter)
                    }
                };
                let modified = buf.is_modified();
                state::TabTitle {
                    text,
                    modified,
                    active,
                }
            }
            None => {
                let text = "∞".to_string();
                let modified = false;
                state::TabTitle {
                    text,
                    modified,
                    active,
                }
            }
        }
    }
}
