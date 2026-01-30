use crate::{model::event::Event, stage::planet::Planet};

pub trait EventDriven {
    fn get_event(&mut self) -> Vec<Event>;
    fn handle_event(&mut self, event: &Event);
    fn take_turn(&mut self, planet: &Planet);
}
