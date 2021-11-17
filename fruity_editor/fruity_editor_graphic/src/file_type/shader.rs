use fruity_core::resource::resource_reference::ResourceReference;
use fruity_editor::hooks::use_global;
use fruity_editor::state::inspector::InspectorState;
use fruity_editor::state::world::WorldState;
use fruity_graphic::resources::shader_resource::ShaderResource;
use fruity_graphic::resources::texture_resource::TextureResource;

pub fn get_thumbnail_shader(_file_path: &str) -> Option<ResourceReference<dyn TextureResource>> {
    let world_state = use_global::<WorldState>();

    world_state
        .resource_container
        .get::<dyn TextureResource>("Editor/Icons/shader")
}

pub fn on_selected_shader(file_path: &str) {
    let world_state = use_global::<WorldState>();

    if let Some(texture) = world_state
        .resource_container
        .get::<dyn ShaderResource>(file_path)
    {
        let inspector_state = use_global::<InspectorState>();
        inspector_state.select(Box::new(texture.clone()));
    } else {
        if let Err(_) = world_state
            .resource_container
            .load_resource_file(file_path, "wgsl")
        {
            return;
        }

        if let Some(texture) = world_state
            .resource_container
            .get::<dyn ShaderResource>(file_path)
        {
            let inspector_state = use_global::<InspectorState>();
            inspector_state.select(Box::new(texture.clone()));
        }
    };
}
