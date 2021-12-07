use crate::frame_service::FrameService;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::settings::Settings;
use std::sync::Arc;

pub mod frame_service;
pub mod window_service;

/// The module name
pub static MODULE_NAME: &str = "fruity_windows";

pub fn initialize(resource_container: Arc<ResourceContainer>, _settings: &Settings) {
    let frame_service = FrameService::new(resource_container.clone());

    resource_container.add::<FrameService>("frame_service", Box::new(frame_service));
}
