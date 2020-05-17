use std::mem;

use crate::{
    cmd::Command,
    event::Event,
    keymap::Keymap,
    state::Context,
    window::{new_window_line, Coord, Cursor, Span, Window},
    window_file::WindowFile,
    window_line::WindowLine,
    Result,
};

pub enum Message {
    Status(Span),
    TabComplete(Span),
}

#[derive(Clone, Default)]
pub struct WindowCode {
    coord: Coord,
    w: WindowFile,
    stsline: WindowLine,
    tbcline: WindowLine,
    keymap: Keymap,
    inner: Inner,
}

#[derive(Clone)]
enum Inner {
    Regular,
    Command { w: WindowLine, cmd: Command },
}

impl Default for Inner {
    fn default() -> Inner {
        Inner::Regular
    }
}

impl WindowCode {
    pub fn new(coord: Coord) -> WindowCode {
        WindowCode {
            coord,
            w: WindowFile::new(coord),
            stsline: new_window_line("stsline", coord),
            tbcline: new_window_line("tbcline", coord),
            inner: Default::default(),
            keymap: Default::default(),
        }
    }

    pub fn set_keymap(&mut self, keymap: Keymap) -> &mut Self {
        self.keymap = keymap;
        self
    }
}

impl WindowCode {
    #[inline]
    pub fn post<Message>(&mut self, _: &mut Context, _msg: Message) {
        //match (name, msg) {
        //    ("status", Message::Status(sl)) -> self.stsline.set(sl),
        //    ("tabcomplete", Message::TabComplete(sl) -> self.tbcline.set(sl),
        //}
    }

    pub fn to_cursor(&self) -> Cursor {
        match &self.inner {
            Inner::Regular => self.w.to_cursor(),
            Inner::Command { w, .. } => w.to_cursor(),
        }
    }

    pub fn on_event(&mut self, c: &mut Context, evnt: Event) -> Result<Event> {
        let mut keymap = mem::replace(&mut self.keymap, Default::default());
        c.w = Window::Code(Box::new(mem::replace(self, Default::default())));

        let evnt = match keymap.fold(c, evnt)? {
            Event::Noop => Event::Noop,
            evnt => match &mut self.inner {
                Inner::Regular => self.w.on_event(c, evnt)?,
                Inner::Command { w, .. } => w.on_event(c, evnt)?,
            },
        };

        *self = match mem::replace(&mut c.w, Default::default()) {
            Window::Code(w) => *w,
            _ => unreachable!(),
        };
        self.keymap = keymap;

        Ok(evnt)
    }

    pub fn on_refresh(&mut self, c: &mut Context) -> Result<()> {
        self.w.on_refresh(c)?;
        self.stsline.on_refresh(c)?;
        match &mut self.inner {
            Inner::Regular => (),
            Inner::Command { w: _w, cmd: _cmd } => {
                // self.cmd.on_refresh(c)?;
                // w.on_refresh(c)?;
                todo!()
            }
        }
        self.tbcline.on_refresh(c)
    }
}
