//! Module `location` implement the backing source for a buffer.

use dirs;
use lazy_static::lazy_static;

use std::{convert::TryFrom, env, ffi, fmt, fs, iter::FromIterator, path, result, sync::Mutex};

use crate::{Error, Result};

lazy_static! {
    static ref ANONYMOUS_COUNT: Mutex<usize> = Mutex::new(0);
}

/// Location of buffer's content, typically a persistent medium.
#[derive(Clone, Eq, PartialEq)]
pub enum Location {
    Anonymous(String),
    Disk(ffi::OsString),
}

impl Location {
    /// Create a anonymous location for buffer.
    pub fn new_anonymous() -> Location {
        let mut count = ANONYMOUS_COUNT.lock().unwrap();
        *count = *count + 1;
        Location::Anonymous(format!("newfile-{}", count))
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

    /// Return full path of the location, for display purpose.
    pub fn to_long_string(&self) -> Result<ffi::OsString> {
        match self {
            Location::Anonymous(_) => Ok("[no name]".to_string().into()),
            Location::Disk(s) => Ok(s.to_os_string()),
        }
    }

    /// Return shrunk, but meaningful, version of path for display purpose.
    pub fn to_short_string(&self) -> Result<ffi::OsString> {
        match self {
            Location::Anonymous(_) => Ok("[no name]".to_string().into()),
            Location::Disk(s) => Self::shrink_home(&Self::shrink_cwd(s)?),
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

pub enum OpenFile {
    ReadWrite(fs::File, ffi::OsString),
    ReadOnly(fs::File, ffi::OsString),
    NotFound(ffi::OsString),
    NoPermission(ffi::OsString),
}

impl Clone for OpenFile {
    fn clone(&self) -> Self {
        match self {
            OpenFile::ReadWrite(_, floc) => {
                let mut opts = fs::OpenOptions::new();
                let fd = opts.read(true).write(true).open(floc).unwrap();
                OpenFile::ReadWrite(fd, floc.clone())
            }
            OpenFile::ReadOnly(_, floc) => {
                let mut opts = fs::OpenOptions::new();
                let fd = opts.read(true).open(floc).unwrap();
                OpenFile::ReadOnly(fd, floc.clone())
            }
            OpenFile::NotFound(floc) => OpenFile::NotFound(floc.clone()),
            OpenFile::NoPermission(floc) => OpenFile::NoPermission(floc.clone()),
        }
    }
}

impl From<ffi::OsString> for OpenFile {
    fn from(floc: ffi::OsString) -> Self {
        let mut opts = fs::OpenOptions::new();
        match opts.read(true).write(true).open(&floc) {
            Ok(fd) => OpenFile::ReadWrite(fd, floc),
            Err(_) => match opts.read(true).open(&floc) {
                Ok(fd) => OpenFile::ReadOnly(fd, floc),
                Err(_) => {
                    let p = path::Path::new(&floc);
                    if p.is_file() {
                        OpenFile::NoPermission(floc)
                    } else {
                        OpenFile::NotFound(floc)
                    }
                }
            },
        }
    }
}

impl From<String> for OpenFile {
    fn from(floc: String) -> Self {
        let f: &ffi::OsStr = floc.as_ref();
        f.to_os_string().into()
    }
}

impl TryFrom<OpenFile> for fs::File {
    type Error = Error;

    fn try_from(of: OpenFile) -> Result<fs::File> {
        match of {
            OpenFile::ReadWrite(fd, _) => Ok(fd),
            OpenFile::ReadOnly(fd, _) => Ok(fd),
            OpenFile::NotFound(floc) => {
                let mut opts = fs::OpenOptions::new();
                err_at!(
                    IOError,
                    opts.read(true).write(true).open(&floc),
                    format!("{:?}", floc)
                )?;
                unreachable!()
            }
            OpenFile::NoPermission(floc) => {
                let mut opts = fs::OpenOptions::new();
                err_at!(
                    IOError,
                    opts.read(true).write(true).open(&floc),
                    format!("{:?}", floc)
                )?;
                unreachable!()
            }
        }
    }
}
