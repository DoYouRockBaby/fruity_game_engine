use crate::fields::shader_reference::draw_editor_shader_reference;
use crate::fields::texture_reference::draw_editor_texture_reference;
use crate::fields::vector2d::draw_editor_vector_2d;
use crate::file_type::image::get_thumbnail_image;
use crate::file_type::image::on_selected_image;
use crate::file_type::shader::get_thumbnail_shader;
use crate::file_type::shader::on_selected_shader;
use crate::resources::default_resources::load_default_resources;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::resource::resource_reference::ResourceReference;
use fruity_core::settings::Settings;
use fruity_editor::file_explorer_service::FileExplorerService;
use fruity_editor::introspect_editor_service::IntrospectEditorService;
use fruity_graphic::math::vector2d::Vector2d;
use fruity_graphic::resources::shader_resource::ShaderResource;
use fruity_graphic::resources::texture_resource::TextureResource;
use std::sync::Arc;

pub mod fields;
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

    let introspect_editor_service = resource_container.require::<IntrospectEditorService>();
    let mut introspect_editor_service = introspect_editor_service.write();
    introspect_editor_service.register_field_editor::<Vector2d, _>(draw_editor_vector_2d);
    introspect_editor_service
        .register_field_editor::<Option<ResourceReference<dyn TextureResource>>, _>(
            draw_editor_texture_reference,
        );
    introspect_editor_service
        .register_field_editor::<Option<ResourceReference<dyn ShaderResource>>, _>(
            draw_editor_shader_reference,
        );

    load_default_resources(resource_container.clone());
}
