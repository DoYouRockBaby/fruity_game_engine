use fruity_any::*;
use fruity_core::introspect::FieldInfo;
use fruity_core::introspect::IntrospectObject;
use fruity_core::introspect::MethodInfo;
use fruity_core::resource::resource::Resource;
use fruity_ecs::*;

#[repr(C)]
#[derive(
    Copy, Clone, Default, FruityAny, SerializableObject, Debug, bytemuck::Pod, bytemuck::Zeroable,
)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
    pub normal: [f32; 3],
}

pub trait MeshResource: Resource {}

#[derive(Debug, Clone, FruityAny, SerializableObject)]
pub struct MeshResourceSettings {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
}

// TODO: Complete that
impl IntrospectObject for MeshResourceSettings {
    fn get_class_name(&self) -> String {
        "MeshResourceSettings".to_string()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}

// TODO: Complete that
impl IntrospectObject for Vertex {
    fn get_class_name(&self) -> String {
        "Vertex".to_string()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}
