use log::{info, warn};
use toml;

use std::{
    convert::{TryFrom, TryInto},
    ffi, fs, path,
};

use crate::{Error, Result};

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

impl<'a> From<&'a str> for ConfigFile {
    fn from(stem: &str) -> ConfigFile {
        ConfigFile(format!("{}.toml", stem).into())
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

/// Read ted configuration from:
///
/// * default configuration.
/// * ~/.ted.toml
/// * ~/.ted/<ftype>/<ftype>.toml
/// * command line argument.
///
pub fn read_config(toml_file: &str, ftype: Option<&str>) -> Result<toml::Value> {
    let mut files: Vec<path::PathBuf> = vec![];
    match dirs::home_dir() {
        Some(home) => {
            let home = home.clone().into_os_string();
            // ~/.ted.toml
            files.push({
                let ted_toml: ConfigFile = ".ted".into();
                [home.clone(), ted_toml.0].iter().collect()
            });
            // ~/.ted/ftype/<ftype>.toml
            if let Some(ftype) = ftype {
                let ftype_toml: ConfigFile = ftype.into();
                files.push({
                    [home.clone(), ".ted".into(), "ftype".into(), ftype_toml.0]
                        .iter()
                        .collect()
                });
            }
        }
        None => (),
    }

    if toml_file.len() > 0 {
        files.push({
            let toml_file: ffi::OsString = toml_file.clone().into();
            err_at!(IOError, fs::canonicalize(&toml_file))?
        });
    }

    let mut config: toml::map::Map<String, toml::Value> = Default::default();
    for fname in files.into_iter() {
        if !path::Path::new(&fname).exists() {
            warn!("fail reading config from {:?}", fname);
            continue;
        }

        let conf: toml::Value = {
            let cf = ConfigFile(fname.clone().into_os_string());
            cf.try_into()?
        };
        match conf.as_table() {
            Some(table) => config.extend(table.clone().into_iter()),
            None => warn!("config file {:?} not valid", fname),
        };
        info!("load configuration from {:?}", fname);
    }

    Ok(toml::Value::Table(config))
}

/// Extract application configuration from `ted.toml`.
pub fn to_app_config(config: &toml::Value, app: &str) -> toml::Value {
    match config.get(app) {
        Some(value) => value.clone(),
        None => toml::Value::Table(Default::default()),
    }
}
