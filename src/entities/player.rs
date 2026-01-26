use crate::{
    coords::Coords,
    entities::Entity,
    event::{Event, EventDriven},
    input,
    movement::Movement,
    planet::Planet,
    renderable::Renderable,
    terminal,
};

pub struct Player {
    coords: Coords,
    sprite: String,
    z_index: i32,
    sent_event: Vec<Event>,
}

impl EventDriven for Player {
    fn get_event(&self) -> Vec<Event> {
        self.sent_event.clone()
    }

    fn handle_event(&mut self, _: &Event) {
        return;
    }

    fn take_turn(&mut self, planet: &Planet) {
        self.sent_event = vec![];
        let movement = input::get_input();
        if movement == Movement::Quit {
            terminal::restore_terminal_properties();
            std::process::exit(0);
        }
        let validated_movement = planet.check_movement_collision(&self.coords, movement);
        self.coords.do_movement(validated_movement);
        self.sent_event
            .push(Event::PlayerMovedTo(self.coords.clone()));
    }
}

impl Renderable for Player {
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

impl Entity for Player {}

impl Player {
    pub fn new(coords: Coords, sprite: String) -> Player {
        Player {
            coords,
            sprite,
            z_index: 0,
            sent_event: vec![],
        }
    }
}
