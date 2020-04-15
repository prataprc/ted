use std::{result, fmt};

#[macro_use]
mod util;
pub mod view_port;

pub enum Error {
    Fatal(String)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        match self {
            Error::Fatal(msg) => write!(f, "Fatal:{}", msg),
        }
    }
}
