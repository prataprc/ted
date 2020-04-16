use std::{fmt, result};

#[macro_use]
pub mod util;
pub mod buffer;
pub mod config;
pub mod event;
pub mod file_window;
pub mod window;

pub use buffer::Buffer;
pub use config::Config;
pub use event::Event;
pub use window::Window;

pub type Result<T> = result::Result<T, Error>;

pub enum Error {
    Fatal(String),
    FailBuffer(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        match self {
            Error::Fatal(msg) => write!(f, "Fatal: {}", msg),
            Error::FailBuffer(msg) => write!(f, "FailBuffer: {}", msg),
        }
    }
}
