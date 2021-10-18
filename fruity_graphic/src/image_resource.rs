use fruity_any_derive::*;
use fruity_ecs::resource::resource::Resource;
use image::DynamicImage;

#[derive(Debug, FruityAnySyncSend)]
pub struct ImageResource {
    image: DynamicImage,
}

impl ImageResource {
    pub fn new(image: DynamicImage) -> ImageResource {
        ImageResource { image }
    }
}

impl Resource for ImageResource {}
