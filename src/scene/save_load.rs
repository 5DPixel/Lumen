use std::io::{Write, Read};
use serde::Serialize;
use crate::ecs::components::Component;
use serde::de::DeserializeOwned;
use std::any::Any;

pub trait SceneFormat {
    fn serialize_component_data(&self, writer: &mut dyn Write);
    fn deserialize_component_data(&mut self, reader: &mut dyn Read);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl <T> SceneFormat for Vec<Option<T>>
where
    T: Component + Serialize + DeserializeOwned
{
    fn serialize_component_data(&self, writer: &mut dyn Write) {
        for (entity_id, comp) in self.iter().enumerate() {
            if let Some(component) = comp {
                bincode::serialize_into(&mut *writer, &(entity_id as u32)).unwrap();
                bincode::serialize_into(&mut *writer, component).unwrap();
            }
        }
    }

    fn deserialize_component_data(&mut self, reader: &mut dyn Read) {
        loop {
            let entity_id_result = bincode::deserialize_from::<_, u32>(&mut *reader);
            if let Ok(entity_id) = entity_id_result {
                // Try to read the component
                match bincode::deserialize_from::<_, T>(&mut *reader) {
                    Ok(component) => {
                        let index = entity_id as usize;
                        if self.len() <= index {
                            self.resize_with(index + 1, || None);
                        }
                        self[index] = Some(component);
                    }
                    Err(_) => break,
                }
            } else {
                break;
            }
        }
    }


    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
