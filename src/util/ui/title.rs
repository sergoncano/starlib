use std::{thread::sleep, time::Duration};

use crate::renderer::centered_render;

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
