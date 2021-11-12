use crate::file_type::js::get_thumbnail_js;
use crate::file_type::js::on_selected_js;
use crate::resources::default_resources::load_default_resources;
use fruity_core::resource::resource_manager::ResourceManager;
use fruity_core::settings::Settings;
use fruity_editor::file_explorer_manager::FileExplorerManager;
use std::sync::Arc;

pub mod file_type;
pub mod resources;

// #[no_mangle]
pub fn initialize(resource_manager: Arc<ResourceManager>, _settings: &Settings) {
    let file_explorer_manager =
        resource_manager.require::<FileExplorerManager>("file_explorer_manager");
    let mut file_explorer_manager = file_explorer_manager.write();

    file_explorer_manager.register_file_type("js", get_thumbnail_js, on_selected_js);

    load_default_resources(resource_manager.clone());
}
