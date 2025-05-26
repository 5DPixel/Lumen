use std::collections::HashMap;
use std::any::Any;
use std::any::TypeId;
use crate::ecs::entity::{EntityManager, Entity};
use crate::ecs::components::Component;

pub struct World {
    entity_manager: EntityManager,
    components: HashMap<TypeId, Box<dyn Any>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entity_manager: EntityManager::new(),
            components: HashMap::new(),
        }
    }

    pub fn register_component<T: Component + 'static>(&mut self) {
        self.components.insert(TypeId::of::<T>(), Box::new(Vec::<Option<T>>::new()));
    }

    pub fn get_storage<T: Component + 'static>(&mut self) -> Option<&mut Vec<Option<T>>> {
        self.components
            .get_mut(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_mut::<Vec<Option<T>>>())
    }

    pub fn add_component<T: Component + 'static>(&mut self, entity: Entity, component: T) {
        let storage = self.get_storage::<T>().expect("component not registered!");
        let id = entity.id() as usize;
        if storage.len() <= id {
            storage.resize_with(id + 1, || None);
        }
        storage[id] = Some(component);
    }


    pub fn get_component<T: Component + 'static>(&self, entity: Entity) -> Option<&T> {
        self.components.get(&TypeId::of::<T>()).and_then(|boxed| {
            boxed.downcast_ref::<Vec<Option<T>>>().and_then(|storage| {
                storage.get(entity.id() as usize).and_then(|opt| opt.as_ref())
            })
        })
    }

    pub fn get_component_mut<T: Component + 'static>(&mut self, entity: Entity) -> Option<&mut T> {
        self.components.get_mut(&TypeId::of::<T>()).and_then(|boxed| {
            boxed.downcast_mut::<Vec<Option<T>>>().and_then(|storage| {
                storage.get_mut(entity.id() as usize).and_then(|opt| opt.as_mut())
            })
        })
    }

    pub fn remove_component<T: Component + 'static>(&mut self, entity: Entity) {
        if let Some(storage) = self.get_storage::<T>() {
            let id = entity.id() as usize;
            if id < storage.len() {
                storage[id] = None;
            }
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        self.entity_manager.create_entity()
    }
}
