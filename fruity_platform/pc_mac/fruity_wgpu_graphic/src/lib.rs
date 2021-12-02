use crate::graphic_service::WgpuGraphicManager;
use crate::resources::shader_resource::load_shader;
use crate::resources::texture_resource::load_texture;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::settings::Settings;
use fruity_graphic::graphic_service::GraphicService;
use std::sync::Arc;

pub mod graphic_service;
pub mod math;
pub mod resources;

// #[no_mangle]
pub fn initialize(resource_container: Arc<ResourceContainer>, _settings: &Settings) {
    let graphic_service = WgpuGraphicManager::new(resource_container.clone());

    resource_container
        .add::<dyn GraphicService>("graphic_service", Box::new(graphic_service))
        .unwrap();

    resource_container.add_resource_loader("png", load_texture);
    resource_container.add_resource_loader("jpeg", load_texture);
    resource_container.add_resource_loader("jpg", load_texture);
    resource_container.add_resource_loader("gif", load_texture);
    resource_container.add_resource_loader("bmp", load_texture);
    resource_container.add_resource_loader("ico", load_texture);
    resource_container.add_resource_loader("tiff", load_texture);
    resource_container.add_resource_loader("wgsl", load_shader);
}
