use crate::resources::image_resource::ImageResource;
use crate::resources::texture_resource::TextureResource;
use crate::GraphicsManager;
use fruity_ecs::resource::resources_manager::ResourceIdentifier;
use fruity_ecs::resource::resources_manager::ResourceLoaderParams;
use fruity_ecs::resource::resources_manager::ResourcesManager;
use fruity_ecs::service::service_manager::ServiceManager;
use image::load_from_memory;
use std::io::Read;
use std::sync::Arc;
use std::sync::RwLock;

pub fn image_loader(
    resources_manager: &mut ResourcesManager,
    identifier: ResourceIdentifier,
    reader: &mut dyn Read,
    params: ResourceLoaderParams,
    service_manager: Arc<RwLock<ServiceManager>>,
) {
    let load_type = params.get::<String>("type", "image".to_string());

    // read the whole file
    let mut buffer = Vec::new();
    if let Err(err) = reader.read_to_end(&mut buffer) {
        log::error!("{}", err.to_string());
        return;
    }

    // Load the image
    let image = load_from_memory(&buffer).unwrap();

    if load_type != "texture" {
        // Store the resource if it's a simple image
        let resource = ImageResource::new(image);
        if let Err(_) = resources_manager.add_resource(identifier.clone(), resource) {
            log::error!(
                "Couldn't add a resource cause the identifier \"{}\" already exists",
                &identifier.0
            );
        }
    } else {
        // Get the graphic manager state
        let service_manager = service_manager.read().unwrap();
        let graphics_manager = service_manager.read::<GraphicsManager>();
        let device = graphics_manager.get_device().unwrap();
        let queue = graphics_manager.get_queue().unwrap();

        // Create the texture
        let resource = if let Ok(value) =
            TextureResource::from_image(device, queue, &image, Some(&identifier.0))
        {
            value
        } else {
            log::error!("Couldn't parse a texture");
            return;
        };

        // Store the texture
        if let Err(_) = resources_manager.add_resource(identifier.clone(), resource) {
            log::error!(
                "Couldn't add a resource cause the identifier \"{}\" already exists",
                &identifier.0
            );
        }
    }
}
