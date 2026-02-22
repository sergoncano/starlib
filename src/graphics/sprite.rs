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
    pub fn overlap (self, other: Sprite) -> Self {
        if self.z_index == other.z_index {
            panic!("Z-fighting between {:?} and {:?}", self, other);
        } else if self.z_index > other.z_index {
            self
        } else {
            other
        }
    }
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
}
