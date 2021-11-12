use crate::resources::shader_resource::ShaderResource;
use crate::resources::texture_resource::TextureResource;
use fruity_core::resource::resource::Resource;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::resource::resource_reference::ResourceReference;
use fruity_core::settings::build_settings_from_yaml;
use fruity_core::settings::Settings;
use std::io::Read;
use std::sync::Arc;
use yaml_rust::YamlLoader;

pub trait MaterialResource: Resource {}

pub struct MaterialParams {
    pub shader: ResourceReference<dyn ShaderResource>,
    pub binding_groups: Vec<MaterialParamsBindingGroup>,
}

pub struct MaterialParamsBindingGroup {
    pub index: u32,
    pub ty: MaterialParamsBindingGroupType,
}

pub struct MaterialParamsBinding {
    pub index: u32,
    pub ty: MaterialParamsBindingType,
}

pub enum MaterialParamsBindingType {
    Texture {
        texture: ResourceReference<dyn TextureResource>,
    },
    Sampler {
        texture: ResourceReference<dyn TextureResource>,
    },
    Uniform,
}

pub enum MaterialParamsBindingGroupType {
    Camera,
    Custom(Vec<MaterialParamsBinding>),
}

pub fn load_material_settings(
    reader: &mut dyn Read,
    resource_container: Arc<ResourceContainer>,
) -> Option<MaterialParams> {
    // read the whole file
    let mut buffer = String::new();
    if let Err(err) = reader.read_to_string(&mut buffer) {
        log::error!("{}", err.to_string());
        return None;
    }
    let docs = YamlLoader::load_from_str(&buffer).unwrap();
    let root = &docs[0];
    let settings = if let Some(settings) = build_settings_from_yaml(root) {
        settings
    } else {
        return None;
    };

    // Parse settings
    build_material_params(&settings, resource_container)
}

pub fn build_material_params(
    settings: &Settings,
    resource_container: Arc<ResourceContainer>,
) -> Option<MaterialParams> {
    let shader_identifier = settings.get::<String>("shader", String::default());
    let shader = resource_container.get::<dyn ShaderResource>(&shader_identifier);
    let shader = if let Some(shader) = shader {
        shader
    } else {
        return None;
    };

    let binding_groups = settings.get::<Vec<Settings>>("binding_groups", Vec::new());
    let binding_groups = binding_groups
        .iter()
        .filter_map(|params| build_material_bind_group_params(params, resource_container.clone()))
        .collect::<Vec<_>>();

    Some(MaterialParams {
        shader,
        binding_groups,
    })
}

fn build_material_bind_group_params(
    settings: &Settings,
    resource_container: Arc<ResourceContainer>,
) -> Option<MaterialParamsBindingGroup> {
    match &settings.get::<String>("type", String::default()) as &str {
        "camera" => {
            let index = settings.get::<u32>("index", 0);

            Some(MaterialParamsBindingGroup {
                index,
                ty: MaterialParamsBindingGroupType::Camera,
            })
        }
        "custom" => {
            let index = settings.get::<u32>("index", 0);
            let bindings = settings.get::<Vec<Settings>>("bindings", Vec::new());
            let bindings = bindings
                .iter()
                .filter_map(|params| build_material_bind_params(params, resource_container.clone()))
                .collect::<Vec<_>>();

            Some(MaterialParamsBindingGroup {
                index,
                ty: MaterialParamsBindingGroupType::Custom(bindings),
            })
        }
        _ => None,
    }
}

fn build_material_bind_params(
    params: &Settings,
    resource_container: Arc<ResourceContainer>,
) -> Option<MaterialParamsBinding> {
    match &params.get::<String>("type", String::default()) as &str {
        "texture" => {
            let texture_identifier = params.get::<String>("texture", String::default());
            let texture = resource_container.get::<dyn TextureResource>(&texture_identifier);

            if let Some(texture) = texture {
                Some(MaterialParamsBindingType::Texture { texture })
            } else {
                None
            }
        }
        "sampler" => {
            let texture_identifier = params.get::<String>("texture", String::default());
            let texture = resource_container.get::<dyn TextureResource>(&texture_identifier);

            if let Some(texture) = texture {
                Some(MaterialParamsBindingType::Sampler { texture })
            } else {
                None
            }
        }
        "uniform" => Some(MaterialParamsBindingType::Uniform),
        _ => None,
    }
    .map(|ty| MaterialParamsBinding {
        index: params.get::<u32>("index", 0),
        ty,
    })
}
