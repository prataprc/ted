use crossterm;

use std::{fmt, ops::Add, result};

use crate::{
    buffer::{self, Buffer},
    event::Event,
    event::DP,
    term::Spanline,
    Result,
};

#[macro_export]
macro_rules! cursor {
    ($col:expr, $row:expr) => {
        Cursor {
            col: $col,
            row: $row,
        }
    };
}

/// Window trait for all screen areas defined by ted applications.
pub trait Window {
    type App;

    fn to_cursor(&self) -> Cursor;

    fn on_event(&mut self, app: &mut Self::App, evnt: Event) -> Result<Event>;

    fn on_refresh(&mut self, app: &mut Self::App) -> Result<()>;
}

/// This is a simple abstraction trait for [buffer::Buffer]. Gives an idea
/// on window's api dependency with `Buffer`.
pub trait WinBuffer<'a> {
    type IterLine: Iterator<Item = &'a str>;
    type IterChar: Iterator<Item = char>;

    /// Return the cursor position, as (col, row) starting from (0,), within
    /// this buffer.
    fn to_xy_cursor(&self) -> buffer::Cursor;

    /// Return an iterator starting from line_idx. `dp` can either be
    /// [DP::Right] or [DP::Left] for either forward iteration or reverse
    /// iteration. In the forward direction, iteration will start from
    /// the cursor's current line. In reverse direction, iteration will start
    /// from the one before cursor's current line. Note that,
    /// `0 <= line_idx < n_lines`.
    fn lines_at(&'a self, line_idx: usize, dp: DP) -> Result<Self::IterLine>;

    /// Return an iterator starting from char_idx. `dp` can either be
    /// [DP::Right] or [DP::Left] for either forward iteration or reverse
    /// iteration. In the forward direction, iteration will start from
    /// the cursor position. In reverse direction, iteration will start
    /// from the one before cursor position. Note that,
    /// `0 <= char_idx < n_chars`.
    fn chars_at(&'a self, char_idx: usize, dp: DP) -> Result<Self::IterChar>;

    /// Return the character offset of first character for the requested
    /// `line_idx`. Note that, `0 <= line_idx < n_lines`.
    fn line_to_char(&self, line_idx: usize) -> usize;

    /// Return the line offset for requested `char_idx`, which must be a valid
    /// character offset within the buffer. [Buffer::to_cursor] is a `char_idx`.
    /// Note that, `0 <= char_idx < n_chars`.
    fn char_to_line(&self, char_idx: usize) -> usize;

    /// Return the number of characters in the buffer.
    fn n_chars(&self) -> usize;

    /// Return the number of lines in the buffer.
    fn n_lines(&self) -> usize;

    /// Return the number of characters in line `line_idx`, starts from ZERO.
    fn len_line(&self, line_idx: usize) -> usize;

    /// Return whether the last character in buffer is NEWLINE.
    fn is_trailing_newline(&self) -> bool;
}

/// Render trait for window objects.
pub trait Render {
    fn to_span_line(&self, buf: &Buffer, from: usize, to: usize) -> Result<Spanline>;
}

// Terminal coordinates, describes the four corners of a window.
// Origin is at (1, 1).
#[derive(Clone, Copy, Debug)]
pub struct Coord {
    pub col: u16,
    pub row: u16,
    pub hgt: u16,
    pub wth: u16,
}

impl Default for Coord {
    fn default() -> Coord {
        Coord {
            col: 1,
            row: 1,
            hgt: 0,
            wth: 0,
        }
    }
}

impl Coord {
use lazy_static::lazy_static;
use log::{info, warn};
use toml;

use std::{
    convert::{TryFrom, TryInto},
    ffi, fs, path,
};

use crate::{colors::ColorScheme, Error, Result};

/// Single invocation of this macros, accepts an array of configuration params
/// shall introduce the following artifacts:
///
/// a. Define a `Config` type in the invoking module.
/// b. Derive serde::Serialize for `Config` type.
/// c. Create a shadow type `ConfigToml` for serde Deserialize and mixins.
/// d. Implement `Default` trait for `Config` type.
/// c. Implement `FromStr`, TryFrom<ffi::OsString>, TryFrom<&str>, TryFrom<&[u8]>
///    TryFrom<toml::Value> traits for `ConfigToml` that make deserialization
///    of config from common types like - string, file, bytes and toml.
///
macro_rules! config {
    ($(($field:ident, $t:ty, $val:expr)),*) => (
        use serde_derive::{Serialize, Deserialize};

        /// Generated using config![] macro. All configuration fields are public
        /// and macros automatically implements serialization, deserialization,
        /// default and mixin functionalities. Refer [config] macro for
        /// more detail.
        #[derive(Clone, Debug, Serialize)]
        pub struct Config {
            $(pub $field: $t,)*
        }

        /// Generated using config![] macro. Shadow type for [Config] type.
        /// Refer [config] macros for more detail.
        #[derive(Clone, Debug, Deserialize)]
        pub struct ConfigToml {
            $(pub $field: Option<$t>,)*
        }

        impl Default for Config {
            fn default() -> Self {
                Config {
                    $($field: $val,)*
                }
            }
        }

        impl Config {
            pub fn mixin(mut self, other: ConfigToml) -> Config {
                $(
                    if let Some(value) = other.$field {
                        self.$field = value
                    }
                )*
                self
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

        impl TryFrom<toml::Value> for ConfigToml {
            type Error = Error;

            fn try_from(value: toml::Value) -> Result<Self> {
                let ctml: ConfigToml = err_at!(FailConvert, value.to_string().parse())?;
                Ok(ctml)
            }
        }
    );
}

struct ConfigFile(ffi::OsString);

impl From<String> for ConfigFile {
    fn from(stem: String) -> ConfigFile {
        ConfigFile(format!("{}.toml", stem).into())
    }
}

impl From<ConfigFile> for ffi::OsString {
    fn from(cf: ConfigFile) -> ffi::OsString {
        cf.0
    }
}

impl From<ConfigFile> for path::PathBuf {
    fn from(cf: ConfigFile) -> path::PathBuf {
        cf.0.into()
    }
}

impl TryFrom<ConfigFile> for toml::Value {
    type Error = Error;

    fn try_from(cf: ConfigFile) -> Result<Self> {
        use std::str::from_utf8;

        let bytes = err_at!(IOError, fs::read(cf.0))?;
        let s = err_at!(FailConvert, from_utf8(&bytes))?;
        err_at!(FailConvert, s.parse())
    }
}

lazy_static! {
    static ref TOML_FILES: Vec<path::PathBuf> = {
        let mut toml_files = vec![];
        dirs::home_dir().map(|home| {
            let home = home.clone().into_os_string();
            let cf: ConfigFile = ".ted".to_string().into();
            toml_files.push([home.clone(), cf.into()].iter().collect());
        });
        toml_files
    };
    static ref FTYPES_DIRS: Vec<path::PathBuf> = {
        let mut ftype_dirs = vec![];
        dirs::home_dir().map(|home| {
            let home = home.clone().into_os_string();
            ftype_dirs.push(
                [home.clone(), ".ted".into(), "ftypes".into()]
                    .iter()
                    .collect(),
            );
        });
        ftype_dirs
    };
    static ref COLORS_DIRS: Vec<path::PathBuf> = {
        let mut colors_dirs = vec![];
        dirs::home_dir().map(|home| {
            let home = home.clone().into_os_string();
            colors_dirs.push(
                [home.clone(), ".ted".into(), "colors".into()]
                    .iter()
                    .collect(),
            );
        });
        colors_dirs
    };
}

/// Read ted configuration from:
///
/// * default configuration.
/// * ~/.ted.toml
/// * ~/.ted/<ftypes>/<ftype>.toml
/// * command line argument.
///
pub fn read_config(toml_file: Option<String>, ftype: Option<String>) -> Result<toml::Value> {
    let mut files: Vec<path::PathBuf> = TOML_FILES.clone();
    if let Some(ftype) = ftype {
        for ftypes_dir in FTYPES_DIRS.clone().into_iter() {
            let cf: ConfigFile = ftype.clone().into();
            files.push([ftypes_dir, cf.into()].iter().collect());
        }
    }

    if let Some(toml_file) = toml_file {
        let toml_file: ffi::OsString = toml_file.into();
        files.push(err_at!(IOError, fs::canonicalize(&toml_file))?);
    }

    let mut config: toml::map::Map<String, toml::Value> = Default::default();
    for fl in files.into_iter() {
        if !path::Path::new(&fl).exists() {
            warn!("fail reading config from {:?}", fl);
            continue;
        }

        let conf: toml::Value = {
            let cf = ConfigFile(fl.clone().into_os_string());
            cf.try_into()?
        };
        match conf.as_table() {
            Some(table) => config.extend(table.clone().into_iter()),
            None => warn!("config file {:?} not valid", fl),
        };
        info!("load configuration from {:?}", fl);
    }

    Ok(toml::Value::Table(config))
}

/// Read ted color-schemes from:
///
/// * ~/.ted/colors/<scheme>.toml
///
pub fn read_color_schemes() -> Result<Vec<ColorScheme>> {
    use std::str::from_utf8;

    let mut schemes = vec![];
    for colors_dir in COLORS_DIRS.clone().into_iter() {
        let entries: Vec<fs::DirEntry> = {
            let de = err_at!(IOError, fs::read_dir(&colors_dir))?;
            de.filter_map(|de| de.ok()).collect()
        };
        for entry in entries.into_iter() {
            let fp: path::PathBuf = {
                let parts = [entry.path(), entry.file_name().into()];
                parts.iter().collect()
            };
            let value: Option<toml::Value> = {
                let bytes = err_at!(IOError, fs::read(&fp))?;
                match from_utf8(&bytes) {
                    Ok(s) => s.parse().ok(),
                    Err(_) => {
                        warn!("not a valid color-scheme {:?}", fp);
                        None
                    }
                }
            };
            value.map(|val| {
                let cs: Option<ColorScheme> = TryFrom::try_from(val).ok();
                cs.map(|cs| schemes.push(cs))
            });
        }
    }

    Ok(schemes)
}

/// Extract configuration for specified section. Section can be
/// dot-separated.
pub fn to_section(mut config: toml::Value, section: &str) -> toml::Value {
    for sec in section.split(".") {
        config = match config.get(sec) {
            Some(value) => value.clone(),
            None => toml::Value::Table(Default::default()),
        };
    }
    config
}
    /// Create a new viewport for window.
    pub fn new(col: u16, row: u16, hgt: u16, wth: u16) -> Coord {
        Coord { col, row, hgt, wth }
    }

    /// Move the window viewport by `col_off` and `row_off`.
    #[inline]
    pub fn move_by(mut self, col_off: i16, row_off: i16) -> Self {
        self.col = ((self.col as i16) + col_off) as u16;
        self.row = ((self.row as i16) + row_off) as u16;
        self
    }

    /// Resize the window viewport by `height` and `width`.
    #[inline]
    pub fn resize_to(mut self, height: u16, width: u16) -> Self {
        self.hgt = height;
        self.wth = width;
        self
    }

    /// Return the origin point, top-left of the viewport. Position starts
    /// from (1, 1).
    #[inline]
    pub fn to_origin(&self) -> (u16, u16) {
        (self.col, self.row)
    }

    /// Return the origin point in cursor parlance, position starts from
    /// (0, 0)
    #[inline]
    pub fn to_origin_cursor(&self) -> (u16, u16) {
        (self.col.saturating_sub(1), self.row.saturating_sub(1))
    }

    /// Return the origin point as window [Cursor] object.
    #[inline]
    pub fn to_top_left(&self) -> Cursor {
        let (col, row) = self.to_origin_cursor();
        cursor!(col, row)
    }

    /// Return the height and width of the viewport. Height and width counting
    /// starts from 1, similar to len().
    #[inline]
    pub fn to_size(&self) -> (u16, u16) {
        (self.hgt, self.wth)
    }
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(
            f,
            "Coord<{},{},{},{}>",
            self.col, self.row, self.hgt, self.wth
        )
    }
}

// Cursor within the Window object, starts from (0, 0)
#[derive(Clone, Copy, Default, Debug, Eq, PartialEq)]
pub struct Cursor {
    pub col: u16,
    pub row: u16,
}

impl From<(u16, u16)> for Cursor {
    fn from((col, row): (u16, u16)) -> Cursor {
        Cursor { col, row }
    }
}

impl fmt::Display for Cursor {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "Cursor<{},{}>", self.col, self.row)
    }
}

impl Add for Cursor {
    type Output = Cursor;

    fn add(self, rhs: Cursor) -> Cursor {
        cursor!(self.col + rhs.col, self.row + rhs.row)
    }
}

impl From<Cursor> for crossterm::cursor::MoveTo {
    fn from(cursor: Cursor) -> crossterm::cursor::MoveTo {
        let Cursor { col, row } = cursor;
        crossterm::cursor::MoveTo(col, row)
    }
}

impl Cursor {
    pub fn new(col: u16, row: u16) -> Cursor {
        Cursor { col, row }
    }

    pub fn move_by(mut self, col: i16, row: i16) -> Self {
        self.col = ((self.col as i16) + col) as u16;
        self.row = ((self.row as i16) + row) as u16;
        self
    }

    pub fn discount_nu(mut self, nu_wth: u16) -> Self {
        self.col -= nu_wth;
        self
    }

    pub fn account_nu(mut self, nu_wth: u16) -> Self {
        self.col += nu_wth;
        self
    }
}
