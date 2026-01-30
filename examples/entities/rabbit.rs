use rand::Rng;

use starlib::{
    interface::{entity::Entity, event_driven::EventDriven, renderable::Renderable},
    model::{coords::Coords, event::Event, movement::Movement},
    stage::planet::Planet,
};

pub struct Rabbit {
    coords: Coords,
    sprite: String,
    z_index: i32,
    sent_events: Vec<Event>,
    player_coords: Coords,
}

impl EventDriven for Rabbit {
    fn get_event(&mut self) -> Vec<Event> {
        let sent = self.sent_events.clone();
        self.sent_events = vec![];
        sent
    }

    fn handle_event(&mut self, event: &Event) {
        match event {
            Event::PlayerMovedTo(coords) => self.player_coords = coords.clone(),
            Event::PlayerInteracted => {
                if self.player_coords == self.coords {
                    self.sent_events.push(Event::ExitLevel);
                }
            }
            _ => (),
        }
    }

    fn take_turn(&mut self, planet: &Planet) {
        if self.sent_events.contains(&Event::ExitLevel) {
            return;
        }
        let mut rng = rand::rng();
        let movement: Movement;
        if rng.random() {
            if self.coords.get_x() > self.player_coords.get_x() {
                movement = Movement::Right;
            } else if self.coords.get_x() == self.player_coords.get_x() {
                movement = if rng.random() || rng.random() {
                    Movement::Right
                } else {
                    Movement::Left
                };
            } else {
                movement = Movement::Left;
            }
        } else if self.coords.get_y() > self.player_coords.get_y() {
            movement = Movement::Down;
        } else if self.coords.get_y() == self.player_coords.get_y() {
            movement = if rng.random() || rng.random() {
                Movement::Up
            } else {
                Movement::Down
            };
        } else {
            movement = Movement::Up;
        }
        self.coords
            .do_movement(planet.check_movement_collision(&self.coords, movement));
        if self.coords == self.player_coords {
            self.sent_events
                .push(Event::ShowTip(String::from("E: Catch rabbit")));
        }
    }
}

impl Renderable for Rabbit {
    fn get_sprite(&self) -> String {
        self.sprite.clone()
    }
    fn get_coords(&self) -> Coords {
        self.coords.clone()
    }
    fn get_z_index(&self) -> i32 {
        self.z_index
    }
}

impl Entity for Rabbit {}

impl Rabbit {
    pub fn new(coords: Coords) -> Rabbit {
        Rabbit {
            coords,
            sprite: String::from("*"),
            z_index: -1,
            sent_events: vec![],
            player_coords: Coords::new(0, 0),
        }
    }
}
