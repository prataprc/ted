use log::{info, warn};
use serde_derive::Deserialize;

use std::{convert::TryInto, ffi, path};

use crate::Result;

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

struct ConfigFile(ffi::OsString);

impl<'a> From<&'a str> for ConfigFile {
    fn from(stem: &str) -> ConfigFile {
        ConfigFile(format!("{}.toml", stem).into())
    }
}

pub fn load(app: &str, ftype: &str) -> Result<Config> {
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
