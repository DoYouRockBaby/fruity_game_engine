use fruity_core::resource::resource::Resource;
use fruity_core::settings::Settings;

pub struct TextureResourceSettings {}

pub trait TextureResource: Resource {}

pub fn read_texture_settings(_settings: Settings) -> TextureResourceSettings {
    TextureResourceSettings {}
}
