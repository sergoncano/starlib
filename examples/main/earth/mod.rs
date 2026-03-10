use std::time::Duration;

use starlib::{
    Collider, Coords, Entity, Level, Map, Stage, graphics::sprite::sprite_vector_from_lines,
    model::collider::collider_vector_from_lines, Title,
};

use crate::{earth::rabbit::Rabbit, player::Player, user_event::UserEvent};

mod rabbit;

pub(crate) fn get_level() -> Level<UserEvent> {
    let coords = Coords::new(2, 2);
    let player = Player::new(coords, Duration::from_millis(100));
    let rabbit = Rabbit::new(Coords::new(13, 2));
    let entities: Vec<Box<dyn Entity<UserEvent>>> = vec![Box::new(player), Box::new(rabbit)];
    let map_lines = vec![
        "...............@....",
        "....@..........|....",
        "....|...............",
        "..................@.",
        "............@.....|.",
        "..@.........|.......",
    ];
    let decorations = sprite_vector_from_lines(&map_lines, '@', 3);
    let colliders = collider_vector_from_lines(&map_lines, '|', Collider::build("nsew"));
    let map = Map::build(map_lines);
    let stage = Stage::new(map, decorations, colliders);
    Level::build(vec![stage], entities)
}

pub(crate) fn get_title() -> Title {
    let title_text = vec!["EARTH", "", "Catch the rabbit"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    Title::new(title_text, Duration::from_secs(2))
}

pub(crate) fn get_victory_title() -> Title {
    let title_text = vec!["You caught the rabbit", "", "Yum"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    Title::new(title_text, Duration::from_secs(3))
}
