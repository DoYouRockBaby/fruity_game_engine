use crate::math::Color;
use crate::resources::shader_resource::ShaderResource;
use crate::resources::texture_resource::TextureResource;
use fruity_any::*;
use fruity_core::convert::FruityInto;
use fruity_core::convert::FruityTryFrom;
use fruity_core::introspect::FieldInfo;
use fruity_core::introspect::IntrospectObject;
use fruity_core::introspect::MethodInfo;
use fruity_core::resource::resource::Resource;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::resource::resource_reference::ResourceReference;
use fruity_core::serialize::serialized::SerializableObject;
use fruity_core::serialize::serialized::Serialized;
use fruity_core::settings::build_settings_from_yaml;
use fruity_core::settings::Settings;
use fruity_ecs::*;
use std::collections::HashMap;
use std::io::Read;
use std::sync::Arc;
use yaml_rust::YamlLoader;

#[derive(Debug, Clone, Default, FruityAny, IntrospectObject)]
pub struct MaterialResource {
    pub shader: Option<ResourceReference<dyn ShaderResource>>,
    pub fields: HashMap<String, Vec<MaterialField>>,
}

impl Resource for MaterialResource {}

#[derive(Debug, Clone, FruityAny)]
pub enum MaterialField {
    Texture {
        default: ResourceReference<dyn TextureResource>,
        bind_group: u32,
        bind: u32,
    },
    Sampler {
        default: ResourceReference<dyn TextureResource>,
        bind_group: u32,
        bind: u32,
    },
    Color {
        default: Color,
        bind_group: u32,
        bind: u32,
    },
    Camera {
        bind_group: u32,
    },
}

// TODO: Complete that
impl IntrospectObject for MaterialField {
    fn get_class_name(&self) -> String {
        "MaterialField".to_string()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}

impl SerializableObject for MaterialField {
    fn duplicate(&self) -> Box<dyn SerializableObject> {
        Box::new(self.clone())
    }
}

impl FruityTryFrom<Serialized> for MaterialField {
    type Error = String;

    fn fruity_try_from(value: Serialized) -> Result<Self, Self::Error> {
        match value {
            Serialized::NativeObject(value) => {
                match value.as_any_box().downcast::<MaterialField>() {
                    Ok(value) => Ok(*value),
                    Err(_) => Err(format!("Couldn't convert a MaterialField to native object")),
                }
            }
            _ => Err(format!("Couldn't convert {:?} to native object", value)),
        }
    }
}

impl FruityInto<Serialized> for MaterialField {
    fn fruity_into(self) -> Serialized {
        Serialized::NativeObject(Box::new(self))
    }
}

pub fn load_material(
    identifier: &str,
    reader: &mut dyn Read,
    _settings: Settings,
    resource_container: Arc<ResourceContainer>,
) {
    // read the whole file
    let mut buffer = String::new();
    if let Err(err) = reader.read_to_string(&mut buffer) {
        log::error!("{}", err.to_string());
        return;
    }

    let docs = YamlLoader::load_from_str(&buffer).unwrap();
    let root = &docs[0];
    let settings = if let Some(settings) = build_settings_from_yaml(root) {
        settings
    } else {
        return;
    };

    // Build the resource
    let resource = build_material(&settings, resource_container.clone());

    // Store the resource
    if let Err(_) = resource_container.add::<MaterialResource>(identifier, Box::new(resource)) {
        log::error!(
            "Couldn't add a resource cause the identifier \"{}\" already exists",
            identifier
        );
        return;
    }
}

pub fn build_material(
    settings: &Settings,
    resource_container: Arc<ResourceContainer>,
) -> MaterialResource {
    let shader_identifier = settings.get::<String>("shader", String::default());
    let shader = resource_container.get::<dyn ShaderResource>(&shader_identifier);

    let fields_settings = settings.get::<Vec<Settings>>("fields", Vec::new());
    let mut fields = HashMap::<String, Vec<MaterialField>>::new();
    fields_settings.iter().for_each(|params| {
        let name = params.get::<Option<String>>("name", None);

        if let Some(name) = name {
            if let Some(field) = build_material_field(params, resource_container.clone()) {
                if let Some(fields) = fields.get_mut(&name) {
                    fields.push(field);
                } else {
                    fields.insert(name, vec![field]);
                }
            }
        }
    });

    MaterialResource { shader, fields }
}

fn build_material_field(
    settings: &Settings,
    resource_container: Arc<ResourceContainer>,
) -> Option<MaterialField> {
    match &settings.get::<String>("type", String::default()) as &str {
        "texture" => {
            let default = settings.get::<String>("default", String::default());
            let default = resource_container.get::<dyn TextureResource>(&default);
            let bind_group = settings.get::<u32>("bind_group", u32::default());
            let bind = settings.get::<u32>("bind", u32::default());

            if let Some(default) = default {
                Some(MaterialField::Texture {
                    default,
                    bind_group,
                    bind,
                })
            } else {
                None
            }
        }
        "sampler" => {
            let default = settings.get::<String>("default", String::default());
            let default = resource_container.get::<dyn TextureResource>(&default);
            let bind_group = settings.get::<u32>("bind_group", u32::default());
            let bind = settings.get::<u32>("bind", u32::default());

            if let Some(default) = default {
                Some(MaterialField::Sampler {
                    default,
                    bind_group,
                    bind,
                })
            } else {
                None
            }
        }
        "color" => {
            let default = settings.get::<Color>("default", Color::default());
            let bind_group = settings.get::<u32>("bind_group", u32::default());
            let bind = settings.get::<u32>("bind", u32::default());

            Some(MaterialField::Color {
                default,
                bind_group,
                bind,
            })
        }
        "camera" => {
            let bind_group = settings.get::<u32>("bind_group", u32::default());
            Some(MaterialField::Camera { bind_group })
        }
        _ => None,
    }
}
