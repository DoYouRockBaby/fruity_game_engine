use fruity_any::*;
use fruity_ecs::resource::resource::Resource;

#[derive(Debug, FruityAnySyncSend)]
pub struct TextureResource {
    pub texture: wgpu::Texture,
}

impl TextureResource {
    pub fn new(texture: wgpu::Texture) -> TextureResource {
        TextureResource { texture }
    }
}

impl Resource for TextureResource {}
