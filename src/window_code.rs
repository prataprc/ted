use std::mem;

use crate::{
    cmd::Command,
    event::Event,
    keymap::Keymap,
    state::Context,
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
    pub fn post(&mut self, _: &mut Context, _msg: Message) -> Result<()> {
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

    pub fn on_event(&mut self, c: &mut Context, evnt: Event) -> Result<Event> {
        let mut keymap = mem::replace(&mut self.keymap, Default::default());

        let evnt = with_window!(c, self, Code, keymap.fold(c, evnt))?;
        let evnt = match &mut self.inner {
            Inner::Regular { .. } => self.wfile.on_event(c, evnt)?,
            Inner::Command { cmdline, .. } => cmdline.on_event(c, evnt)?,
        };

        self.keymap = keymap;
        Ok(evnt)
    }

    pub fn on_refresh(&mut self, c: &mut Context) -> Result<()> {
        self.wfile.on_refresh(c)?;
        match &mut self.inner {
            Inner::Regular { stsline } => stsline.on_refresh(c)?,
            Inner::Command { cmdline, cmd } => {
                // self.cmd.on_refresh(c)?;
                cmdline.on_refresh(c)?;
            }
        }
        self.tbcline.on_refresh(c)
    }
}
