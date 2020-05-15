use crate::{
    event::Event,
    state::State,
    window::{Coord, Cursor, Window},
    window_file::WindowFile,
    Result,
};

pub enum WindowCode {
    Buffer { w: Window },
}

impl WindowCode {
    pub fn new(coord: Coord) -> WindowCode {
        let w = Window::File(Box::new(WindowFile::new(coord)));
        WindowCode::Buffer { w }
    }
}

impl WindowCode {
    pub fn on_event(&mut self, s: &mut State, evnt: Event) -> Result<Event> {
        match self {
            WindowCode::Buffer { w } => w.on_event(s, evnt),
        }
    }

    pub fn on_refresh(&mut self, s: &mut State) -> Result<()> {
        match self {
            WindowCode::Buffer { w } => w.on_refresh(s),
        }
    }

    pub fn to_cursor(&self) -> Cursor {
        match self {
            WindowCode::Buffer { w } => w.to_cursor(),
        }
    }
}
