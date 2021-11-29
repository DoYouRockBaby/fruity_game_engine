use fruity_any::*;
use fruity_core::convert::FruityInto;
use fruity_core::convert::FruityTryFrom;
use fruity_core::resource::resource::Resource;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::serialize::serialized::Serialized;
use fruity_core::settings::Settings;
use maplit::hashmap;
use std::sync::Arc;

pub trait ShaderResource: Resource {}

#[derive(Debug, Clone, FruityAny)]
pub struct ShaderParams {
    pub bindings: Vec<ShaderBinding>,
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

#[derive(Debug, Clone, FruityAny)]
pub struct ShaderBinding {
    pub id: u32,
    pub visibility: ShaderBindingVisibility,
    pub ty: ShaderBindingType,
}

pub fn load_shader_settings(
    settings: &Settings,
    _resource_container: Arc<ResourceContainer>,
) -> ShaderParams {
    let bindings = settings.get::<Vec<Settings>>("bindings", Vec::new());
    let bindings = bindings
        .iter()
        .map(|params| ShaderBinding {
            id: params.get::<u32>("id", 0),
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

    ShaderParams { bindings }
}

impl FruityTryFrom<Serialized> for ShaderParams {
    type Error = String;

    fn fruity_try_from(value: Serialized) -> Result<Self, Self::Error> {
        if let Serialized::SerializedObject { fields, .. } = value {
            Ok(ShaderParams {
                bindings: Vec::<ShaderBinding>::fruity_try_from(fields.get("bindings"))
                    .unwrap_or_default(),
            })
        } else {
            Err(format!("Couldn't convert {:?} to object", value))
        }
    }
}

impl FruityInto<Serialized> for ShaderParams {
    fn fruity_into(self) -> Serialized {
        Serialized::SerializedObject {
            class_name: "unknown".to_string(),
            fields: hashmap! {
                "bindings".to_string() => self.bindings.fruity_into(),
            },
        }
    }
}

impl FruityTryFrom<Serialized> for ShaderBinding {
    type Error = String;

    fn fruity_try_from(value: Serialized) -> Result<Self, Self::Error> {
        if let Serialized::SerializedObject { fields, .. } = value {
            Ok(ShaderBinding {
                id: u32::fruity_try_from(fields.get("id")).unwrap_or_default(),
                visibility: ShaderBindingVisibility::fruity_try_from(fields.get("visibility"))
                    .unwrap_or_default(),
                ty: ShaderBindingType::fruity_try_from(fields.get("type")).unwrap_or_default(),
            })
        } else {
            Err(format!("Couldn't convert {:?} to object", value))
        }
    }
}

impl FruityInto<Serialized> for ShaderBinding {
    fn fruity_into(self) -> Serialized {
        Serialized::SerializedObject {
            class_name: "unknown".to_string(),
            fields: hashmap! {
                "id".to_string() => self.id.fruity_into(),
                "visibility".to_string() => self.visibility.fruity_into(),
                "type".to_string() => self.ty.fruity_into(),
            },
        }
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
