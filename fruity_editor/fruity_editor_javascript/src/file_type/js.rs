use fruity_core::resource::resource_reference::ResourceReference;
use fruity_editor::hooks::use_global;
use fruity_editor::state::world::WorldState;
use fruity_graphic::resources::texture_resource::TextureResource;

pub fn get_thumbnail_js(_file_path: &str) -> Option<ResourceReference<dyn TextureResource>> {
    let world_state = use_global::<WorldState>();

    world_state
        .resource_container
        .get::<dyn TextureResource>("Editor/Icons/js")
}

pub fn on_selected_js(file_path: &str) {
    // TODO: Display an error popup if failed
    edit::edit_file(file_path).unwrap();
}
