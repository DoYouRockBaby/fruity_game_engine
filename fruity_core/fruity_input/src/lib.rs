use crate::input_manager::InputManager;
use fruity_core::resource::resource_manager::ResourceManager;
use fruity_core::settings::Settings;
use std::sync::Arc;

pub mod input_manager;

// #[no_mangle]
pub fn initialize(resource_manager: Arc<ResourceManager>, settings: &Settings) {
    let mut input_manager = InputManager::new(resource_manager.clone());
    input_manager.read_input_settings(settings);

    resource_manager
        .add::<InputManager>("input_manager", Box::new(input_manager))
        .unwrap();
}
