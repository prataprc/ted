use serde_derive::Deserialize;
use toml;

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
            fn do_mixin(mut self, other: ConfigToml) -> Config {
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

impl Config {
    pub fn mixin(self, toml_text: &str) -> Result<Self> {
        let cfg: ConfigToml = err_at!(FailConvert, toml::from_str(toml_text))?;
        Ok(self.do_mixin(cfg))
    }
}
