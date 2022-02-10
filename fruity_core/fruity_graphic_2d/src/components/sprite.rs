use fruity_any::*;
use fruity_core::resource::resource_reference::ResourceReference;
use fruity_ecs::*;
use fruity_graphic::math::material_reference::MaterialReference;
use fruity_graphic::resources::texture_resource::TextureResource;

#[derive(Debug, Clone, Default, Component, FruityAny)]
pub struct Sprite {
    pub material: Option<Box<dyn MaterialReference>>,
    pub texture: Option<ResourceReference<dyn TextureResource>>,
    pub z_index: i32,
}
