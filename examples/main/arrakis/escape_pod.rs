use std::time::Duration;

use starlib::{Coords, Entity, Event, Sprite};

use crate::user_event::UserEvent;

pub(crate) struct EscapePod {
    current_coords: Coords,
    coords: Coords,
    sprite: Sprite,
}

impl EscapePod {
    pub(crate) fn new(coords: Coords) -> Self {
        Self {coords, sprite: Sprite::new('M', 5), current_coords: Coords::new(-10, -10)}
    }
}

impl Entity<UserEvent> for EscapePod {
    fn take_turn(&mut self, _stage: &mut starlib::Stage) -> Vec<starlib::Event<UserEvent>> {
        vec![]
    }

    fn get_render(&self) -> (Coords, starlib::Sprite) {
        (self.current_coords.clone(), self.sprite.clone())
    }

    fn handle_event(&mut self, event: &UserEvent, _stage: &mut starlib::Stage) -> Vec<starlib::Event<UserEvent>> {
        if let UserEvent::PlayerMoved(c) = event && c == &self.current_coords {
            vec![Event::ExitLevel(2)]
        } else {
            if let UserEvent::ChangedStage(stage_i) = event && stage_i == &2 {
                self.current_coords = self.coords.clone();
            }
            vec![]
        }
    }

    fn get_turn_delay(&self) -> std::time::Duration {
        Duration::ZERO
    }
}
