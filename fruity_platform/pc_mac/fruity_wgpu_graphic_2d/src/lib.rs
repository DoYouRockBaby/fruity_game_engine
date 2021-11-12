use crate::graphic_2d_manager::WgpuGraphics2dManager;
use fruity_core::resource::resource_manager::ResourceManager;
use fruity_core::settings::Settings;
use fruity_graphic_2d::graphic_2d_manager::Graphic2dManager;
use std::sync::Arc;

pub mod graphic_2d_manager;

// #[no_mangle]
pub fn initialize(resource_manager: Arc<ResourceManager>, _settings: &Settings) {
    let graphic_2d_manager = WgpuGraphics2dManager::new(resource_manager.clone());

    resource_manager
        .add::<dyn Graphic2dManager>("graphic_2d_manager", Box::new(graphic_2d_manager))
        .unwrap();
}
