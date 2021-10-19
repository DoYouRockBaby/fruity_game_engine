use fruity_any::*;
use fruity_ecs::resource::resource::Resource;
use image::DynamicImage;

#[derive(Debug, FruityAnySyncSend)]
pub struct TextureResource {
    image: DynamicImage,
}

impl TextureResource {
    pub fn new(image: DynamicImage) -> TextureResource {
        TextureResource { image }
    }
}

impl Resource for TextureResource {}
