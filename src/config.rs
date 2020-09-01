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
        /// default and mixin functionalities. Refer [crate::config] macro for
        /// more detail.
        #[derive(Clone, Debug, Serialize)]
        pub struct Config {
            $(pub $field: $t,)*
        }

        /// Generated using config![] macro. Shadow type for [Config] type.
        /// Refer [crate::config] macros for more detail.
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
        let entries: Vec<fs::DirEntry> = match fs::read_dir(&colors_dir) {
            Ok(de) => de.filter_map(|de| de.ok()).collect(),
            Err(err) => {
                warn!("colors dir {:?} : {}", colors_dir, err);
                vec![]
            }
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
