use std::time::Duration;

use rand::prelude::*;

use starlib::{Coords, Entity, Event, Sprite, Tip, model::movement::Movement};

use crate::user_event::UserEvent;

pub(crate) struct Rabbit {
    coords: Coords,
    player_coords: Coords,
    speed: u32,
    tip_shown: bool,
}

impl Rabbit {
    pub fn new(coords: Coords) -> Self {
        Rabbit {
            coords,
            player_coords: Coords::new(-1, -1),
            speed: 1,
            tip_shown: false,
        }
    }

    pub fn assess_danger(&mut self) {
        let distance = self.coords.distance(&self.player_coords);
        if distance < 5 {
            self.speed = 3;
        } else if distance < 5 {
            self.speed = 2;
        } else if distance < 10 {
            self.speed = 1;
        } else {
            self.speed = 0;
        }
    }
}

impl Entity<UserEvent> for Rabbit {
    fn take_turn(&mut self, stage: &mut starlib::Stage) -> Vec<starlib::Event<UserEvent>> {
        let mut rng = rand::rng();
        let movement = match rng.random_range(1..=4) {
            1 => Movement::Up,
            2 => Movement::Down,
            3 => Movement::Left,
            4 => Movement::Right,
            _ => {
                panic!("Wrong random range in rabbit implementation")
            }
        };
        if !stage.collides(&self.coords, &movement) {
            self.coords = self.coords.clone().do_movement(&movement);
        }
        self.assess_danger();
        if !self.tip_shown {
            let tip = Tip::new(
                String::from("Catch me if you can!"),
                Duration::from_secs(3),
                2,
            );
            self.tip_shown = true;
            vec![Event::Tip(tip)]
        } else {
            if self.coords == self.player_coords {
                vec![Event::Tip(Tip::new(
                    String::from("Press E to catch"),
                    self.get_turn_delay(),
                    1,
                ))]
            } else {
                vec![Event::Tip(Tip::new(String::from(""), Duration::ZERO, 1))]
            }
        }
    }

    fn get_render(&self) -> (Coords, starlib::Sprite) {
        (self.coords.clone(), Sprite::build("*", 1))
    }

    fn handle_event(
        &mut self,
        event: &UserEvent,
        _stage: &mut starlib::Stage,
    ) -> Vec<starlib::Event<UserEvent>> {
        match event {
            UserEvent::CatchRabbit => {
                if self.player_coords == self.coords {
                    vec![Event::ExitLevel(1)]
                } else {
                    vec![]
                }
            }
            UserEvent::PlayerMoved(coords) => {
                self.player_coords = coords.clone();
                self.assess_danger();
                if &self.coords == coords {
                    vec![Event::Tip(Tip::new(
                        String::from("Press E to catch"),
                        self.get_turn_delay(),
                        1,
                    ))]
                } else {
                    vec![Event::Tip(Tip::new(String::from(""), Duration::ZERO, 1))]
                }
            }
        }
    }

    fn get_turn_delay(&self) -> std::time::Duration {
        Duration::from_millis((1000 - 250 * self.speed) as u64)
    }
}
