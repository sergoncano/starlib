use crate::{coords::Coords, planet::Planet};

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    PlayerMovedTo(Coords),
    PlayerInteracted,
    ExitLevel,
    ShowTip(String),
}

pub trait EventDriven {
    fn get_event(&mut self) -> Vec<Event>;
    fn handle_event(&mut self, event: &Event);
    fn take_turn(&mut self, planet: &Planet);
}
