use crate::components::dynamic_rigid_body::DynamicRigidBody;
use crate::components::kinematic_rigid_body::KinematicRigidBody;
use crate::components::rapier_circle_collider::RapierCircleCollider;
use crate::components::rapier_rect_collider::RapierRectCollider;
use crate::components::static_rigid_body::StaticRigidBody;
use crate::rapier_2d_service::Rapier2dService;
use crate::systems::dynamic_initialize_rigid_body::dynamic_initialize_rigid_body;
use crate::systems::dynamic_update_rigid_body::dynamic_update_rigid_body;
use crate::systems::dynamic_update_rigid_body_prepare::dynamic_update_rigid_body_prepare;
use crate::systems::initialize_circle_collider::initialize_circle_collider;
use crate::systems::initialize_rect_collider::initialize_rect_collider;
use crate::systems::kinematic_initialize_rigid_body::kinematic_initialize_rigid_body;
use crate::systems::kinematic_update_rigid_body::kinematic_update_rigid_body;
use crate::systems::kinematic_update_rigid_body_prepare::kinematic_update_rigid_body_prepare;
use crate::systems::update_circle_collider::update_circle_collider;
use crate::systems::update_physics::update_physics;
use crate::systems::update_rect_collider::update_rect_collider;
use fruity_core::inject::Inject1;
use fruity_core::inject::Inject2;
use fruity_core::object_factory_service::ObjectFactoryService;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::settings::Settings;
use fruity_ecs::extension_component_service::ExtensionComponentService;
use fruity_ecs::system::system_service::StartupSystemParams;
use fruity_ecs::system::system_service::SystemParams;
use fruity_ecs::system::system_service::SystemService;
use fruity_physic_2d::components::circle_collider::CircleCollider;
use fruity_physic_2d::components::rect_collider::RectCollider;
use std::sync::Arc;

pub mod components;
pub mod rapier_2d_service;
pub mod systems;

/// The module name
pub static MODULE_NAME: &str = "fruity_physic_2d_rapier";

// #[no_mangle]
pub fn initialize(resource_container: Arc<ResourceContainer>, _settings: &Settings) {
    let rapier_2d_service = Rapier2dService::new(resource_container.clone());

    resource_container.add::<Rapier2dService>("rapier_2d_service", Box::new(rapier_2d_service));

    let object_factory_service = resource_container.require::<ObjectFactoryService>();
    let mut object_factory_service = object_factory_service.write();

    object_factory_service.register::<DynamicRigidBody>("DynamicRigidBody");
    object_factory_service.register::<KinematicRigidBody>("KinematicRigidBody");
    object_factory_service.register::<StaticRigidBody>("StaticRigidBody");

    let extension_component_service = resource_container.require::<ExtensionComponentService>();
    let mut extension_component_service = extension_component_service.write();

    extension_component_service.register::<CircleCollider, RapierCircleCollider>();
    extension_component_service.register::<RectCollider, RapierRectCollider>();

    let system_service = resource_container.require::<SystemService>();
    let mut system_service = system_service.write();

    system_service.add_system(
        "update_physics",
        MODULE_NAME,
        Inject1::new(update_physics),
        SystemParams {
            pool_index: 52,
            ..Default::default()
        },
    );
    system_service.add_startup_system(
        "initialize_circle_collider",
        MODULE_NAME,
        Inject2::new(initialize_circle_collider),
        StartupSystemParams { ignore_pause: true },
    );
    system_service.add_system(
        "update_circle_collider",
        MODULE_NAME,
        Inject2::new(update_circle_collider),
        SystemParams {
            ignore_pause: true,
            pool_index: 51,
            ..Default::default()
        },
    );
    system_service.add_startup_system(
        "initialize_rect_collider",
        MODULE_NAME,
        Inject2::new(initialize_rect_collider),
        StartupSystemParams { ignore_pause: true },
    );
    system_service.add_system(
        "update_rect_collider",
        MODULE_NAME,
        Inject2::new(update_rect_collider),
        SystemParams {
            ignore_pause: true,
            pool_index: 51,
            ..Default::default()
        },
    );
    system_service.add_startup_system(
        "kinematic_initialize_rigid_body",
        MODULE_NAME,
        Inject2::new(kinematic_initialize_rigid_body),
        StartupSystemParams { ignore_pause: true },
    );
    system_service.add_system(
        "kinematic_update_rigid_body_prepare",
        MODULE_NAME,
        Inject2::new(kinematic_update_rigid_body_prepare),
        SystemParams {
            ignore_pause: true,
            pool_index: 51,
            ..Default::default()
        },
    );
    system_service.add_system(
        "kinematic_update_rigid_body",
        MODULE_NAME,
        Inject2::new(kinematic_update_rigid_body),
        SystemParams {
            ignore_pause: true,
            pool_index: 53,
            ..Default::default()
        },
    );
    system_service.add_startup_system(
        "dynamic_initialize_rigid_body",
        MODULE_NAME,
        Inject2::new(dynamic_initialize_rigid_body),
        StartupSystemParams { ignore_pause: true },
    );
    system_service.add_system(
        "dynamic_update_rigid_body_prepare",
        MODULE_NAME,
        Inject2::new(dynamic_update_rigid_body_prepare),
        SystemParams {
            ignore_pause: true,
            pool_index: 51,
            ..Default::default()
        },
    );
    system_service.add_system(
        "dynamic_update_rigid_body",
        MODULE_NAME,
        Inject2::new(dynamic_update_rigid_body),
        SystemParams {
            ignore_pause: true,
            pool_index: 53,
            ..Default::default()
        },
    );
}
