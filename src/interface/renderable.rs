use std::collections::HashMap;

use crate::model::coords::Coords;

pub trait Renderable {
    fn get_sprite(&self) -> String;
    fn get_coords(&self) -> Coords;
    fn get_z_index(&self) -> i32;
}

pub fn get_rendering_hashmap(
    list: Vec<Box<dyn Renderable>>,
) -> HashMap<Coords, Box<dyn Renderable>> {
    let mut map: HashMap<Coords, Box<dyn Renderable>> = HashMap::new();
    for renderable_item in list {
        let coords = renderable_item.get_coords();
        if !map.contains_key(&coords)
            || (map.contains_key(&coords)
                && map.get(&coords).unwrap().get_z_index() < renderable_item.get_z_index())
        {
            map.insert(coords, renderable_item);
        }
    }
    map
}
