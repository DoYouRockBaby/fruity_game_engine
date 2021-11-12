use crate::resources::texture_resource::WgpuTextureResource;
use crate::GraphicManager;
use crate::WgpuGraphicsManager;
use fruity_any::*;
use fruity_core::resource::resource::Resource;
use fruity_core::resource::resource_manager::ResourceManager;
use fruity_core::settings::Settings;
use fruity_graphic::resources::image_resource::ImageResource;
use fruity_graphic::resources::texture_resource::TextureResource;
use fruity_introspect::FieldInfo;
use fruity_introspect::IntrospectObject;
use fruity_introspect::MethodInfo;
use image::load_from_memory;
use image::DynamicImage;
use std::io::Read;
use std::sync::Arc;

#[derive(Debug, FruityAny)]
pub struct WgpuImageResource {
    image: DynamicImage,
}

impl ImageResource for WgpuImageResource {}

impl WgpuImageResource {
    fn from_buffer(buffer: &[u8]) -> WgpuImageResource {
        let image = load_from_memory(&buffer).unwrap();
        WgpuImageResource { image }
    }
}

impl Resource for WgpuImageResource {}

pub fn load_image(
    identifier: &str,
    reader: &mut dyn Read,
    settings: Settings,
    resource_manager: Arc<ResourceManager>,
) {
    let load_type = settings.get::<String>("type", "image".to_string());

    // read the whole file
    let mut buffer = Vec::new();
    if let Err(err) = reader.read_to_end(&mut buffer) {
        log::error!("{}", err.to_string());
        return;
    }

    if load_type != "texture" {
        // Store the resource if it's a simple image
        let resource = WgpuImageResource::from_buffer(&buffer);
        if let Err(_) =
            resource_manager.add::<dyn ImageResource>(identifier.clone(), Box::new(resource))
        {
            log::error!(
                "Couldn't add a resource cause the identifier \"{}\" already exists",
                identifier
            );
        }
    } else {
        // Load the image
        let image = load_from_memory(&buffer).unwrap();

        // Get the graphic manager state
        let graphic_manager = resource_manager.require::<dyn GraphicManager>("graphic_manager");
        let graphic_manager = graphic_manager.read();
        let graphic_manager = graphic_manager.downcast_ref::<WgpuGraphicsManager>();

        let device = graphic_manager.get_device();
        let queue = graphic_manager.get_queue();

        // Create the texture
        let resource = if let Ok(value) =
            WgpuTextureResource::from_image(device, queue, &image, Some(&identifier))
        {
            value
        } else {
            log::error!("Couldn't parse a texture");
            return;
        };

        // Store the texture
        if let Err(_) =
            resource_manager.add::<dyn TextureResource>(identifier.clone(), Box::new(resource))
        {
            log::error!(
                "Couldn't add a resource cause the identifier \"{}\" already exists",
                identifier
            );
        }
    }
}

impl IntrospectObject for WgpuImageResource {
    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}
