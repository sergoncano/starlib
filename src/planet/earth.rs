use crate::{
    model::{collider::collider_vector_from_map, decoration},
    planet::Planet,
};

pub fn generate() -> Planet {
    let map = [
        ".......@...",
        ".......|...",
        "@..........",
        "|........@.",
        ".........|.",
    ];

    let leaves = decoration::decoration_vector_from_map(&map, '@', 1);

    let logs = collider_vector_from_map(&map, '|');

    let earth: Planet = Planet::new("Earth", Vec::from(map), leaves, logs);

    earth
}
