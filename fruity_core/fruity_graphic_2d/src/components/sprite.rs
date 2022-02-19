use fruity_any::*;
use fruity_core::resource::resource_reference::ResourceReference;
use fruity_ecs::*;
use fruity_graphic::resources::material_resource::MaterialResource;
use fruity_graphic::resources::texture_resource::TextureResource;

#[derive(Debug, Clone, Default, Component, FruityAny)]
pub struct Sprite {
    pub material: Option<ResourceReference<dyn MaterialResource>>,
    pub texture: Option<ResourceReference<dyn TextureResource>>,
    pub z_index: i32,
}
