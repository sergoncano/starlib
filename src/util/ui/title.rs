//! Contains the [Title] struct.

use std::{thread::sleep, time::Duration};

use crate::renderer::centered_render;

/// A splash title. Used for showing non-interactable text or ascii art on-screen.
pub struct Title {
    text: Vec<String>,
    duration: Duration,
    max_length: usize,
}

impl Title {
    pub fn new(text: Vec<String>, duration: Duration) -> Self {
        let mut max_length: usize = 0;
        for line in &text {
            if line.len() > max_length {
                max_length = line.len();
            }
        }
        Title {
            text,
            duration,
            max_length,
        }
    }

    /// For the title's [Duration](std::time::Duration), its text is the only thing shown
    /// on-screen. It is centered.
    pub fn show(&self) {
        centered_render(
            &self
                .text
                .iter()
                .map(|s| format!("{s: ^l$}", l = self.max_length))
                .collect(),
            None,
        );
        sleep(self.duration);
    }
}
