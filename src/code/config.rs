use toml;

use std::{convert::TryFrom, convert::TryInto, ffi, fs, str::FromStr};

use crate::{Error, Result};

config![
    // read-only will force all files to be opened in read-only mode.
    (read_only, bool, false),
    (scroll_off, u16, 0),
    (line_number, bool, true),
    (wrap, bool, true),
    (left_margin_char, char, '|'),
    (top_margin_char, char, '-'),
    (color_scheme, String, "default".to_string())
];
