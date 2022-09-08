use crate::systems::draw_physic_debug::draw_physic_debug;
use fruity_core::inject::Inject2;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::settings::Settings;
use fruity_ecs::system::system_service::SystemParams;
use fruity_ecs::system::system_service::SystemService;

pub mod systems;

/// The module name
pub static MODULE_NAME: &str = "fruity_physic_2d_rapier";

// #[no_mangle]
pub fn initialize(resource_container: ResourceContainer, _settings: &Settings) {
    let system_service = resource_container.require::<SystemService>();
    let mut system_service = system_service.write();

    system_service.add_system(
        "draw_physic_debug",
        MODULE_NAME,
        Inject2::new(draw_physic_debug),
        SystemParams {
            pool_index: 98,
            ignore_pause: true,
        },
    );
}
