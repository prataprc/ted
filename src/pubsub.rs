use std::{cmp, fmt, result, sync::mpsc};

use crate::{term::Span, Error, Result};

/// Publisher-Subscriber type. Different applications and components can
/// subscribe to one or more `topic` with an IPC channel. And communicated with
/// each other by publishing messages on topics.
#[derive(Clone, Default)]
pub struct PubSub {
    topics: Vec<Subscriber>,
}

#[derive(Clone)]
struct Subscriber {
    topic: String,
    chans: Vec<mpsc::Sender<Notify>>,
}

impl Eq for Subscriber {}

impl PartialEq for Subscriber {
    fn eq(&self, other: &Self) -> bool {
        self.topic.eq(&other.topic)
    }
}

impl PartialOrd for Subscriber {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.topic.partial_cmp(&other.topic)
    }
}

impl Ord for Subscriber {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.topic.cmp(&other.topic)
    }
}

impl PubSub {
    fn find_topic(tp: &str, subs: &[Subscriber]) -> Option<usize> {
        match subs.len() {
            0 => None,
            1 if subs[0].topic == tp => Some(0),
            1 => None,
            m if tp < &subs[m / 2].topic => Self::find_topic(tp, &subs[..m / 2]),
            m => Self::find_topic(tp, &subs[m / 2..]).map(|n| (m / 2) + n),
        }
    }
}

impl PubSub {
    /// Subscribe a sender channel to `topic`.
    pub fn subscribe(&mut self, topic: &str, chan: mpsc::Sender<Notify>) {
        match Self::find_topic(topic, &self.topics) {
            Some(off) => self.topics[off].chans.push(chan),
            None => self.topics.push(Subscriber {
                topic: topic.to_string(),
                chans: vec![chan],
            }),
        };
        self.topics.sort();
    }

    /// Notify all subscribers to `topic` with notification `msg`.
    pub fn notify(&self, topic: &str, msg: Notify) -> Result<()> {
        match Self::find_topic(&topic, &self.topics) {
            Some(off) => {
                assert!(self.topics[off].topic == topic, "assert {}", topic);
                for chan in self.topics[off].chans.iter() {
                    err_at!(IPC, chan.send(msg.clone()))?;
                }
                Ok(())
            }
            None => Err(Error::NoTopic),
        }
    }

    /// Return a list of all topics and subscribers, this is useful to
    /// move around the pub-sub topics within the ted-applications.
    pub fn to_subscribers(&self) -> Vec<(String, Vec<mpsc::Sender<Notify>>)> {
        self.topics
            .iter()
            .map(|s| (s.topic.clone(), s.chans.clone()))
            .collect()
    }
}

/// Notification messages for `PubSub` topics.
#[derive(Clone)]
pub enum Notify {
    Status(Vec<Span>), // TODO: rename this to StatusCursor
    None,
}

impl Eq for Notify {}

impl PartialEq for Notify {
    fn eq(&self, other: &Self) -> bool {
        use Notify::Status;

        match (self, other) {
            (Status(_), Status(_)) => true,
            (Notify::None, Notify::None) => true,
            (_, _) => false,
        }
    }
}

impl fmt::Display for Notify {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        match self {
            Notify::Status(_) => write!(f, "status"),
            Notify::None => write!(f, "none"),
        }
    }
}
