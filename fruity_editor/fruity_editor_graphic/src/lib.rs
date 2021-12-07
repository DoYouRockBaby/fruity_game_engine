use crate::file_type::image::get_thumbnail_image;
use crate::file_type::image::on_selected_image;
use crate::file_type::shader::get_thumbnail_shader;
use crate::file_type::shader::on_selected_shader;
use crate::resources::default_resources::load_default_resources;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::settings::Settings;
use fruity_editor::file_explorer_service::FileExplorerService;
use std::sync::Arc;

pub mod file_type;
pub mod resources;

/// The module name
pub static MODULE_NAME: &str = "fruity_editor_graphic";

// #[no_mangle]
pub fn initialize(resource_container: Arc<ResourceContainer>, _settings: &Settings) {
    let file_explorer_service = resource_container.require::<FileExplorerService>();
    let mut file_explorer_service = file_explorer_service.write();

    file_explorer_service.register_file_type("png", get_thumbnail_image, on_selected_image);
    file_explorer_service.register_file_type("jpeg", get_thumbnail_image, on_selected_image);
    file_explorer_service.register_file_type("jpg", get_thumbnail_image, on_selected_image);
    file_explorer_service.register_file_type("gif", get_thumbnail_image, on_selected_image);
    file_explorer_service.register_file_type("bmp", get_thumbnail_image, on_selected_image);
    file_explorer_service.register_file_type("ico", get_thumbnail_image, on_selected_image);
    file_explorer_service.register_file_type("tiff", get_thumbnail_image, on_selected_image);
    file_explorer_service.register_file_type("wgsl", get_thumbnail_shader, on_selected_shader);

    load_default_resources(resource_container.clone());
}
