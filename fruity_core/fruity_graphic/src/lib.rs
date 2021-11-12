use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::settings::Settings;
use std::sync::Arc;

pub mod graphic_service;
pub mod math;
pub mod resources;

// #[no_mangle]
pub fn initialize(_resource_container: Arc<ResourceContainer>, _settings: &Settings) {}
