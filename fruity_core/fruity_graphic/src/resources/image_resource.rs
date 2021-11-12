use fruity_core::resource::resource::Resource;
use fruity_core::settings::Settings;

pub enum ImageResourceSettings {
    Image {},
    Texture {},
}

pub trait ImageResource: Resource {}

pub fn read_image_settings(settings: Settings) -> ImageResourceSettings {
    match &settings.get::<String>("type", "image".to_string()) as &str {
        "image" => ImageResourceSettings::Image {},
        "texture" => ImageResourceSettings::Texture {},
        _ => ImageResourceSettings::Image {},
    }
}
