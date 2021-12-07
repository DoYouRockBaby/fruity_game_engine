use crate::graphic_service::GraphicService;
use fruity_any::*;
use fruity_core::convert::FruityInto;
use fruity_core::convert::FruityTryFrom;
use fruity_core::resource::resource::Resource;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::serialize::serialized::SerializableObject;
use fruity_core::serialize::serialized::Serialized;
use fruity_core::settings::Settings;
use fruity_ecs::*;
use std::io::Read;
use std::sync::Arc;

pub trait ShaderResource: Resource {}

#[derive(Debug, Default, Clone, FruityAny, IntrospectObject, InstantiableObject)]
pub struct ShaderResourceSettings {
    pub binding_groups: Vec<ShaderBindingGroup>,
}

impl SerializableObject for ShaderResourceSettings {
    fn duplicate(&self) -> Box<dyn SerializableObject> {
        Box::new(self.clone())
    }
}

impl FruityTryFrom<Serialized> for ShaderResourceSettings {
    type Error = String;

    fn fruity_try_from(value: Serialized) -> Result<Self, Self::Error> {
        match value {
            Serialized::NativeObject(value) => {
                match value.as_any_box().downcast::<ShaderResourceSettings>() {
                    Ok(value) => Ok(*value),
                    Err(_) => Err(format!("Couldn't convert a ShaderParams to native object")),
                }
            }
            _ => Err(format!("Couldn't convert {:?} to native object", value)),
        }
    }
}

impl FruityInto<Serialized> for ShaderResourceSettings {
    fn fruity_into(self) -> Serialized {
        Serialized::NativeObject(Box::new(self))
    }
}

#[derive(Debug, Default, Clone, FruityAny, IntrospectObject, InstantiableObject)]
pub struct ShaderBindingGroup {
    pub bindings: Vec<ShaderBinding>,
}

impl SerializableObject for ShaderBindingGroup {
    fn duplicate(&self) -> Box<dyn SerializableObject> {
        Box::new(self.clone())
    }
}

impl FruityTryFrom<Serialized> for ShaderBindingGroup {
    type Error = String;

    fn fruity_try_from(value: Serialized) -> Result<Self, Self::Error> {
        match value {
            Serialized::NativeObject(value) => {
                match value.as_any_box().downcast::<ShaderBindingGroup>() {
                    Ok(value) => Ok(*value),
                    Err(_) => Err(format!(
                        "Couldn't convert a ShaderBindingGroup to native object"
                    )),
                }
            }
            _ => Err(format!("Couldn't convert {:?} to native object", value)),
        }
    }
}

impl FruityInto<Serialized> for ShaderBindingGroup {
    fn fruity_into(self) -> Serialized {
        Serialized::NativeObject(Box::new(self))
    }
}

#[derive(Debug, Default, Clone, FruityAny, IntrospectObject, InstantiableObject)]
pub struct ShaderBinding {
    pub visibility: ShaderBindingVisibility,
    pub ty: ShaderBindingType,
}

impl SerializableObject for ShaderBinding {
    fn duplicate(&self) -> Box<dyn SerializableObject> {
        Box::new(self.clone())
    }
}

impl FruityTryFrom<Serialized> for ShaderBinding {
    type Error = String;

    fn fruity_try_from(value: Serialized) -> Result<Self, Self::Error> {
        match value {
            Serialized::NativeObject(value) => match value.as_any_box().downcast::<ShaderBinding>()
            {
                Ok(value) => Ok(*value),
                Err(_) => Err(format!("Couldn't convert a ShaderBinding to native object")),
            },
            _ => Err(format!("Couldn't convert {:?} to native object", value)),
        }
    }
}

impl FruityInto<Serialized> for ShaderBinding {
    fn fruity_into(self) -> Serialized {
        Serialized::NativeObject(Box::new(self))
    }
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

pub fn load_shader(
    identifier: &str,
    reader: &mut dyn Read,
    settings: Settings,
    resource_container: Arc<ResourceContainer>,
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

    match result {
        Ok(resource) => {
            // Store the resource
            if let Err(_) = resource_container.add::<dyn ShaderResource>(identifier, resource) {
                log::error!(
                    "Couldn't add a resource cause the identifier \"{}\" already exists",
                    identifier
                );
                return;
            }
        }
        Err(err) => {
            log::error!("{}", err);
        }
    }
}

pub fn read_shader_settings(
    settings: &Settings,
    _resource_container: Arc<ResourceContainer>,
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
        .map(|params| read_shader_binding_group_settings(params, _resource_container.clone()))
        .collect::<Vec<_>>();

    ShaderResourceSettings { binding_groups }
}

pub fn read_shader_binding_group_settings(
    settings: &Vec<Settings>,
    _resource_container: Arc<ResourceContainer>,
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
