use crate::planet::Planet;
use crate::coords::Coords;
use crate::decoration;

pub fn generate() -> Planet<10, 5> {

    let map = 
       [".......@..",
        ".......|..",
        "@.........",
        "|......@..",
        ".......|.."];

    let trees = decoration::decoration_vector_from_map(&map, '@', 1);

    let earth: Planet<10, 5> = Planet::new(
        "Earth",
        map,
        Coords::new(4, 2),
        "ඞ",
        trees,
    );

    earth
}
