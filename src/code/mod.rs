//mod cmd;
//mod cmd_edit;
//mod cmd_file;
//mod cmd_write;

mod col_nu;
mod config;
mod ftype;
mod ftype_txt_en;
mod keymap;
mod view;
mod window_edit;
mod window_file;
mod window_line;
mod window_prompt;

use crossterm::{cursor as term_cursor, queue};
use log::trace;

use std::{
    ffi,
    io::{self, Write},
    mem,
    sync::mpsc,
};

use crate::{
    buffer::Buffer,
    code::window_prompt::WindowPrompt,
    code::{config::Config, keymap::Keymap},
    code::{window_file::WindowFile, window_line::WindowLine},
    color_scheme::{ColorScheme, Highlight},
    event::Event,
    location::Location,
    pubsub::PubSub,
    state::Opt,
    window::{Coord, Cursor, Notify, Span, Spanline},
    Error, Result,
};

pub struct App {
    coord: Coord,
    config: Config,
    color_scheme: ColorScheme,
    subscribers: PubSub,
    buffers: Vec<Buffer>,

    wfile: WindowFile,
    tbcline: WindowLine,
    keymap: Keymap,
    inner: Inner,
}

enum Inner {
    Regular {
        stsline: WindowLine,
    },
    AnyKey {
        stsline: Option<WindowLine>,
        prompts: Vec<WindowPrompt>,
    },
    //Command { cmdline: WindowLine, cmd: Command },
    None,
}

impl Default for Inner {
    fn default() -> Inner {
        Inner::None
    }
}

impl AsRef<Config> for App {
    fn as_ref(&self) -> &Config {
        &self.config
    }
}

impl App {
    pub fn new(config: toml::Value, coord: Coord, opts: Opt) -> Result<App> {
        let config = {
            let cnf: Config = Default::default();
            cnf.mixin(config.try_into().unwrap())
        };

        trace!("starting app `code` coord:{} config...\n{}", coord, config);

        let mut app = App {
            coord,
            config,
            color_scheme: Default::default(),
            subscribers: Default::default(),
            buffers: Default::default(),

            wfile: Default::default(),
            tbcline: {
                let (col, _) = coord.to_origin();
                let (hgt, wth) = coord.to_size();
                let hgt = hgt.saturating_sub(1);
                WindowLine::new("tbcline", Coord::new(col, hgt, 1, wth))
            },
            keymap: Default::default(),
            inner: Default::default(),
        };

        let stsline = {
            let (col, _) = coord.to_origin();
            let (hgt, wth) = coord.to_size();
            WindowLine::new("stsline", Coord::new(col, hgt, 1, wth))
        };

        let wps = app.open_cmd_files(opts.files.clone())?;
        app.inner = if wps.len() > 0 {
            Inner::AnyKey {
                stsline: Some(stsline),
                prompts: wps,
            }
        } else {
            Inner::Regular { stsline }
        };

        App::draw_screen(app.coord, &app.color_scheme)?;

        app.wfile = {
            let wf_coord = {
                let mut wf_coord = coord;
                wf_coord.hgt -= 1;
                wf_coord
            };
            match app.buffers.last() {
                Some(buf) => WindowFile::new(wf_coord, buf, app.as_ref()),
                None => {
                    let buf = Buffer::empty();
                    let wfile = WindowFile::new(wf_coord, &buf, app.as_ref());
                    app.add_buffer(buf);
                    wfile
                }
            }
        };

        Ok(app)
    }
}

impl App {
    pub fn subscribe(&mut self, topic: &str, tx: mpsc::Sender<Notify>) {
        self.subscribers.subscribe(topic, tx);
    }

    pub fn notify(&self, topic: &str, msg: Notify) -> Result<()> {
        self.subscribers.notify(topic, msg)
    }

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
}

impl App {
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
        &self.color_scheme
    }
}

impl App {
    #[inline]
    pub fn post(&mut self, _msg: Notify) -> Result<()> {
        //match msg {
        //    Notify::Status(sl)) -> self.stsline.set(sl),
        //    Notify::TabComplete(sl) -> self.tbcline.set(sl),
        //}
        Ok(())
    }

    pub fn to_cursor(&self) -> Cursor {
        match &self.inner {
            Inner::Regular { .. } => self.wfile.to_cursor(),
            Inner::AnyKey { prompts, .. } => prompts[0].to_cursor(),
            // Inner::Command { cmdline, .. } => cmdline.to_cursor(),
            Inner::None => Default::default(),
        }
    }

    pub fn on_event(&mut self, evnt: Event) -> Result<Event> {
        let mut keymap = mem::replace(&mut self.keymap, Default::default());
        let evnt = {
            let buf = self.as_mut_buffer(&self.wfile.to_buffer_id());
            keymap.fold(buf, evnt)?
        };
        self.keymap = keymap;

        let mut inner = mem::replace(&mut self.inner, Default::default());
        let evnt = match &mut inner {
            Inner::Regular { .. } => {
                let mut wfile = mem::replace(&mut self.wfile, Default::default());
                let evnt = wfile.on_event(self, evnt)?;
                self.wfile = wfile;
                evnt
            }
            Inner::AnyKey { prompts, stsline } => {
                let evnt = prompts[0].on_event(self, evnt)?;
                match prompts[0].prompt_match() {
                    Some(_) if prompts.len() > 1 => {
                        prompts.remove(0);
                    }
                    Some(_) => {
                        prompts.remove(0);
                        inner = Inner::Regular {
                            stsline: stsline.take().unwrap(),
                        };
                    }
                    None => (),
                }
                evnt
            }
            //Inner::Command { cmdline, .. } => {
            //    let wline = mem::replace(cmdline, Default::default());
            //    let evnt = wline.on_event(self, evnt)?;
            //    *cmdline = wline;
            //    evnt
            //}
            Inner::None => evnt,
        };
        self.inner = inner;
        Ok(evnt)
    }

    pub fn on_refresh(&mut self) -> Result<()> {
        let mut wfile = mem::replace(&mut self.wfile, Default::default());
        wfile.on_refresh(self)?;
        self.wfile = wfile;

        let mut inner = mem::replace(&mut self.inner, Default::default());
        match &mut inner {
            Inner::Regular { stsline } => stsline.on_refresh(self)?,
            Inner::AnyKey { prompts, .. } => prompts[0].on_refresh(self)?,
            //Inner::Command { cmdline, cmd } => {
            //    // self.cmd.on_refresh()?;
            //    let wline = mem::replace(cmdline, Default::default());
            //    wline.on_refresh(self)?;
            //    *cmdline = wline;
            //}
            Inner::None => (),
        }
        self.inner = inner;

        //let mut wline = mem::replace(&mut self.tbcline, Default::default());
        //wline.on_refresh(self)?;
        //self.tbcline = wline;

        Ok(())
    }
}

impl App {
    fn draw_screen(coord: Coord, scheme: &ColorScheme) -> Result<()> {
        use crossterm::style::{SetBackgroundColor, SetForegroundColor};
        use std::iter::{repeat, FromIterator};

        let mut stdout = io::stdout();
        {
            let style = scheme.to_style(Highlight::Canvas);
            err_at!(Fatal, queue!(stdout, SetForegroundColor(style.fg)))?;
            err_at!(Fatal, queue!(stdout, SetBackgroundColor(style.fg)))?;
        }

        let (col, row) = coord.to_origin_cursor();
        let (hgt, wth) = coord.to_size();
        for r in row..(row + hgt) {
            let span: Span = {
                let s = String::from_iter(repeat(' ').take(wth as usize));
                s.into()
            };
            err_at!(Fatal, queue!(stdout, term_cursor::MoveTo(col, r)))?;
            err_at!(Fatal, queue!(stdout, span))?;
        }

        Ok(())
    }

    fn open_cmd_files(&mut self, files: Vec<String>) -> Result<Vec<WindowPrompt>> {
        let locs: Vec<Location> = files
            .into_iter()
            .map(|f| {
                let f: ffi::OsString = f.into();
                Location::new_disk(&f)
            })
            .collect();
        let mut efiles = vec![];
        for loc in locs.into_iter() {
            match loc.to_rw_file() {
                Some(f) => match Buffer::from_reader(f, loc.clone()) {
                    Ok(buf) => {
                        trace!("opening {} in write-mode", loc);
                        self.add_buffer(buf)
                    }
                    Err(err) => efiles.push((loc, err)),
                },
                None => match loc.to_r_file() {
                    Some(f) => match Buffer::from_reader(f, loc.clone()) {
                        Ok(mut buf) => {
                            trace!("opening {} in read-mode", loc);
                            buf.set_read_only(true);
                            self.add_buffer(buf);
                        }
                        Err(err) => efiles.push((loc, err)),
                    },
                    None => {
                        let err = "file missing/no-permission".to_string();
                        efiles.push((loc, Error::IOError(err)))
                    }
                },
            }
        }

        let mut wps = vec![];
        for (loc, err) in efiles.into_iter() {
            let span1 = {
                let st = format!("{:?} : {}", loc.to_long_string()?, err);
                span!(st: st).using(self.color_scheme.to_style(Highlight::Error))
            };
            let span2 = {
                let span = span!(st: format!(" press any key to continue"));
                span.using(self.color_scheme.to_style(Highlight::Prompt))
            };
            let span_lines: Vec<Spanline> = vec![span1.into(), span2.into()];
            wps.push(WindowPrompt::new(self.coord, span_lines));
        }

        Ok(wps)
    }
}
