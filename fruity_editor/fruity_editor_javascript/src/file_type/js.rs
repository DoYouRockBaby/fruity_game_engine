use fruity_core::resource::resource_reference::ResourceReference;
use fruity_editor::ui::context::UIContext;
use fruity_graphic::resources::texture_resource::TextureResource;

pub fn get_thumbnail_js(
    ctx: &UIContext,
    _file_path: &str,
) -> Option<ResourceReference<dyn TextureResource>> {
    let resource_container = ctx.resource_container();
    resource_container.get::<dyn TextureResource>("Editor/Icons/js")
}

pub fn on_selected_js(_ctx: &UIContext, file_path: &str) {
    // TODO: Display an error popup if failed
    edit::edit_file(file_path).unwrap();
}
