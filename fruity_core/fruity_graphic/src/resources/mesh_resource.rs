use fruity_any::*;
use fruity_core::convert::FruityInto;
use fruity_core::convert::FruityTryFrom;
use fruity_core::introspect::FieldInfo;
use fruity_core::introspect::IntrospectObject;
use fruity_core::introspect::MethodInfo;
use fruity_core::resource::resource::Resource;
use fruity_core::serialize::serialized::SerializableObject;
use fruity_core::serialize::serialized::Serialized;

#[repr(C)]
#[derive(Copy, Clone, Default, FruityAny, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
    pub normal: [f32; 3],
}

pub trait MeshResource: Resource {}

#[derive(Debug, Clone, FruityAny)]
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

impl SerializableObject for MeshResourceSettings {
    fn duplicate(&self) -> Box<dyn SerializableObject> {
        Box::new(self.clone())
    }
}

impl FruityTryFrom<Serialized> for MeshResourceSettings {
    type Error = String;

    fn fruity_try_from(value: Serialized) -> Result<Self, Self::Error> {
        match value {
            Serialized::NativeObject(value) => {
                match value.as_any_box().downcast::<MeshResourceSettings>() {
                    Ok(value) => Ok(*value),
                    Err(_) => Err(format!(
                        "Couldn't convert a MeshResourceSettings to native object"
                    )),
                }
            }
            _ => Err(format!("Couldn't convert {:?} to native object", value)),
        }
    }
}

impl FruityInto<Serialized> for MeshResourceSettings {
    fn fruity_into(self) -> Serialized {
        Serialized::NativeObject(Box::new(self))
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

impl SerializableObject for Vertex {
    fn duplicate(&self) -> Box<dyn SerializableObject> {
        Box::new(self.clone())
    }
}

impl FruityTryFrom<Serialized> for Vertex {
    type Error = String;

    fn fruity_try_from(value: Serialized) -> Result<Self, Self::Error> {
        match value {
            Serialized::NativeObject(value) => match value.as_any_box().downcast::<Vertex>() {
                Ok(value) => Ok(*value),
                Err(_) => Err(format!("Couldn't convert a Vertex to native object")),
            },
            _ => Err(format!("Couldn't convert {:?} to native object", value)),
        }
    }
}

impl FruityInto<Serialized> for Vertex {
    fn fruity_into(self) -> Serialized {
        Serialized::NativeObject(Box::new(self))
    }
}