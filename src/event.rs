use crate::{coords::Coords, planet::Planet};

#[derive(Debug, Clone)]
pub enum Event {
    PlayerMovedTo(Coords),
}

pub trait EventDriven {
    fn get_event(&self) -> Option<Event>;
    fn handle_event(&mut self, event: &Event);
    fn take_turn(&mut self, planet: &Planet);
}
