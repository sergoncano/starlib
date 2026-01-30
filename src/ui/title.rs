use std::{thread::sleep, time::Duration};

use crate::util::renderer::render_title;

pub struct Title {
    title: String,
    subtitle: Option<String>,
    duration: Duration,
}

impl Title {

    pub fn new(title: String, subtitle: Option<String>, duration: Duration) -> Title{
        Title {title, subtitle, duration}
    } 

    pub fn show(self) {
        render_title(self.title, self.subtitle);
        sleep(self.duration);
    }
}
