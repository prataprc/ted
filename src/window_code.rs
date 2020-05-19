use std::mem;

use crate::{
    cmd::Command,
    event::Event,
    keymap::Keymap,
    state::State,
    tabc::TabComplete,
    window::{new_window_line, Coord, Cursor, Span, Window},
    window_file::WindowFile,
    window_line::WindowLine,
    Result,
};

pub enum Message {
    Status(Span),
    TabComplete(TabComplete),
}

#[derive(Clone, Default)]
pub struct WindowCode {
    coord: Coord,
    wfile: WindowFile,
    tbcline: WindowLine,
    keymap: Keymap,
    inner: Inner,
}

#[derive(Clone)]
enum Inner {
    Regular { stsline: WindowLine },
    Command { cmdline: WindowLine, cmd: Command },
}

impl Default for Inner {
    fn default() -> Self {
        Inner::Regular {
            stsline: Default::default(),
        }
    }
}

impl WindowCode {
    pub fn new(coord: Coord) -> WindowCode {
        let inner = Inner::Regular {
            stsline: new_window_line("stsline", coord),
        };

        WindowCode {
            coord,
            wfile: WindowFile::new(coord),
            tbcline: new_window_line("tbcline", coord),
            keymap: Default::default(),
            inner,
        }
    }

    pub fn set_keymap(&mut self, keymap: Keymap) -> &mut Self {
        self.keymap = keymap;
        self
    }
}

impl WindowCode {
    #[inline]
    pub fn post(&mut self, s: &mut State, msg: Message) -> Result<()> {
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

    pub fn on_event(&mut self, s: &mut State, evnt: Event) -> Result<Event> {
        let mut keymap = mem::replace(&mut self.keymap, Default::default());

        let buf = self.wfile.as_mut_buffer();
        let evnt = keymap.fold(buf, s, evnt)?;
        let evnt = match &mut self.inner {
            Inner::Regular { .. } => self.wfile.on_event(s, evnt)?,
            Inner::Command { cmdline, .. } => cmdline.on_event(s, evnt)?,
        };

        self.keymap = keymap;
        Ok(evnt)
    }

    pub fn on_refresh(&mut self, s: &mut State) -> Result<()> {
        self.wfile.on_refresh(s)?;
        match &mut self.inner {
            Inner::Regular { stsline } => stsline.on_refresh(s)?,
            Inner::Command { cmdline, cmd } => {
                // self.cmd.on_refresh(s)?;
                cmdline.on_refresh(s)?;
            }
        }
        self.tbcline.on_refresh(s)
    }
}
