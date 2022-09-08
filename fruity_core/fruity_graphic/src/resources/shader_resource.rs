use crate::graphic_service::GraphicService;
use fruity_any::*;
use fruity_core::convert::FruityInto;
use fruity_core::convert::FruityTryFrom;
use fruity_core::resource::resource::Resource;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::serialize::serialized::Serialized;
use fruity_core::settings::Settings;
use fruity_ecs::*;
use std::io::Read;

pub trait ShaderResource: Resource {}

#[derive(
    Debug, Default, Clone, FruityAny, SerializableObject, IntrospectObject, InstantiableObject,
)]
pub struct ShaderResourceSettings {
    pub binding_groups: Vec<ShaderBindingGroup>,
    pub instance_attributes: Vec<ShaderInstanceAttribute>,
}

#[derive(
    Debug, Default, Clone, FruityAny, SerializableObject, IntrospectObject, InstantiableObject,
)]
pub struct ShaderBindingGroup {
    pub bindings: Vec<ShaderBinding>,
}

#[derive(
    Debug, Default, Clone, FruityAny, SerializableObject, IntrospectObject, InstantiableObject,
)]
pub struct ShaderBinding {
    pub visibility: ShaderBindingVisibility,
    pub ty: ShaderBindingType,
}

#[derive(Debug, Clone)]
pub enum ShaderBindingVisibility {
    Vertex,
    Fragment,
}

impl Default for ShaderBindingVisibility {
    fn default() -> Self {
        ShaderBindingVisibility::Vertex
    }
}

impl FruityTryFrom<Serialized> for ShaderBindingVisibility {
    type Error = String;

    fn fruity_try_from(value: Serialized) -> Result<Self, Self::Error> {
        if let Serialized::String(value) = &value {
            match value as &str {
                "vertex" => Ok(ShaderBindingVisibility::Vertex),
                "fragment" => Ok(ShaderBindingVisibility::Fragment),
                _ => Err(format!(
                    "Couldn't convert {:?} to ShaderBindingVisibility",
                    value
                )),
            }
        } else {
            Err(format!(
                "Couldn't convert {:?} to ShaderBindingVisibility",
                value
            ))
        }
    }
}

impl FruityInto<Serialized> for ShaderBindingVisibility {
    fn fruity_into(self) -> Serialized {
        Serialized::String(
            match self {
                ShaderBindingVisibility::Vertex => "vertex",
                ShaderBindingVisibility::Fragment => "fragment",
            }
            .to_string(),
        )
    }
}

#[derive(Debug, Clone)]
pub enum ShaderBindingType {
    Texture,
    Sampler,
    Uniform,
}

impl Default for ShaderBindingType {
    fn default() -> Self {
        ShaderBindingType::Texture
    }
}

impl FruityTryFrom<Serialized> for ShaderBindingType {
    type Error = String;

    fn fruity_try_from(value: Serialized) -> Result<Self, Self::Error> {
        if let Serialized::String(value) = &value {
            match value as &str {
                "texture" => Ok(ShaderBindingType::Texture),
                "sampler" => Ok(ShaderBindingType::Sampler),
                "uniform" => Ok(ShaderBindingType::Uniform),
                _ => Err(format!("Couldn't convert {:?} to ShaderBindingType", value)),
            }
        } else {
            Err(format!("Couldn't convert {:?} to ShaderBindingType", value))
        }
    }
}

impl FruityInto<Serialized> for ShaderBindingType {
    fn fruity_into(self) -> Serialized {
        Serialized::String(
            match self {
                ShaderBindingType::Texture => "texture",
                ShaderBindingType::Sampler => "sampler",
                ShaderBindingType::Uniform => "uniform",
            }
            .to_string(),
        )
    }
}

#[derive(
    Debug, Default, Clone, FruityAny, SerializableObject, IntrospectObject, InstantiableObject,
)]
pub struct ShaderInstanceAttribute {
    pub location: u32,
    pub ty: ShaderInstanceAttributeType,
}

#[derive(Debug, Clone)]
pub enum ShaderInstanceAttributeType {
    Int,
    UInt,
    Float,
    Vector2,
    Vector4,
}

impl Default for ShaderInstanceAttributeType {
    fn default() -> Self {
        ShaderInstanceAttributeType::Float
    }
}

impl FruityTryFrom<Serialized> for ShaderInstanceAttributeType {
    type Error = String;

    fn fruity_try_from(value: Serialized) -> Result<Self, Self::Error> {
        if let Serialized::String(value) = &value {
            match value as &str {
                "int" => Ok(ShaderInstanceAttributeType::Int),
                "uint" => Ok(ShaderInstanceAttributeType::UInt),
                "float" => Ok(ShaderInstanceAttributeType::Float),
                "vec2" => Ok(ShaderInstanceAttributeType::Vector2),
                "vec4" => Ok(ShaderInstanceAttributeType::Vector4),
                _ => Err(format!(
                    "Couldn't convert {:?} to ShaderInstanceAttributeType",
                    value
                )),
            }
        } else {
            Err(format!(
                "Couldn't convert {:?} to ShaderInstanceAttributeType",
                value
            ))
        }
    }
}

impl FruityInto<Serialized> for ShaderInstanceAttributeType {
    fn fruity_into(self) -> Serialized {
        Serialized::String(
            match self {
                ShaderInstanceAttributeType::Int => "int",
                ShaderInstanceAttributeType::UInt => "uint",
                ShaderInstanceAttributeType::Float => "float",
                ShaderInstanceAttributeType::Vector2 => "vec2",
                ShaderInstanceAttributeType::Vector4 => "vec4",
            }
            .to_string(),
        )
    }
}

pub fn load_shader(
    identifier: &str,
    reader: &mut dyn Read,
    settings: Settings,
    resource_container: ResourceContainer,
) {
    // Get the graphic service state
    let graphic_service = resource_container.require::<dyn GraphicService>();
    let graphic_service = graphic_service.read();

    // read the whole file
    let mut buffer = String::new();
    if let Err(err) = reader.read_to_string(&mut buffer) {
        log::error!("{}", err.to_string());
        return;
    }

    // Parse settings
    let settings = read_shader_settings(&settings, resource_container.clone());

    // Build the resource
    let result = graphic_service.create_shader_resource(identifier, buffer, settings);

    // Store the resource
    match result {
        Ok(resource) => {
            resource_container.add::<dyn ShaderResource>(identifier, resource);
        }
        Err(err) => {
            log::error!("{}", err);
        }
    }
}

pub fn read_shader_settings(
    settings: &Settings,
    resource_container: ResourceContainer,
) -> ShaderResourceSettings {
    let binding_groups = settings.get::<Vec<Settings>>("binding_groups", Vec::new());
    let binding_groups = binding_groups
        .iter()
        .filter_map(|params| {
            if let Settings::Array(params) = params {
                Some(params)
            } else {
                None
            }
        })
        .map(|params| read_shader_binding_group_settings(params, resource_container.clone()))
        .collect::<Vec<_>>();

    let instance_attributes = settings.get::<Vec<Settings>>("instance_attributes", Vec::new());
    let instance_attributes =
        read_shader_instance_attributes_settings(&instance_attributes, resource_container.clone());

    ShaderResourceSettings {
        binding_groups,
        instance_attributes,
    }
}

pub fn read_shader_binding_group_settings(
    settings: &Vec<Settings>,
    _resource_container: ResourceContainer,
) -> ShaderBindingGroup {
    let bindings = settings
        .iter()
        .map(|params| ShaderBinding {
            visibility: match &params.get::<String>("visibility", String::default()) as &str {
                "vertex" => ShaderBindingVisibility::Vertex,
                "fragment" => ShaderBindingVisibility::Fragment,
                _ => ShaderBindingVisibility::default(),
            },
            ty: match &params.get::<String>("type", String::default()) as &str {
                "texture" => ShaderBindingType::Texture,
                "sampler" => ShaderBindingType::Sampler,
                "uniform" => ShaderBindingType::Uniform,
                _ => ShaderBindingType::default(),
            },
        })
        .collect::<Vec<_>>();

    ShaderBindingGroup { bindings }
}

pub fn read_shader_instance_attributes_settings(
    settings: &Vec<Settings>,
    _resource_container: ResourceContainer,
) -> Vec<ShaderInstanceAttribute> {
    settings
        .iter()
        .map(|params| ShaderInstanceAttribute {
            location: params.get::<u32>("location", u32::default()),
            ty: match &params.get::<String>("type", String::default()) as &str {
                "int" => ShaderInstanceAttributeType::Int,
                "uint" => ShaderInstanceAttributeType::UInt,
                "float" => ShaderInstanceAttributeType::Float,
                "vec2" => ShaderInstanceAttributeType::Vector2,
                "vec4" => ShaderInstanceAttributeType::Vector4,
                _ => ShaderInstanceAttributeType::default(),
            },
        })
        .collect::<Vec<_>>()
}
