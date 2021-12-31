use fruity_any::*;
use fruity_core::resource::resource_reference::ResourceReference;
use fruity_ecs::*;
use fruity_graphic::math::Color;
use fruity_graphic::resources::texture_resource::TextureResource;

#[derive(Debug, Clone, Component, FruityAny)]
pub struct Camera {
    pub near: f32,
    pub far: f32,
    pub target: Option<ResourceReference<dyn TextureResource>>,
    pub background_color: Color,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            near: -1.0,
            far: 1.0,
            target: None,
            background_color: Color::default(),
        }
    }
}
