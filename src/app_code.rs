use crate::{
    event::Event,
    state::State,
    window::{Coord, Cursor, Window},
    window_file::WindowFile,
    Result,
};

pub enum Code {
    Buffer { w: Window },
}

impl Code {
    pub fn new(coord: Coord) -> Code {
        let w = Window::WF(WindowFile::new(coord));
        Code::Buffer { w }
    }

    pub fn to_cursor(&self) -> Cursor {
        match self {
            Code::Buffer { w } => w.to_cursor(),
        }
    }
}

impl Code {
    pub fn on_event(&mut self, s: &mut State, evnt: Event) -> Result<Event> {
        match self {
            Code::Buffer { w } => w.on_event(s, evnt),
        }
    }

    pub fn on_refresh(&mut self, s: &mut State) -> Result<()> {
        match self {
            Code::Buffer { w } => w.on_refresh(s),
        }
    }

    //fn do_open_files(mut self, mut flocs: Vec<event::OpenFile>) -> Result<Self> {
    //    let inner = mem::replace(&mut self.inner, Default::default());
    //    self.inner = match inner {
    //        Inner::Usual { window } => {
    //            let mut fds = vec![];
    //            loop {
    //                let floc = flocs.remove(0);
    //                let pw: Result<WindowPrompt> = floc.clone().try_into();
    //                match pw {
    //                    Err(_) => {
    //                        fds.push(floc);
    //                        break Inner::Usual { window };
    //                    }
    //                    Ok(pw) => {
    //                        flocs.insert(0, floc);
    //                        break Inner::OpenFiles {
    //                            pw,
    //                            window,
    //                            flocs,
    //                            fds,
    //                        };
    //                    }
    //                }
    //            }
    //        }
    //        val @ Inner::OpenFiles { .. } => val,
    //        Inner::None => err_at!(Fatal, msg: format!("unreachable"))?,
    //    };

    //    Ok(self)
    //}
}
