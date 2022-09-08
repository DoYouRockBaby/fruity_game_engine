use crate::components::circle_collider::CircleCollider;
use crate::components::rect_collider::RectCollider;
use fruity_core::object_factory_service::ObjectFactoryService;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::settings::Settings;

pub mod components;

/// The module name
pub static MODULE_NAME: &str = "fruity_physic_2d";

// #[no_mangle]
pub fn initialize(resource_container: ResourceContainer, _settings: &Settings) {
    let object_factory_service = resource_container.require::<ObjectFactoryService>();
    let mut object_factory_service = object_factory_service.write();

    object_factory_service.register::<CircleCollider>("CircleCollider");
    object_factory_service.register::<RectCollider>("RectCollider");
}
