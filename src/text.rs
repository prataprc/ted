//! Module implement types and functions to handle text.

use unicode_width::UnicodeWidthChar;

use std::{convert::TryFrom, io};

use crate::{Error, Result};

/// Text encoding. This is required for serializing string from buffer
/// to disk/network or vice-versa.
pub enum Encoding {
    Utf8(String),
}

impl TryFrom<(String, String)> for Encoding {
    type Error = Error;

    fn try_from((s, enc): (String, String)) -> Result<Encoding> {
        match enc.as_str() {
            "utf-8" => Ok(Encoding::Utf8(s)),
            "utf8" => Ok(Encoding::Utf8(s)),
            enc => err_at!(Invalid, msg: format!("encoding `{}`", enc)),
        }
    }
}

impl From<Encoding> for String {
    fn from(enc: Encoding) -> String {
        match enc {
            Encoding::Utf8(s) => s,
        }
    }
}

impl Encoding {
    /// Read bytes from `r`, using file-encoding `fenc`. If successful,
    /// resulting `Encoding` value can be converted to String.
    pub fn from_reader<R>(mut r: R, fenc: &str) -> Result<Encoding>
    where
        R: io::Read,
    {
        use std::str::from_utf8;

        let mut buf = vec![];
        err_at!(IOError, r.read_to_end(&mut buf))?;

        match fenc {
            "utf8" | "utf-8" => {
                let buf = err_at!(FailConvert, from_utf8(&buf))?;
                Ok(Encoding::Utf8(buf.to_string()))
            }
            _ => {
                let s = format!("encoding {}", fenc);
                err_at!(Err(Error::Invalid(String::new(), s)))
            }
        }
    }

    /// Serialize string into specified encoding and save them to `w`.
    pub fn save<W>(&self, mut w: W) -> Result<()>
    where
        W: io::Write,
    {
        match self {
            Encoding::Utf8(s) => err_at!(IOError, w.write_all(s.as_bytes())),
        }
    }
}

/// Text format. Mostly to deal with new-line.
#[derive(Clone)]
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
    /// Return the new-line string for this text-format variant.
    pub fn newline(&self) -> &'static str {
        match self {
            Format::Dos => "\r\n",
            Format::Mac => "\r",
            Format::Unix => "\n",
        }
    }

    /// Return trimed string, and number of bytes trimmed at the end.
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

/// Return the visual-line as string-reference, ignoring the trailing
/// new line.
#[inline]
pub fn visual_line(text: &str) -> &str {
    Format::trim_newline(text).0
}

/// Return the number of characters in the visual-line.
#[inline]
pub fn visual_line_n(text: &str) -> usize {
    Format::trim_newline(text).0.chars().count()
}

/// Return the total visual-width of all characters emitted by `iter`.
#[inline]
pub fn width<I>(iter: I) -> usize
where
    I: Iterator<Item = char>,
{
    iter.filter_map(|ch| ch.width()).sum()
}

/// Take characters from `iter`, whose total visual-width does not exceed
/// `wth` visual-width.
pub fn take_width<I>(mut iter: I, wth: usize) -> std::vec::IntoIter<char>
where
    I: Iterator<Item = char>,
{
    let mut n = 0_usize;
    let mut chars = vec![];
    loop {
        match iter.next() {
            Some(ch) => {
                let w = ch.width().unwrap_or(0);
                if (n + w) > wth {
                    break chars.into_iter();
                } else {
                    n += w;
                    chars.push(ch);
                }
            }
            None => break chars.into_iter(),
        }
    }
}
