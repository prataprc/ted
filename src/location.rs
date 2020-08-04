//! Module `location` implement the backing source for a buffer.

use dirs;
use lazy_static::lazy_static;

use std::{ffi, fmt, io, path, result, sync::Mutex};

use crate::{text, Error, Result};

lazy_static! {
    static ref MEM_BUFFER_N: Mutex<usize> = Mutex::new(0);
}

/// Location of buffer's content, typically a persistent medium.
#[derive(Clone)]
pub enum Location {
    Memory {
        name: String,
        text: String,
        read_only: bool,
    },
    Disk {
        loc: ffi::OsString,
        path_file: ffi::OsString,
        enc: String,
        read_only: bool,
    },
    Ted {
        name: String,
        text: String,
        read_only: bool,
    },
}

// TODO: new_disk() to accept `enc` argument as Option<&str>.
// if None, we need to figure out the file-encoding  using other
// means.

impl Location {
    /// Create a new Disk location for buffer. `loc` can be absolute path,
    /// relative path to current-directory, or start with `~` relative to
    /// home-directory.
    pub fn new_disk(loc: &ffi::OsStr, enc: &str) -> Result<Location> {
        use std::fs;

        let fp = {
            let res = loc.to_os_string().into_string();
            err_at!(IOError, res.map_err(|e| format!("{:?}", e)))?
        };
        let path_file = Self::canonicalize(fp).into_os_string();
        let m = err_at!(IOError, fs::metadata(&path_file))?;
        Ok(Location::Disk {
            loc: loc.to_os_string(),
            path_file,
            enc: enc.to_string(),
            read_only: m.permissions().readonly(),
        })
    }

    /// Create a memory-only buffer.
    pub fn new_memory<R>(r: R, enc: &str, read_only: bool) -> Result<Location>
    where
        R: io::Read,
    {
        let name = {
            let mut count = err_at!(Fatal, MEM_BUFFER_N.lock())?;
            *count = *count + 1;
            format!("[no-name-{}]", count)
        };
        let text: String = text::Encoding::from_reader(r, enc)?.into();
        Ok(Location::Memory {
            name,
            text,
            read_only,
        })
    }

    /// Create a new buffer to be used within `ted` windows.
    pub fn new_ted<R>(name: &str, r: R, read_only: bool) -> Result<Location>
    where
        R: io::Read,
    {
        let text: String = text::Encoding::from_reader(r, "utf-8")?.into();
        let name = name.to_string();
        Ok(Location::Ted {
            name: format!("[{}]", name),
            text,
            read_only,
        })
    }
}

impl Location {
    pub fn read(&self) -> Result<String> {
        use std::fs;

        match self {
            Location::Disk { path_file, enc, .. } => {
                let fd = {
                    let mut oo = fs::OpenOptions::new();
                    err_at!(IOError, oo.read(true).open(path_file))?
                };
                Ok(text::Encoding::from_reader(fd, enc)?.into())
            }
            Location::Memory { text, .. } => Ok(text.clone()),
            Location::Ted { text, .. } => Ok(text.clone()),
        }
    }

    pub fn is_read_only(&self) -> bool {
        match self {
            Location::Memory { read_only, .. } => *read_only,
            Location::Disk { read_only, .. } => *read_only,
            Location::Ted { read_only, .. } => *read_only,
        }
    }

    pub fn to_tab_title(&self, wth: usize) -> Result<String> {
        match self {
            Location::Disk { loc, .. } => disk_to_tab_title(loc, wth),
            Location::Memory { name, .. } => Ok(format!(" M({:13}) ", name)),
            Location::Ted { name, .. } => Ok(format!(" T({:13}) ", name)),
        }
    }

    /// Return full path of the location, for display purpose.
    pub fn to_long_string(&self) -> Result<String> {
        let name = match self {
            Location::Disk { path_file, .. } => {
                let s = path_file.to_str().map(|s| s.to_string());
                s.unwrap_or(format!("<invalid path {:?}>", path_file))
            }
            Location::Memory { name, .. } => name.clone(),
            Location::Ted { name, .. } => name.clone(),
        };
        Ok(name)
    }

    /// Return shrunk, but meaningful, version of path for display purpose.
    pub fn to_short_string(&self) -> Result<String> {
        let name = match self {
            Location::Memory { name, .. } => name.clone(),
            Location::Disk { path_file, .. } => {
                let loc = disk_cwd_loc(path_file.into())?;
                loc.clone().into_string().unwrap_or(format!("{:?}", loc))
            }
            Location::Ted { name, .. } => name.clone(),
        };
        Ok(name)
    }

    fn canonicalize(loc: String) -> path::PathBuf {
        use std::iter::FromIterator;

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
        let read_only = false;
        Location::new_memory(io::empty(), "utf-8", read_only).unwrap()
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

// convert obsolute path to path relative to cwd or home.
fn disk_cwd_loc(loc: path::PathBuf) -> Result<ffi::OsString> {
    use std::env;

    let cwd = err_at!(IOError, env::current_dir())?;
    let loc = if loc.starts_with(&cwd) {
        err_at!(Invalid, loc.strip_prefix(&cwd))?.into()
    } else if let Some(home) = dirs::home_dir() {
        if loc.starts_with(&home) {
            let prefix = path::Path::new("~");
            let suffix = err_at!(Invalid, loc.strip_prefix(&cwd))?;
            [prefix, suffix].iter().collect::<path::PathBuf>().into()
        } else {
            loc.into()
        }
    } else {
        loc.into()
    };

    Ok(loc)
}

fn disk_to_tab_title_file<'a, 'b>(
    loc: &'a [path::Component<'b>],
    wth: usize,
) -> Result<(String, &'a [path::Component<'b>])> {
    use std::iter::FromIterator;

    let acc = match loc.iter().rev().next().clone() {
        Some(path::Component::RootDir) => path::MAIN_SEPARATOR.to_string(),
        Some(path::Component::Normal(s)) => {
            let s = match s.to_str() {
                Some(s) => s.to_string(),
                None => format!("{:?}", s),
            };
            String::from_iter(text::take_width(s.chars(), wth))
        }
        Some(p) => err_at!(Fatal, msg: format!("invalid loc {:?}", p))?,
        None => "".to_string(),
    };
    Ok((acc, &loc[..loc.len().saturating_sub(1)]))
}

fn disk_to_tab_title_path<'a, 'b>(
    loc: &'a [path::Component<'b>],
    mut acc: String,
    mut wth: usize,
) -> Result<String> {
    use std::iter::FromIterator;

    match loc.iter().rev().next().clone() {
        Some(path::Component::RootDir) => {
            acc = path::MAIN_SEPARATOR.to_string() + &acc;
            wth = wth.saturating_sub(1);
        }
        Some(path::Component::Normal(s)) => {
            let s = match s.to_str() {
                Some(s) => s.to_string(),
                None => format!("{:?}", s),
            };
            let mut s = String::from_iter(text::take_width(s.chars(), wth));
            s.push(path::MAIN_SEPARATOR);
            wth = wth.saturating_sub(text::width(s.chars()));
            acc = s + &acc;
        }
        Some(p) => err_at!(Fatal, msg: format!("invalid loc {:?}", p))?,
        None => (),
    };

    let loc = &loc[..loc.len().saturating_sub(1)];
    if loc.len() > 0 && wth > 0 {
        disk_to_tab_title_path(loc, acc, wth)
    } else {
        Ok(acc)
    }
}

fn disk_to_tab_title(loc: &ffi::OsStr, mut wth: usize) -> Result<String> {
    let loc = disk_cwd_loc(loc.into())?;
    let parts: Vec<path::Component> = path::Path::new(&loc).components().collect();
    let (prefix, loc) = match parts.first() {
        Some(path::Component::Prefix(p)) => {
            let prefix = match p.as_os_str().to_str() {
                Some(s) => s.to_string(),
                None => format!("{:?}", p),
            };
            (prefix + &path::MAIN_SEPARATOR.to_string(), &parts[1..])
        }
        Some(path::Component::RootDir) => {
            let prefix = path::MAIN_SEPARATOR.to_string();
            (prefix, &parts[1..])
        }
        Some(path::Component::Normal(s)) => {
            if s == &"~".parse::<ffi::OsString>().unwrap() {
                let prefix = "~".to_string() + &path::MAIN_SEPARATOR.to_string();
                (prefix, &parts[1..])
            } else {
                ("".to_string(), &parts[..])
            }
        }
        Some(p) => (format!("{:?}", p), &parts[1..]),
        None => ("".to_string(), &parts[parts.len()..]),
    };

    wth = wth.saturating_sub(text::width(prefix.chars()));
    let (acc, loc) = disk_to_tab_title_file(loc, wth)?;
    wth = wth.saturating_sub(text::width(acc.chars()));

    let acc = if loc.len() > 0 && wth > 0 {
        prefix + &disk_to_tab_title_path(loc, acc, wth)?
    } else {
        prefix + &acc
    };

    Ok(acc)
}
