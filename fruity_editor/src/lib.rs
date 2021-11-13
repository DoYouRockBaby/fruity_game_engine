use crate::component_editor_service::ComponentEditorService;
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
use crate::file_explorer_service::FileExplorerService;
use crate::hooks::declare_global;
use crate::resources::default_resources::load_default_resources;
use crate::state::entity::EntityState;
use crate::state::file_explorer::FileExplorerState;
use crate::state::theme::ThemeState;
use crate::state::world::WorldState;
use crate::systems::pause_at_startup::pause_at_startup;
use fruity_core::inject::Inject1;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::settings::Settings;
use fruity_ecs::system::system_service::SystemService;
use std::sync::Arc;

#[macro_use]
extern crate lazy_static;

pub mod component_editor_service;
pub mod components;
pub mod file_explorer_service;
pub mod hooks;
pub mod resources;
pub mod state;
pub mod systems;
pub mod ui_element;

// #[no_mangle]
pub fn initialize(resource_container: Arc<ResourceContainer>, _settings: &Settings) {
    let component_editor_service = ComponentEditorService::new(resource_container.clone());
    let file_explorer_service = FileExplorerService::new(resource_container.clone());

    resource_container
        .add_require::<ComponentEditorService>(
            "component_editor_service",
            Box::new(component_editor_service),
        )
        .unwrap();
    resource_container
        .add_require::<FileExplorerService>(
            "file_explorer_service",
            Box::new(file_explorer_service),
        )
        .unwrap();

    declare_global(WorldState::new(resource_container.clone()));
    declare_global(ThemeState::default());
    declare_global(EntityState::default());
    declare_global(FileExplorerState::default());

    let system_service = resource_container.require::<SystemService>();
    let mut system_service = system_service.write();

    system_service.add_begin_system(Inject1::new(pause_at_startup), None);

    let component_editor_service = resource_container.require::<ComponentEditorService>();
    let mut component_editor_service = component_editor_service.write();

    component_editor_service.register_component_field_editor::<i8, _>(draw_editor_i8);
    component_editor_service.register_component_field_editor::<i16, _>(draw_editor_i16);
    component_editor_service.register_component_field_editor::<i32, _>(draw_editor_i32);
    component_editor_service.register_component_field_editor::<i64, _>(draw_editor_i64);
    component_editor_service.register_component_field_editor::<isize, _>(draw_editor_isize);
    component_editor_service.register_component_field_editor::<u8, _>(draw_editor_u8);
    component_editor_service.register_component_field_editor::<u16, _>(draw_editor_u16);
    component_editor_service.register_component_field_editor::<u32, _>(draw_editor_u32);
    component_editor_service.register_component_field_editor::<u64, _>(draw_editor_u64);
    component_editor_service.register_component_field_editor::<usize, _>(draw_editor_usize);
    component_editor_service.register_component_field_editor::<f32, _>(draw_editor_f32);
    component_editor_service.register_component_field_editor::<f64, _>(draw_editor_f64);
    component_editor_service.register_component_field_editor::<bool, _>(draw_editor_bool);
    component_editor_service.register_component_field_editor::<String, _>(draw_editor_string);

    load_default_resources(resource_container);
}
