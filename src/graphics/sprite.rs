#[derive(Debug, PartialEq)]
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
}
