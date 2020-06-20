//! Package implement editing tool-kit for terminal based apps.

#![feature(box_syntax, box_patterns)]

use lazy_static::lazy_static;

use std::{fmt, result};

#[macro_use]
pub mod util;
#[macro_use]
pub mod config;
#[macro_use]
pub mod window;

pub mod state;
pub mod term;

pub mod buffer;
pub mod colors;
pub mod event;
pub mod location;
pub mod pubsub;
pub mod syntax;
pub mod tabc;
pub mod tss;

pub mod app;
mod code;

mod ftypes;

lazy_static! {
    /// Global collection of all pre-packaged color-schemes.
    static ref COLORS: Vec<colors::ColorScheme> = {
        let colors = colors::ColorScheme::load_color_schemes().unwrap();
        colors
    };
}

/// Result returned by all `Ted` API.
pub type Result<T> = result::Result<T, Error>;

/// Collection of all `Ted` errors.
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
