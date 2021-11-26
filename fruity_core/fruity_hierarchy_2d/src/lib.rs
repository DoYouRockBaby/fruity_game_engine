use crate::systems::transform_2d_cascade::transform_2d_cascade;
use fruity_core::inject::Inject1;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::settings::Settings;
use fruity_ecs::system::system_service::SystemService;
use std::sync::Arc;

pub mod systems;

// #[no_mangle]
pub fn initialize(resource_container: Arc<ResourceContainer>, _settings: &Settings) {
    let system_service = resource_container.require::<SystemService>();
    let mut system_service = system_service.write();

    system_service.add_system_that_ignore_pause(Inject1::new(transform_2d_cascade), Some(97));
}
