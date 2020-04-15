use crossterm::event::{KeyCode, KeyModifiers};

enum Event {
    Noop,
    Esc,
    Backspace,
    Enter,
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
    Tab,
    BackTab,
    Delete,
    Insert,
    F(u8, KeyModifiers),
    Char(char, KeyModifiers),
}

impl From<term::Event> for Event {
    fn from(evnt: term::Event) -> Event {
        let m = evnt.to_modifiers();
        let ctrl = m.contains(KeyModifiers::CONTROL);
        // let shift = m.contains(KeyModifiers::SHIFT);
        match evnt {
            Event::Key { code, modifiers } => match code {
                KeyCode::Backspace if m.is_empty() => Event::Backspace,
                KeyCode::Enter if m.is_empty() => Event::Enter,
                KeyCode::Left if m.is_empty() => Event::Left,
                KeyCode::Right if m.is_empty() => Event::Right,
                KeyCode::Up if m.is_empty() => Event::Up,
                KeyCode::Down if m.is_empty() => Event::Down,
                KeyCode::Home if m.is_empty() => Event::Home,
                KeyCode::End if m.is_empty() => Event::End,
                KeyCode::PageUp if m.is_empty() => Event::PageUp,
                KeyCode::PageDown if m.is_empty() => Event::PageDown,
                KeyCode::Tab if m.is_empty() => Event::Tab,
                KeyCode::BackTab if m.is_empty() => Event::BackTab,
                KeyCode::Delete if m.is_empty() => Event::Delete,
                KeyCode::F(f) if m.is_empty() => Event::F(f, modifiers),
                KeyCode::Char('[') if ctrl => Event::Esc,
                KeyCode::Char(ch) if m.is_empty() => Event::Char(ch, modifiers),
                KeyCode::Esc if m.is_empty() => Event::Esc,
                KeyCode::Insert | KeyCode::Null => Event::Noop,
                _ => Event::Noop,
            },
            _ => Event::Noop,
        }
    }
}

impl Event {
    fn to_modifiers(evnt: &Event) -> KeyModifiers {
        match evnt {
            Event::F(_, modifiers) => modifiers.clone(),
            Event::Char(_, modifiers) => modifiers.clone(),
            _ => KeyModifiers::empty(),
        }
    }
}

