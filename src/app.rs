//! Application specific traits and functions. For example `code` is a
//! ted-application.

use std::sync::mpsc;

use crate::{event::Event, pubsub::Notify, window::Cursor, Result};

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
