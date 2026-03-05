use std::time::Duration;

use crossterm::event::{Event, KeyCode, poll, read};

/// Returns the lowercase version of the character being pressed, if one. Otherwise returns None
pub fn get_input() -> Option<char> {
    let is_input = poll(Duration::ZERO).expect("Running in non-interactive terminal");
    if is_input
        && let Event::Key(k) = read().unwrap()
        && let KeyCode::Char(c) = k.code
    {
        empty_event_queue();
        Some(c.to_ascii_lowercase())
    } else {
        None
    }
}

fn empty_event_queue() {
    while poll(Duration::ZERO).expect("Running in non-interactive terminal") {
        let _ = read();
    }
}
