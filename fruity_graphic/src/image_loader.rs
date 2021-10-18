use crate::image_resource::ImageResource;
use fruity_ecs::resource::resources_manager::ResourceIdentifier;
use fruity_ecs::resource::resources_manager::ResourcesManager;
use image::load_from_memory;
use std::io::Read;

pub fn image_loader(
    resources_manager: &mut ResourcesManager,
    identifier: ResourceIdentifier,
    reader: &mut dyn Read,
) {
    // read the whole file
    let mut buffer = Vec::new();
    if let Err(err) = reader.read_to_end(&mut buffer) {
        log::error!("{}", err.to_string());
        return;
    }

    // Load the image
    let image = load_from_memory(&buffer).unwrap();
    let resource = ImageResource::new(image);

    // Store the resource
    if let Err(_) = resources_manager.add_resource(identifier.clone(), resource) {
        log::error!(
            "Couldn't add a resource cause the identifier \"{}\" already exists",
            &identifier.0
        );
        return;
    }
}
