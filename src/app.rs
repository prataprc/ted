//! Application specific traits and functions. For example `code` is a
//! ted-application.

use std::sync::mpsc;

use crate::{code, event::Event, pubsub::Notify, window::Cursor, Result};

pub trait Application {
    /// Subscribe a channel for a topic. Any number of components can
    /// subscribe to the same topic. Refer [pubsub::PubSub] for more detail.
    fn subscribe(&mut self, topic: &str, tx: mpsc::Sender<Notify>);

    /// Notify all subscribers for `topic` with `msg`. Refer [pubsub::PubSub]
    /// for more detail.
    fn notify(&self, topic: &str, msg: Notify) -> Result<()>;

    /// Handle event. Refer [event::Event] for details.
    fn on_event(&mut self, evnt: Event) -> Result<Event>;

    /// Refresh terminal window, application is responsible for its view-port,
    /// typically configured using [window::Coord] when the application was
    /// created.
    fn on_refresh(&mut self) -> Result<()>;

    /// Return the cursor within application's view-port.
    fn to_cursor(&self) -> Cursor;
}

pub enum App {
    Code(code::Code),
    None,
}

impl Default for App {
    fn default() -> App {
        App::None
    }
}

impl App {
    pub fn subscribe(&mut self, topic: &str, tx: mpsc::Sender<Notify>) {
        use App::Code;

        match self {
            Code(app) => app.subscribe(topic, tx),
            App::None => (),
        }
    }

    pub fn notify(&self, topic: &str, msg: Notify) -> Result<()> {
        use App::Code;

        match self {
            Code(app) => app.notify(topic, msg),
            App::None => Ok(()),
        }
    }

    pub fn to_cursor(&self) -> Cursor {
        use App::Code;

        match self {
            Code(app) => app.to_cursor(),
            App::None => Default::default(),
        }
    }

    pub fn on_event(&mut self, evnt: Event) -> Result<Event> {
        use App::Code;

        match self {
            Code(app) => app.on_event(evnt),
            App::None => Ok(evnt),
        }
    }

    pub fn on_refresh(&mut self) -> Result<()> {
        use App::Code;

        match self {
            Code(app) => app.on_refresh(),
            App::None => Ok(()),
        }
    }
}
