use log::{info, warn};
use serde_derive::Deserialize;
use toml;

use std::{convert::TryFrom, convert::TryInto, ffi, fs, path, str::FromStr};

use crate::{Error, Result};

macro_rules! config {
    ($(($field:ident, $t:ty, $val:expr)),*) => (
        #[derive(Clone, Debug)]
        pub struct Config {
            $(pub $field: $t,)*
        }

        #[derive(Clone, Debug, Deserialize)]
        struct ConfigToml {
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
            fn mixin(mut self, other: ConfigToml) -> Config {
                $(
                    if let Some(value) = other.$field {
                        self.$field = value
                    }
                )*
                self
            }
        }
    );
}

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

struct ConfigFile(ffi::OsString);

impl<'a> From<&'a str> for ConfigFile {
    fn from(stem: &str) -> ConfigFile {
        ConfigFile(format!("{}.toml", stem).into())
    }
}

pub fn load_config(app: &str, ftype: &str) -> Result<Config> {
    let files = match dirs::home_dir() {
        Some(home) => {
            let home = home.clone().into_os_string();
            let file1: path::PathBuf = {
                let ted_toml: ConfigFile = ".ted".into();
                [home.clone(), ted_toml.0].iter().collect()
            };
            let file2: path::PathBuf = {
                let app_toml: ConfigFile = app.into();
                [home.clone(), ".ted".into(), "apps".into(), app_toml.0]
                    .iter()
                    .collect()
            };
            let file3: path::PathBuf = {
                let ftype_toml: ConfigFile = ftype.into();
                [home.clone(), ".ted".into(), "ftype".into(), ftype_toml.0]
                    .iter()
                    .collect()
            };
            vec![file1, file2, file3]
        }
        None => vec![],
    };

    let mut config: Config = Default::default();
    for fname in files.into_iter() {
        if path::Path::new(&fname).exists() {
            let ctml: ConfigToml = fname.clone().into_os_string().try_into()?;
            config = config.mixin(ctml);
            info!("loading configuration file {:?}", fname);
        } else {
            warn!("try adding config file {:?}", fname);
        }
    }

    Ok(config)
}
