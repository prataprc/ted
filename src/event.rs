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

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum DP {
    Left,
    Right,
    Find,
    Till,
    Start,
    End,
    LineBound,
    Nobound,
    Caret,
    Nope,
}

#[derive(Clone)]
pub enum Event {
    Noop,
    // Return events
    List(Vec<Event>),
    // Input events
    F(u8, KeyModifiers),
    BackTab,
    // Commands
    Dec(Vec<char>),
    N(usize, Box<Event>), // (n, event)
    G(Box<Event>),
    B(DP), // (Left/Right,)
    // Modal events
    ModeEsc,
    ModeInsert(DP), // (Nope/Caret,)
    ModeAppend(DP), // (Right/End,)
    ModeOpen(DP),   // (Left/Right,)
    // Motion events
    MtoLeft(DP),  // (LineBound/Nobound,)
    MtoRight(DP), // (LineBound/Nobound,)
    MtoUp(DP),    // (Caret/Nope,)
    MtoDown(DP),  // (Caret/Nope,)
    MtoCol,
    MtoRow(DP), // (Caret/Nope,)
    MtoPercent,
    MtoHome(DP), // (Caret/Nope,)
    MtoEnd,
    MtoCursor,
    MtoCharF(Option<char>, DP), // (ch, Left/Right)
    MtoCharT(Option<char>, DP), // (ch, Left/Right)
    MtoCharR(DP),               // repeat MtoCharF/MtoCharT (Left/Right,)
    MtoWord(DP, DP),            // (Left/Right, Start/End)
    MtoWWord(DP, DP),           // (Left/Right, Start/End)
    MtoSentence(DP),            // (Left/Right,)
    MtoPara(DP),                // (Left/Right,)
    MtoBracket(char, char, DP), // (yin, yan, Left/Right)
    MtoChar(char),
    MtoPattern(Option<String>, DP), // (pattern, Left/Right)
    // Insert events
    Backspace,
    Char(char, KeyModifiers),
    Delete,
    Enter,
    Tab,
    Esc,
    // Application events
    NewBuffer,
    OpenFiles { flocs: Vec<OpenFile> },
    UseBuffer { buffer_id: String },
    PromptReply { input: String },
}

impl Default for Event {
    fn default() -> Event {
        Event::Noop
    }
}

impl From<TermEvent> for Event {
    fn from(evnt: TermEvent) -> Event {
        use Event::*;
        use DP::*;

        match evnt {
            TermEvent::Key(KeyEvent { code, modifiers: m }) => {
                let ctrl = m.contains(KeyModifiers::CONTROL);
                match code {
                    //
                    KeyCode::Backspace if m.is_empty() => Backspace,
                    KeyCode::Enter if m.is_empty() => Enter,
                    KeyCode::Tab if m.is_empty() => Tab,
                    KeyCode::Delete if m.is_empty() => Delete,
                    KeyCode::Char(ch) if m.is_empty() => Char(ch, m),
                    //
                    KeyCode::BackTab if m.is_empty() => BackTab,
                    KeyCode::F(f) if m.is_empty() => F(f, m),
                    //
                    KeyCode::Char('[') if ctrl => Esc,
                    KeyCode::Esc if m.is_empty() => Esc,
                    KeyCode::Insert => ModeInsert(Nope),
                    //
                    KeyCode::Left if m.is_empty() => MtoLeft(LineBound),
                    KeyCode::Right if m.is_empty() => MtoRight(LineBound),
                    KeyCode::Up if m.is_empty() => MtoUp(Nope),
                    KeyCode::Down if m.is_empty() => MtoDown(Nope),
                    KeyCode::Home if m.is_empty() => MtoHome(Nope),
                    KeyCode::End if m.is_empty() => MtoEnd,
                    KeyCode::Null => Noop,
                    _ => Event::Noop,
                }
            }
            _ => Event::Noop,
        }
    }
}

impl From<Vec<Event>> for Event {
    fn from(evnts: Vec<Event>) -> Event {
        let mut out: Vec<Event> = vec![];
        for evnt in evnts.into_iter() {
            match evnt {
                Event::List(es) => out.extend(es.into_iter()),
                e => out.push(e),
            }
        }
        Event::List(out)
    }
}

impl Event {
    pub fn transform(self, dp: DP) -> Result<Self> {
        use Event::*;
        use DP::*;

        match (self, dp) {
            (MtoCharF(ch, DP::Left), DP::Right) => Ok(MtoCharF(ch, Left)),
            (MtoCharF(ch, DP::Left), DP::Left) => Ok(MtoCharF(ch, Right)),
            (MtoCharF(ch, DP::Right), DP::Right) => Ok(MtoCharF(ch, Right)),
            (MtoCharF(ch, DP::Right), DP::Left) => Ok(MtoCharF(ch, Left)),
            (MtoCharT(ch, DP::Left), DP::Right) => Ok(MtoCharT(ch, Left)),
            (MtoCharT(ch, DP::Left), DP::Left) => Ok(MtoCharT(ch, Right)),
            (MtoCharT(ch, DP::Right), DP::Right) => Ok(MtoCharT(ch, Right)),
            (MtoCharT(ch, DP::Right), DP::Left) => Ok(MtoCharT(ch, Left)),
            (MtoPattern(ch, DP::Left), DP::Right) => Ok(MtoPattern(ch, Left)),
            (MtoPattern(ch, DP::Left), DP::Left) => Ok(MtoPattern(ch, Right)),
            (MtoPattern(ch, DP::Right), DP::Right) => Ok(MtoPattern(ch, Right)),
            (MtoPattern(ch, DP::Right), DP::Left) => Ok(MtoPattern(ch, Left)),
            _ => err_at!(Fatal, msg: format!("unreachable")),
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
