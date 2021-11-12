use fruity_core::resource::resource_manager::ResourceIdentifier;
use fruity_core::resource::resource_manager::ResourceManager;
use fruity_editor::hooks::use_global;
use fruity_editor::state::world::WorldState;
use fruity_graphic::resources::texture_resource::TextureResource;
use std::sync::Arc;

pub fn get_thumbnail_js(_file_path: &str) -> Option<Arc<TextureResource>> {
    let world_state = use_global::<WorldState>();
    let service_manager = world_state.service_manager.read().unwrap();
    let resource_manager = service_manager.read::<ResourceManager>();

    resource_manager.get::<TextureResource>(ResourceIdentifier("Editor/Icons/js".to_string()))
}

pub fn on_selected_js(file_path: &str) {
    // TODO: Display an error popup if failed
    edit::edit_file(file_path).unwrap();
}
