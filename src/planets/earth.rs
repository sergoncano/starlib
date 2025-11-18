use crate::coords::Coords;
use crate::collider;
use crate::decoration;
use crate::planet::Planet;

pub fn generate() -> Planet {
    let map = [
        ".......@..",
        ".......|..",
        "@.........",
        "|......@..",
        ".......|..",
    ];

    let leaves = decoration::decoration_vector_from_map(&map, '@', 1);

    let logs = collider::collider_vector_from_map(&map, '|');

    let earth: Planet = Planet::new("Earth", Vec::from(map), Coords::new(4, 2), "ඞ", leaves, logs);

    earth
}
