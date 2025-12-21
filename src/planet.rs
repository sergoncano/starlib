use crate::coords::Coords;
use crate::collider::{self, Collider};
use crate::entities::Entity;
use crate::map;
use crate::movement::Movement;
use crate::renderable::{self, Renderable};
use std::collections::HashMap;

pub struct Planet {
    name: &'static str,
    map: Vec<&'static str>,
    decorations: HashMap<Coords, Box<dyn Renderable>>,
    colliders: HashMap<Coords, Collider>
}

impl Planet {
    pub fn new(
        name: &'static str,
        map: Vec<&'static str>,
        decorations: Vec<Box<dyn Renderable>>,
        colliders: Vec<Collider>
    ) -> Planet {
        map::check_map(&map);
        Planet {
            name,
            map,
            decorations: renderable::get_rendering_hashmap(decorations),
            colliders: collider::collider_map_from_vector(colliders),
        }
    }

    fn get_map_size_x(&self) -> usize {
        let mut size_x: usize = 0;
        for _ in self.map[0].chars() {
            size_x += 1;
        }
        size_x
    }

    fn get_map_size_y(&self) -> usize {
        self.map.len()
    }

    pub fn generate_map(&self, entities: &Vec<Box<dyn Entity>>) -> String {
        let mut map_str = String::from("");
        let mut entity_hashmap = HashMap::new();
        for entity in entities {
            entity_hashmap.insert(entity.get_coords(), entity);
        }
        let mut y: i32 = 0;
        for line in &self.map {
            let mut x: i32 = 0;
            for character in line.chars() {
                let current_coords = &Coords::new(x, y);
                if self.decorations.contains_key(current_coords) || entity_hashmap.contains_key(current_coords) {
                    if self.decorations.contains_key(current_coords) && entity_hashmap.contains_key(current_coords) {
                        if self.decorations[current_coords].get_z_index() > entity_hashmap[current_coords].get_z_index() {
                            map_str.push_str(self.decorations[current_coords].get_sprite());
                        } else if self.decorations[current_coords].get_z_index() < entity_hashmap[current_coords].get_z_index()  {
                            map_str.push_str(entity_hashmap[current_coords].get_sprite());
                        } else {
                            panic!("Z fighting between during map generation!");
                        }
                    } else if self.decorations.contains_key(current_coords) {
                        map_str.push_str(self.decorations[current_coords].get_sprite());
                    } else {
                        map_str.push_str(entity_hashmap[current_coords].get_sprite());
                    }
                    x += 1;
                    continue;
                }
                map_str.push(character);
                x += 1;
            }
            map_str.push('\n');
            y += 1;
        }
        map_str
    }

    pub fn generate_banner(&self) -> String {
        let mut banner = String::from("");
        if self.name.len() > self.get_map_size_x() {
            return String::from("");
        }
        let dashes: f64 = (self.get_map_size_x()- self.name.len()) as f64 / 2.0;
        for _i in 0..(dashes.floor() as i32) {
            banner.push('-');
        }
        banner.push_str(self.name);
        for _i in 0..(dashes.ceil() as i32) {
            banner.push('-');
        }
        banner.push('\n');
        banner
    }

    //Takes coordinates and a movement and returns the same movement if it is valid. If invalid
    //returns Movement::Invalid.
    pub fn check_movement_collision(&self, coords: &Coords, movement: Movement) -> Movement {
        match movement {
            Movement::Up => {
                let future_coords = Coords::new(coords.get_x(), coords.get_y() - 1);
                if coords.get_y() < 1 || (self.colliders.contains_key(&future_coords) && self.colliders[&future_coords].collides(&movement)) {
                    Movement::Invalid
                } else {
                    movement
                }
            },
            Movement::Left => {
                let future_coords = Coords::new(coords.get_x() - 1, coords.get_y());
                if coords.get_x() < 1  || (self.colliders.contains_key(&future_coords) && self.colliders[&future_coords].collides(&movement)) {
                    Movement::Invalid
                } else {
                    movement
                }
            },
            Movement::Down => {
                let future_coords = Coords::new(coords.get_x(), coords.get_y() + 1);
                if coords.get_y() + 1 >= self.get_map_size_y() as i32 || (self.colliders.contains_key(&future_coords) && self.colliders[&future_coords].collides(&movement)) { 
                    Movement::Invalid
                } else {
                    movement
                }
            },
            Movement::Right => {
                let future_coords = Coords::new(coords.get_x() + 1, coords.get_y());
                if coords.get_x() + 1 >= self.get_map_size_x() as i32 || (self.colliders.contains_key(&future_coords) && self.colliders[&future_coords].collides(&movement)) { 
                    Movement::Invalid
                } else {
                    movement
                }
            },
            _ => { movement }
        }
    }
}
