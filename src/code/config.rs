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
