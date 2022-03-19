use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub enum Key {
    Char(char),
    Ctrl(char),
    Backspace,
    Enter,
    Escape,
    Tab,
    Up,
    Right,
    Down,
    Left,
    Other,
}

impl From<KeyEvent> for Key {
    fn from(data: KeyEvent) -> Self {
        match data {
            KeyEvent {
                code: KeyCode::Esc, ..
            } => Key::Escape,

            KeyEvent {
                code: KeyCode::Enter,
                ..
            } => Key::Enter,

            KeyEvent {
                code: KeyCode::Backspace,
                ..
            } => Key::Backspace,

            KeyEvent {
                code: KeyCode::Tab, ..
            } => Key::Tab,

            KeyEvent {
                code: KeyCode::Up, ..
            } => Key::Up,

            KeyEvent {
                code: KeyCode::Right,
                ..
            } => Key::Right,

            KeyEvent {
                code: KeyCode::Down,
                ..
            } => Key::Down,

            KeyEvent {
                code: KeyCode::Left,
                ..
            } => Key::Left,

            KeyEvent {
                code: KeyCode::Char(c),
                modifiers: KeyModifiers::CONTROL,
            } => Key::Ctrl(c),

            KeyEvent {
                code: KeyCode::Char(c),
                ..
            } => Key::Char(c),

            _ => Key::Other,
        }
    }
}
