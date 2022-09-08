use crate::javascript_watcher_service::JavascriptWatcherService;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::settings::Settings;

pub mod javascript_watcher_service;

/// The module name
pub static MODULE_NAME: &str = "fruity_javascript_watcher";

// #[no_mangle]
pub fn initialize(resource_container: ResourceContainer, _settings: &Settings) {
    let javascript_watcher_service = JavascriptWatcherService::new(resource_container.clone());

    resource_container.add::<JavascriptWatcherService>(
        "javascript_watcher_service",
        Box::new(javascript_watcher_service),
    );
}
