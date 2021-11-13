use crate::editor_service::EditorService;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::settings::Settings;
use std::sync::Arc;

pub mod editor_service;
pub mod ui_element;

// #[no_mangle]
pub fn initialize(resource_container: Arc<ResourceContainer>, _settings: &Settings) {
    let editor_service = EditorService::new(resource_container.clone());

    resource_container
        .add_require::<EditorService>("editor_service", Box::new(editor_service))
        .unwrap();
}
