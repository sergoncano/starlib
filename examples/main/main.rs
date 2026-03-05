use starlib::{
    Collider, Coords, Entity, Level, Map, Stage, graphics::sprite::sprite_vector_from_lines,
    model::collider::collider_vector_from_lines, renderer::{restore_terminal_properties, setup_terminal_properties},
};

use crate::{player::Player, user_event::UserEvent};

pub mod player;
pub mod user_event;

fn main() {
    setup_terminal_properties();
    let coords = Coords::new(0, 0);
    let player = Player::new(coords);
    let entities: Vec<Box<dyn Entity<UserEvent>>> = vec![Box::new(player)];
    let map_lines = vec!["...,...@.,", ".,@....|..", "..|...,.o."];
    let decorations = sprite_vector_from_lines(&map_lines, '@', 2);
    let colliders =
        collider_vector_from_lines(&map_lines, '|', Collider::try_from("nsew").unwrap());
    let map = Map::build(map_lines);
    let stage = Stage::build(map, decorations, colliders);
    let mut level = Level::new(stage, entities);
    level.run();
    restore_terminal_properties();
}
