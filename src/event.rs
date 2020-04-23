use crossterm::event::{Event as TermEvent, KeyCode, KeyEvent, KeyModifiers};

use std::{convert::TryFrom, ffi, fs, path};

use crate::{Error, Result};

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

#[derive(Clone)]
pub enum Event {
    Noop,
    // Input events
    F(u8, KeyModifiers),
    Char(char, KeyModifiers),
    // Processed Input events
    Esc,
    Backspace(usize),
    Enter,
    Left(usize, bool),
    Right(usize, bool),
    Up(usize),
    Down(usize),
    Home,
    End,
    PageUp,
    PageDown,
    Tab,
    BackTab,
    Delete,
    Insert,
    // Motion Events
    GotoCol(usize),
    FChar(usize, Option<char>, bool /*direction*/),
    TChar(usize, Option<char>, bool /*direction*/),
    // Partial events
    PartialN(Vec<char>),
    // Application events
    NewBuffer,
    OpenFiles { flocs: Vec<OpenFile> },
    UseBuffer { buffer_id: String },
    PromptReply { input: String },
}

impl From<TermEvent> for Event {
    fn from(evnt: TermEvent) -> Event {
        match evnt {
            TermEvent::Key(KeyEvent { code, modifiers: m }) => {
                let ctrl = m.contains(KeyModifiers::CONTROL);
                // let shift = m.contains(KeyModifiers::SHIFT);
                match code {
                    KeyCode::Backspace if m.is_empty() => Event::Backspace(1),
                    KeyCode::Enter if m.is_empty() => Event::Enter,
                    KeyCode::Left if m.is_empty() => Event::Left(1, true /*line_bound*/),
                    KeyCode::Right if m.is_empty() => Event::Right(1, true /*line_bound*/),
                    KeyCode::Up if m.is_empty() => Event::Up(1),
                    KeyCode::Down if m.is_empty() => Event::Down(1),
                    KeyCode::Home if m.is_empty() => Event::Home,
                    KeyCode::End if m.is_empty() => Event::End,
                    KeyCode::PageUp if m.is_empty() => Event::PageUp,
                    KeyCode::PageDown if m.is_empty() => Event::PageDown,
                    KeyCode::Tab if m.is_empty() => Event::Tab,
                    KeyCode::BackTab if m.is_empty() => Event::BackTab,
                    KeyCode::Delete if m.is_empty() => Event::Delete,
                    KeyCode::F(f) if m.is_empty() => Event::F(f, m),
                    KeyCode::Char('[') if ctrl => Event::Esc,
                    KeyCode::Char(ch) if m.is_empty() => Event::Char(ch, m),
                    KeyCode::Esc if m.is_empty() => Event::Esc,
                    KeyCode::Insert | KeyCode::Null => Event::Noop,
                    _ => Event::Noop,
                }
            }
            _ => Event::Noop,
        }
    }
}

impl Event {
    pub fn to_modifiers(&self) -> KeyModifiers {
        match self {
            Event::F(_, modifiers) => modifiers.clone(),
            Event::Char(_, modifiers) => modifiers.clone(),
            _ => KeyModifiers::empty(),
        }
    }
}
