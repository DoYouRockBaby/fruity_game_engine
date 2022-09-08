use fruity_core::resource::resource_reference::ResourceReference;
use fruity_editor::state::inspector::InspectorState;
use fruity_editor::ui::context::UIContext;
use fruity_editor::ui::hooks::use_write_service;
use fruity_graphic::resources::texture_resource::TextureResource;

pub fn get_thumbnail_image(
    ctx: &UIContext,
    file_path: &str,
) -> Option<ResourceReference<dyn TextureResource>> {
    let resource_container = ctx.resource_container();

    if let Some(texture) = resource_container.get::<dyn TextureResource>(file_path) {
        Some(texture)
    } else {
        resource_container
            .load_resource_file(file_path, "png")
            .ok()?;

        resource_container.get::<dyn TextureResource>(file_path)
    }
}

pub fn on_selected_image(ctx: &UIContext, file_path: &str) {
    let resource_container = ctx.resource_container();
    let mut inspector_state = use_write_service::<InspectorState>(ctx);

    if let Some(texture) = resource_container.get::<dyn TextureResource>(file_path) {
        inspector_state.select(Box::new(texture.clone()));
    } else {
        if let Err(_) = resource_container.load_resource_file(file_path, "png") {
            return;
        }

        if let Some(texture) = resource_container.get::<dyn TextureResource>(file_path) {
            inspector_state.select(Box::new(texture.clone()));
        }
    };
}
