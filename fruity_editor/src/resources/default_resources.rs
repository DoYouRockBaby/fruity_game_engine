use fruity_core::resource::resource_manager::ResourceIdentifier;
use fruity_core::resource::resource_manager::ResourceManager;
use fruity_core::service::service_rwlock::ServiceRwLock;
use fruity_core::settings::Settings;
use maplit::hashmap;
use std::io::Cursor;

pub fn load_default_resources(resource_manager: ServiceRwLock<ResourceManager>) {
    load_default_icons(resource_manager.clone());
}

pub fn load_default_icons(resource_manager: ServiceRwLock<ResourceManager>) {
    load_icon(
        "Editor/Icons/unknown",
        include_bytes!("unknown_thumbnail.png"),
        "png",
        resource_manager.clone(),
    );

    load_icon(
        "Editor/Icons/folder",
        include_bytes!("folder_thumbnail.png"),
        "png",
        resource_manager.clone(),
    );

    load_icon(
        "Editor/Icons/material",
        include_bytes!("material_thumbnail.png"),
        "png",
        resource_manager.clone(),
    );

    load_icon(
        "Editor/Icons/settings",
        include_bytes!("settings_thumbnail.png"),
        "png",
        resource_manager.clone(),
    );

    load_icon(
        "Editor/Icons/shader",
        include_bytes!("shader_thumbnail.png"),
        "png",
        resource_manager.clone(),
    );
}

pub fn load_icon(
    name: &str,
    bytes: &[u8],
    image_type: &str,
    mut resource_manager: ServiceRwLock<ResourceManager>,
) {
    let settings = Settings::Object(hashmap! {
        "type".to_string() => Settings::String("texture".to_string()),
    });

    let mut image_reader = Cursor::new(bytes);

    resource_manager
        .load_resource(
            ResourceIdentifier(name.to_string()),
            image_type,
            &mut image_reader,
            settings,
        )
        .unwrap();
}
