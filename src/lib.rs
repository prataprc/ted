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
#[macro_use]
pub mod term;

pub mod state;

pub mod buffer;
mod col_nu;
pub mod colors;
pub mod event;
pub mod location;
mod mark;
pub mod pubsub;
pub mod tabc;
pub mod tss;
mod view;

pub mod app;
mod code;

mod syntax;
mod text;

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
#[derive(Clone)]
pub enum Error {
    Fatal(String, String),
    BadPattern(String, String),
    IOError(String, String),
    IPC(String, String),
    NoTopic(String),
    Invalid(String, String),
    FailConvert(String, String),
    FailParse(String, String),
    FailBuffer(String, String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        use Error::{BadPattern, Fatal, IOError, Invalid, NoTopic, IPC};
        use Error::{FailBuffer, FailConvert, FailParse};

        match self {
            Fatal(p, msg) => write!(f, "{} Fatal: {}", p, msg),
            BadPattern(p, msg) => write!(f, "{} BadPattern: {}", p, msg),
            IOError(p, msg) => write!(f, "{} IOError: {}", p, msg),
            IPC(p, msg) => write!(f, "{} IPC: {}", p, msg),
            NoTopic(p) => write!(f, "{} NoTopic", p),
            Invalid(p, msg) => write!(f, "{} Invalid: {}", p, msg),
            FailConvert(p, msg) => write!(f, "{} FailConvert: {}", p, msg),
            FailParse(p, msg) => write!(f, "{} FailParse: {}", p, msg),
            FailBuffer(p, msg) => write!(f, "{} FailBuffer: {}", p, msg),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}

impl Error {
    fn to_loc(&self) -> String {
        use Error::{BadPattern, Fatal, IOError, Invalid, NoTopic, IPC};
        use Error::{FailBuffer, FailConvert, FailParse};

        match self {
            Fatal(p, _) => p.clone(),
            BadPattern(p, _) => p.clone(),
            IOError(p, _) => p.clone(),
            IPC(p, _) => p.clone(),
            NoTopic(p) => p.clone(),
            Invalid(p, _) => p.clone(),
            FailConvert(p, _) => p.clone(),
            FailParse(p, _) => p.clone(),
            FailBuffer(p, _) => p.clone(),
        }
    }

    fn to_error(&self) -> String {
        use Error::{BadPattern, Fatal, IOError, Invalid, NoTopic, IPC};
        use Error::{FailBuffer, FailConvert, FailParse};

        match self {
            Fatal(_, msg) => msg.clone(),
            BadPattern(_, msg) => msg.clone(),
            IOError(_, msg) => msg.clone(),
            IPC(_, msg) => msg.clone(),
            NoTopic(_) => "".to_string().clone(),
            Invalid(_, msg) => msg.clone(),
            FailConvert(_, msg) => msg.clone(),
            FailParse(_, msg) => msg.clone(),
            FailBuffer(_, msg) => msg.clone(),
        }
    }
}
