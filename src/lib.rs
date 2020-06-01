//! Package implement editing tool-kit for terminal based apps.

#![feature(box_syntax, box_patterns)]
use std::{fmt, result};

#[macro_use]
pub mod util;
#[macro_use]
pub mod config;
pub mod color_scheme;
pub mod location;
mod tabc;
pub mod tss;

pub mod buffer;
pub mod event;
pub mod pubsub;
pub mod state;
#[macro_use]
pub mod window;

mod code;

pub type Result<T> = result::Result<T, Error>;

pub enum Error {
    Fatal(String),
    BadPattern(String),
    IOError(String),
    IPC(String),
    NoTopic,
    Invalid(String),
    FailConvert(String),
    FailParse(String),
    FailBuffer(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        match self {
            Error::Fatal(msg) => write!(f, "Fatal: {}", msg),
            Error::BadPattern(msg) => write!(f, "BadPattern: {}", msg),
            Error::IOError(msg) => write!(f, "IOError: {}", msg),
            Error::IPC(msg) => write!(f, "IPC: {}", msg),
            Error::NoTopic => write!(f, "NoTopic"),
            Error::Invalid(msg) => write!(f, "Invalid: {}", msg),
            Error::FailConvert(msg) => write!(f, "FailConvert: {}", msg),
            Error::FailParse(msg) => write!(f, "FailParse: {}", msg),
            Error::FailBuffer(msg) => write!(f, "FailBuffer: {}", msg),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}
