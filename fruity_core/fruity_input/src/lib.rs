use crate::drag_service::DragService;
use crate::input_service::InputService;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::settings::Settings;

pub mod drag_service;
pub mod input_service;

/// The module name
pub static MODULE_NAME: &str = "fruity_input";

// #[no_mangle]
pub fn initialize(resource_container: ResourceContainer, settings: &Settings) {
    let mut input_service = InputService::new(resource_container.clone());
    input_service.read_input_settings(settings);
    resource_container.add::<InputService>("input_service", Box::new(input_service));

    let drag_service = DragService::new(resource_container.clone());
    resource_container.add::<DragService>("drag_service", Box::new(drag_service));
}
