use fruity_core::resource::resource_reference::ResourceReference;
use fruity_editor::hooks::use_global;
use fruity_editor::state::inspector::InspectorState;
use fruity_editor::state::world::WorldState;
use fruity_graphic::resources::texture_resource::TextureResource;

pub fn get_thumbnail_image(file_path: &str) -> Option<ResourceReference<dyn TextureResource>> {
    let world_state = use_global::<WorldState>();

    if let Some(texture) = world_state
        .resource_container
        .get::<dyn TextureResource>(file_path)
    {
        Some(texture)
    } else {
        world_state
            .resource_container
            .load_resource_file(file_path, "png")
            .ok()?;

        world_state
            .resource_container
            .get::<dyn TextureResource>(file_path)
    }
}

pub fn on_selected_image(file_path: &str) {
    let world_state = use_global::<WorldState>();

    if let Some(texture) = world_state
        .resource_container
        .get::<dyn TextureResource>(file_path)
    {
        let inspector_state = use_global::<InspectorState>();
        inspector_state.select(Box::new(texture.clone()));
    } else {
        if let Err(_) = world_state
            .resource_container
            .load_resource_file(file_path, "png")
        {
            return;
        }

        if let Some(texture) = world_state
            .resource_container
            .get::<dyn TextureResource>(file_path)
        {
            let inspector_state = use_global::<InspectorState>();
            inspector_state.select(Box::new(texture.clone()));
        }
    };
}
