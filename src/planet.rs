use crate::coords::Coords;
use crate::collider::{self, Collider};
use crate::movement::Movement;
use crate::renderable::{self, Renderable};
use std::collections::HashMap;

pub struct Planet {
    name: &'static str,
    map: Vec<&'static str>,
    player_coords: Coords,
    player_sprite: &'static str,
    decorations: HashMap<Coords, Box<dyn Renderable>>,
    colliders: HashMap<Coords, Collider>
}

impl Planet {
    pub fn new(
        name: &'static str,
        map: Vec<&'static str>,
        player_coords: Coords,
        player_sprite: &'static str,
        decorations: Vec<Box<dyn Renderable>>,
        colliders: Vec<Collider>
    ) -> Planet {
        Planet {
            name,
            map,
            player_coords,
            player_sprite,
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

    pub fn generate_map(&self) -> String {
        let mut map_str = String::from("");
        let mut y: i32 = 0;
        for line in &self.map {
            let mut x: i32 = 0;
            for character in line.chars() {
                let current_coords = &Coords::new(x, y);
                if self.decorations.contains_key(current_coords) {
                    if self.decorations[current_coords].get_z_index() > 0 {
                        map_str.push_str(self.decorations[current_coords].get_sprite());
                        x += 1;
                        continue;
                    }
                }
                if x == self.player_coords.get_x() && y == self.player_coords.get_y() {
                    map_str.push_str(self.player_sprite);
                } else {
                    map_str.push(character);
                }
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

    pub fn move_player(&mut self, movement: Movement) {
        match movement {
            Movement::Up => self.player_coords.set_y(self.player_coords.get_y() - 1),
            Movement::Left => self.player_coords.set_x(self.player_coords.get_x() - 1),
            Movement::Down => self.player_coords.set_y(self.player_coords.get_y() + 1),
            Movement::Right => self.player_coords.set_x(self.player_coords.get_x() + 1),
            Movement::Wait => (),
            _other => panic!("Unsupported struct Movement passed to move_player!"),
        }
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

    pub fn get_player_coords(&self) -> &Coords {
        &self.player_coords
    }
}
