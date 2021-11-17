use crate::graphic_service::WgpuGraphicManager;
use crate::resources::image_resource::load_image;
use crate::resources::material_resource::load_material;
use crate::resources::shader_resource::load_shader;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::settings::Settings;
use fruity_graphic::graphic_service::GraphicService;
use std::sync::Arc;

pub mod graphic_service;
pub mod resources;

// #[no_mangle]
pub fn initialize(resource_container: Arc<ResourceContainer>, _settings: &Settings) {
    let graphic_service = WgpuGraphicManager::new(resource_container.clone());

    resource_container
        .add::<dyn GraphicService>("graphic_service", Box::new(graphic_service))
        .unwrap();

    resource_container.add_resource_loader("png", load_image);
    resource_container.add_resource_loader("jpeg", load_image);
    resource_container.add_resource_loader("jpg", load_image);
    resource_container.add_resource_loader("gif", load_image);
    resource_container.add_resource_loader("bmp", load_image);
    resource_container.add_resource_loader("ico", load_image);
    resource_container.add_resource_loader("tiff", load_image);
    resource_container.add_resource_loader("wgsl", load_shader);
    resource_container.add_resource_loader("material", load_material);
}
