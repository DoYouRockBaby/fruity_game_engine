use crate::input_service::InputService;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::settings::Settings;
use std::sync::Arc;

pub mod input_service;

// #[no_mangle]
pub fn initialize(resource_container: Arc<ResourceContainer>, settings: &Settings) {
    let mut input_service = InputService::new(resource_container.clone());
    input_service.read_input_settings(settings);

    resource_container
        .add::<InputService>("input_service", Box::new(input_service))
        .unwrap();
}
