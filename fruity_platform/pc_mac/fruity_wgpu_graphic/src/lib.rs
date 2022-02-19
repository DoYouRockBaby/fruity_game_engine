use crate::graphic_service::WgpuGraphicService;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::settings::Settings;
use fruity_graphic::graphic_service::GraphicService;
use std::sync::Arc;

pub mod graphic_service;
pub mod resources;
pub mod wgpu_bridge;

/// The module name
pub static MODULE_NAME: &str = "fruity_wgpu_graphic";

// #[no_mangle]
pub fn initialize(resource_container: Arc<ResourceContainer>, _settings: &Settings) {
    let graphic_service = WgpuGraphicService::new(resource_container.clone());

    resource_container.add::<dyn GraphicService>("graphic_service", Box::new(graphic_service));
}
