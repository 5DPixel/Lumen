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
        bincode::serialize_into(writer, self).unwrap();
    }

    fn deserialize_component_data(&mut self, reader: &mut dyn Read) {
        *self = bincode::deserialize_from(reader).unwrap();
    }


    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
