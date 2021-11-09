use crate::component_editor_manager::ComponentEditorManager;
use crate::components::fields::primitive::draw_editor_bool;
use crate::components::fields::primitive::draw_editor_f32;
use crate::components::fields::primitive::draw_editor_f64;
use crate::components::fields::primitive::draw_editor_i16;
use crate::components::fields::primitive::draw_editor_i32;
use crate::components::fields::primitive::draw_editor_i64;
use crate::components::fields::primitive::draw_editor_i8;
use crate::components::fields::primitive::draw_editor_isize;
use crate::components::fields::primitive::draw_editor_string;
use crate::components::fields::primitive::draw_editor_u16;
use crate::components::fields::primitive::draw_editor_u32;
use crate::components::fields::primitive::draw_editor_u64;
use crate::components::fields::primitive::draw_editor_u8;
use crate::components::fields::primitive::draw_editor_usize;
use crate::editor_manager::EditorManager;
use crate::file_explorer_manager::FileExplorerManager;
use crate::resources::default_resources::load_default_resources;
use crate::systems::pause_at_startup::pause_at_startup;
use fruity_core::resource::resources_manager::ResourcesManager;
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
pub mod file_explorer_manager;
pub mod hooks;
pub mod resources;
pub mod state;
pub mod systems;
pub mod ui_element;

// #[no_mangle]
pub fn initialize(service_manager: &Arc<RwLock<ServiceManager>>, _settings: &Settings) {
    let editor_manager = EditorManager::new(service_manager);
    let component_editor_manager = ComponentEditorManager::new(service_manager);
    let file_explorer_manager = FileExplorerManager::new(service_manager);

    let mut service_manager_writer = service_manager.write().unwrap();
    service_manager_writer.register("editor_manager", editor_manager);
    service_manager_writer.register("component_editor_manager", component_editor_manager);
    service_manager_writer.register("file_explorer_manager", file_explorer_manager);

    let mut system_manager = service_manager_writer.write::<SystemManager>();
    system_manager.add_begin_system(pause_at_startup, Some(98));

    let mut component_editor_manager = service_manager_writer.write::<ComponentEditorManager>();
    component_editor_manager.register_component_field_editor::<i8, _>(draw_editor_i8);
    component_editor_manager.register_component_field_editor::<i16, _>(draw_editor_i16);
    component_editor_manager.register_component_field_editor::<i32, _>(draw_editor_i32);
    component_editor_manager.register_component_field_editor::<i64, _>(draw_editor_i64);
    component_editor_manager.register_component_field_editor::<isize, _>(draw_editor_isize);
    component_editor_manager.register_component_field_editor::<u8, _>(draw_editor_u8);
    component_editor_manager.register_component_field_editor::<u16, _>(draw_editor_u16);
    component_editor_manager.register_component_field_editor::<u32, _>(draw_editor_u32);
    component_editor_manager.register_component_field_editor::<u64, _>(draw_editor_u64);
    component_editor_manager.register_component_field_editor::<usize, _>(draw_editor_usize);
    component_editor_manager.register_component_field_editor::<f32, _>(draw_editor_f32);
    component_editor_manager.register_component_field_editor::<f64, _>(draw_editor_f64);
    component_editor_manager.register_component_field_editor::<bool, _>(draw_editor_bool);
    component_editor_manager.register_component_field_editor::<String, _>(draw_editor_string);

    let resources_manager = service_manager_writer.get::<ResourcesManager>().unwrap();
    std::mem::drop(system_manager);
    std::mem::drop(component_editor_manager);
    std::mem::drop(service_manager_writer);

    load_default_resources(resources_manager);
}
