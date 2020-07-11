//! Module `location` implement the backing source for a buffer.

use dirs;
use lazy_static::lazy_static;

use std::{env, ffi, fmt, fs, io, iter::FromIterator, path, result, sync::Mutex};

use crate::{text, Error, Result};

lazy_static! {
    static ref MEM_BUFFER_N: Mutex<usize> = Mutex::new(0);
}

/// Location of buffer's content, typically a persistent medium.
#[derive(Clone)]
pub enum Location {
    Memory {
        name: String,
        buf: String,
    },
    Disk {
        path_file: ffi::OsString,
        enc: String,
        read_only: bool,
    },
    Ted {
        name: String,
        buf: String,
    },
    Err(Error),
}

impl Location {
    /// Create a memory-only buffer.
    pub fn new_memory<R>(r: R, enc: &str) -> Result<Location>
    where
        R: io::Read,
    {
        let name = {
            let mut count = MEM_BUFFER_N.lock().unwrap();
            *count = *count + 1;
            format!("[no-name-{}]", count)
        };
        let buf: String = text::Encoded::from_reader(r, enc)?.into();
        Ok(Location::Memory { name, buf })
    }

    /// Create a new Disk location for buffer. `loc` can be absolute path,
    /// relative path to current-directory or start with `~` relative to
    /// home-directory.
    pub fn new_disk(fp: &ffi::OsStr, enc: &str) -> Result<Location> {
        match fp.to_os_string().into_string() {
            Ok(fp) => {
                let path_file = Self::canonicalize(fp).into_os_string();
                match fs::metadata(&path_file) {
                    Ok(m) => Ok(Location::Disk {
                        path_file,
                        enc: enc.to_string(),
                        read_only: m.permissions().readonly(),
                    }),
                    err => Ok(Location::Err(err_at!(IOError, err).unwrap_err())),
                }
            }
            err => {
                let err = Error::Invalid(String::new(), format!("{:?}", err));
                Ok(Location::Err(err))
            }
        }
    }

    /// Create a new buffer to be used within the system.
    pub fn new_ted<R>(name: &str, r: R) -> Result<Location>
    where
        R: io::Read,
    {
        let buf: String = text::Encoded::from_reader(r, "utf-8")?.into();
        let name = name.to_string();
        Ok(Location::Ted {
            name: format!("[{}]", name),
            buf,
        })
    }
}

impl Location {
    pub fn read(&self) -> Result<String> {
        match self {
            Location::Memory { buf, .. } => Ok(buf.clone()),
            Location::Disk { path_file, enc, .. } => {
                let fd = {
                    let mut oo = fs::OpenOptions::new();
                    err_at!(IOError, oo.read(true).open(path_file))?
                };
                Ok(text::Encoded::from_reader(fd, enc)?.into())
            }
            Location::Ted { buf, .. } => Ok(buf.clone()),
            Location::Err(err) => Err(err.clone()),
        }
    }

    pub fn is_read_only(&self) -> bool {
        match self {
            Location::Memory { .. } => false,
            Location::Disk { read_only, .. } => *read_only,
            Location::Ted { .. } => false,
            Location::Err(_) => true,
        }
    }

    /// Return full path of the location, for display purpose.
    pub fn to_long_string(&self) -> Result<String> {
        match self {
            Location::Memory { name, .. } => Ok(name.clone()),
            Location::Disk { path_file, .. } => {
                let s = path_file.to_str().map(|s| s.to_string());
                Ok(s.unwrap_or(format!("<invalid path {:?}>", path_file)))
            }
            Location::Ted { name, .. } => Ok(name.clone()),
            Location::Err(err) => Ok(format!("<err-{}>", err)),
        }
    }

    /// Return shrunk, but meaningful, version of path for display purpose.
    pub fn to_short_string(&self) -> Result<String> {
        match self {
            Location::Memory { name, .. } => Ok(name.clone()),
            Location::Disk { path_file, .. } => {
                let fp = Self::shrink_home(&Self::shrink_cwd(path_file)?)?;
                Ok(fp
                    .to_str()
                    .map(|s| s.to_string())
                    .unwrap_or(format!("<invalid path {:?}>", fp)))
            }
            Location::Ted { name, .. } => Ok(name.clone()),
            Location::Err(err) => Ok(format!("<err-{}>", err)),
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
        Location::new_memory(io::empty(), "utf-8").unwrap()
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        match self {
            Location::Memory { name, .. } => write!(f, "{}", name),
            Location::Disk { path_file, .. } => match path_file.to_str() {
                Some(s) => write!(f, "{}", s),
                None => write!(f, "{:?}", path_file),
            },
            Location::Ted { name, .. } => write!(f, "{}", name),
            Location::Err(err) => write!(f, "<err-{}>", err),
        }
    }
}
