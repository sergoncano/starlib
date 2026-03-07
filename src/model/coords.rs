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

    /// Returns the Manhattan distance between to points.
    /// ```
    /// use starlib::Coords;
    /// let c1 = Coords::new(1,1);
    /// let c2 = Coords::new(-1,-1);
    /// assert_eq!(c1.distance(&c2), 4);
    /// ```
    pub fn distance(&self, other: &Coords) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
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

    #[test]
    fn test_do_movement() {
        let mut c = Coords::new(2, 2);
        c = c.do_movement(&Movement::Right);
        assert_eq!(Coords::new(3, 2), c);
        c = c.do_movement(&Movement::Down);
        assert_eq!(Coords::new(3, 3), c);
        c = c.do_movement(&Movement::Left);
        assert_eq!(Coords::new(2, 3), c);
        c = c.do_movement(&Movement::Up);
        assert_eq!(Coords::new(2, 2), c);
    }
}
