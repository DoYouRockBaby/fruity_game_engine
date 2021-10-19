use crate::image_resource::ImageResource;
use crate::texture_resource::TextureResource;
use crate::GraphicsManager;
use fruity_ecs::resource::resources_manager::ResourceIdentifier;
use fruity_ecs::resource::resources_manager::ResourceLoaderParams;
use fruity_ecs::resource::resources_manager::ResourcesManager;
use fruity_ecs::service::service_manager::ServiceManager;
use image::load_from_memory;
use image::GenericImageView;
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

        // If we need to create a texture, we proceed
        let dimensions = image.dimensions();
        let texture_size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            // All textures are stored as 3D, we represent our 2D texture
            // by setting depth to 1.
            size: texture_size,
            mip_level_count: 1, // We'll talk about this a little later
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            // Most images are stored using sRGB so we need to reflect that here.
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            // TEXTURE_BINDING tells wgpu that we want to use this texture in shaders
            // COPY_DST means that we want to copy data to this texture
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            label: Some("texture"),
        });

        // Store the resource if it's a simple image
        let resource = TextureResource::new(texture);
        if let Err(_) = resources_manager.add_resource(identifier.clone(), resource) {
            log::error!(
                "Couldn't add a resource cause the identifier \"{}\" already exists",
                &identifier.0
            );
        }
    }
}
