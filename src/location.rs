use dirs;
use lazy_static::lazy_static;

use std::{ffi, fmt, iter::FromIterator, path, result, sync::Mutex};

use crate::{Error, Result};

lazy_static! {
    static ref ANONYMOUS_COUNT: Mutex<usize> = Mutex::new(0);
}

// Location of buffer's content, typically a persistent medium.
#[derive(Clone)]
pub enum Location {
    Anonymous(String),
    Disk(ffi::OsString),
}

impl Location {
    pub fn new_anonymous() -> Location {
        let mut count = ANONYMOUS_COUNT.lock().unwrap();
        *count = *count + 1;
        Location::Anonymous(format!("anonymous-{}", count))
    }

    pub fn new_disk(loc: &ffi::OsStr) -> Location {
        match loc.to_os_string().into_string() {
            Ok(loc) => Location::Disk(Self::canonicalize(loc).into_os_string()),
            Err(loc) => Location::Disk(loc),
        }
    }

    pub fn to_short_string(&self) -> Result<String> {
        match self {
            Location::Anonymous(s) => Ok(s.clone()),
            Location::Disk(s) => Self::shrink_home(s),
        }
    }

    pub fn to_long_string(&self) -> Result<String> {
        match self {
            Location::Anonymous(s) => Ok(s.clone()),
            Location::Disk(s) => err_at!(
                FailConvert,
                s.to_os_string()
                    .into_string()
                    .map_err(|_| Error::FailConvert("".to_string()))
            ),
        }
    }

    fn shrink_home(s: &ffi::OsStr) -> Result<String> {
        let res = match dirs::home_dir() {
            Some(home) => {
                let pb: path::PathBuf = s.to_os_string().into();
                if pb.starts_with(&home) {
                    let mut shrnk = path::PathBuf::new();
                    shrnk.push(&home);
                    shrnk.push(err_at!(FailConvert, pb.strip_prefix(&home))?);
                    shrnk.into_os_string().into_string()
                } else {
                    pb.into_os_string().into_string()
                }
            }
            None => s.to_os_string().into_string(),
        };
        err_at!(
            FailConvert,
            res.map_err(|_| Error::FailConvert("".to_string()))
        )
    }

    fn canonicalize(loc: String) -> path::PathBuf {
        let home = dirs::home_dir();
        let mut pbuf = path::PathBuf::new();
        let (one, two) = {
            let mut chars = loc.chars();
            (chars.next(), chars.next())
        };
        match (one, two) {
            (Some('~'), Some(path::MAIN_SEPARATOR)) if home.is_some() => {
                pbuf.push(home.unwrap());
                pbuf.push(String::from_iter(
                    loc.chars().take(2).collect::<Vec<char>>(),
                ));
                pbuf
            }
            (Some('~'), Some(path::MAIN_SEPARATOR)) => loc.into(),
            _ => match pbuf.canonicalize() {
                Ok(pbuf) => pbuf,
                Err(_) => loc.into(),
            },
        }
    }
}

impl Default for Location {
    fn default() -> Location {
        Location::new_anonymous()
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        match self {
            Location::Anonymous(s) => write!(f, "{}", s),
            Location::Disk(s) => {
                let s = s.clone().into_string().unwrap();
                write!(f, "{}", s)
            }
        }
    }
}
