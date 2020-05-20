//mod config;
//mod state;
//
//pub use config::Config;
//pub use state::State;
//
//mod ftype;
//mod ftype_txt_en;
//
//mod keymap;
//mod keymap_code;
//
//mod cmd;
//mod cmd_edit;
//mod cmd_file;
//mod cmd_write;

mod config;
mod keymap;
mod window_file;
mod window_line;

use std::mem;

use crate::{
    buffer::Buffer,
    code::{config::Config, keymap::Keymap},
    code::{window_file::WindowFile, window_line::WindowLine},
    event::Event,
    pubsub::PubSub,
    window::{Coord, Cursor, Message, Span},
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
    Command { cmdline: WindowLine, cmd: Command },
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

    pub fn subscribe(&mut self, topic: &str, tx: mpsc::Sender<Message>) {
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
    pub fn post(&mut self, msg: Message) -> Result<()> {
        //match (name, msg) {
        //    ("status", Message::Status(sl)) -> self.stsline.set(sl),
        //    ("tabcomplete", Message::TabComplete(sl) -> self.tbcline.set(sl),
        //}
        Ok(())
    }

    pub fn to_cursor(&self) -> Cursor {
        match &self.inner {
            Inner::Regular { .. } => self.wfile.to_cursor(),
            Inner::Command { cmdline, .. } => cmdline.to_cursor(),
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

        let evnt = match &mut self.inner {
            Inner::Regular { .. } => {
                let wfile = mem::replace(&mut self.wfile, Default::default());
                let evnt = wfile.on_event(self, evnt)?;
                self.wfile = wfile;
                evnt
            }
            Inner::Command { cmdline, .. } => {
                let wline = mem::replace(cmdline, Default::default());
                let evnt = wline.on_event(app, evnt)?;
                *cmdline = wline;
                evnt
            }
        };

        Ok(evnt)
    }

    pub fn on_refresh(&mut self) -> Result<()> {
        let wfile = mem::replace(&mut self.wfile, Default::default());
        self.wfile.on_refresh(app)?;
        self.wfile = wfile;

        match &mut self.inner {
            Inner::Regular { stsline } => {
                let wline = mem::replace(stsline, Default::default());
                wline.on_refresh()?;
                *stsline = wline;
            }
            Inner::Command { cmdline, cmd } => {
                // self.cmd.on_refresh()?;
                let wline = mem::replace(cmdline, Default::default());
                wline.on_refresh()?;
                *cmdline = wline;
            }
        }
        let wline = mem::replace(&mut self.tbcline, Default::default());
        self.tbcline.on_refresh()?;
        self.tbcline = wline;
    }
}
