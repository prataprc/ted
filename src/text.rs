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
            _ => {
                let s = format!("encoding {}", fenc);
                err_at!(Err(Error::Invalid(String::new(), s)))
            }
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

pub enum Format {
    Dos,
    Mac,
    Unix,
}

impl Default for Format {
    fn default() -> Format {
        Format::Unix
    }
}

impl Format {
    pub fn newline(&self) -> &'static str {
        match self {
            Format::Dos => "\r\n",
            Format::Mac => "\r",
            Format::Unix => "\n",
        }
    }

    // return trimed string, and number of bytes trimmed at the end.
    pub fn trim_newline(text: &str) -> (&str, usize) {
        use std::{slice::from_raw_parts, str::from_utf8_unchecked};

        let mut chars = text.chars().rev();
        let n = match (chars.next(), chars.next()) {
            (Some('\n'), Some('\r')) => 2,
            (Some('\n'), None) | (Some('\n'), Some(_)) => 1,
            (Some('\r'), None) | (Some('\r'), Some(_)) => 1,
            (_, _) => 0,
        };
        let len = text.len().saturating_sub(n);
        (
            unsafe { from_utf8_unchecked(from_raw_parts(text.as_ptr(), len)) },
            n,
        )
    }
}

#[inline]
pub fn visual_line(text: &str) -> &str {
    Format::trim_newline(text).0
}

#[inline]
pub fn visual_line_n(text: &str) -> usize {
    Format::trim_newline(text).0.chars().count()
}
