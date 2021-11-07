use fruity_core::resource::resources_manager::ResourceIdentifier;
use fruity_core::resource::resources_manager::ResourcesManager;
use fruity_core::service::service_rwlock::ServiceRwLock;
use fruity_core::settings::Settings;
use maplit::hashmap;
use std::io::Cursor;

pub fn load_default_resources(resources_manager: ServiceRwLock<ResourcesManager>) {
    load_default_icons(resources_manager.clone());
}

pub fn load_default_icons(resources_manager: ServiceRwLock<ResourcesManager>) {
    load_icon(
        "Editor/Icons/unknown",
        include_bytes!("unknown_thumbnail.png"),
        "png",
        resources_manager.clone(),
    );

    load_icon(
        "Editor/Icons/folder",
        include_bytes!("folder_thumbnail.png"),
        "png",
        resources_manager.clone(),
    );

    load_icon(
        "Editor/Icons/js",
        include_bytes!("js_thumbnail.png"),
        "png",
        resources_manager.clone(),
    );

    load_icon(
        "Editor/Icons/material",
        include_bytes!("material_thumbnail.png"),
        "png",
        resources_manager.clone(),
    );

    load_icon(
        "Editor/Icons/settings",
        include_bytes!("settings_thumbnail.png"),
        "png",
        resources_manager.clone(),
    );

    load_icon(
        "Editor/Icons/shader",
        include_bytes!("shader_thumbnail.png"),
        "png",
        resources_manager.clone(),
    );
}

pub fn load_icon(
    name: &str,
    bytes: &[u8],
    image_type: &str,
    mut resources_manager: ServiceRwLock<ResourcesManager>,
) {
    let settings = Settings::Object(hashmap! {
        "type".to_string() => Settings::String("texture".to_string()),
    });

    let mut image_reader = Cursor::new(bytes);

    resources_manager
        .load_resource(
            ResourceIdentifier(name.to_string()),
            image_type,
            &mut image_reader,
            settings,
        )
        .unwrap();
}
