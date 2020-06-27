use toml;

use std::{convert::TryFrom, convert::TryInto, ffi, fmt, fs, result, str::FromStr};

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

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(
            f,
            concat!(
                "{{ scroll_off = {}, line_number = {}, wrap = {}, ",
                "left_margin_char = {:?}, top_margin_char = {:?} }}"
            ),
            self.scroll_off,
            self.line_number,
            self.wrap,
            self.left_margin_char,
            self.top_margin_char
        )?;
        write!(f, concat!("{{ read_only = {} }}"), self.read_only)
    }
}
