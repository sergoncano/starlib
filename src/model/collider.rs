//! Contains the [Collider] struct and a helper function to create instances of it.
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

    /// Creates a collider from a template string. The string must have the format 'nsew' where
    /// each letter represents the collider blocking from a cardinal. If the collider should not
    /// block from that direction, use '-' instead.
    /// # Examples
    /// ```
    /// use starlib::Collider;
    /// let col1 = Collider::build("ns--");
    /// let col2 = Collider::new(true, true, false, false);
    /// assert_eq!(col1, col2);
    /// ```
    /// # Panics
    /// Panics if the value does not match the specified format. E.g.: When it is not 4 chars long,
    /// when one of its chars is not 'n', 's', 'e', 'w' or '-' or when those characters aren't in
    /// order.
    /// ```should_panic
    /// use starlib::Collider;
    /// Collider::build("nwe-"); // Out of order
    /// ```
    /// ```should_panic
    /// use starlib::Collider;
    /// Collider::build("nse"); // Wrong length
    /// ```
    /// ```should_panic
    /// use starlib::Collider;
    /// Collider::build("Starlib!"); // Wrong chars
    /// ```
    pub fn build(template: &str) -> Self {
        if template.len() != 4 {
            panic!("Value is not formatted as nsew.");
        }
        let expected_chars = ['n', 's', 'e', 'w'];
        let mut cardinals = [false, false, false, false];
        for (i, (character, (expected, cardinal))) in template
            .chars()
            .zip(expected_chars.iter().zip(cardinals.iter_mut()))
            .enumerate()
        {
            *cardinal = if &character == expected {
                true
            } else if character == '-' {
                false
            } else {
                panic!(
                    "Character at position {i} is not '{expected}' nor '-'. Instead it is: '{character}'."
                );
            }
        }
        Collider {
            north: cardinals[0],
            south: cardinals[1],
            east: cardinals[2],
            west: cardinals[3],
        }
    }
}

/// If two colliders are added together, their collisions are OR'd.
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

/// Returns a Vec<(Coords, Collider)>, based on every appearance the character provided in the
/// lines parameter. The collider copied for every appearance is always the provided one.
/// Useful when you want to create a large amount of colliders in a map but don't want to do it
/// manually.
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
    fn test_build() {
        let north_south_from = Collider::build("ns--");
        let north_south = Collider::new(true, true, false, false);
        assert_eq!(north_south, north_south_from);
        let east_from = Collider::build("--e-");
        let east = Collider::new(false, false, true, false);
        assert_eq!(east, east_from);
        let void_from = Collider::build("----");
        let void = Collider::new(false, false, false, false);
        assert_eq!(void, void_from);
    }

    #[test]
    #[should_panic(expected = "Value is not formatted as nsew.")]
    fn test_build_fail_5_chars() {
        Collider::build("stars!");
    }

    #[test]
    #[should_panic(expected = "Character at position 0 is not 'n' nor '-'. Instead it is: 'l'.")]
    fn test_build_fail_first_character() {
        Collider::build("l-ve");
    }

    #[test]
    #[should_panic(expected = "Character at position 2 is not 'e' nor '-'. Instead it is: 'i'.")]
    fn test_build_fail_third_character() {
        Collider::build("n-ie");
    }

    #[test]
    fn test_add() {
        let ns = Collider::build("ns--");
        let ew = Collider::build("--ew");
        let all = Collider::build("nsew");
        assert_eq!(ns + ew, all);
        let sw = Collider::build("-s-w");
        let w = Collider::build("---w");
        let s = Collider::build("-s--");
        assert_ne!(sw + w, s);
        let n1 = Collider::build("---w");
        let n2 = Collider::build("---w");
        let n3 = Collider::build("---w");
        assert_eq!(n1 + n2, n3);
    }

    #[test]
    fn test_blocks() {
        let nw = Collider::build("n--w");
        assert!(!nw.blocks(&Movement::Left));
        assert!(!nw.blocks(&Movement::Up));
        assert!(nw.blocks(&Movement::Down));
        assert!(nw.blocks(&Movement::Right));
    }

    #[test]
    fn test_collider_vector_from_lines() {
        let lines = vec!["..........@", "..@.......|", "..|........"];
        let collider = Collider::build("nsew");
        let expected1 = (Coords::new(2, 2), collider.clone());
        let expected2 = (Coords::new(10, 1), collider.clone());
        let res = collider_vector_from_lines(&lines, '|', collider);
        assert!(res.contains(&expected1));
        assert!(res.contains(&expected2));
        assert_eq!(res.len(), 2);
    }
}
