//! Contains the [Menu] struct.

use std::thread::sleep;

use crate::{FRAME_DURATION, Input, renderer::centered_render, util::input::get_input};

/// A menu with one or more options and a header. The header may be an empty string.
pub struct Menu {
    header_text: String,
    options: Vec<String>,
    selected: usize,
    max_length: usize,
}

impl Menu {
    /// Creates a title with the provided header and options. Panics if the options vector is
    /// empty.
    pub fn build(header_text: String, options: Vec<String>) -> Menu {
        if options.is_empty() {
            panic!("Tried to create menu without options! Use title instead.");
        }
        let mut max_length: usize = 0;
        for option in &options {
            if option.len() > max_length {
                max_length = option.len();
            }
        }
        Menu {
            header_text,
            options,
            selected: 0,
            max_length,
        }
    }

    /// Shows the menu in the center of the screen and handles user input until they choose an
    /// option. The return value is the index of the chosen option.
    pub fn prompt(&mut self) -> usize {
        crate::util::input::empty_event_queue();
        let n_options = &self.options.len();
        loop {
            self.render();
            sleep(FRAME_DURATION);
            if let Some(input) = get_input() {
                match input {
                    Input::Char('w') | Input::Up => {
                        self.selected = self.selected.checked_sub(1).unwrap_or(n_options - 1)
                    }
                    Input::Char('s') | Input::Down => {
                        self.selected = (self.selected + 1).rem_euclid(*n_options)
                    }
                    Input::Char('e') | Input::Spacebar | Input::Enter => break,
                    _ => (),
                }
            }
        }
        self.selected
    }

    fn render(&self) {
        let mut text = vec![self.header_text.clone()];
        for (i, option) in self.options.iter().enumerate() {
            text.push(String::from(""));
            let formatted_option = format!(
                "{}{: <l$}",
                if i == self.selected { "·" } else { " " },
                option,
                l = self.max_length
            );
            text.push(formatted_option);
        }
        centered_render(&text, None);
    }
}
