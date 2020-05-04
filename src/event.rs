use crossterm::event::{Event as TermEvent, KeyCode, KeyEvent, KeyModifiers};

use std::{convert::TryFrom, ffi, fs, ops::Bound, path};

use crate::{location::Location, Error, Result};

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
pub enum Opr {
    Change(usize, Mto),    // (n, motion-command)
    Delete(usize, Mto),    // (n, motion-command)
    Yank(usize, Mto),      // (n, motion-command)
    Swapcase(usize, Mto),  // (n, motion-command)
    Lowercase(usize, Mto), // (n, motion-command)
    Uppercase(usize, Mto), // (n, motion-command)
    Filter(usize, Mto),    // (n, motion-command)
    Equal(usize, Mto),     // (n, motion-command)
    Format(usize, Mto),    // (n, motion-command)
    Encode(usize, Mto),    // (n, motion-command)
    RShift(usize, Mto),    // (n, motion-command)
    LShift(usize, Mto),    // (n, motion-command)
    Fold(usize, Mto),      // (n, motion-command)
    Func(usize, Mto),      // (n, motion-command)
}

#[derive(Clone)]
pub enum Mod {
    Esc,
    Insert(usize, DP), // (n, Nope/Caret)
    Append(usize, DP), // (n, Right/End)
    Open(usize, DP),   // (n, Left/Right)
}

#[derive(Clone)]
pub enum Mto {
    Left(usize, DP),  // (n, LineBound/Nobound)
    Right(usize, DP), // (n, LineBound/Nobound)
    Up(usize, DP),    // (n, Caret/Nope)
    Down(usize, DP),  // (n, Caret/Nope)
    Col(usize),       // (n,)
    Home(DP),         // (n, Caret/Nope)
    End,
    Row(usize, DP),                     // (n, Caret/Nope)
    Percent(usize),                     // (n,)
    Cursor(usize),                      // (n,)
    CharF(usize, Option<char>, DP),     // (n, ch, Left/Right)
    CharT(usize, Option<char>, DP),     // (n, ch, Left/Right)
    CharR(usize, DP),                   // repeat CharF/CharT (n, Left/Right)
    Word(usize, DP, DP),                // (n, Left/Right, Start/End)
    WWord(usize, DP, DP),               // (n, Left/Right, Start/End)
    Sentence(usize, DP),                // (n, Left/Right)
    Para(usize, DP),                    // (n, Left/Right)
    Bracket(usize, char, char, DP),     // (n, yin, yan, Left/Right)
    Pattern(usize, Option<String>, DP), // (n, pattern, Left/Right)
}

#[derive(Clone)]
enum N {}

#[derive(Clone)]
enum Ted {
    NewBuffer,
    OpenFiles { flocs: Vec<Location> },
    UseBuffer { buffer_id: String },
    PromptReply { input: String },
    StatusFile,
    StatusCursor,
}

#[derive(Clone)]
pub enum Event {
    // Insert events
    Backspace,
    Enter,
    Tab,
    Delete,
    Esc,
    Char(char, KeyModifiers),
    // folded events
    B(usize, DP),   // (n, Left/Right)
    G(usize),       // (n,)
    F(usize, DP),   // (n, Left/Right)
    T(usize, DP),   // (n, Left/Right)
    N(usize),       // (n,)
    Op(usize, Opr), // (n, op-event)
    Md(Mod),        // (n, mode-event)
    Mt(Mto),        // (n, motion-event)
    List(Vec<Event>),
    Ted(Ted),
    // other events
    F(u8, KeyModifiers),
    BackTab,
    Noop,
}

impl Default for Event {
    fn default() -> Event {
        Event::Noop
    }
}

impl From<TermEvent> for Event {
    fn from(evnt: TermEvent) -> Event {
        use {Event::*, DP::*};

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
                    KeyCode::Insert => Md(Mod::Insert(1, Nope)),
                    //
                    KeyCode::Left if m.is_empty() => Mt(Mto::Left(1, LineBound)),
                    KeyCode::Right if m.is_empty() => Mt(Mto::Right(1, LineBound)),
                    KeyCode::Up if m.is_empty() => Mt(Mto::Up(1, Nope)),
                    KeyCode::Down if m.is_empty() => Mt(Mto::Down(1, Nope)),
                    KeyCode::Home if m.is_empty() => Mt(Mto::Home(1, Nope)),
                    KeyCode::End if m.is_empty() => Mt(Mto::End),
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

impl Mto {
    pub fn transform(self, dp: DP) -> Result<Self> {
        use {Mto::*, DP::*};

        match (self, dp) {
            (CharF(ch, DP::Left), DP::Right) => Ok(CharF(ch, Left)),
            (CharF(ch, DP::Left), DP::Left) => Ok(CharF(ch, Right)),
            (CharF(ch, DP::Right), DP::Right) => Ok(CharF(ch, Right)),
            (CharF(ch, DP::Right), DP::Left) => Ok(CharF(ch, Left)),
            (CharT(ch, DP::Left), DP::Right) => Ok(CharT(ch, Left)),
            (CharT(ch, DP::Left), DP::Left) => Ok(CharT(ch, Right)),
            (CharT(ch, DP::Right), DP::Right) => Ok(CharT(ch, Right)),
            (CharT(ch, DP::Right), DP::Left) => Ok(CharT(ch, Left)),
            (Pattern(ch, DP::Left), DP::Right) => Ok(Pattern(ch, Left)),
            (Pattern(ch, DP::Left), DP::Left) => Ok(Pattern(ch, Right)),
            (Pattern(ch, DP::Right), DP::Right) => Ok(Pattern(ch, Right)),
            (Pattern(ch, DP::Right), DP::Left) => Ok(Pattern(ch, Left)),
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

    pub fn is_insert(&self) -> bool {
        use Event::*;

        match self {
            Mod::Insert(_) | Mod::Append(_) | Mod::Open(_) => true,
            _ => false,
        }
    }
}
