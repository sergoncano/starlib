use std::time::Duration;

use crossterm::event::{Event, KeyCode, poll, read};

pub enum Input {
    Char(char),
    Up,
    Down,
    Left,
    Right,
    Spacebar,
    Enter,
}

/// Returns the lowercase version of the character being pressed, if one. Otherwise returns None
pub fn get_input() -> Option<Input> {
    let is_input = poll(Duration::ZERO).expect("Running in non-interactive terminal");
    if is_input && let Event::Key(k) = read().unwrap() {
        empty_event_queue();
        match k.code {
            KeyCode::Char(' ') => Some(Input::Spacebar),
            KeyCode::Char(c) => Some(Input::Char(c.to_ascii_lowercase())),
            KeyCode::Up => Some(Input::Up),
            KeyCode::Down => Some(Input::Down),
            KeyCode::Left => Some(Input::Left),
            KeyCode::Right => Some(Input::Right),
            KeyCode::Enter => Some(Input::Enter),
            _ => None,
        }
    } else {
        None
    }
}

fn empty_event_queue() {
    while poll(Duration::ZERO).expect("Running in non-interactive terminal") {
        let _ = read();
    }
}
