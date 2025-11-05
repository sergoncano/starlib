use crate::coords::Coords;
use crate::movement::Movement;
use crate::renderable::{self, Renderable};
use std::collections::HashMap;

pub struct Planet<const MAP_SIZE_X: usize, const MAP_SIZE_Y: usize> {
    name: &'static str,
    map: [&'static str; MAP_SIZE_Y],
    player_coords: Coords,
    player_sprite: &'static str,
    decorations: HashMap<Coords, Box<dyn Renderable>>,
}

impl<const MAP_SIZE_X: usize, const MAP_SIZE_Y: usize> Planet<MAP_SIZE_X, MAP_SIZE_Y> {
    pub fn new(
        name: &'static str,
        map: [&'static str; MAP_SIZE_Y],
        player_coords: Coords,
        player_sprite: &'static str,
        decorations: Vec<Box<dyn Renderable>>,

    ) -> Planet<MAP_SIZE_X, MAP_SIZE_Y> {
        Planet::<MAP_SIZE_X, MAP_SIZE_Y> {
            name,
            map,
            player_coords,
            player_sprite,
            decorations: renderable::get_rendering_hashmap(decorations),
        }
    }

    pub fn generate_map(&self) -> String {
        let mut map_str = String::from("");
        let mut y: i32 = 0;
        for line in self.map {
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
        if self.name.len() > MAP_SIZE_X {
            return String::from("");
        }
        let dashes: f64 = (MAP_SIZE_X - self.name.len()) as f64 / 2.0;
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
                if coords.get_y() >= 1 {
                    Movement::Up
                } else {
                    Movement::Invalid
                }
            }
            Movement::Left => {
                if coords.get_x() >= 1 {
                    Movement::Left
                } else {
                    Movement::Invalid
                }
            }
            Movement::Down => {
                if coords.get_y() + 1 < MAP_SIZE_Y as i32 {
                    Movement::Down
                } else {
                    Movement::Invalid
                }
            }
            Movement::Right => {
                if coords.get_x() + 1 < MAP_SIZE_X as i32 {
                    Movement::Right
                } else {
                    Movement::Invalid
                }
            }
            Movement::Wait => Movement::Wait,
            Movement::Invalid => Movement::Invalid,
            Movement::Quit => Movement::Quit,
        }
    }

    pub fn get_player_coords(&self) -> &Coords {
        &self.player_coords
    }
}
