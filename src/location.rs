//! Module `location` implement the backing source for a buffer.

use dirs;
use lazy_static::lazy_static;

use std::{env, ffi, fmt, fs, iter::FromIterator, path, result, sync::Mutex};

use crate::{Error, Result};

lazy_static! {
    static ref MEM_BUFFER_N: Mutex<usize> = Mutex::new(0);
}

/// Location of buffer's content, typically a persistent medium.
#[derive(Clone, Eq, PartialEq)]
pub enum Location {
    Memory(String),
    Disk(ffi::OsString),
    Ted(String),
}

impl Location {
    /// Create a memory-only buffer.
    pub fn new_memory() -> Location {
        let mut count = MEM_BUFFER_N.lock().unwrap();
        *count = *count + 1;
        Location::Memory(format!("memtext-{}", count))
    }

    /// Create a new Disk location for buffer. `loc` can be absolute path,
    /// relative path to current-directory or start with `~` relative to
    /// home-directory.
    pub fn new_disk(loc: &ffi::OsStr) -> Location {
        match loc.to_os_string().into_string() {
            Ok(loc) => Location::Disk(Self::canonicalize(loc).into_os_string()),
            Err(loc) => Location::Disk(loc),
        }
    }

    /// Create a new buffer to be used within the system.
    pub fn new_ted(name: &str) -> Location {
        Location::Ted(name.to_string())
    }
}

impl Location {
    pub fn to_rw_file(&self) -> Option<fs::File> {
        match self {
            Location::Memory(_) => None,
            Location::Disk(f) => {
                let mut oo = fs::OpenOptions::new();
                oo.read(true).write(true).open(f).ok()
            }
            Location::Ted(_) => None,
        }
    }

    pub fn to_r_file(&self) -> Option<fs::File> {
        match self {
            Location::Memory(_) => None,
            Location::Disk(f) => {
                let mut oo = fs::OpenOptions::new();
                oo.read(true).open(f).ok()
            }
            Location::Ted(_) => None,
        }
    }

    /// Return full path of the location, for display purpose.
    pub fn to_long_string(&self) -> Result<ffi::OsString> {
        match self {
            Location::Memory(_) => Ok("[mem-text]".to_string().into()),
            Location::Disk(s) => Ok(s.to_os_string()),
            Location::Ted(name) => Ok(format!("[{:?}]", name).into()),
        }
    }

    /// Return shrunk, but meaningful, version of path for display purpose.
    pub fn to_short_string(&self) -> Result<ffi::OsString> {
        match self {
            Location::Memory(_) => Ok("[mem-text]".to_string().into()),
            Location::Disk(s) => Self::shrink_home(&Self::shrink_cwd(s)?),
            Location::Ted(name) => Ok(format!("[{:?}]", name).into()),
        }
    }

    fn shrink_home(s: &ffi::OsStr) -> Result<ffi::OsString> {
        match dirs::home_dir() {
            Some(home) => {
                let pb: path::PathBuf = s.to_os_string().into();
                if pb.starts_with(&home) {
                    let mut shrnk = path::PathBuf::new();
                    shrnk.push(&home);
                    shrnk.push(err_at!(FailConvert, pb.strip_prefix(&home))?);
                    Ok(shrnk.into_os_string())
                } else {
                    Ok(pb.into_os_string())
                }
            }
            None => Ok(s.to_os_string()),
        }
    }

    fn shrink_cwd(s: &ffi::OsStr) -> Result<ffi::OsString> {
        let cwd = err_at!(Fatal, env::current_dir())?;
        let pb: path::PathBuf = s.to_os_string().into();
        if pb.starts_with(&cwd) {
            let mut shrnk = path::PathBuf::new();
            shrnk.push(err_at!(FailConvert, pb.strip_prefix(&cwd))?);
            Ok(shrnk.into_os_string())
        } else {
            Ok(pb.into_os_string())
        }
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
        Location::new_memory()
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        match self {
            Location::Memory(s) => write!(f, "{}", s),
            Location::Disk(s) => {
                let s = s.clone().into_string().unwrap();
                write!(f, "{}", s)
            }
            Location::Ted(name) => write!(f, "{}", name),
        }
    }
}
