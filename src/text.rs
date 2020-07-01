use std::io;

use crate::{Error, Result};

pub enum Encoded {
    Utf8(String),
}

impl From<String> for Encoded {
    fn from(s: String) -> Encoded {
        Encoded::Utf8(s)
    }
}

impl From<Encoded> for String {
    fn from(enc: Encoded) -> String {
        match enc {
            Encoded::Utf8(s) => s,
        }
    }
}

impl Encoded {
    pub fn from_reader<R>(mut r: R, fenc: &str) -> Result<Encoded>
    where
        R: io::Read,
    {
        use std::str::from_utf8;

        let mut buf = vec![];
        err_at!(IOError, r.read_to_end(&mut buf))?;

        match fenc {
            "utf8" | "utf-8" => {
                let buf = err_at!(FailConvert, from_utf8(&buf))?;
                Ok(Encoded::Utf8(buf.to_string()))
            }
            _ => err_at!(Err(Error::Invalid(format!("encoding {}", fenc)))),
        }
    }

    pub fn save<W>(&self, mut w: W, _: &str) -> Result<()>
    where
        W: io::Write,
    {
        match self {
            Encoded::Utf8(s) => err_at!(IOError, w.write_all(s.as_bytes())),
        }
    }
}
