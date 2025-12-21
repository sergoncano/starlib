use crate::coords::Coords;
use crate::movement::Movement;
use std::collections::HashMap;
use std::panic;

pub struct Collider {
    coords: Coords,
    collides_entering_from_up: bool,
    collides_entering_from_left: bool,
    collides_entering_from_down: bool,
    collides_entering_from_right: bool,
}

impl Collider {
    pub fn new(
        coords: Coords,
        collides_entering_from_up: bool,
        collides_entering_from_left: bool,
        collides_entering_from_down: bool,
        collides_entering_from_right: bool,
    ) -> Collider {
        Collider {
            coords,
            collides_entering_from_up,
            collides_entering_from_left,
            collides_entering_from_down,
            collides_entering_from_right,
        }
    }

    pub fn get_coords(&self) -> Coords {
        self.coords.clone()
    }

    fn add(&self, other: &Collider) -> Collider {
        if self.coords != other.coords {
            panic!(
                "Tried to add two colliders without equal coordinates: {:?} and {:?}",
                self.coords, other.coords
            );
        }
        Collider {
            coords: self.coords.clone(),
            collides_entering_from_up: self.collides_entering_from_up
                || other.collides_entering_from_up,
            collides_entering_from_left: self.collides_entering_from_left
                || other.collides_entering_from_left,
            collides_entering_from_down: self.collides_entering_from_down
                || other.collides_entering_from_down,
            collides_entering_from_right: self.collides_entering_from_right
                || other.collides_entering_from_right,
        }
    }

    pub fn collides(&self, movement: &Movement) -> bool {
        match movement {
            Movement::Up => self.collides_entering_from_down,
            Movement::Left => self.collides_entering_from_right,
            Movement::Down => self.collides_entering_from_up,
            Movement::Right => self.collides_entering_from_left,
            Movement::Wait => false,
            _ => panic!("Wrong movement in collision"),
        }
    }
}

//This function takes a map and a character as arguments, returns a vector of colliders which
//contain the coordinates in which the provided character appears in the map. These colliders block
//movement in all four directions inward.
pub fn collider_vector_from_map(map: &[&'static str], character: char) -> Vec<Collider> {
    let mut res: Vec<Collider> = Vec::new();
    for (y, line) in map.iter().enumerate() {
        for (x, map_character) in line.chars().enumerate() {
            if map_character != character {
                continue;
            }
            let collider = Collider {
                coords: Coords::new(x as i32, y as i32),
                collides_entering_from_up: true,
                collides_entering_from_left: true,
                collides_entering_from_down: true,
                collides_entering_from_right: true,
            };
            res.push(collider);
        }
    }
    res
}

//This function does the same as collider_vector_from_map except it takes an additional direction
//movement, this dictates from which direction the movement is blocked in the generated colliders.
//E.g.: If you pass Movement::Left, an entity right of the colliders moving right will be blocked.
pub fn directed_collider_vector_from_map(
    map: &[&'static str],
    character: char,
    direction: Movement,
) -> Vec<Collider> {
    let mut res: Vec<Collider> = Vec::new();
    for (y, line) in map.iter().enumerate() {
        for (x, map_char) in line.chars().enumerate() {
            if map_char != character {
                continue;
            }
            let mut up = false;
            let mut down = false;
            let mut left = false;
            let mut right = false;
            match direction {
                Movement::Up => up = true,
                Movement::Down => down = true,
                Movement::Left => left = true,
                Movement::Right => right = true,
                _ => panic!("Invalid collider direction!"),
            };
            let collider = Collider {
                coords: Coords::new(x as i32, y as i32),
                collides_entering_from_up: up,
                collides_entering_from_left: left,
                collides_entering_from_down: down,
                collides_entering_from_right: right,
            };
            res.push(collider);
        }
    }
    res
}

pub fn collider_map_from_vector(vector: Vec<Collider>) -> HashMap<Coords, Collider> {
    let mut map: HashMap<Coords, Collider> = HashMap::new();
    for collider in vector {
        let coords = collider.get_coords();
        if !map.contains_key(&coords) {
            map.insert(coords, collider);
            continue;
        } else {
            let existing_collider = map.get_mut(&coords).unwrap();
            *existing_collider = collider.add(existing_collider);
        }
    }
    map
}
