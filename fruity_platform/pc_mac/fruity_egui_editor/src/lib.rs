use crate::dialog_service::WgpuDialogService;
use crate::editor_service::EditorService;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::settings::Settings;
use fruity_editor::dialog_service::DialogService;
use std::sync::Arc;

pub mod dialog_service;
pub mod editor_service;
pub mod ui_element;

/// The module name
pub static MODULE_NAME: &str = "fruity_egui_editor";

// #[no_mangle]
pub fn initialize(resource_container: Arc<ResourceContainer>, _settings: &Settings) {
    let editor_service = EditorService::new(resource_container.clone());
    let dialog_service = WgpuDialogService::new(resource_container.clone());

    resource_container.add::<EditorService>("editor_service", Box::new(editor_service));
    resource_container.add::<dyn DialogService>("dialog_service", Box::new(dialog_service));
}
