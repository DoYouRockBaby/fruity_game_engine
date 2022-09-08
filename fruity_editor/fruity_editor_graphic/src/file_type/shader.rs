use fruity_core::resource::resource_reference::ResourceReference;
use fruity_editor::state::inspector::InspectorState;
use fruity_editor::ui::context::UIContext;
use fruity_editor::ui::hooks::use_write_service;
use fruity_graphic::resources::shader_resource::ShaderResource;
use fruity_graphic::resources::texture_resource::TextureResource;

pub fn get_thumbnail_shader(
    ctx: &UIContext,
    _file_path: &str,
) -> Option<ResourceReference<dyn TextureResource>> {
    let resource_container = ctx.resource_container();
    resource_container.get::<dyn TextureResource>("Editor/Icons/shader")
}

pub fn on_selected_shader(ctx: &UIContext, file_path: &str) {
    let resource_container = ctx.resource_container();
    let mut inspector_state = use_write_service::<InspectorState>(&ctx);

    if let Some(texture) = resource_container.get::<dyn ShaderResource>(file_path) {
        inspector_state.select(Box::new(texture.clone()));
    } else {
        if let Err(_) = resource_container.load_resource_file(file_path, "wgsl") {
            return;
        }

        if let Some(texture) = resource_container.get::<dyn ShaderResource>(file_path) {
            inspector_state.select(Box::new(texture.clone()));
        }
    };
}
