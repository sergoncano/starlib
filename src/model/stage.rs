use std::collections::HashMap;

use crate::{
    Sprite,
    graphics::map::Map,
    model::{collider::Collider, coords::Coords},
};

pub struct Stage {
    pub(crate) map: Map,
    pub(crate) decorations: HashMap<Coords, Sprite>,
    colliders: HashMap<Coords, Collider>,
}

impl Stage {
    pub fn build(
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

    pub fn get_collider(self, coords: &Coords) -> Option<Collider> {
        Option::<&Collider>::cloned(self.colliders.get(coords))
    }

    pub fn set_collider(&mut self, coords: Coords, collider: Collider) {
        self.colliders.insert(coords, collider);
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
            Map::test_map(),
            vec![(position, decoration)],
            vec![],
        );
    }

    #[test]
    fn test_get_collider() {
        let collider = Collider::try_from("n-e-").unwrap();
        let coords = Coords::new(2, 1);
        let new_collider = collider.clone();
        let new_coords = coords.clone();
        let stage = Stage::build(
            Map::test_map(),
            vec![],
            vec![(coords, collider)],
        );
        assert_eq!(
            new_collider,
            stage
                .get_collider(&new_coords)
                .expect("get_collider() returned None.")
        );
    }

    #[test]
    fn test_set_collider() {
        let collider = Collider::try_from("n-e-").unwrap();
        let collider2 = Collider::try_from("n--w").unwrap();
        let coords = Coords::new(2, 1);
        let mut stage = Stage::build(
            Map::test_map(),
            vec![],
            vec![(coords.clone(), collider)],
        );
        stage.set_collider(coords.clone(), collider2.clone());
        assert_eq!(stage.get_collider(&coords), Some(collider2));
    }
}
