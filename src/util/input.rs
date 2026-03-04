use std::time::Duration;

use crossterm::event::{Event, KeyCode, poll, read};

/// Returns the lowercase version of the character being pressed, if one. Otherwise returns None
pub fn get_input() -> Option<char> {
let is_input = poll(Duration::from_secs(0)).expect("Running in non-interactive terminal");
        if is_input && let Event::Key(k) = read().unwrap() && let KeyCode::Char(c) = k.code {
            Some(c.to_ascii_lowercase())
        } else {
            None
        }
}
