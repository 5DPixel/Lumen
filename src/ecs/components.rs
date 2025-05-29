use serde::{Serialize, Deserialize};

pub trait Component: 'static {}

impl<T: 'static> Component for T {}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct SerializableVector3<T> {
    pub x: T,
    pub y: T,
    pub z: T
}

impl<T> SerializableVector3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self {
            x,
            y,
            z
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transform {
    pub position: SerializableVector3<f32>,
    pub rotation: SerializableVector3<f32>,
    pub scale: SerializableVector3<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeshData {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelRenderer {
    pub meshes: Vec<MeshData>,
}
