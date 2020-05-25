use std::{cmp, sync::mpsc};

use crate::{window::Notify, Error, Result};

#[derive(Clone, Default)]
pub struct PubSub {
    topics: Vec<Subscriber>,
}

#[derive(Clone)]
struct Subscriber {
    topic: String,
    tx: mpsc::Sender<Notify>,
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

    pub fn subscribe(&mut self, topic: &str, tx: mpsc::Sender<Notify>) {
        self.topics.push(Subscriber {
            topic: topic.to_string(),
            tx,
        });
        self.topics.sort();
    }

    pub fn notify(&self, topic: &str, msg: Notify) -> Result<()> {
        match Self::find_topic(&topic, &self.topics) {
            Some(n) => {
                assert!(self.topics[n].topic == topic, "assert {}", topic);
                err_at!(IPC, self.topics[n].tx.send(msg))?;
                Ok(())
            }
            None => Err(Error::NoTopic),
        }
    }

    pub fn to_subscribers(&self) -> Vec<(String, mpsc::Sender<Notify>)> {
        self.topics
            .iter()
            .map(|s| (s.topic.clone(), s.tx.clone()))
            .collect()
    }
}
