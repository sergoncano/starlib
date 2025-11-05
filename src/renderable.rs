use crate::coords::Coords;
use std::collections::HashMap;

pub trait Renderable {
    fn get_sprite(&self) -> &'static str;
    fn get_coords(&self) -> Coords;
    fn get_z_index(&self) -> i32;
}

pub fn get_rendering_hashmap(
    list: Vec<Box<dyn Renderable>>,
) -> HashMap<Coords, Box<dyn Renderable>> {
    let mut map: HashMap<Coords, Box<dyn Renderable>> = HashMap::new();
    for renderable_item in list {
        let coords = renderable_item.get_coords();
        if !map.contains_key(&coords) {
            map.insert(coords, renderable_item);
        } else {
            let current_item = &map[&coords];
            if current_item.get_z_index() < renderable_item.get_z_index() {
                *map.get_mut(&coords).unwrap() = renderable_item;
            } else if current_item.get_z_index() < renderable_item.get_z_index() {
                panic!(
                    "Z-fighting between objects {:?} and {:?}",
                    stringify!(current_item),
                    stringify!(renderable_item)
                );
            }
        }
    }
    map
}
