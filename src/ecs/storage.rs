use std::collections::HashMap;
use crate::ecs::entity::Entity;

pub struct ComponentStorage<T> {
    data: HashMap<Entity, T>
}

impl<T> ComponentStorage<T> {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
}
