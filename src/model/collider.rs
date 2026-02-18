use std::ops::Add;

#[derive(Debug, PartialEq, Clone)]
pub struct Collider {
    north: bool,
    south: bool,
    east: bool,
    west: bool,
}

impl Collider {
    fn new(north: bool, south: bool, east: bool, west: bool, ) -> Self {
        Collider { north, south, east, west }
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
        let north_south = Collider::new(true,true,false,false);
        assert_eq!(north_south, north_south_from);
        let east_from = Collider::try_from("--e-").unwrap_or_else(|e| panic!("{e}"));
        let east = Collider::new(false,false,true,false);
        assert_eq!(east, east_from);
        let void_from = Collider::try_from("----").unwrap_or_else(|e| panic!("{e}"));
        let void = Collider::new(false,false,false,false);
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
}
