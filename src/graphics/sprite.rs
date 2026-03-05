use crate::Coords;

#[derive(Debug, PartialEq, Clone)]
pub struct Sprite {
    character: char,
    z_index: i32,
}

impl Sprite {
    pub fn new(character: char, z_index: i32) -> Self {
        Self { z_index, character }
    }

    /// Creates a sprite from a string slice.
    /// The slice is cloned so lifetimes are irrelevant.
    /// The slice *must* be 1 character long, or the function will panic.
    /// # Examples
    /// ```
    /// use starlib::Sprite;
    /// let sprite1 = Sprite::new('ඞ', 2);
    /// let sprite2 = Sprite::build("ඞ", 2);
    /// assert_eq!(sprite1, sprite2);
    /// ```
    /// # Panics
    /// The function panics when the slice is not 1 character long (Note character: utf
    /// characters aren't 1 byte long, but this function will work on them. Likewise, it will not
    /// accept pieces of a utf character).
    /// ```should_panic
    /// use starlib::Sprite;
    /// Sprite::build("Hello, World!", 3);
    /// ```
    pub fn build(string: &str, z_index: i32) -> Self {
        if string.chars().count() != 1 {
            panic!("Tried to create a sprite of size > 1!");
        }
        Self {
            z_index,
            character: string.chars().next().unwrap(),
        }
    }

    /// Get the sprite that should be rendered according to their z-index.
    /// Panics if both have the same z-index. The user must take care of z-fighting themselves.
    /// # Panics
    /// ```should_panic
    /// use starlib::Sprite;
    /// let a = Sprite::new('A', 2);
    /// let b = Sprite::new('B', 2);
    /// let rendered = a.overlap(b);
    /// ```
    pub fn overlap(self, other: Sprite) -> Self {
        if self.z_index == other.z_index {
            panic!("Z-fighting between {:?} and {:?}", self, other);
        } else if self.z_index > other.z_index {
            self
        } else {
            other
        }
    }

    pub(crate) fn get_z_index(&self) -> i32 {
        self.z_index
    }

    pub(crate) fn get_character(&self) -> char {
        self.character
    }
}

pub fn sprite_vector_from_lines(
    lines: &Vec<&str>,
    character: char,
    z_index: i32,
) -> Vec<(Coords, Sprite)> {
    let mut res = vec![];
    for (y, line) in lines.iter().enumerate() {
        for (x, current_character) in line.chars().enumerate() {
            if current_character == character {
                res.push((
                    Coords::new(x as i32, y as i32),
                    Sprite::new(character, z_index),
                ));
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
        let sprite1 = Sprite {
            z_index: 1,
            character: '@',
        };
        let sprite2 = Sprite::new('@', 1);
        assert_eq!(sprite1, sprite2);
    }

    #[test]
    fn test_builder() {
        let sprite1 = Sprite::new('#', 0);
        let sprite2 = Sprite::build("#", 0);
        assert_eq!(sprite1, sprite2);
    }

    #[test]
    #[should_panic]
    fn test_builder_fail() {
        Sprite::build("Hello, World!", 0);
    }

    #[test]
    fn test_overlap() {
        let x = Sprite::build("X", 2);
        let x2 = x.clone();
        let y = Sprite::build("Y", 1);
        assert_eq!(x, y.overlap(x2));
        let a = Sprite::build("A", 2);
        let a2 = a.clone();
        let b = Sprite::build("B", 1);
        assert_eq!(a, a2.overlap(b));
    }

    #[test]
    fn test_sprite_vector_from_lines() {
        let lines = vec!["..........@", "..@.......|", "..|........"];
        let sprite = Sprite::new('@', 2);
        let expected1 = (Coords::new(2, 1), sprite.clone());
        let expected2 = (Coords::new(10, 0), sprite.clone());
        let res = sprite_vector_from_lines(&lines, '@', 2);
        assert!(res.contains(&expected1));
        assert!(res.contains(&expected2));
        assert_eq!(res.len(), 2);
    }
}
