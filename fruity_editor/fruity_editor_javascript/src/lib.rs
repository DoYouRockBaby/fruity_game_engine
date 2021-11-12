use crate::file_type::js::get_thumbnail_js;
use crate::file_type::js::on_selected_js;
use crate::resources::default_resources::load_default_resources;
use fruity_core::resource::resource_manager::ResourceManager;
use fruity_core::service::service_manager::ServiceManager;
use fruity_core::settings::Settings;
use fruity_editor::file_explorer_manager::FileExplorerManager;
use std::sync::Arc;
use std::sync::RwLock;

pub mod file_type;
pub mod resources;

// #[no_mangle]
pub fn initialize(service_manager: &Arc<RwLock<ServiceManager>>, _settings: &Settings) {
    let service_manager = service_manager.read().unwrap();
    let mut file_explorer_manager = service_manager.write::<FileExplorerManager>();
    file_explorer_manager.register_file_type("js", get_thumbnail_js, on_selected_js);

    let resource_manager = service_manager.get::<ResourceManager>().unwrap();
    load_default_resources(resource_manager);
}
