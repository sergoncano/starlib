use std::collections::HashMap;

use crate::{Sprite, model::{collider::Collider, coords::Coords}};

pub struct Stage {
    name: String,
    map: Vec<String>,
    decorations: HashMap<Coords, Sprite>,
    colliders: HashMap<Coords, Collider>,
}

impl Stage {
    pub fn build(name: String, map: Vec<String>, decorations: Vec<(Coords, Sprite)>, colliders: Vec<(Coords, Collider)>) -> Self {
        let mut map_is_valid = true;
        for (line, prev_line) in map.iter().zip(map.iter().skip(1)) {
            map_is_valid  = map_is_valid && line.len() == prev_line.len();
        }
        assert!(map_is_valid, "Map of stage {name} is not rectangular!");
        Self {
            name,
            map,
            decorations: Self::decoration_vector_to_hashmap(decorations),
            colliders: Self::collider_vector_to_hashmap(colliders),
        }
    }

    fn decoration_vector_to_hashmap(decorations: Vec<(Coords, Sprite)>) -> HashMap<Coords, Sprite> {
        let mut map: HashMap<Coords, Sprite> = HashMap::new();
        for (coords, sprite) in decorations {
            map.entry(coords).and_modify(|c| { c.clone().overlap(sprite.clone()); }).or_insert(sprite);
        };
        map
    }

    fn collider_vector_to_hashmap(decorations: Vec<(Coords, Collider)>) -> HashMap<Coords, Collider> {
        let mut map: HashMap<Coords, Collider> = HashMap::new();
        for (coords, collider) in decorations {
            map.entry(coords).and_modify(|c| { *c = c.clone() + collider.clone(); }).or_insert(collider);
        };
        map
    }

    pub fn get_collider(self, coords: Coords) -> Option<Collider> {
        Option::<&Collider>::cloned(self.colliders.get(&coords))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder() {
        let decoration = Sprite::build("@", 2);
        let position = Coords::new(2, 0);
        let _stage = Stage::build(
            String::from("Test stage"),
            vec!["...", "..|", "o.."].iter().map(|&s| String::from(s)).collect(),
            vec![(position, decoration)],
            vec![],
        );
    }

    #[test]
    #[should_panic]
    fn name() {
        let _stage = Stage::build(
            String::from("Fail stage"),
            vec!["..", "..|", "o.."].iter().map(|&s| String::from(s)).collect(),
            vec![],
            vec![],
        );
    }

    #[test]
    fn test_get_collider() {
        let collider = Collider::try_from("n-e-").unwrap();
        let coords = Coords::new(2,1);
        let new_collider = collider.clone();
        let new_coords= coords.clone();
        let stage = Stage::build(
            String::from("Test stage"),
            vec!["..@", "..|", "o.."].iter().map(|&s| String::from(s)).collect(),
            vec![],
            vec![(coords, collider)],
        );
        assert_eq!(new_collider, stage.get_collider(new_coords).expect("get_collider() returned None."));
    }
}
