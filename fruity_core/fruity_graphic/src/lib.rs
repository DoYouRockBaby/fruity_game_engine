use fruity_core::resource::resource_manager::ResourceManager;
use fruity_core::settings::Settings;
use std::sync::Arc;

pub mod graphic_manager;
pub mod math;
pub mod resources;

// #[no_mangle]
pub fn initialize(_resource_manager: Arc<ResourceManager>, _settings: &Settings) {}
