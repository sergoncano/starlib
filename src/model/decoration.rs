use crate::{interfaces::renderable::Renderable, model::coords::Coords};

struct Decoration {
    sprite: String,
    coords: Coords,
    z_index: i32,
}

impl Renderable for Decoration {
    fn get_sprite(&self) -> String {
        self.sprite.clone()
    }

    fn get_coords(&self) -> Coords {
        self.coords.clone()
    }

    fn get_z_index(&self) -> i32 {
        self.z_index
    }
}

pub fn decoration_vector_from_map(
    map: &[&'static str],
    character: char,
    z_index: i32,
) -> Vec<Box<dyn Renderable>> {
    let mut res: Vec<Box<dyn Renderable>> = Vec::new();
    for (y, line) in map.iter().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == character {
                let decoration = Decoration {
                    sprite: String::from(char),
                    coords: Coords::new(x as i32, y as i32),
                    z_index,
                };
                res.push(Box::from(decoration));
            }
        }
    }
    res
}
