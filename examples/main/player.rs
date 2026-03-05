use std::time::Duration;

use starlib::{Coords, Entity, Event, Sprite, model::movement::Movement, util::input::get_input};

use crate::user_event::UserEvent;

pub(crate) struct Player {
    coords: Coords,
}

impl Player {
    pub fn new(coords: Coords) -> Self {
        Player { coords }
    }
}

impl Entity<UserEvent> for Player {
    fn get_render(&self) -> (Coords, starlib::Sprite) {
        (self.coords.clone(), Sprite::build("ඞ", 2))
    }

    fn get_turn_delay(&self) -> std::time::Duration {
        Duration::from_millis(100)
    }

    fn handle_event(
        &mut self,
        _event: &UserEvent,
        _stage: &mut starlib::Stage,
    ) -> Vec<starlib::Event<UserEvent>> {
        vec![]
    }

    fn take_turn(&mut self, stage: &mut starlib::Stage) -> Vec<starlib::Event<UserEvent>> {
        let mut res = vec![];
        let input = get_input();
        if let Some(input) = input {
            let movement = match input {
                'w' => Some(Movement::Up),
                'a' => Some(Movement::Left),
                's' => Some(Movement::Down),
                'd' => Some(Movement::Right),
                'e' => {
                    res.push(Event::User(UserEvent::CatchRabbit));
                    None
                }
                _ => None,
            };
            if let Some(movement) = movement {
                if !stage.collides(&self.coords, &movement) {
                    self.coords = self.coords.clone().do_movement(&movement);
                    res.push(Event::User(UserEvent::PlayerMoved(self.coords.clone())));
                }
            }
        };
        res
    }
}
