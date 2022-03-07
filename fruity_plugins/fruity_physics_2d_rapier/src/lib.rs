use crate::components::dynamic_rigid_body::DynamicRigidBody;
use crate::components::kinematic_rigid_body::KinematicRigidBody;
use crate::components::static_rigid_body::StaticRigidBody;
use crate::rapier_2d_service::Rapier2dService;
use crate::systems::update_circle_collider::update_circle_collider;
use fruity_core::inject::Inject2;
use fruity_core::object_factory_service::ObjectFactoryService;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::settings::Settings;
use fruity_ecs::system::system_service::SystemService;
use std::sync::Arc;

pub mod components;
pub mod rapier_2d_service;
pub mod systems;

/// The module name
pub static MODULE_NAME: &str = "fruity_physics_2d_rapier";

// #[no_mangle]
pub fn initialize(resource_container: Arc<ResourceContainer>, _settings: &Settings) {
    let rapier_2d_service = Rapier2dService::new(resource_container.clone());

    resource_container.add::<Rapier2dService>("rapier_2d_service", Box::new(rapier_2d_service));

    let object_factory_service = resource_container.require::<ObjectFactoryService>();
    let mut object_factory_service = object_factory_service.write();

    object_factory_service.register::<DynamicRigidBody>("DynamicRigidBody");
    object_factory_service.register::<KinematicRigidBody>("KinematicRigidBody");
    object_factory_service.register::<StaticRigidBody>("StaticRigidBody");

    let system_service = resource_container.require::<SystemService>();
    let mut system_service = system_service.write();

    system_service.add_system(
        "update_circle_collider",
        MODULE_NAME,
        Inject2::new(update_circle_collider),
        None,
    );
}
