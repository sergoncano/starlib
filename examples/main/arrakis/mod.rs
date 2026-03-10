use std::time::Duration;

use starlib::{
    Collider, Coords, Entity, Level, Map, Stage, graphics::sprite::sprite_vector_from_lines,
    model::collider::collider_vector_from_lines, Title,
};

use crate::{
    arrakis::{
        elder::Elder, escape_pod::EscapePod, sandworm::Sandworm, stage_handler::StageHandler,
    },
    player::Player,
    user_event::UserEvent,
};

mod elder;
mod escape_pod;
mod sandworm;
mod stage_handler;

const ELDER_COORDS: Coords = Coords { x: 9, y: 1 };

pub(crate) fn get_level() -> Level<UserEvent> {
    let player = Box::new(Player::new(Coords::new(9, 3), Duration::from_millis(105)));
    let elder = Box::new(Elder::new(ELDER_COORDS));
    let sandworm = Box::new(Sandworm::new());
    let escape_pod = Box::new(EscapePod::new(Coords::new(110, 2)));
    let mut entities: Vec<Box<dyn Entity<UserEvent>>> = vec![player, elder, sandworm, escape_pod];
    for y in 0..=4 {
        entities.push(Box::new(StageHandler::new(Coords::new(119, y))));
    }
    Level::build(
        vec![get_first_stage(), get_second_stage(), get_third_stage()],
        entities,
    )
}

fn get_first_stage() -> Stage {
    let overlay_lines = vec![
        ".....................................................__..........................__________.............................",
        "..................................................._/  |......................../          |............................",
        "......................./|.........................<____|......................./___________|............................",
        "....................../_|................__......................_......................................................",
        "......................................../  \\....................|_>.....................................................",
    ];
    let mut colliders = vec![];
    let mut decorations = vec![];
    let chars = vec!['|', '_', '/', '>', '<', ' ', '\\'];
    for c in chars {
        colliders.extend(collider_vector_from_lines(
            &overlay_lines,
            c,
            Collider::build("nsew"),
        ));
        decorations.extend(sprite_vector_from_lines(&overlay_lines, c, 3));
    }
    let map_lines = vec![
        ",,,,,,,,,,,,,,,,,,_.....................................................................................................",
        ",,,,,,,,,,,,,,,,,,,|....................................................................................................",
        ",,,,,,,,,,,,,,,,,,,,....................................................................................................",
        ",,,,,,,,,,,,,,,,,,,|....................................................................................................",
        ",,,,,,,,,,,,,,,,,,_.....................................................................................................",
    ];
    colliders.extend(collider_vector_from_lines(
        &map_lines,
        '|',
        Collider::build("nse-"),
    ));
    colliders.extend(collider_vector_from_lines(
        &map_lines,
        '_',
        Collider::build("--e-"),
    ));
    let map = Map::build(map_lines);
    colliders.push((ELDER_COORDS, Collider::build("nsew")));
    Stage::new(map, decorations, colliders)
}

fn get_second_stage() -> Stage {
    let overlay_lines = vec![
        "..............____..................__......................................................._______....................",
        "............./    |..............__/ |___......................../------\\.................../_______\\...................",
        "............<_____|...___........|      /.......................\\       |...............................................",
        "......................| |........|_____/......_..................\\_____/................................................",
        "......................|_|..................../ \\......................................................._--\\.............",
    ];
    let mut colliders = vec![];
    let mut decorations = vec![];
    let rock_chars = vec!['\\', '|', '_', '/', '<', ' ', '-'];
    for rock_char in rock_chars {
        colliders.extend(collider_vector_from_lines(
            &overlay_lines,
            rock_char,
            Collider::build("nsew"),
        ));
        decorations.extend(sprite_vector_from_lines(&overlay_lines, rock_char, 3));
    }
    let map_lines = vec![
        "........................................................................................................................",
        "........................................................................................................................",
        "........................................................................................................................",
        "........................................................................................................................",
        "........................................................................................................................",
    ];
    Stage::new(Map::build(map_lines), decorations, colliders)
}

fn get_third_stage() -> Stage {
    let overlay_lines = vec![
        "..........................................________................................................../..................",
        "........................................./        \\................................................|..........^........",
        ".........................................|        |............................................................M.......",
        "..........................................\\       |................................................|...................",
        "...........................................\\_____/..................................................\\..................",
    ];
    let mut colliders = vec![];
    let mut decorations = vec![];
    let rock_chars = vec!['\\', '|', '_', '/', '<', ' ', '-'];
    for rock_char in rock_chars {
        colliders.extend(collider_vector_from_lines(
            &overlay_lines,
            rock_char,
            Collider::build("nsew"),
        ));
        decorations.extend(sprite_vector_from_lines(&overlay_lines, rock_char, 3));
    }
    let map_lines = vec![
        "....................................................................................................,,,,,,,,,,,,,,,,,,,,",
        "....................................................................................................,,,,,,,,,,,,,,,,,,,,",
        "...................................................................................................,,,,,,,,,,,,,,,,,,,,,",
        "....................................................................................................,,,,,,,,,,,,,,,,,,,,",
        ".....................................................................................................,,,,,,,,,,,,,,,,,,,",
    ];
    Stage::new(Map::build(map_lines), decorations, colliders)
}

pub(crate) fn get_title() -> Title {
    let title_text = vec!["ARRAKIS", "", "Escape the planet"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    Title::new(title_text, Duration::from_secs(2))
}

pub(crate) fn get_victory_title() -> Title {
    let title_text = vec!["You escaped the planet"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    Title::new(title_text, Duration::from_secs(3))
}

pub(crate) fn get_death_title() -> Title {
    let title_text = vec!["You were devoured by the worms"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    Title::new(title_text, Duration::from_secs(3))
}
