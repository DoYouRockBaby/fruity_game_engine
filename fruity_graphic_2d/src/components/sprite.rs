use fruity_any::*;
use fruity_core::resource::resource_reference::ResourceReference;
use fruity_core::*;
use fruity_graphic::resources::material_resource::MaterialResource;

#[derive(Debug, Clone, Component, FruityAny)]
pub struct Sprite {
    pub material: ResourceReference<MaterialResource>,
    pub z_index: usize,
}

impl Default for Sprite {
    fn default() -> Self {
        Self {
            material: ResourceReference::new(),
            z_index: 0,
        }
    }
}
