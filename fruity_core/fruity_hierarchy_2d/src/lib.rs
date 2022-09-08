use crate::systems::transform_2d_cascade::transform_2d_cascade;
use fruity_core::inject::Inject2;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::settings::Settings;
use fruity_ecs::system::system_service::SystemParams;
use fruity_ecs::system::system_service::SystemService;

pub mod systems;

/// The module name
pub static MODULE_NAME: &str = "fruity_hierarchy_2d";

// #[no_mangle]
pub fn initialize(resource_container: ResourceContainer, _settings: &Settings) {
    let system_service = resource_container.require::<SystemService>();
    let mut system_service = system_service.write();

    system_service.add_system(
        "transform_2d_cascade",
        MODULE_NAME,
        Inject2::new(transform_2d_cascade),
        SystemParams {
            pool_index: 96,
            ignore_pause: true,
        },
    );
}
