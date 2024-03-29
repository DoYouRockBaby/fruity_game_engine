use crate::components::file_explorer::file_explorer_component;
use crate::components::inspector::inspector_component;
use crate::components::scene::scene_component;
use crate::editor_component_service::EditorComponentService;
use crate::editor_menu_service::EditorMenuService;
use crate::editor_menu_service::MenuItemOptions;
use crate::editor_panels_service::EditorPanelsService;
use crate::file_explorer_service::FileExplorerService;
use crate::inspect::inspect_entity::inspect_entity;
use crate::inspector_service::InspectorService;
use crate::introspect_editor_service::IntrospectEditorService;
use crate::menu::is_redo_enabled;
use crate::menu::is_save_enabled;
use crate::menu::is_undo_enabled;
use crate::menu::open;
use crate::menu::redo;
use crate::menu::save;
use crate::menu::save_as;
use crate::menu::undo;
use crate::mutations::mutation_service::MutationService;
use crate::resources::default_resources::load_default_resources;
use crate::state::file_explorer::FileExplorerState;
use crate::state::inspector::InspectorState;
use crate::state::scene::SceneState;
use crate::state::theme::ThemeState;
use crate::systems::pause_at_startup::pause_at_startup;
use crate::ui::elements::pane::UIPaneSide;
use crate::ui::elements::profiling::Profiling;
use crate::ui::elements::UIWidget;
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
pub mod inspect;
pub mod inspector_service;
pub mod introspect_editor_service;
pub mod menu;
pub mod mutations;
pub mod resources;
pub mod state;
pub mod systems;
pub mod ui;

/// The module name
pub static MODULE_NAME: &str = "fruity_editor";

// #[no_mangle]
pub fn initialize(resource_container: ResourceContainer, _settings: &Settings) {
    let inspector_service = InspectorService::new(resource_container.clone());
    let introspect_editor_service = IntrospectEditorService::new(resource_container.clone());
    let editor_menu_service = EditorMenuService::new(resource_container.clone());
    let editor_panels_service = EditorPanelsService::new(resource_container.clone());
    let editor_component_service = EditorComponentService::new(resource_container.clone());
    let file_explorer_service = FileExplorerService::new(resource_container.clone());
    let mutation_service = MutationService::new(resource_container.clone());

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
    resource_container.add::<MutationService>("mutation_service", Box::new(mutation_service));

    resource_container.add::<ThemeState>("theme_state", Box::new(ThemeState::default()));
    resource_container.add::<InspectorState>(
        "inspector_state",
        Box::new(InspectorState::new(resource_container.clone())),
    );
    resource_container.add::<SceneState>(
        "scene_state",
        Box::new(SceneState::new(resource_container.clone())),
    );
    resource_container.add::<FileExplorerState>(
        "file_explorer_state",
        Box::new(FileExplorerState::default()),
    );

    let system_service = resource_container.require::<SystemService>();
    let mut system_service = system_service.write();

    system_service.add_startup_system(
        "pause_at_startup",
        MODULE_NAME,
        Inject1::new(pause_at_startup),
        Default::default(),
    );
    system_service.disable_pool(&99);

    let inspector_service = resource_container.require::<InspectorService>();
    let mut inspector_service = inspector_service.write();

    inspector_service.register_inspect_type(inspect_entity);

    let editor_menu_service = resource_container.require::<EditorMenuService>();
    let mut editor_menu_service = editor_menu_service.write();

    editor_menu_service.add_section("File", 10);
    editor_menu_service.add_section("Edit", 20);
    editor_menu_service.add_menu(
        "Open",
        "File",
        open,
        MenuItemOptions {
            shortcut: Some("Ctrl + O".to_string()),
            ..Default::default()
        },
    );
    editor_menu_service.add_menu(
        "Save",
        "File",
        save,
        MenuItemOptions {
            is_enabled: Some(Arc::new(is_save_enabled)),
            shortcut: Some("Ctrl + S".to_string()),
            ..Default::default()
        },
    );
    editor_menu_service.add_menu(
        "Save as",
        "File",
        save_as,
        MenuItemOptions {
            shortcut: Some("Ctrl + Shift + S".to_string()),
            ..Default::default()
        },
    );
    editor_menu_service.add_menu(
        "Undo",
        "Edit",
        undo,
        MenuItemOptions {
            is_enabled: Some(Arc::new(is_undo_enabled)),
            shortcut: Some("Ctrl + Z".to_string()),
            ..Default::default()
        },
    );
    editor_menu_service.add_menu(
        "Redo",
        "Edit",
        redo,
        MenuItemOptions {
            is_enabled: Some(Arc::new(is_redo_enabled)),
            shortcut: Some("Ctrl + Shift + Z".to_string()),
            ..Default::default()
        },
    );

    let editor_panels_service = resource_container.require::<EditorPanelsService>();
    let mut editor_panels_service = editor_panels_service.write();

    editor_panels_service.add_panel("Scene", UIPaneSide::Center, scene_component);
    editor_panels_service.add_panel("Inspector", UIPaneSide::Right, inspector_component);
    editor_panels_service.add_panel("Profiling", UIPaneSide::Right, |_ctx| Profiling {}.elem());
    editor_panels_service.add_panel("File explorer", UIPaneSide::Bottom, file_explorer_component);

    load_default_resources(resource_container);
}
