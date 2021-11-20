use crate::components::local_position::LocalPosition;
use crate::components::local_size::LocalSize;
use crate::systems::position_cascade::position_cascade;
use crate::systems::size_cascade::size_cascade;
use fruity_core::inject::Inject1;
use fruity_core::object_factory_service::ObjectFactoryService;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::settings::Settings;
use fruity_ecs::system::system_service::SystemService;
use std::sync::Arc;

pub mod components;
pub mod systems;

// #[no_mangle]
pub fn initialize(resource_container: Arc<ResourceContainer>, _settings: &Settings) {
    let object_factory_service = resource_container.require::<ObjectFactoryService>();
    let mut object_factory_service = object_factory_service.write();

    object_factory_service.register::<LocalPosition>("LocalPosition");
    object_factory_service.register::<LocalSize>("LocalSize");

    let system_service = resource_container.require::<SystemService>();
    let mut system_service = system_service.write();

    system_service.add_system_that_ignore_pause(Inject1::new(position_cascade), None);
    system_service.add_system_that_ignore_pause(Inject1::new(size_cascade), None);
}
