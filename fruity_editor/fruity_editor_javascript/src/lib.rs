use crate::file_type::js::get_thumbnail_js;
use crate::file_type::js::on_selected_js;
use crate::resources::default_resources::load_default_resources;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::settings::Settings;
use fruity_editor::file_explorer_service::FileExplorerService;

pub mod file_type;
pub mod resources;

/// The module name
pub static MODULE_NAME: &str = "fruity_editor_javascript";

// #[no_mangle]
pub fn initialize(resource_container: ResourceContainer, _settings: &Settings) {
    let file_explorer_service = resource_container.require::<FileExplorerService>();
    let mut file_explorer_service = file_explorer_service.write();

    file_explorer_service.register_file_type("js", get_thumbnail_js, on_selected_js);

    load_default_resources(resource_container.clone());
}
