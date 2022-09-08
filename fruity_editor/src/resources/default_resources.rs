use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::settings::Settings;
use maplit::hashmap;
use std::io::Cursor;

pub fn load_default_resources(resource_container: ResourceContainer) {
    load_default_icons(resource_container.clone());
}

pub fn load_default_icons(resource_container: ResourceContainer) {
    load_icon(
        "Editor/Icons/unknown",
        include_bytes!("unknown_thumbnail.png"),
        "png",
        resource_container.clone(),
    );

    load_icon(
        "Editor/Icons/folder",
        include_bytes!("folder_thumbnail.png"),
        "png",
        resource_container.clone(),
    );

    load_icon(
        "Editor/Icons/settings",
        include_bytes!("settings_thumbnail.png"),
        "png",
        resource_container.clone(),
    );
}

pub fn load_icon(
    name: &str,
    bytes: &[u8],
    image_type: &str,
    resource_container: ResourceContainer,
) {
    let settings = Settings::Object(hashmap! {
        "type".to_string() => Settings::String("texture".to_string()),
    });

    let mut image_reader = Cursor::new(bytes);

    resource_container
        .load_resource(name, image_type, &mut image_reader, settings)
        .unwrap();
}
