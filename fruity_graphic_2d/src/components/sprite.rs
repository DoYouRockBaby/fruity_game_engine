use fruity_any::*;
use fruity_core::serialize::serialized::ResourceReference;
use fruity_core::*;
use fruity_graphic::resources::material_resource::MaterialResource;
use fruity_introspect::*;

#[derive(Debug, Clone, Component, IntrospectFields, FruityAny)]
pub struct Sprite {
    pub material: ResourceReference<MaterialResource>,
}
