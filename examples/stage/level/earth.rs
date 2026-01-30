use starlib::{
    interface::entity::{Entity, Player},
    model::coords::Coords,
    stage::level::Level,
};

use crate::{entities::rabbit::Rabbit, stage::planet::earth};

pub fn build() -> Level {
    let planet = earth::generate();
    let entities = generate_entities();
    Level::new(planet, entities)
}

fn generate_entities() -> Vec<Box<dyn Entity>> {
    let player: Box<dyn Entity> = Box::new(Player::new(Coords::new(3, 2), String::from("ඞ")));
    let rabbit: Box<dyn Entity> = Box::new(Rabbit::new(Coords::new(5, 4)));
    vec![player, rabbit]
}
