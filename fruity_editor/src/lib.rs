use crate::components::file_explorer::file_explorer_component;
use crate::components::inspector::inspector_component;
use crate::editor_component_service::EditorComponentService;
use crate::editor_menu_service::EditorMenuService;
use crate::editor_panels_service::EditorPanelsService;
use crate::file_explorer_service::FileExplorerService;
use crate::hooks::declare_global;
use crate::hooks::use_global;
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
use crate::ui_element::pane::UIPaneSide;
use crate::ui_element::profiling::Profiling;
use crate::ui_element::UIWidget;
use fruity_core::inject::Inject1;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::settings::Settings;
use fruity_ecs::system::system_service::SystemService;
use std::sync::Arc;

#[macro_use]
extern crate lazy_static;

pub mod components;
pub mod dialog_service;
pub mod editor_component_service;
pub mod editor_menu_service;
pub mod editor_panels_service;
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
    let editor_menu_service = EditorMenuService::new(resource_container.clone());
    let editor_panels_service = EditorPanelsService::new(resource_container.clone());
    let editor_component_service = EditorComponentService::new(resource_container.clone());
    let file_explorer_service = FileExplorerService::new(resource_container.clone());

    resource_container.add::<InspectorService>("inspector_service", Box::new(inspector_service));
    resource_container.add::<IntrospectEditorService>(
        "introspect_editor_service",
        Box::new(introspect_editor_service),
    );
    resource_container
        .add::<FileExplorerService>("file_explorer_service", Box::new(file_explorer_service));
    resource_container
        .add::<EditorMenuService>("editor_menu_service", Box::new(editor_menu_service));
    resource_container
        .add::<EditorPanelsService>("editor_panels_service", Box::new(editor_panels_service));
    resource_container.add::<EditorComponentService>(
        "editor_component_service",
        Box::new(editor_component_service),
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

    let editor_menu_service = resource_container.require::<EditorMenuService>();
    let mut editor_menu_service = editor_menu_service.write();

    editor_menu_service.add_menu("Open", "File", move || {
        let scene_state = use_global::<SceneState>();
        scene_state.open();
    });
    editor_menu_service.add_menu("Save", "File", move || {
        let scene_state = use_global::<SceneState>();
        scene_state.save();
    });
    editor_menu_service.add_menu("Save as", "File", move || {
        let scene_state = use_global::<SceneState>();
        scene_state.save_as();
    });

    let editor_panels_service = resource_container.require::<EditorPanelsService>();
    let mut editor_panels_service = editor_panels_service.write();

    editor_panels_service.add_panel("Inspector", UIPaneSide::Right, inspector_component);
    editor_panels_service.add_panel("Profiling", UIPaneSide::Right, || Profiling {}.elem());
    editor_panels_service.add_panel("File explorer", UIPaneSide::Bottom, file_explorer_component);

    load_default_resources(resource_container);
}
