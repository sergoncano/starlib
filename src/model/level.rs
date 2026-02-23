use crate::model::{entity::Entity, stage::Stage};

pub struct Level<T> {
    stage: Stage,
    entities: Vec<Box<dyn Entity<T>>>,
}
