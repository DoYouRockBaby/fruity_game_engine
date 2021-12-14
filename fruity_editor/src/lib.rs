use crate::component_inspector_service::ComponentInspectorService;
use crate::file_explorer_service::FileExplorerService;
use crate::hooks::declare_global;
use crate::inspect::inspect_entity::inspect_entity;
use crate::inspector_service::InspectorService;
use crate::introspect_editor_service::IntrospectEditorService;
use crate::resources::default_resources::load_default_resources;
use crate::state::file_explorer::FileExplorerState;
use crate::state::inspector::InspectorState;
use crate::state::scene::SceneState;
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

pub mod component_inspector_service;
pub mod components;
pub mod dialog_service;
pub mod fields;
pub mod file_explorer_service;
pub mod hooks;
pub mod inspect;
pub mod inspector_service;
pub mod introspect_editor_service;
pub mod resources;
pub mod state;
pub mod systems;
pub mod ui_element;

/// The module name
pub static MODULE_NAME: &str = "fruity_editor";

// #[no_mangle]
pub fn initialize(resource_container: Arc<ResourceContainer>, _settings: &Settings) {
    let inspector_service = InspectorService::new(resource_container.clone());
    let introspect_editor_service = IntrospectEditorService::new(resource_container.clone());
    let component_inspector_service = ComponentInspectorService::new(resource_container.clone());
    let file_explorer_service = FileExplorerService::new(resource_container.clone());

    resource_container.add::<InspectorService>("inspector_service", Box::new(inspector_service));
    resource_container.add::<IntrospectEditorService>(
        "introspect_editor_service",
        Box::new(introspect_editor_service),
    );
    resource_container
        .add::<FileExplorerService>("file_explorer_service", Box::new(file_explorer_service));
    resource_container.add::<ComponentInspectorService>(
        "component_inspector_service",
        Box::new(component_inspector_service),
    );

    declare_global(WorldState::new(resource_container.clone()));
    declare_global(ThemeState::default());
    declare_global(SceneState::new(resource_container.clone()));
    declare_global(InspectorState::new(resource_container.clone()));
    declare_global(FileExplorerState::default());

    let system_service = resource_container.require::<SystemService>();
    let mut system_service = system_service.write();

    system_service.add_begin_system(
        "pause_at_startup",
        MODULE_NAME,
        Inject1::new(pause_at_startup),
        None,
    );

    let inspector_service = resource_container.require::<InspectorService>();
    let mut inspector_service = inspector_service.write();

    inspector_service.register_inspect_type(inspect_entity);
    load_default_resources(resource_container);
}
