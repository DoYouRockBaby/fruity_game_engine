use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::settings::Settings;
use maplit::hashmap;
use std::io::Cursor;
use std::sync::Arc;

pub fn load_default_resources(resource_container: Arc<ResourceContainer>) {
    load_default_icons(resource_container.clone());
}

pub fn load_default_icons(resource_container: Arc<ResourceContainer>) {
    load_icon(
        "Editor/Icons/material",
        include_bytes!("material_thumbnail.png"),
        "png",
        resource_container.clone(),
    );

    load_icon(
        "Editor/Icons/shader",
        include_bytes!("shader_thumbnail.png"),
        "png",
        resource_container.clone(),
    );
}

pub fn load_icon(
    name: &str,
    bytes: &[u8],
    image_type: &str,
    resource_container: Arc<ResourceContainer>,
) {
    let settings = Settings::Object(hashmap! {
        "type".to_string() => Settings::String("texture".to_string()),
    });

    let mut image_reader = Cursor::new(bytes);

    resource_container
        .load_resource(name, image_type, &mut image_reader, settings)
        .unwrap();
}
