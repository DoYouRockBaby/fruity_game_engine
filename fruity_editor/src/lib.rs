use crate::component_editor_manager::ComponentEditorManager;
use crate::editor_manager::EditorManager;
use crate::gizmos_service::GizmosService;
use crate::systems::draw_gizmos_2d::draw_gizmos_2d_untyped;
use fruity_core::service::service_manager::ServiceManager;
use fruity_core::settings::Settings;
use fruity_core::system::system_manager::SystemManager;
use std::sync::Arc;
use std::sync::RwLock;

#[macro_use]
extern crate lazy_static;

pub mod component_editor_manager;
pub mod components;
pub mod editor_manager;
pub mod gizmos_service;
pub mod hooks;
pub mod state;
pub mod systems;
pub mod ui_element;

// #[no_mangle]
pub fn initialize(service_manager: &Arc<RwLock<ServiceManager>>, _settings: &Settings) {
    let editor_manager = EditorManager::new(service_manager);
    let gizmos_service = GizmosService::new(service_manager);
    let component_editor_manager = ComponentEditorManager::new(service_manager);

    let mut service_manager_writer = service_manager.write().unwrap();
    service_manager_writer.register("editor_manager", editor_manager);
    service_manager_writer.register("component_editor_manager", component_editor_manager);
    service_manager_writer.register("gizmos_service", gizmos_service);

    let mut system_manager = service_manager_writer.write::<SystemManager>();
    system_manager.add_system(draw_gizmos_2d_untyped, Some(101));

    let mut component_editor_manager = service_manager_writer.write::<ComponentEditorManager>();
    component_editor_manager.register_component_field_editor::<i8>();
    component_editor_manager.register_component_field_editor::<i16>();
    component_editor_manager.register_component_field_editor::<i32>();
    component_editor_manager.register_component_field_editor::<i64>();
    component_editor_manager.register_component_field_editor::<isize>();
    component_editor_manager.register_component_field_editor::<u8>();
    component_editor_manager.register_component_field_editor::<u16>();
    component_editor_manager.register_component_field_editor::<u32>();
    component_editor_manager.register_component_field_editor::<u64>();
    component_editor_manager.register_component_field_editor::<usize>();
    component_editor_manager.register_component_field_editor::<f32>();
    component_editor_manager.register_component_field_editor::<f64>();
    component_editor_manager.register_component_field_editor::<bool>();
    component_editor_manager.register_component_field_editor::<String>();
}
