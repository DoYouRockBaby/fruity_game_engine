use fruity_any::*;
use fruity_core::resource::resource_reference::ResourceReference;
use fruity_core::*;
use fruity_graphic::resources::material_resource::MaterialResource;

#[derive(Debug, Clone, Component, FruityAny)]
pub struct Sprite {
    pub material: ResourceReference<MaterialResource>,
}
