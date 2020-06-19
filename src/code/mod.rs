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

use crossterm::{cursor as term_cursor, queue};
use log::trace;
use toml;

use std::{
    ffi,
    io::{self, Write},
    mem,
    sync::mpsc,
};

use crate::{
    app::Application,
    buffer::Buffer,
    code::{
        cmd::Command, config::Config, window_file::WindowFile, window_less::WindowLess,
        window_line::WindowLine, window_prompt::WindowPrompt,
    },
    color_scheme::{ColorScheme, Highlight},
    event::{self, Event},
    location::Location,
    pubsub::PubSub,
    state::Opt,
    term::{Span, Spanline},
    window::{Coord, Cursor, Notify, Window},
    Error, Result,
};

pub struct Code {
    coord: Coord,
    config: Config,
    color_scheme: ColorScheme,
    subscribers: PubSub,
    buffers: Vec<Buffer>,

    wfile: WindowFile,
    tbcline: WindowLine,
    inner: Inner,
}

enum Inner {
    Edit {
        stsline: WindowLine,
    },
    AnyKey {
        stsline: WindowLine,
        prompts: Vec<WindowPrompt>,
    },
    Command {
        cmdline: WindowLine,
    },
    Less {
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

impl Code {
    pub fn new(config: toml::Value, coord: Coord, opts: Opt) -> Result<Code> {
        let config = {
            let cnf: Config = Default::default();
            cnf.mixin(config.try_into().unwrap())
        };

        trace!(
            "starting app `code` coord:{} config...\n{}",
            coord,
            err_at!(FailConvert, toml::to_string(&config))?
        );

        let mut app = Code {
            coord,
            config,
            color_scheme: Default::default(),
            subscribers: Default::default(),
            buffers: Default::default(),

            wfile: Default::default(),
            tbcline: Code::new_tbcline(coord),
            inner: Default::default(),
        };

        let stsline = Code::new_stsline(coord);
        let wps = app.open_cmd_files(opts.files.clone())?;
        app.inner = if wps.len() > 0 {
            Inner::AnyKey {
                stsline,
                prompts: wps,
            }
        } else {
            Inner::Edit { stsline }
        };

        Code::draw_screen(app.coord, &app.color_scheme)?;

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

    fn new_stsline(coord: Coord) -> WindowLine {
        let (col, _) = coord.to_origin();
        let (hgt, wth) = coord.to_size();
        WindowLine::new_status(Coord::new(col, hgt, 1, wth))
    }

    fn new_cmdline(coord: Coord) -> WindowLine {
        let (col, _) = coord.to_origin();
        let (hgt, wth) = coord.to_size();
        WindowLine::new_cmd(Coord::new(col, hgt, 1, wth))
    }

    fn new_tbcline(coord: Coord) -> WindowLine {
        let (col, _) = coord.to_origin();
        let (hgt, wth) = coord.to_size();
        let hgt = hgt.saturating_sub(1);
        WindowLine::new_tab(Coord::new(col, hgt, 1, wth))
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
        &self.color_scheme
    }
}

impl Code {
    fn draw_screen(coord: Coord, scheme: &ColorScheme) -> Result<()> {
        use crossterm::style::{SetBackgroundColor, SetForegroundColor};
        use std::iter::{repeat, FromIterator};

        let mut stdout = io::stdout();
        {
            let style = scheme.to_style(Highlight::Canvas);
            err_at!(
                Fatal,
                queue!(
                    stdout,
                    SetForegroundColor(style.fg.clone().into()),
                    SetBackgroundColor(style.bg.clone().into())
                )
            )?;
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

    fn open_cmd_files(&mut self, fls: Vec<String>) -> Result<Vec<WindowPrompt>> {
        let locs: Vec<Location> = fls
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
                    Ok(mut buf) if self.config.read_only => {
                        trace!("opening {} in read-mode", loc);
                        buf.set_read_only(true);
                        self.add_buffer(buf)
                    }
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
        let prompt_coord = {
            let (col, row) = self.coord.to_origin();
            let (hgt, wth) = self.coord.to_size();
            Coord::new(col, row, hgt - 1, wth)
        };
        for (loc, err) in efiles.into_iter() {
            let span1 = {
                let st = format!("{:?} : {}", loc.to_long_string()?, err);
                span!(st: st).using(self.color_scheme.to_style(Highlight::Error))
            };
            let span2 = {
                let span = span!(st: format!("-press any key to continue-"));
                span.using(self.color_scheme.to_style(Highlight::Prompt))
            };
            let span_lines: Vec<Spanline> = vec![span1.into(), span2.into()];
            wps.push(WindowPrompt::new(prompt_coord, span_lines));
        }

        Ok(wps)
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
            Inner::Edit { .. } => self.wfile.to_cursor(),
            Inner::AnyKey { prompts, .. } => prompts[0].to_cursor(),
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
        let mut wfile = mem::replace(&mut self.wfile, Default::default());
        wfile.on_refresh(self)?;
        self.wfile = wfile;

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
                evnt => {
                    let mut wfile = {
                        let def_wfile: WindowFile = Default::default();
                        mem::replace(&mut app.wfile, def_wfile)
                    };
                    let evnt = wfile.on_event(app, evnt)?;
                    app.wfile = wfile;
                    Ok((Inner::Edit { stsline }, evnt))
                }
            },
            Inner::AnyKey {
                mut prompts,
                stsline,
            } => {
                let evnt = prompts[0].on_event(app, evnt)?;
                if prompts[0].prompt_match().is_some() {
                    prompts.remove(0);
                }
                Ok(match prompts.len() {
                    0 => (Inner::Edit { stsline }, evnt),
                    _ => (Inner::AnyKey { prompts, stsline }, evnt),
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
            Inner::AnyKey { prompts, stsline } => {
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
