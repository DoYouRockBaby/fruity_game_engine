use crate::components::component::math::draw_editor_vector_2d;
use crate::gizmos_service::GizmosService;
use crate::systems::draw_gizmos_2d::draw_gizmos_2d_untyped;
use fruity_core::service::service_manager::ServiceManager;
use fruity_core::settings::Settings;
use fruity_core::system::system_manager::SystemManager;
use fruity_editor::component_editor_manager::ComponentEditorManager;
use fruity_graphic_2d::math::vector2d::Vector2d;
use std::sync::Arc;
use std::sync::RwLock;

pub mod components;
pub mod gizmos_service;
pub mod systems;

// #[no_mangle]
pub fn initialize(service_manager: &Arc<RwLock<ServiceManager>>, _settings: &Settings) {
    let gizmos_service = GizmosService::new(service_manager);

    let mut service_manager_writer = service_manager.write().unwrap();
    service_manager_writer.register("gizmos_service", gizmos_service);

    let mut system_manager = service_manager_writer.write::<SystemManager>();
    system_manager.add_system(draw_gizmos_2d_untyped, Some(98));

    let mut component_editor_manager = service_manager_writer.write::<ComponentEditorManager>();
    component_editor_manager.register_component_field_editor::<Vector2d, _>(draw_editor_vector_2d);
}
