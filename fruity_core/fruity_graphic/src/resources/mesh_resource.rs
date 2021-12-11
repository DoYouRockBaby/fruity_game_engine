use crate::math::vector3d::Vector3d;
use crate::Vector2d;
use fruity_any::*;
use fruity_core::resource::resource::Resource;
use fruity_ecs::*;

#[repr(C)]
#[derive(
    Copy,
    Clone,
    Default,
    FruityAny,
    IntrospectObject,
    SerializableObject,
    Debug,
    bytemuck::Pod,
    bytemuck::Zeroable,
)]
pub struct Vertex {
    pub position: Vector3d,
    pub tex_coords: Vector2d,
    pub normal: Vector3d,
}

pub trait MeshResource: Resource {}

#[derive(Debug, Clone, FruityAny, IntrospectObject, SerializableObject)]
pub struct MeshResourceSettings {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
}
