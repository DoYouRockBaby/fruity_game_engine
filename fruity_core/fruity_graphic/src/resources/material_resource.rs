use crate::resources::shader_resource::ShaderResource;
use crate::resources::texture_resource::TextureResource;
use fruity_core::convert::FruityInto;
use fruity_core::convert::FruityTryFrom;
use fruity_core::resource::resource::Resource;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::resource::resource_reference::ResourceReference;
use fruity_core::serialize::serialized::Serialized;
use fruity_core::settings::build_settings_from_yaml;
use fruity_core::settings::Settings;
use maplit::hashmap;
use std::collections::HashMap;
use std::io::Read;
use std::sync::Arc;
use yaml_rust::YamlLoader;

pub trait MaterialResource: Resource {}

#[derive(Debug, Clone)]
pub struct MaterialParams {
    pub shader: ResourceReference<dyn ShaderResource>,
    pub binding_groups: Vec<MaterialParamsBindingGroup>,
}

#[derive(Debug, Clone)]
pub struct MaterialParamsBindingGroup {
    pub index: u32,
    pub ty: MaterialParamsBindingGroupType,
}

#[derive(Debug, Clone)]
pub struct MaterialParamsBinding {
    pub index: u32,
    pub ty: MaterialParamsBindingType,
}

#[derive(Debug, Clone)]
pub enum MaterialParamsBindingType {
    Texture {
        texture: ResourceReference<dyn TextureResource>,
    },
    Sampler {
        texture: ResourceReference<dyn TextureResource>,
    },
    Uniform,
}

impl Default for MaterialParamsBindingType {
    fn default() -> Self {
        Self::Uniform
    }
}

#[derive(Debug, Clone)]
pub enum MaterialParamsBindingGroupType {
    Camera,
    Custom(Vec<MaterialParamsBinding>),
}

impl Default for MaterialParamsBindingGroupType {
    fn default() -> Self {
        Self::Custom(Vec::new())
    }
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

impl FruityTryFrom<Serialized> for MaterialParams {
    type Error = String;

    fn fruity_try_from(value: Serialized) -> Result<Self, Self::Error> {
        if let Serialized::SerializedObject { fields, .. } = value {
            Ok(MaterialParams {
                shader: ResourceReference::<dyn ShaderResource>::fruity_try_from(
                    fields.get("shader"),
                )?,
                binding_groups: Vec::<MaterialParamsBindingGroup>::fruity_try_from(
                    fields.get("binding_groups"),
                )
                .unwrap_or_default(),
            })
        } else {
            Err(format!("Couldn't convert {:?} to object", value))
        }
    }
}

impl FruityInto<Serialized> for MaterialParams {
    fn fruity_into(self) -> Serialized {
        Serialized::SerializedObject {
            class_name: "unknown".to_string(),
            fields: hashmap! {
                "shader".to_string() => self.shader.fruity_into(),
                "binding_groups".to_string() => self.binding_groups.fruity_into(),
            },
        }
    }
}

impl FruityTryFrom<Serialized> for MaterialParamsBindingGroup {
    type Error = String;

    fn fruity_try_from(value: Serialized) -> Result<Self, Self::Error> {
        if let Serialized::SerializedObject { fields, .. } = value {
            Ok(MaterialParamsBindingGroup {
                index: u32::fruity_try_from(fields.get("index")).unwrap_or_default(),
                ty: MaterialParamsBindingGroupType::fruity_try_from(fields.get("type"))
                    .unwrap_or_default(),
            })
        } else {
            Err(format!("Couldn't convert {:?} to object", value))
        }
    }
}

impl FruityInto<Serialized> for MaterialParamsBindingGroup {
    fn fruity_into(self) -> Serialized {
        Serialized::SerializedObject {
            class_name: "unknown".to_string(),
            fields: hashmap! {
                "index".to_string() => self.index.fruity_into(),
                "type".to_string() => self.ty.fruity_into(),
            },
        }
    }
}

impl FruityTryFrom<Serialized> for MaterialParamsBinding {
    type Error = String;

    fn fruity_try_from(value: Serialized) -> Result<Self, Self::Error> {
        if let Serialized::SerializedObject { fields, .. } = value {
            Ok(MaterialParamsBinding {
                index: u32::fruity_try_from(fields.get("index")).unwrap_or_default(),
                ty: MaterialParamsBindingType::fruity_try_from(fields.get("type"))
                    .unwrap_or_default(),
            })
        } else {
            Err(format!("Couldn't convert {:?} to object", value))
        }
    }
}

impl FruityInto<Serialized> for MaterialParamsBinding {
    fn fruity_into(self) -> Serialized {
        Serialized::SerializedObject {
            class_name: "unknown".to_string(),
            fields: hashmap! {
                "index".to_string() => self.index.fruity_into(),
                "type".to_string() => self.ty.fruity_into(),
            },
        }
    }
}

impl FruityTryFrom<Serialized> for MaterialParamsBindingType {
    type Error = String;

    fn fruity_try_from(value: Serialized) -> Result<Self, Self::Error> {
        if let Serialized::SerializedObject { class_name, fields } = value {
            match &class_name as &str {
                "texture" => Ok(MaterialParamsBindingType::Texture {
                    texture: ResourceReference::<dyn TextureResource>::fruity_try_from(
                        fields.get("texture"),
                    )?,
                }),
                "sampler" => Ok(MaterialParamsBindingType::Sampler {
                    texture: ResourceReference::<dyn TextureResource>::fruity_try_from(
                        fields.get("texture"),
                    )?,
                }),
                "uniform" => Ok(MaterialParamsBindingType::Uniform),
                _ => Err(format!("Couldn't convert {:?} to object", class_name)),
            }
        } else {
            Err(format!("Couldn't convert {:?} to object", value))
        }
    }
}

impl FruityInto<Serialized> for MaterialParamsBindingType {
    fn fruity_into(self) -> Serialized {
        match self {
            MaterialParamsBindingType::Texture { texture } => Serialized::SerializedObject {
                class_name: "texture".to_string(),
                fields: hashmap! {
                    "texture".to_string() => texture.fruity_into(),
                },
            },
            MaterialParamsBindingType::Sampler { texture } => Serialized::SerializedObject {
                class_name: "sampler".to_string(),
                fields: hashmap! {
                    "texture".to_string() => texture.fruity_into(),
                },
            },
            MaterialParamsBindingType::Uniform => Serialized::SerializedObject {
                class_name: "uniform".to_string(),
                fields: HashMap::new(),
            },
        }
    }
}

impl FruityTryFrom<Serialized> for MaterialParamsBindingGroupType {
    type Error = String;

    fn fruity_try_from(value: Serialized) -> Result<Self, Self::Error> {
        if let Serialized::SerializedObject { class_name, fields } = value {
            match &class_name as &str {
                "custom" => Ok(MaterialParamsBindingGroupType::Custom(Vec::<
                    MaterialParamsBinding,
                >::fruity_try_from(
                    fields.get("group_types"),
                )?)),
                "camera" => Ok(MaterialParamsBindingGroupType::Camera),
                _ => Err(format!("Couldn't convert {:?} to object", class_name)),
            }
        } else {
            Err(format!("Couldn't convert {:?} to object", value))
        }
    }
}

impl FruityInto<Serialized> for MaterialParamsBindingGroupType {
    fn fruity_into(self) -> Serialized {
        match self {
            MaterialParamsBindingGroupType::Custom(group_types) => Serialized::SerializedObject {
                class_name: "custom".to_string(),
                fields: hashmap! {
                    "group_types".to_string() => group_types.fruity_into(),
                },
            },
            MaterialParamsBindingGroupType::Camera => Serialized::SerializedObject {
                class_name: "camera".to_string(),
                fields: HashMap::new(),
            },
        }
    }
}
