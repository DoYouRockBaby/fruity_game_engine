use crate::frame_manager::FrameManager;
use fruity_core::resource::resource_manager::ResourceManager;
use fruity_core::settings::Settings;
use std::sync::Arc;

pub mod frame_manager;
pub mod windows_manager;

pub fn initialize(resource_manager: Arc<ResourceManager>, _settings: &Settings) {
    let frame_manager = FrameManager::new(resource_manager.clone());

    resource_manager
        .add::<FrameManager>("frame_manager", Box::new(frame_manager))
        .unwrap();
}
