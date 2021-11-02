use crate::components::panes::Panes;
use crate::editor_manager::EditorManager;
use fruity_core::service::service_manager::ServiceManager;
use fruity_core::settings::Settings;
use std::sync::Arc;
use std::sync::RwLock;

#[macro_use]
extern crate lazy_static;

pub mod components;
pub mod editor_manager;
pub mod hooks;
pub mod state;
pub mod style;
pub mod ui_element;

// #[no_mangle]
pub fn initialize(service_manager: &Arc<RwLock<ServiceManager>>, _settings: &Settings) {
    let editor_manager = EditorManager::new(service_manager);

    let mut service_manager_writer = service_manager.write().unwrap();
    service_manager_writer.register("editor_manager", editor_manager);
}
