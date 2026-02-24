use std::time::Duration;

use crate::model::{event::Event, stage::Stage};

pub trait Entity<T> {
    fn take_turn(&mut self, stage: &mut Stage) -> Vec<Event<T>>;
    fn handle_event(&mut self, event: &T, stage: &mut Stage) -> Vec<Event<T>>;
    fn get_turn_delay(&self) -> Duration;
}
