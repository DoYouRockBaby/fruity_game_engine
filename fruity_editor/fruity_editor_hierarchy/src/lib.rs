use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::settings::Settings;
use fruity_editor::component_editor_service::ComponentEditorService;
use fruity_editor::component_editor_service::RegisterComponentParams;
use std::sync::Arc;

/// The module name
pub static MODULE_NAME: &str = "fruity_editor_hierarchy";

// #[no_mangle]
pub fn initialize(resource_container: Arc<ResourceContainer>, _settings: &Settings) {
    let component_editor_service = resource_container.require::<ComponentEditorService>();
    let mut component_editor_service = component_editor_service.write();

    component_editor_service.register_component("Parent", RegisterComponentParams::default());
}
