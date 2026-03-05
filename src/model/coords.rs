use crate::model::movement::Movement;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Coords {
    pub x: i32,
    pub y: i32,
}

impl Coords {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn do_movement(self, movement: &Movement) -> Coords {
        let x = self.x;
        let y = self.y;
        match movement {
            Movement::Up => Coords::new(x, y - 1),
            Movement::Down => Coords::new(x, y + 1),
            Movement::Left => Coords::new(x - 1, y),
            Movement::Right => Coords::new(x + 1, y),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_constructor() {
        let default = Coords { x: 1, y: 7 };
        let constructor = Coords::new(1, 7);
        assert_eq!(default, constructor);
    }

    #[test]
    fn test_mutable_fields() {
        let mut coords = Coords::new(2, 5);
        coords.x = 3;
        coords.y = 4;
        assert_eq!(coords, Coords::new(3, 4));
    }
}
