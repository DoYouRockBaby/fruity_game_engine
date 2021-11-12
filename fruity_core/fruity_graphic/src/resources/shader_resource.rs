use fruity_core::resource::resource::Resource;
use fruity_core::resource::resource_manager::ResourceManager;
use fruity_core::settings::Settings;
use std::sync::Arc;

pub trait ShaderResource: Resource {}

pub struct ShaderParams {
    pub bindings: Vec<ShaderBinding>,
}

pub enum ShaderBindingVisibility {
    Vertex,
    Fragment,
}

pub enum ShaderBindingType {
    Texture,
    Sampler,
    Uniform,
}

pub struct ShaderBinding {
    pub id: u32,
    pub visibility: ShaderBindingVisibility,
    pub ty: ShaderBindingType,
}

pub fn load_shader_settings(
    settings: &Settings,
    _resource_manager: Arc<ResourceManager>,
) -> ShaderParams {
    let bindings = settings.get::<Vec<Settings>>("bindings", Vec::new());
    let bindings = bindings
        .iter()
        .map(|params| ShaderBinding {
            id: params.get::<u32>("id", 0),
            visibility: match &params.get::<String>("visibility", String::default()) as &str {
                "vertex" => ShaderBindingVisibility::Vertex,
                "fragment" => ShaderBindingVisibility::Fragment,
                _ => ShaderBindingVisibility::Vertex,
            },
            ty: match &params.get::<String>("type", String::default()) as &str {
                "texture" => ShaderBindingType::Texture,
                "sampler" => ShaderBindingType::Sampler,
                "uniform" => ShaderBindingType::Uniform,
                _ => ShaderBindingType::Texture,
            },
        })
        .collect::<Vec<_>>();

    ShaderParams { bindings }
}
