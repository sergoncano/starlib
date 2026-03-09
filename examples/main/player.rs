use std::time::Duration;

use starlib::{
    Coords, Entity, Event, Input, Sprite, model::movement::Movement, util::input::get_input,
};

use crate::user_event::UserEvent;

pub(crate) struct Player {
    coords: Coords,
    turn_delay: Duration,
}

impl Player {
    pub fn new(coords: Coords, turn_delay: Duration) -> Self {
        Player { coords, turn_delay }
    }
}

impl Entity<UserEvent> for Player {
    fn get_render(&self) -> (Coords, starlib::Sprite) {
        (self.coords.clone(), Sprite::build("ඞ", 2))
    }

    fn get_turn_delay(&self) -> std::time::Duration {
        self.turn_delay
    }

    fn handle_event(
        &mut self,
        event: &UserEvent,
        _stage: &mut starlib::Stage,
    ) -> Vec<starlib::Event<UserEvent>> {
        if let UserEvent::ChangedStage(_) = event {
            self.coords.x = 0;
        }
        vec![]
    }

    fn take_turn(&mut self, stage: &mut starlib::Stage) -> Vec<starlib::Event<UserEvent>> {
        let mut res = vec![];
        let input = get_input();
        if let Some(input) = input {
            let movement = match input {
                Input::Char('w') | Input::Up => Some(Movement::Up),
                Input::Char('a') | Input::Left => Some(Movement::Left),
                Input::Char('s') | Input::Down => Some(Movement::Down),
                Input::Char('d') | Input::Right => Some(Movement::Right),
                Input::Char('e') | Input::Enter | Input::Spacebar => {
                    res.push(Event::User(UserEvent::PlayerInteracted));
                    None
                }
                Input::Esc => match crate::util::get_pause_menu().prompt() {
                    0 => None,
                    1 => {
                        res.push(Event::ExitLevel(0));
                        None
                    }
                    _ => unreachable!(),
                },
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
