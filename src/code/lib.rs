//mod config;
//mod state;
//
//pub use config::Config;
//pub use state::State;
//
//mod ftype;
//mod ftype_txt_en;
//
//mod keymap;
//mod keymap_code;
//
//mod cmd;
//mod cmd_edit;
//mod cmd_file;
//mod cmd_write;

use crate::{buffer::Buffer, code::config::Config, state::PubSub};

struct App {
    config: Config,
    subscribers: PubSub,
    buffers: Vec<Buffer>,
}

impl AsRef<Config> for App {
    fn as_ref(&self) -> &Config {
        &self.config
    }
}

impl App {
    pub fn new(config: toml::Value, subscribers: PubSub) -> App {
        App {
            config,
            subscribers,
            buffers: Default::default(),
        }
    }

    pub fn subscribe(&mut self, topic: &str, tx: mpsc::Sender<Message>) {
        self.subscribers.subscribe(topic, tx);
    }

    pub fn notify(&self, topic: &str, msg: Notify) -> Result<()> {
        self.subscribers.notify(topic, msg)
    }

    pub fn add_buffer(&mut self, buffer: Buffer) {
        self.buffers.insert(0, buffer)
    }

    pub fn take_buffer(&mut self, id: &str) -> Option<Buffer> {
        let i = {
            let mut iter = self.buffers.iter().enumerate();
            loop {
                match iter.next() {
                    Some((i, b)) if b.to_id() == id => break Some(i),
                    None => break None,
                    _ => (),
                }
            }
        };
        match i {
            Some(i) => Some(self.buffers.remove(i)),
            None => None,
        }
    }
}

impl App {
    pub fn as_buffer(&self, id: &str) -> &Buffer {
        for b in self.buffers.iter() {
            if b.to_id() == id {
                return b;
            }
        }
        unreachable!()
    }

    pub fn as_mut_buffer(&mut self, id: &str) -> &mut Buffer {
        for b in self.buffers.iter_mut() {
            if b.to_id() == id {
                return b;
            }
        }
        unreachable!()
    }
}
