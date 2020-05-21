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

use std::{mem, sync::mpsc};

use crate::{
    buffer::Buffer,
    code::{config::Config, keymap::Keymap},
    code::{window_file::WindowFile, window_line::WindowLine},
    event::Event,
    pubsub::PubSub,
    window::{Coord, Cursor, Notify},
    Result,
};

pub fn new_window_line(typ: &str, mut coord: Coord) -> WindowLine {
    let (col, _) = coord.to_origin();
    let (hgt, wth) = coord.to_size();
    let row = match typ {
        "cmdline" => hgt.saturating_sub(2),
        "stsline" => hgt.saturating_sub(2),
        "tbcline" => hgt.saturating_sub(3),
        _ => unreachable!(),
    };
    coord = Coord::new(col, row, 1, wth);
    WindowLine::new(typ, coord)
}

pub struct App {
    config: Config,
    subscribers: PubSub,
    buffers: Vec<Buffer>,

    coord: Coord,
    wfile: WindowFile,
    tbcline: WindowLine,
    keymap: Keymap,
    inner: Inner,
}

enum Inner {
    Regular { stsline: WindowLine },
    // Command { cmdline: WindowLine, cmd: Command },
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
    pub fn new(config: toml::Value, subscribers: PubSub, coord: Coord) -> App {
        let inner = Inner::Regular {
            stsline: new_window_line("stsline", coord),
        };

        let config = {
            let cnf: Config = Default::default();
            cnf.mixin(config.try_into().unwrap())
        };
        App {
            config,
            subscribers,
            buffers: Default::default(),

            coord,
            wfile: WindowFile::new(coord),
            tbcline: new_window_line("tbcline", coord),
            keymap: Default::default(),
            inner: inner,
        }
    }

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
}

impl App {
    #[inline]
    pub fn post(&mut self, msg: Notify) -> Result<()> {
        //match (name, msg) {
        //    ("status", Notify::Status(sl)) -> self.stsline.set(sl),
        //    ("tabcomplete", Notify::TabComplete(sl) -> self.tbcline.set(sl),
        //}
        Ok(())
    }

    pub fn to_cursor(&self) -> Cursor {
        match &self.inner {
            Inner::Regular { .. } => self.wfile.to_cursor(),
            // Inner::Command { cmdline, .. } => cmdline.to_cursor(),
            Inner::None => Default::default(),
        }
    }

    pub fn on_event(&mut self, evnt: Event) -> Result<Event> {
        let mut keymap = mem::replace(&mut self.keymap, Default::default());
        let evnt = {
            let buf = self.as_mut_buffer(&self.wfile.to_buffer_id());
            let evnt = keymap.fold(buf, evnt)?;
            evnt
        };
        self.keymap = keymap;

        let mut inner = mem::replace(&mut self.inner, Default::default());
        let evnt = match &mut inner {
            Inner::Regular { .. } => {
                let mut wfile = mem::replace(&mut self.wfile, Default::default());
                let evnt = wfile.on_event(self, evnt)?;
                self.wfile = wfile;
                evnt
            } //Inner::Command { cmdline, .. } => {
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
            Inner::Regular { stsline } => {
                stsline.on_refresh(self)?;
            } //Inner::Command { cmdline, cmd } => {
            //    // self.cmd.on_refresh()?;
            //    let wline = mem::replace(cmdline, Default::default());
            //    wline.on_refresh(self)?;
            //    *cmdline = wline;
            //}
            Inner::None => (),
        }
        self.inner = inner;

        let mut wline = mem::replace(&mut self.tbcline, Default::default());
        wline.on_refresh(self)?;
        self.tbcline = wline;
        Ok(())
    }
}
