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
}

impl Location {
    /// Create a new Disk location for buffer. `loc` can be absolute path,
    /// relative path to current-directory, or start with `~` relative to
    /// home-directory.
    pub fn new_disk(fp: &ffi::OsStr, enc: &str) -> Result<Location> {
        let fp = {
            let res = fp.to_os_string().into_string();
            err_at!(IOError, res.map_err(|e| format!("{:?}", e)))?
        };
        let path_file = Self::canonicalize(fp).into_os_string();
        let m = err_at!(IOError, fs::metadata(&path_file))?;
        Ok(Location::Disk {
            path_file,
            enc: enc.to_string(),
            read_only: m.permissions().readonly(),
        })
    }

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
        }
    }

    pub fn is_read_only(&self) -> bool {
        match self {
            Location::Memory { .. } => false,
            Location::Disk { read_only, .. } => *read_only,
            Location::Ted { .. } => false,
        }
    }

    pub fn to_tab_title(&self, wth: u16) -> String {
        let to_string = |s: &ffi::OsStr| -> String {
            match s.to_str() {
                Some(s) => s.to_string(),
                None => format!("{:?}", s),
            }
        };
        let comp_to_string = |c: path::Component| -> Option<String> {
            match c {
                path::Component::Normal(s) => match to_string(s).chars().next() {
                    Some(ch) => Some(format!("{}", ch)),
                    None => None,
                },
                _ => None,
            }
        };
        let empty = "".to_string();

        match self {
            Location::Disk { path_file, .. } => {
                let p = path::Path::new(&path_file);

                let mut parts: Vec<path::Component> = p.components().collect();
                let file_part = parts
                    .pop()
                    .map(|c| comp_to_string(c).unwrap_or(empty.clone()))
                    .unwrap_or(empty.clone());
                let parts: Vec<String> = {
                    let iter = parts.into_iter().filter_map(comp_to_string);
                    iter.collect()
                };
                to_tab_title(parts, file_part, wth)
            }
            Location::Memory { name, .. } => format!(" M({:13}) ", name),
            Location::Ted { name, .. } => format!(" T({:13}) ", name),
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
        }
    }
}

fn to_tab_title(parts: Vec<String>, file_part: String, mut wth: u16) -> String {
    let filee = loop {
        let chars: Vec<char> = file_part.chars().collect();
        let start = chars.len().saturating_sub(wth as usize);
        let s = String::from_iter(&chars[start..]);
        wth = match text::width(s.chars()) {
            m if m <= (wth as usize) => break s,
            _ => wth.saturating_sub(1),
        }
    };
    let dirs = parts
        .into_iter()
        .fold(path::PathBuf::new(), |p, c| p.join(&c));
    dirs.join(&filee).to_str().unwrap_or("∞").to_string()
}
