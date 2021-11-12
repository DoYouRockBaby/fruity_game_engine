use fruity_any::*;
use fruity_core::resource::resource_reference::OptionResourceReference;
use fruity_core::*;
use fruity_graphic::resources::material_resource::MaterialResource;

#[derive(Debug, Clone, Component, FruityAny)]
pub struct Sprite {
    pub material: OptionResourceReference<dyn MaterialResource>,
    pub z_index: usize,
}

impl Default for Sprite {
    fn default() -> Self {
        Self {
            material: OptionResourceReference::empty(),
            z_index: 0,
        }
    }
}
