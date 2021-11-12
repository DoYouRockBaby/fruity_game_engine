use crate::editor_manager::EditorManager;
use fruity_core::resource::resource_manager::ResourceManager;
use fruity_core::settings::Settings;
use std::sync::Arc;

pub mod editor_manager;
pub mod ui_element;

// #[no_mangle]
pub fn initialize(resource_manager: Arc<ResourceManager>, _settings: &Settings) {
    let editor_manager = EditorManager::new(resource_manager.clone());

    resource_manager
        .add::<EditorManager>("editor_manager", Box::new(editor_manager))
        .unwrap();
}
