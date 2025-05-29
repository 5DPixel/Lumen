use std::collections::HashMap;
use std::any::TypeId;
use crate::ecs::entity::{EntityManager, Entity};
use crate::ecs::components::Component;
use crate::scene::save_load::SceneFormat;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Write;
use std::io::Read;
use zstd::stream::{Encoder, Decoder};
use std::io::{BufReader, BufWriter};

pub struct World {
    entity_manager: EntityManager,
    components: HashMap<TypeId, Box<dyn SceneFormat>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entity_manager: EntityManager::new(),
            components: HashMap::new(),
        }
    }

    pub fn register_component<T>(&mut self)
    where
        T: Component + Serialize + for<'de> Deserialize<'de> + 'static,
    {
        self.components.insert(TypeId::of::<T>(), Box::new(Vec::<Option<T>>::new()));
    }

    pub fn get_storage<T: Component + 'static>(&mut self) -> Option<&mut Vec<Option<T>>> {
        self.components
            .get_mut(&TypeId::of::<T>())
            .and_then(|boxed| boxed.as_any_mut().downcast_mut::<Vec<Option<T>>>())
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
            boxed.as_any().downcast_ref::<Vec<Option<T>>>().and_then(|storage| {
                storage.get(entity.id() as usize).and_then(|opt| opt.as_ref())
            })
        })
    }

    pub fn get_component_mut<T: Component + 'static>(&mut self, entity: Entity) -> Option<&mut T> {
        self.components.get_mut(&TypeId::of::<T>()).and_then(|boxed| {
            boxed.as_any_mut().downcast_mut::<Vec<Option<T>>>().and_then(|storage| {
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

    pub fn save_to_file(&self, path: &str) {
        let file = File::create(path).expect("failed to create file!");
        let mut writer = BufWriter::new(file);

        writer.write_all(b"LSCN").expect("failed to write magic bytes!");

        let mut encoder = Encoder::new(writer, 0).expect("failed to create zstd encoder");

        bincode::serialize_into(&mut encoder, &self.components.len()).expect("failed to write component count!");

        for (_type_id, component_storage) in &self.components {
            component_storage.serialize_component_data(&mut encoder);
        }

        encoder.finish().expect("failed to finish compression");
    }

    pub fn load_from_file(&mut self, path: &str) {
        let file = File::open(path).expect("failed to open file!");
        let mut reader = BufReader::new(file);

        let mut magic = [0u8; 4];
        reader.read_exact(&mut magic).expect("failed to read magic bytes!");
        if &magic != b"LSCN" {
            panic!("invalid file format: magic bytes do not match!");
        }

        let mut decoder = Decoder::new(reader).expect("failed to create zstd decoder");

        let count: usize = bincode::deserialize_from(&mut decoder).expect("failed to read component count!");
        assert_eq!(count, self.components.len(), "component count mismatch!");

        for component_storage in self.components.values_mut() {
            component_storage.deserialize_component_data(&mut decoder);
        }
    }

}
