use std::time::Duration;

use starlib::{
    Collider, Coords, Entity, Level, Map, Stage,
    graphics::sprite::sprite_vector_from_lines,
    model::collider::collider_vector_from_lines,
    renderer::{restore_terminal_properties, setup_terminal_properties},
    ui::{menu::Menu, title::Title},
};

use crate::{player::Player, rabbit::Rabbit, user_event::UserEvent};

pub mod player;
pub mod rabbit;
pub mod user_event;

fn main() {
    setup_terminal_properties();
    let mut level = create_level();
    let options = vec!["Earth", "Exit"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let title_text = vec!["EARTH", "", "Catch the rabbit"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let title = Title::new(title_text, Duration::from_secs(2));
    let mut menu = Menu::build("Choose destination".to_string(), options);
    loop {
        match menu.prompt() {
            0 => {
                title.show();
                level.run();
            }
            1 => break,
            _ => panic!("Nonexistant menu option chosen"),
        }
    }
    restore_terminal_properties();
}

fn create_level() -> Level<UserEvent> {
    let coords = Coords::new(2, 2);
    let player = Player::new(coords);
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
    let colliders =
        collider_vector_from_lines(&map_lines, '|', Collider::try_from("nsew").unwrap());
    let map = Map::build(map_lines);
    let stage = Stage::build(map, decorations, colliders);
    Level::new(stage, entities)
}
