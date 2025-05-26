use cgmath::Vector3;

pub trait Component: 'static {}

impl<T: 'static> Component for T {}

#[derive(Debug)]
pub struct Transform {
    pub position: Vector3<f32>,
    pub rotation: Vector3<f32>,
    pub scale: Vector3<f32>,
}
