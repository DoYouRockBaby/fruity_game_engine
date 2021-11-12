use crate::graphic_manager::WgpuGraphicsManager;
use crate::resources::image_resource::load_image;
use crate::resources::material_resource::load_material;
use crate::resources::shader_resource::load_shader;
use fruity_core::resource::resource_manager::ResourceManager;
use fruity_core::settings::Settings;
use fruity_graphic::graphic_manager::GraphicManager;
use std::sync::Arc;

pub mod graphic_manager;
pub mod resources;

// #[no_mangle]
pub fn initialize(resource_manager: Arc<ResourceManager>, _settings: &Settings) {
    let graphic_manager = WgpuGraphicsManager::new(resource_manager.clone());

    resource_manager
        .add::<dyn GraphicManager>("graphic_manager", Box::new(graphic_manager))
        .unwrap();

    resource_manager.add_resource_loader("png", load_image);
    resource_manager.add_resource_loader("jpeg", load_image);
    resource_manager.add_resource_loader("jpg", load_image);
    resource_manager.add_resource_loader("gif", load_image);
    resource_manager.add_resource_loader("bmp", load_image);
    resource_manager.add_resource_loader("ico", load_image);
    resource_manager.add_resource_loader("tiff", load_image);
    resource_manager.add_resource_loader("wgsl", load_shader);
    resource_manager.add_resource_loader("material", load_material);
}
