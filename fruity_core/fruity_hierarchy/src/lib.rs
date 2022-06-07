use crate::components::parent::Parent;
use crate::systems::delete_cascade::delete_cascade;
use crate::systems::update_nested_level::update_nested_level;
use fruity_core::inject::Inject2;
use fruity_core::object_factory_service::ObjectFactoryService;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::settings::Settings;
use fruity_ecs::system::system_service::StartupSystemParams;
use fruity_ecs::system::system_service::SystemService;
use std::sync::Arc;

pub mod components;
pub mod systems;

/// The module name
pub static MODULE_NAME: &str = "fruity_hierarchy";

// #[no_mangle]
pub fn initialize(resource_container: Arc<ResourceContainer>, _settings: &Settings) {
    let object_factory_service = resource_container.require::<ObjectFactoryService>();
    let mut object_factory_service = object_factory_service.write();

    object_factory_service.register::<Parent>("Parent");

    let system_service = resource_container.require::<SystemService>();
    let mut system_service = system_service.write();

    system_service.add_startup_system(
        "delete_cascade",
        MODULE_NAME,
        Inject2::new(delete_cascade),
        StartupSystemParams { ignore_pause: true },
    );
    system_service.add_startup_system(
        "update_nested_level",
        MODULE_NAME,
        Inject2::new(update_nested_level),
        StartupSystemParams { ignore_pause: true },
    );
}
