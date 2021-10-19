use fruity_any::*;
use fruity_ecs::serialize::serialized::ResourceReference;
use fruity_ecs::*;
use fruity_graphic::resources::texture_resource::TextureResource;
use fruity_introspect::*;

#[derive(Debug, Clone, Component, IntrospectFields, FruityAny)]
pub struct Sprite {
    pub texture: ResourceReference<TextureResource>,
}
