use crate::inspect_component::inspect_circle_collider::inspect_circle_collider;
use crate::inspect_component::inspect_rect_collider::inspect_rect_collider;
use crate::state::collider::ColliderState;
use crate::systems::draw_circle_collider_2d_gizmos::draw_circle_collider_2d_gizmos;
use crate::systems::draw_rect_collider_2d_gizmos::draw_rectangle_collider_2d_gizmos;
use fruity_core::inject::Inject2;
use fruity_core::inject::Inject3;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::settings::Settings;
use fruity_ecs::system::system_service::SystemParams;
use fruity_ecs::system::system_service::SystemService;
use fruity_editor::component_inspector_service::ComponentInspectorService;
use fruity_editor::hooks::declare_global;
use std::sync::Arc;

pub mod inspect_component;
pub mod state;
pub mod systems;

/// The module name
pub static MODULE_NAME: &str = "fruity_editor_physic_2d";

// #[no_mangle]
pub fn initialize(resource_container: Arc<ResourceContainer>, _settings: &Settings) {
    declare_global(ColliderState::new(resource_container.clone()));

    let system_service = resource_container.require::<SystemService>();
    let mut system_service = system_service.write();

    system_service.add_system(
        "draw_circle_collider_2d_gizmos",
        MODULE_NAME,
        Inject3::new(draw_circle_collider_2d_gizmos),
        Some(SystemParams {
            pool_index: 99,
            ignore_pause: true,
        }),
    );

    system_service.add_system(
        "draw_rectangle_collider_2d_gizmos",
        MODULE_NAME,
        Inject2::new(draw_rectangle_collider_2d_gizmos),
        Some(SystemParams {
            pool_index: 99,
            ignore_pause: true,
        }),
    );

    let component_inspector_service = resource_container.require::<ComponentInspectorService>();
    let mut component_inspector_service = component_inspector_service.write();

    component_inspector_service
        .register_inspect_component("CircleCollider", inspect_circle_collider);
    component_inspector_service.register_inspect_component("RectCollider", inspect_rect_collider);
}
