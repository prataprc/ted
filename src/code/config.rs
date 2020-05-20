use serde_derive::Deserialize;
use toml;

use std::{convert::TryFrom, convert::TryInto, ffi, fs, str::FromStr};

use crate::{Error, Result};

config![
    (scroll_off, u16, 0),
    (line_number, bool, true),
    (wrap, bool, true),
    (left_margin_char, char, '|'),
    (top_margin_char, char, '-')
];

impl TryFrom<toml::Value> for ConfigToml {
    type Error = Error;

    fn try_from(value: toml::Value) -> Result<Self> {
        let ctml: ConfigToml = err_at!(FailConvert, value.to_string().parse())?;
        Ok(ctml)
    }
}

impl FromStr for ConfigToml {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let ctml: ConfigToml = err_at!(FailConvert, s.parse())?;
        Ok(ctml)
    }
}

impl<'a> TryFrom<&'a [u8]> for ConfigToml {
    type Error = Error;

    fn try_from(toml_bin: &[u8]) -> Result<Self> {
        use std::str::from_utf8;

        let s = err_at!(FailConvert, from_utf8(toml_bin))?;
        err_at!(FailConvert, s.parse())
    }
}

impl TryFrom<ffi::OsString> for ConfigToml {
    type Error = Error;

    fn try_from(fname: ffi::OsString) -> Result<Self> {
        err_at!(IOError, fs::read(&fname))?.as_slice().try_into()
    }
}
