use crate::model::movement::Movement;

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct Coords {
    x: i32,
    y: i32,
}

impl Coords {
    pub fn new(x: i32, y: i32) -> Coords {
        Coords { x, y }
    }

    pub fn get_x(&self) -> i32 {
        self.x
    }

    pub fn get_y(&self) -> i32 {
        self.y
    }

    pub fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: i32) {
        self.y = y;
    }

    pub fn do_movement(&mut self, movement: Movement) {
        (self.x, self.y) = match movement {
            Movement::Up => (self.x, self.y - 1),
            Movement::Left => (self.x - 1, self.y),
            Movement::Down => (self.x, self.y + 1),
            Movement::Right => (self.x + 1, self.y),
            _other => (self.x, self.y),
        };
    }
}
