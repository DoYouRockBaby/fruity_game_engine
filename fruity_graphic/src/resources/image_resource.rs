use crate::resources::texture_resource::TextureResource;
use crate::GraphicsManager;
use fruity_any::*;
use fruity_core::resource::resource::Resource;
use fruity_core::resource::resources_manager::ResourceIdentifier;
use fruity_core::resource::resources_manager::ResourceLoaderParams;
use fruity_core::resource::resources_manager::ResourcesManager;
use fruity_core::service::service_manager::ServiceManager;
use fruity_introspect::FieldInfo;
use fruity_introspect::IntrospectObject;
use fruity_introspect::MethodInfo;
use image::load_from_memory;
use image::DynamicImage;
use std::io::Read;
use std::sync::Arc;
use std::sync::RwLock;

#[derive(Debug, FruityAny)]
pub struct ImageResource {
    image: DynamicImage,
}

impl ImageResource {
    fn from_buffer(buffer: &[u8]) -> ImageResource {
        let image = load_from_memory(&buffer).unwrap();
        ImageResource { image }
    }
}

impl Resource for ImageResource {}

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

    if load_type != "texture" {
        // Store the resource if it's a simple image
        let resource = ImageResource::from_buffer(&buffer);
        if let Err(_) = resources_manager.add_resource(identifier.clone(), resource) {
            log::error!(
                "Couldn't add a resource cause the identifier \"{}\" already exists",
                &identifier.0
            );
        }
    } else {
        // Load the image
        let image = load_from_memory(&buffer).unwrap();

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

impl IntrospectObject for ImageResource {
    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }

    fn as_introspect_arc(self: Arc<Self>) -> Arc<dyn IntrospectObject> {
        self
    }
}
