use std::ops::Add;

use crate::{Coords, model::movement::Movement};

/// Represents a collision box. The cardinals denote which INWARD movements are blocked. E.g.: if
/// collider.north = true, then it will not allow something getting into it from the north.
/// However, it will allow moving north from inside of it.
#[derive(Debug, PartialEq, Clone)]
pub struct Collider {
    north: bool,
    south: bool,
    east: bool,
    west: bool,
}

impl Collider {
    pub fn new(north: bool, south: bool, east: bool, west: bool) -> Self {
        Collider {
            north,
            south,
            east,
            west,
        }
    }

    /// Check if a movement directed towards the collider should be blocked.
    pub fn blocks(&self, movement: &Movement) -> bool {
        match movement {
            Movement::Down => self.north,
            Movement::Up => self.south,
            Movement::Left => self.east,
            Movement::Right => self.west,
        }
    }
}

impl Add for Collider {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            north: self.north || other.north,
            south: self.south || other.south,
            east: self.east || other.east,
            west: self.west || other.west,
        }
    }
}

impl TryFrom<&str> for Collider {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() != 4 {
            return Err("Value is not formatted as nsew.");
        }
        let mut characters = value.chars();
        let character = characters.next().unwrap();
        let north = match character {
            'n' => true,
            '-' => false,
            _ => return Err("First character is not 'n' nor '-'."),
        };
        let character = characters.next().unwrap();
        let south = match character {
            's' => true,
            '-' => false,
            _ => return Err("Second character is not 's' nor '-'."),
        };
        let character = characters.next().unwrap();
        let east = match character {
            'e' => true,
            '-' => false,
            _ => return Err("Third character is not 'e' nor '-'."),
        };
        let character = characters.next().unwrap();
        let west = match character {
            'w' => true,
            '-' => false,
            _ => return Err("Fourth character is not 'w' nor '-'."),
        };
        Ok(Collider {
            north,
            south,
            east,
            west,
        })
    }
}

pub fn collider_vector_from_lines(
    lines: &Vec<&str>,
    character: char,
    collider: Collider,
) -> Vec<(Coords, Collider)> {
    let mut res = vec![];
    for (y, line) in lines.iter().enumerate() {
        for (x, current_character) in line.chars().enumerate() {
            if current_character == character {
                res.push((Coords::new(x as i32, y as i32), collider.clone()));
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructor() {
        Collider::new(true, false, true, false);
        Collider::new(false, true, false, true);
        Collider::new(false, false, false, false);
        Collider::new(true, true, true, true);
    }

    #[test]
    fn test_partial_eq() {
        let north1 = Collider::new(true, false, false, false);
        let north2 = Collider::new(true, false, false, false);
        assert_eq!(north1, north2);
        let west = Collider::new(false, false, false, true);
        assert_ne!(north1, west);
    }

    #[test]
    fn test_try_from() {
        let north_south_from = Collider::try_from("ns--").unwrap_or_else(|e| panic!("{e}"));
        let north_south = Collider::new(true, true, false, false);
        assert_eq!(north_south, north_south_from);
        let east_from = Collider::try_from("--e-").unwrap_or_else(|e| panic!("{e}"));
        let east = Collider::new(false, false, true, false);
        assert_eq!(east, east_from);
        let void_from = Collider::try_from("----").unwrap_or_else(|e| panic!("{e}"));
        let void = Collider::new(false, false, false, false);
        assert_eq!(void, void_from);
    }

    #[test]
    fn test_try_from_fail() {
        let res = Collider::try_from("stars!");
        let err = Err("Value is not formatted as nsew.");
        assert_eq!(res, err);
        let res = Collider::try_from("l-ve");
        let err = Err("First character is not 'n' nor '-'.");
        assert_eq!(res, err);
        let res = Collider::try_from("n-ie");
        let err = Err("Third character is not 'e' nor '-'.");
        assert_eq!(res, err);
    }

    #[test]
    fn test_add() {
        let ns = Collider::try_from("ns--").unwrap();
        let ew = Collider::try_from("--ew").unwrap();
        let all = Collider::try_from("nsew").unwrap();
        assert_eq!(ns + ew, all);
        let sw = Collider::try_from("-s-w").unwrap();
        let w = Collider::try_from("---w").unwrap();
        let s = Collider::try_from("-s--").unwrap();
        assert_ne!(sw + w, s);
        let n1 = Collider::try_from("---w").unwrap();
        let n2 = Collider::try_from("---w").unwrap();
        let n3 = Collider::try_from("---w").unwrap();
        assert_eq!(n1 + n2, n3);
    }

    #[test]
    fn test_blocks() {
        let nw = Collider::try_from("n--w").unwrap();
        assert!(!nw.blocks(&Movement::Left));
        assert!(!nw.blocks(&Movement::Up));
        assert!(nw.blocks(&Movement::Down));
        assert!(nw.blocks(&Movement::Right));
    }

    #[test]
    fn test_collider_vector_from_lines() {
        let lines = vec!["..........@", "..@.......|", "..|........"];
        let collider = Collider::try_from("nsew").unwrap();
        let expected1 = (Coords::new(2, 2), collider.clone());
        let expected2 = (Coords::new(10, 1), collider.clone());
        let res = collider_vector_from_lines(&lines, '|', collider);
        assert!(res.contains(&expected1));
        assert!(res.contains(&expected2));
        assert_eq!(res.len(), 2);
    }
}
