use std::sync::mpsc;

use crate::{
    event::Event,
    window::{Cursor, Notify},
    Result,
};

pub trait Application {
    fn subscribe(&mut self, topic: &str, tx: mpsc::Sender<Notify>);

    fn notify(&self, topic: &str, msg: Notify) -> Result<()>;

    fn to_cursor(&self) -> Cursor;

    fn on_event(&mut self, evnt: Event) -> Result<Event>;

    fn on_refresh(&mut self) -> Result<()>;
}
