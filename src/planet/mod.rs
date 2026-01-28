pub mod earth;

use std::collections::HashMap;

use crate::{
    interfaces::{
        entity::Entity,
        renderable::{self, Renderable},
    },
    model::{
        collider::{self, Collider},
        coords::Coords,
        movement::Movement,
    },
    util::map,
};

pub struct Planet {
    name: &'static str,
    map: Vec<&'static str>,
    decorations: HashMap<Coords, Box<dyn Renderable>>,
    colliders: HashMap<Coords, Collider>,
}

impl Planet {
    pub fn new(
        name: &'static str,
        map: Vec<&'static str>,
        decorations: Vec<Box<dyn Renderable>>,
        colliders: Vec<Collider>,
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

    pub fn generate_map(&self, entity_vector: &Vec<Box<dyn Entity>>) -> String {
        let mut map_str = String::from("");
        let mut entities: HashMap<Coords, &Box<dyn Entity>> = HashMap::new();
        for entity in entity_vector {
            let coords = entity.get_coords();
            if !entities.contains_key(&coords)
                || entities[&coords].get_z_index() < entity.get_z_index()
            {
                entities.insert(coords, entity);
            } else if entities[&coords].get_z_index() == entity.get_z_index() {
                panic!("Z fighting during map rendering!");
            }
        }
        for (y, line) in self.map.iter().enumerate() {
            for (x, character) in line.chars().enumerate() {
                let current_coords = Coords::new(x as i32, y as i32);
                let mut sprite = character.to_string();
                let mut z_index = i32::MIN;
                if self.decorations.contains_key(&current_coords) {
                    sprite = self.decorations[&current_coords].get_sprite();
                    z_index = self.decorations[&current_coords].get_z_index();
                }
                if entities.contains_key(&current_coords) {
                    let entity_z_index = entities[&current_coords].get_z_index();
                    if entity_z_index == z_index {
                        panic!("Z fighting during map rendering!");
                    } else if entity_z_index > z_index {
                        sprite = entities[&current_coords].get_sprite();
                    }
                }
                map_str.push_str(&sprite);
            }
            map_str.push('\n');
        }
        map_str
    }

    pub fn generate_banner(&self) -> String {
        let mut banner = String::from("");
        if self.name.len() > self.get_map_size_x() {
            return String::from("");
        }
        let dashes: f64 = (self.get_map_size_x() - self.name.len()) as f64 / 2.0;
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
                if coords.get_y() < 1
                    || (self.colliders.contains_key(&future_coords)
                        && self.colliders[&future_coords].collides(&movement))
                {
                    Movement::Invalid
                } else {
                    movement
                }
            }
            Movement::Left => {
                let future_coords = Coords::new(coords.get_x() - 1, coords.get_y());
                if coords.get_x() < 1
                    || (self.colliders.contains_key(&future_coords)
                        && self.colliders[&future_coords].collides(&movement))
                {
                    Movement::Invalid
                } else {
                    movement
                }
            }
            Movement::Down => {
                let future_coords = Coords::new(coords.get_x(), coords.get_y() + 1);
                if coords.get_y() + 1 >= self.get_map_size_y() as i32
                    || (self.colliders.contains_key(&future_coords)
                        && self.colliders[&future_coords].collides(&movement))
                {
                    Movement::Invalid
                } else {
                    movement
                }
            }
            Movement::Right => {
                let future_coords = Coords::new(coords.get_x() + 1, coords.get_y());
                if coords.get_x() + 1 >= self.get_map_size_x() as i32
                    || (self.colliders.contains_key(&future_coords)
                        && self.colliders[&future_coords].collides(&movement))
                {
                    Movement::Invalid
                } else {
                    movement
                }
            }
            _ => movement,
        }
    }
}
