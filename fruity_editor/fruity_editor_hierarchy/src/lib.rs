use crate::components::entity::entity_list::entity_list_component;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::settings::Settings;
use fruity_editor::editor_component_service::EditorComponentService;
use fruity_editor::editor_component_service::RegisterComponentParams;
use fruity_editor::editor_panels_service::EditorPanelsService;
use fruity_editor::ui_element::pane::UIPaneSide;
use std::sync::Arc;

pub mod components;

/// The module name
pub static MODULE_NAME: &str = "fruity_editor_hierarchy";

// #[no_mangle]
pub fn initialize(resource_container: Arc<ResourceContainer>, _settings: &Settings) {
    let editor_component_service = resource_container.require::<EditorComponentService>();
    let mut editor_component_service = editor_component_service.write();

    editor_component_service.register_component("Parent", RegisterComponentParams::default());

    let editor_panels_service = resource_container.require::<EditorPanelsService>();
    let mut editor_panels_service = editor_panels_service.write();

    editor_panels_service.add_panel("Entities", UIPaneSide::Left, entity_list_component);
}
