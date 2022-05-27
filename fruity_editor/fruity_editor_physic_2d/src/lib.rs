use crate::component_inspector::circle_collider_inspector::circle_collider_inspector;
use crate::component_inspector::rect_collider_inspector::rect_collider_inspector;
use crate::state::collider::ColliderState;
use crate::systems::draw_circle_collider_2d_gizmos::draw_circle_collider_2d_gizmos;
use crate::systems::draw_rect_collider_2d_gizmos::draw_rectangle_collider_2d_gizmos;
use fruity_core::inject::Inject3;
use fruity_core::inject::Inject4;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::settings::Settings;
use fruity_ecs::system::system_service::SystemParams;
use fruity_ecs::system::system_service::SystemService;
use fruity_editor::editor_component_service::EditorComponentService;
use fruity_editor::editor_component_service::RegisterComponentParams;
use fruity_editor::hooks::declare_global;
use std::sync::Arc;

pub mod component_inspector;
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
        Inject4::new(draw_circle_collider_2d_gizmos),
        Some(SystemParams {
            pool_index: 98,
            ignore_pause: true,
        }),
    );

    system_service.add_system(
        "draw_rectangle_collider_2d_gizmos",
        MODULE_NAME,
        Inject3::new(draw_rectangle_collider_2d_gizmos),
        Some(SystemParams {
            pool_index: 98,
            ignore_pause: true,
        }),
    );

    let editor_component_service = resource_container.require::<EditorComponentService>();
    let mut editor_component_service = editor_component_service.write();

    editor_component_service.register_component(
        "CircleCollider",
        RegisterComponentParams {
            inspector: Arc::new(circle_collider_inspector),
            ..Default::default()
        },
    );
    editor_component_service.register_component(
        "RectCollider",
        RegisterComponentParams {
            inspector: Arc::new(rect_collider_inspector),
            ..Default::default()
        },
    );
}
