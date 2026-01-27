use crate::{
    coords::Coords,
    entities::{Entity, player::Player, rabbit::Rabbit},
    level::Level,
    planet::earth,
};

pub fn build() -> Level {
    let planet = earth::generate();
    let entities = generate_entities();
    Level { planet, entities }
}

fn generate_entities() -> Vec<Box<dyn Entity>> {
    let player: Box<dyn Entity> = Box::new(Player::new(Coords::new(3, 2), String::from("ඞ")));
    let rabbit: Box<dyn Entity> = Box::new(Rabbit::new(Coords::new(5, 4)));
    vec![player, rabbit]
}
