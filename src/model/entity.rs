use time::Duration;

use crate::model::event::Event;

pub trait Entity<T> {
    fn take_turn(&mut self) -> Vec<Event<T>>;
    fn handle_event(&mut self, event: Event<T>) -> Vec<Event<T>>;
    fn get_turn_frequency(&mut self) -> Duration;
}
