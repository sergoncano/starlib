use crate::coords::Coords;
use crate::renderable::Renderable;

struct Decoration {
    sprite: &'static str,
    coords: Coords,
    z_index: i32,
}

impl Renderable for Decoration {
    fn get_sprite(&self) -> &'static str {
        self.sprite
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
                    sprite: &line[x..x + 1],
                    coords: Coords::new(x as i32, y as i32),
                    z_index: z_index,
                };
                res.push(Box::from(decoration));
            }
        }
    }
    res
}
