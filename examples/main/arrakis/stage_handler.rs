use std::time::Duration;

use starlib::{Coords, Entity, Event, Sprite};

use crate::user_event::UserEvent;

pub(crate) struct StageHandler {
    coords: Coords,
    stage: usize
}

impl StageHandler {
    pub(crate) fn new(coords: Coords) -> Self {
        StageHandler { coords, stage: 0 }
    }
}

impl Entity<UserEvent> for StageHandler {
    fn handle_event(&mut self, event: &UserEvent, _stage: &mut starlib::Stage) -> Vec<starlib::Event<UserEvent>> {
        match event {
            UserEvent::PlayerMoved(player_coords) => {
                if player_coords == &self.coords && self.stage != 2 {
                    vec![Event::ChangeStage(self.stage + 1), Event::User(UserEvent::ChangedStage(self.stage + 1))]
                } else {
                    vec![]
                }
            }
            UserEvent::ChangedStage(stage_i) => {
                self.stage = *stage_i;
                vec![]
            }
            _ => vec![]
        }
    }

    fn take_turn(&mut self, _stage: &mut starlib::Stage) -> Vec<Event<UserEvent>> {
        vec![]
    }

    fn get_render(&self) -> (Coords, starlib::Sprite) {
        (self.coords.clone(), Sprite::new('?', -1))
    }

    fn get_turn_delay(&self) -> std::time::Duration {
        Duration::ZERO
    }
}
