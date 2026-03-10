//! Contains the [Stage] struct.

use std::collections::HashMap;

use crate::{
    Sprite,
    graphics::map::Map,
    model::{collider::Collider, coords::Coords, movement::Movement},
};

/// A complete setting for a game level. Contains a background ([Map]), a set of decorations and a
/// set of colliders.
pub struct Stage {
    pub(crate) map: Map,
    pub(crate) decorations: HashMap<Coords, Sprite>,
    colliders: HashMap<Coords, Collider>,
}

impl Stage {
    pub fn new(
        map: Map,
        decorations: Vec<(Coords, Sprite)>,
        colliders: Vec<(Coords, Collider)>,
    ) -> Self {
        Self {
            map,
            decorations: Self::decoration_vector_to_hashmap(decorations),
            colliders: Self::collider_vector_to_hashmap(colliders),
        }
    }

    fn decoration_vector_to_hashmap(decorations: Vec<(Coords, Sprite)>) -> HashMap<Coords, Sprite> {
        let mut map: HashMap<Coords, Sprite> = HashMap::new();
        for (coords, sprite) in decorations {
            map.entry(coords)
                .and_modify(|c| {
                    c.clone().overlap(sprite.clone());
                })
                .or_insert(sprite);
        }
        map
    }

    fn collider_vector_to_hashmap(
        decorations: Vec<(Coords, Collider)>,
    ) -> HashMap<Coords, Collider> {
        let mut map: HashMap<Coords, Collider> = HashMap::new();
        for (coords, collider) in decorations {
            map.entry(coords)
                .and_modify(|c| {
                    *c = c.clone() + collider.clone();
                })
                .or_insert(collider);
        }
        map
    }

    /// Returns the collider at the specified coords.
    pub fn get_collider(&self, coords: &Coords) -> Option<Collider> {
        Option::<&Collider>::cloned(self.colliders.get(coords))
    }

    /// Sets the collider at the specified coords.
    pub fn set_collider(&mut self, coords: Coords, collider: Collider) {
        self.colliders.insert(coords, collider);
    }

    /// Sets the sprite at the specified coords.
    pub fn get_decoration(&self, coords: &Coords) -> Option<Sprite> {
        Option::<&Sprite>::cloned(self.decorations.get(coords))
    }

    /// Sets the sprite at the specified coords.
    pub fn set_decoration(&mut self, coords: Coords, decoration: Sprite) {
        self.decorations.insert(coords, decoration);
    }

    /// Checks if an entity at the specified coords is blocked by any collider when performing the
    /// specified movement. Returns true if the movement would result in going off-bounds.
    pub fn collides(&self, coords: &Coords, movement: &Movement) -> bool {
        let next_coords = coords.clone().do_movement(movement);
        if self.in_bounds(&next_coords) {
            if let Some(collider) = self.get_collider(&next_coords) {
                collider.blocks(movement)
            } else {
                false
            }
        } else {
            true
        }
    }

    fn in_bounds(&self, coords: &Coords) -> bool {
        let x = coords.x;
        let y = coords.y;
        let map_dimensions = self.map.get_size();
        let map_x = map_dimensions.x;
        let map_y = map_dimensions.y;
        x >= 0 && y >= 0 && x < map_x && y < map_y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructor() {
        let decoration = Sprite::build("@", 2);
        let position = Coords::new(2, 0);
        let _stage = Stage::new(Map::test_map(), vec![(position, decoration)], vec![]);
    }

    #[test]
    fn test_get_collider() {
        let collider = Collider::build("n-e-");
        let coords = Coords::new(2, 1);
        let new_collider = collider.clone();
        let new_coords = coords.clone();
        let stage = Stage::new(Map::test_map(), vec![], vec![(coords, collider)]);
        assert_eq!(
            new_collider,
            stage
                .get_collider(&new_coords)
                .expect("get_collider() returned None.")
        );
    }

    #[test]
    fn test_set_collider() {
        let collider = Collider::build("n-e-");
        let collider2 = Collider::build("n--w");
        let coords = Coords::new(2, 1);
        let mut stage = Stage::new(Map::test_map(), vec![], vec![(coords.clone(), collider)]);
        stage.set_collider(coords.clone(), collider2.clone());
        assert_eq!(stage.get_collider(&coords), Some(collider2));
    }

    #[test]
    fn test_collides() {
        let stage = Stage::new(
            Map::test_map(),
            vec![],
            vec![(Coords::new(1, 0), Collider::build("--e-"))],
        );
        assert!(!stage.collides(&Coords::new(0, 0), &Movement::Right));
        assert!(stage.collides(&Coords::new(2, 0), &Movement::Left));
        assert!(!stage.collides(&Coords::new(0, 0), &Movement::Down));
        assert!(stage.collides(&Coords::new(300, 300), &Movement::Down));
    }
}
