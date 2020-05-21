use crossterm::event::{Event as TermEvent, KeyCode, KeyEvent, KeyModifiers};

use std::{fmt, mem, result};

use crate::{location::Location, Error, Result};

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

impl fmt::Display for DP {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        match self {
            DP::Left => write!(f, "left"),
            DP::Right => write!(f, "right"),
            DP::Find => write!(f, "find"),
            DP::Till => write!(f, "till"),
            DP::Start => write!(f, "start"),
            DP::End => write!(f, "end"),
            DP::LineBound => write!(f, "line_bound"),
            DP::Nobound => write!(f, "no_bound"),
            DP::Caret => write!(f, "caret"),
            DP::Nope => write!(f, "nope"),
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
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

impl fmt::Display for Opr {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        match self {
            Opr::Change(n, mto) => write!(f, "change({},{})", n, mto),
            Opr::Delete(n, mto) => write!(f, "delete({},{})", n, mto),
            Opr::Yank(n, mto) => write!(f, "yank({},{})", n, mto),
            Opr::Swapcase(n, mto) => write!(f, "swapcase({},{})", n, mto),
            Opr::Lowercase(n, mto) => write!(f, "lowercase({},{})", n, mto),
            Opr::Uppercase(n, mto) => write!(f, "uppercase({},{})", n, mto),
            Opr::Filter(n, mto) => write!(f, "filter({},{})", n, mto),
            Opr::Equal(n, mto) => write!(f, "equal({},{})", n, mto),
            Opr::Format(n, mto) => write!(f, "format({},{})", n, mto),
            Opr::Encode(n, mto) => write!(f, "encode({},{})", n, mto),
            Opr::RShift(n, mto) => write!(f, "rshift({},{})", n, mto),
            Opr::LShift(n, mto) => write!(f, "lshift({},{})", n, mto),
            Opr::Fold(n, mto) => write!(f, "fold({},{})", n, mto),
            Opr::Func(n, mto) => write!(f, "func({},{})", n, mto),
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub enum Mod {
    Esc,
    Insert(usize, DP), // (n, Nope/Caret)
    Append(usize, DP), // (n, Right/End)
    Open(usize, DP),   // (n, Left/Right)
}

impl fmt::Display for Mod {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        match self {
            Mod::Esc => write!(f, "esc"),
            Mod::Insert(n, dp) => write!(f, "insert({},{})", n, dp),
            Mod::Append(n, dp) => write!(f, "append({},{})", n, dp),
            Mod::Open(n, dp) => write!(f, "open({},{})", n, dp),
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
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
    PatternR(usize, DP),                // repeat pattern (n, Left/Right)
    None,
}

impl Default for Mto {
    fn default() -> Mto {
        Mto::None
    }
}

impl fmt::Display for Mto {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        match self {
            Mto::Left(n, dp) => write!(f, "left({},{})", n, dp),
            Mto::Right(n, dp) => write!(f, "right({},{})", n, dp),
            Mto::Up(n, dp) => write!(f, "up({},{})", n, dp),
            Mto::Down(n, dp) => write!(f, "down({},{})", n, dp),
            Mto::Col(n) => write!(f, "col({})", n),
            Mto::Home(dp) => write!(f, "home({})", dp),
            Mto::End => write!(f, "end"),
            Mto::Row(n, dp) => write!(f, "row({},{})", n, dp),
            Mto::Percent(n) => write!(f, "percent({})", n),
            Mto::Cursor(n) => write!(f, "cursor({})", n),
            Mto::CharF(n, ch, dp) => write!(f, "charf({},{:?},{})", n, ch, dp),
            Mto::CharT(n, ch, dp) => write!(f, "chart({},{:?},{})", n, ch, dp),
            Mto::CharR(n, dp) => write!(f, "charr({},{})", n, dp),
            Mto::Word(n, dp1, dp2) => write!(f, "word({},{},{})", n, dp1, dp2),
            Mto::WWord(n, dp1, dp2) => write!(f, "wword({},{},{})", n, dp1, dp2),
            Mto::Sentence(n, dp) => write!(f, "sentence({},{})", n, dp),
            Mto::Para(n, dp) => write!(f, "para({},{})", n, dp),
            Mto::Bracket(n, ch1, ch2, dp) => {
                //
                write!(f, "bracket({},{},{},{})", n, ch1, ch2, dp)
            }
            Mto::Pattern(n, _, dp) => write!(f, "pattern({},{})", n, dp),
            Mto::PatternR(n, dp) => write!(f, "patternr({},{})", n, dp),
            Mto::None => write!(f, "none"),
        }
    }
}

impl Mto {
    pub fn transform(self, m: usize, dp: DP) -> Result<Self> {
        use {
            Mto::{CharF, CharT, Pattern},
            DP::{Left, Right},
        };

        match (self, dp) {
            (CharF(_, ch, Left), Right) => Ok(CharF(m, ch, Left)),
            (CharF(_, ch, Left), Left) => Ok(CharF(m, ch, Right)),
            (CharF(_, ch, Right), Right) => Ok(CharF(m, ch, Right)),
            (CharF(_, ch, Right), Left) => Ok(CharF(m, ch, Left)),
            (CharT(_, ch, Left), Right) => Ok(CharT(m, ch, Left)),
            (CharT(_, ch, Left), Left) => Ok(CharT(m, ch, Right)),
            (CharT(_, ch, Right), Right) => Ok(CharT(m, ch, Right)),
            (CharT(_, ch, Right), Left) => Ok(CharT(m, ch, Left)),
            (Pattern(_, ch, Left), Right) => Ok(Pattern(m, ch, Left)),
            (Pattern(_, ch, Left), Left) => Ok(Pattern(m, ch, Right)),
            (Pattern(_, ch, Right), Right) => Ok(Pattern(m, ch, Right)),
            (Pattern(_, ch, Right), Left) => Ok(Pattern(m, ch, Left)),
            (Mto::None, _) => Ok(Mto::None),
            _ => err_at!(Fatal, msg: format!("unreachable")),
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub enum Ted {
    NewBuffer,
    PromptReply { input: String },
    StatusFile,
    StatusCursor,
}

impl fmt::Display for Ted {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        match self {
            Ted::NewBuffer => write!(f, "new_buffer"),
            Ted::PromptReply { input } => write!(f, "prompt_reply({})", input),
            Ted::StatusFile => write!(f, "status_file"),
            Ted::StatusCursor => write!(f, "status_cursor"),
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub enum Event {
    // Insert events
    Backspace,
    Enter,
    Tab,
    Delete,
    Esc,
    Char(char, KeyModifiers),
    FKey(u8, KeyModifiers),
    BackTab,
    // folded events
    B(usize, DP),   // (n, Left/Right)
    G(usize),       // (n,)
    F(usize, DP),   // (n, Left/Right)
    T(usize, DP),   // (n, Left/Right)
    N(usize),       // (n,)
    Op(usize, Opr), // (n, op-event)
    Md(Mod),        // (n, mode-event)
    Mt(Mto),        // (n, motion-event)
    // other events
    List(Vec<Event>),
    Td(Ted),
    Cmd(String, String), // (command-name, arguments)
    Prompt(String),
    Noop,
}

impl Event {
    pub fn to_modifiers(&self) -> KeyModifiers {
        match self {
            Event::FKey(_, modifiers) => modifiers.clone(),
            Event::Char(_, modifiers) => modifiers.clone(),
            _ => KeyModifiers::empty(),
        }
    }

    pub fn is_insert(&self) -> bool {
        use {
            Event::Md,
            Mod::{Append, Insert, Open},
        };

        match self {
            Md(Insert(_, _)) | Md(Append(_, _)) | Md(Open(_, _)) => true,
            _ => false,
        }
    }

    pub fn push(&mut self, evnt: Event) {
        match self {
            Event::List(events) => events.push(evnt),
            _ => {
                let event = mem::replace(self, Default::default());
                *self = Event::List(vec![event, evnt]);
            }
        }
    }
}

impl Iterator for Event {
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Event::Noop => None,
            Event::List(evnts) if evnts.len() > 0 => Some(evnts.remove(0)),
            Event::List(_) => None,
            _ => {
                let evnt = mem::replace(self, Event::Noop);
                Some(evnt)
            }
        }
    }
}

impl Extend<Event> for Event {
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = Event>,
    {
        let mut evnts: Vec<Event> = iter.into_iter().collect();
        let evnts = match mem::replace(self, Default::default()) {
            Event::List(mut events) => {
                events.extend(evnts);
                events
            }
            Event::Noop => evnts,
            evnt => {
                evnts.insert(0, evnt);
                evnts
            }
        };
        *self = if evnts.len() > 0 {
            Event::List(evnts)
        } else {
            Event::Noop
        };
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        use Event::{BackTab, Cmd, FKey, List, Md, Mt, Noop, Op, Prompt, Td};
        use Event::{Backspace, Char, Delete, Enter, Esc, Tab, B, F, G, N, T};

        match self {
            Backspace => write!(f, "backspace"),
            Enter => write!(f, "enter"),
            Tab => write!(f, "tab"),
            Delete => write!(f, "delete"),
            Esc => write!(f, "esc"),
            Char(ch, _) => write!(f, "char({})", ch),
            B(n, dp) => write!(f, "b({},{})", n, dp),
            G(n) => write!(f, "g({})", n),
            F(n, dp) => write!(f, "f({},{})", n, dp),
            T(n, dp) => write!(f, "t({},{})", n, dp),
            N(n) => write!(f, "b({}", n),
            Op(n, opr) => write!(f, "op({},{})", n, opr),
            Md(md) => write!(f, "md({})", md),
            Mt(mt) => write!(f, "mt({})", mt),
            Td(td) => write!(f, "td({})", td),
            List(es) => write!(f, "list({})", es.len()),
            Cmd(name, _) => write!(f, "cmd({})", name),
            FKey(ch, _) => write!(f, "fkey({})", ch),
            BackTab => write!(f, "backtab"),
            Prompt(s) => write!(f, "prompt({})", s),
            Noop => write!(f, "noop"),
        }
    }
}

impl From<TermEvent> for Event {
    fn from(evnt: TermEvent) -> Event {
        use Event::{BackTab, Backspace, Char, Delete, Enter, Esc, FKey};
        use Event::{Md, Mt, Tab};

        match evnt {
            TermEvent::Key(KeyEvent { code, modifiers: m }) => {
                let ctrl = m.contains(KeyModifiers::CONTROL);
                let empty = m.is_empty();
                match code {
                    //
                    KeyCode::Backspace if empty => Backspace,
                    KeyCode::Enter if empty => Enter,
                    KeyCode::Tab if empty => Tab,
                    KeyCode::Delete if empty => Delete,
                    KeyCode::Char(ch) if empty => Char(ch, m),
                    //
                    KeyCode::BackTab if empty => BackTab,
                    KeyCode::F(f) if empty => FKey(f, m),
                    //
                    KeyCode::Char('[') if ctrl => Esc,
                    KeyCode::Esc if empty => Esc,
                    KeyCode::Insert => Md(Mod::Insert(1, DP::Nope)),
                    //
                    KeyCode::Left if empty => Mt(Mto::Left(1, DP::LineBound)),
                    KeyCode::Right if empty => Mt(Mto::Right(1, DP::LineBound)),
                    KeyCode::Up if empty => Mt(Mto::Up(1, DP::Nope)),
                    KeyCode::Down if empty => Mt(Mto::Down(1, DP::Nope)),
                    KeyCode::Home if empty => Mt(Mto::Home(DP::Nope)),
                    KeyCode::End if empty => Mt(Mto::End),
                    KeyCode::Null => Event::Noop,
                    _ => Event::Noop,
                }
            }
            _ => Event::Noop,
        }
    }
}

impl Default for Event {
    fn default() -> Event {
        Event::Noop
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

impl From<Event> for Vec<Event> {
    fn from(evnt: Event) -> Vec<Event> {
        match evnt {
            Event::List(evnts) => evnts,
            evnt => vec![evnt],
        }
    }
}
