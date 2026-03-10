//! Contains the [Input] enum and the [get_input] function, used to get user input.

use std::time::Duration;

use crossterm::event::{Event, KeyCode, poll, read};

/// An input key.
pub enum Input {
    /// An alphabetic key. It is always lowercase.
    Char(char),

    /// A number.
    Num(u32),

    /// The up arrow key.
    Up,

    /// The down arrow key.
    Down,

    /// The left arrow key.
    Left,

    /// The right arrow key.
    Right,

    /// The Spacebar.
    Spacebar,

    /// Enter key.
    Enter,

    /// Escape key.
    Esc,

    /// Delete key.
    Delete,

    /// Tab key.
    Tab,
}

/// Returns [Some]<[Input]> if the player is pressing a key. Otherwise returns None. If the key is
/// alphabetic, the [Input] returned always contains it in its lowercase format.
pub fn get_input() -> Option<Input> {
    let is_input = poll(Duration::ZERO).expect("Running in non-interactive terminal");
    if is_input && let Event::Key(k) = read().unwrap() {
        empty_event_queue();
        match k.code {
            KeyCode::Char(' ') => Some(Input::Spacebar),
            KeyCode::Char(c) if c.is_ascii_digit() => Some(Input::Num(c.to_digit(10).unwrap())),
            KeyCode::Char(c) => Some(Input::Char(c.to_ascii_lowercase())),
            KeyCode::Up => Some(Input::Up),
            KeyCode::Down => Some(Input::Down),
            KeyCode::Left => Some(Input::Left),
            KeyCode::Right => Some(Input::Right),
            KeyCode::Enter => Some(Input::Enter),
            KeyCode::Delete => Some(Input::Delete),
            KeyCode::Tab => Some(Input::Tab),
            KeyCode::Esc => Some(Input::Esc),
            _ => None,
        }
    } else {
        None
    }
}

pub(crate) fn empty_event_queue() {
    while poll(Duration::ZERO).expect("Running in non-interactive terminal") {
        let _ = read();
    }
}
