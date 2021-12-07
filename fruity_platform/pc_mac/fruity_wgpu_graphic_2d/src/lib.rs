use crate::graphic_2d_service::WgpuGraphic2dManager;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::settings::Settings;
use fruity_graphic_2d::graphic_2d_service::Graphic2dService;
use std::sync::Arc;

pub mod graphic_2d_service;

// #[no_mangle]
pub fn initialize(resource_container: Arc<ResourceContainer>, _settings: &Settings) {
    let graphic_2d_service = WgpuGraphic2dManager::new(resource_container.clone());

    resource_container
        .add::<dyn Graphic2dService>("graphic_2d_service", Box::new(graphic_2d_service))
        .unwrap();
}
