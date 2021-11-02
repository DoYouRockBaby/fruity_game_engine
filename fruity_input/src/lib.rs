use crate::input_manager::InputManager;
use fruity_core::service::service_manager::ServiceManager;
use fruity_core::settings::Settings;
use std::sync::Arc;
use std::sync::RwLock;

pub mod input_manager;

// #[no_mangle]
pub fn initialize(service_manager: &Arc<RwLock<ServiceManager>>, _settings: &Settings) {
    let input_manager = InputManager::new(service_manager);

    let mut service_manager = service_manager.write().unwrap();
    service_manager.register("input_manager", input_manager);
}
