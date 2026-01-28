use crate::{
    entities::{player::Player, rabbit::Rabbit},
    interfaces::entity::Entity,
    level::Level,
    model::coords::Coords,
    planet,
};

pub fn build() -> Level {
    let planet = planet::earth::generate();
    let entities = generate_entities();
    Level { planet, entities }
}

fn generate_entities() -> Vec<Box<dyn Entity>> {
    let player: Box<dyn Entity> = Box::new(Player::new(Coords::new(3, 2), String::from("ඞ")));
    let rabbit: Box<dyn Entity> = Box::new(Rabbit::new(Coords::new(5, 4)));
    vec![player, rabbit]
}
