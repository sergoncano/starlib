#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Coords {
    pub x: i32,
    pub y: i32,
}

impl Coords {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
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
