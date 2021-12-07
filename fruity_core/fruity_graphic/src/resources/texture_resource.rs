use crate::graphic_service::GraphicService;
use fruity_core::resource::resource::Resource;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::settings::Settings;
use std::io::Read;
use std::sync::Arc;

pub struct TextureResourceSettings {}

pub trait TextureResource: Resource {}

pub fn load_texture(
    identifier: &str,
    reader: &mut dyn Read,
    settings: Settings,
    resource_container: Arc<ResourceContainer>,
) {
    // Get the graphic service state
    let graphic_service = resource_container.require::<dyn GraphicService>();
    let graphic_service = graphic_service.read();

    // read the whole file
    let mut buffer = Vec::new();
    if let Err(err) = reader.read_to_end(&mut buffer) {
        log::error!("{}", err.to_string());
        return;
    }

    // Parse settings
    let settings = read_texture_settings(&settings);

    // Build the resource
    let result = graphic_service.create_texture_resource(identifier, &buffer, settings);

    // Store the resource
    match result {
        Ok(resource) => {
            resource_container.add::<dyn TextureResource>(identifier, resource);
        }
        Err(err) => {
            log::error!("{}", err);
        }
    }
}

pub fn read_texture_settings(_settings: &Settings) -> TextureResourceSettings {
    TextureResourceSettings {}
}
