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
use fruity_core::utils::collection::insert_in_hashmap_vec;
use fruity_ecs::*;
use std::collections::HashMap;
use std::io::Read;
use std::sync::Arc;
use yaml_rust::YamlLoader;

#[derive(Debug, Clone, Default, FruityAny, IntrospectObject)]
pub struct MaterialResource {
    pub shader: Option<ResourceReference<dyn ShaderResource>>,
    pub bindings: HashMap<String, Vec<MaterialBinding>>,
    pub instance_attributes: HashMap<String, MaterialInstanceAttribute>,
}

impl Resource for MaterialResource {}

#[derive(Debug, Clone, FruityAny)]
pub enum MaterialBinding {
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
impl IntrospectObject for MaterialBinding {
    fn get_class_name(&self) -> String {
        "MaterialBinding".to_string()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}

impl SerializableObject for MaterialBinding {
    fn duplicate(&self) -> Box<dyn SerializableObject> {
        Box::new(self.clone())
    }
}

impl FruityTryFrom<Serialized> for MaterialBinding {
    type Error = String;

    fn fruity_try_from(value: Serialized) -> Result<Self, Self::Error> {
        match value {
            Serialized::NativeObject(value) => {
                match value.as_any_box().downcast::<MaterialBinding>() {
                    Ok(value) => Ok(*value),
                    Err(_) => Err(format!(
                        "Couldn't convert a MaterialBinding to native object"
                    )),
                }
            }
            _ => Err(format!("Couldn't convert {:?} to native object", value)),
        }
    }
}

impl FruityInto<Serialized> for MaterialBinding {
    fn fruity_into(self) -> Serialized {
        Serialized::NativeObject(Box::new(self))
    }
}

#[derive(Debug, Clone, FruityAny)]
pub enum MaterialInstanceAttribute {
    Matrix4 {
        vec0_location: u32,
        vec1_location: u32,
        vec2_location: u32,
        vec3_location: u32,
    },
    Rect {
        vec0_location: u32,
        vec1_location: u32,
    },
    Vector4 {
        location: u32,
    },
}

// TODO: Complete that
impl IntrospectObject for MaterialInstanceAttribute {
    fn get_class_name(&self) -> String {
        "MaterialInstanceAttribute".to_string()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}

impl SerializableObject for MaterialInstanceAttribute {
    fn duplicate(&self) -> Box<dyn SerializableObject> {
        Box::new(self.clone())
    }
}

impl FruityTryFrom<Serialized> for MaterialInstanceAttribute {
    type Error = String;

    fn fruity_try_from(value: Serialized) -> Result<Self, Self::Error> {
        match value {
            Serialized::NativeObject(value) => {
                match value.as_any_box().downcast::<MaterialInstanceAttribute>() {
                    Ok(value) => Ok(*value),
                    Err(_) => Err(format!(
                        "Couldn't convert a MaterialInstanceAttribute to native object"
                    )),
                }
            }
            _ => Err(format!("Couldn't convert {:?} to native object", value)),
        }
    }
}

impl FruityInto<Serialized> for MaterialInstanceAttribute {
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
    resource_container.add::<MaterialResource>(identifier, Box::new(resource));
}

pub fn build_material(
    settings: &Settings,
    resource_container: Arc<ResourceContainer>,
) -> MaterialResource {
    let shader_identifier = settings.get::<String>("shader", String::default());
    let shader = resource_container.get::<dyn ShaderResource>(&shader_identifier);

    let bindings_settings = settings.get::<Vec<Settings>>("bindings", Vec::new());
    let mut bindings = HashMap::<String, Vec<MaterialBinding>>::new();
    bindings_settings.iter().for_each(|params| {
        let name = params.get::<Option<String>>("name", None);

        if let Some(name) = name {
            if let Some(binding) = build_material_binding(params, resource_container.clone()) {
                insert_in_hashmap_vec(&mut bindings, name, binding);
            }
        }
    });

    let instance_attributes_settings =
        settings.get::<Vec<Settings>>("instance_attributes", Vec::new());
    let mut instance_attributes = HashMap::<String, MaterialInstanceAttribute>::new();
    instance_attributes_settings.iter().for_each(|params| {
        let name = params.get::<Option<String>>("name", None);

        if let Some(name) = name {
            if let Some(instance_attribute) =
                build_material_instance_attribute(params, resource_container.clone())
            {
                instance_attributes.insert(name, instance_attribute);
            }
        }
    });

    MaterialResource {
        shader,
        bindings,
        instance_attributes,
    }
}

fn build_material_binding(
    settings: &Settings,
    resource_container: Arc<ResourceContainer>,
) -> Option<MaterialBinding> {
    match &settings.get::<String>("type", String::default()) as &str {
        "texture" => {
            let default = settings.get::<String>("default", String::default());
            let default = resource_container.get::<dyn TextureResource>(&default);
            let bind_group = settings.get::<u32>("bind_group", u32::default());
            let bind = settings.get::<u32>("bind", u32::default());

            if let Some(default) = default {
                Some(MaterialBinding::Texture {
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
                Some(MaterialBinding::Sampler {
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

            Some(MaterialBinding::Color {
                default,
                bind_group,
                bind,
            })
        }
        "camera" => {
            let bind_group = settings.get::<u32>("bind_group", u32::default());
            Some(MaterialBinding::Camera { bind_group })
        }
        _ => None,
    }
}

fn build_material_instance_attribute(
    settings: &Settings,
    _resource_container: Arc<ResourceContainer>,
) -> Option<MaterialInstanceAttribute> {
    match &settings.get::<String>("type", String::default()) as &str {
        "matrix4" => {
            let vec0_location = settings.get::<u32>("vec0_location", u32::default());
            let vec1_location = settings.get::<u32>("vec1_location", u32::default());
            let vec2_location = settings.get::<u32>("vec2_location", u32::default());
            let vec3_location = settings.get::<u32>("vec3_location", u32::default());

            Some(MaterialInstanceAttribute::Matrix4 {
                vec0_location,
                vec1_location,
                vec2_location,
                vec3_location,
            })
        }
        "rect" => {
            let vec0_location = settings.get::<u32>("vec0_location", u32::default());
            let vec1_location = settings.get::<u32>("vec1_location", u32::default());

            Some(MaterialInstanceAttribute::Rect {
                vec0_location,
                vec1_location,
            })
        }
        "vec4" => {
            let location = settings.get::<u32>("location", u32::default());

            Some(MaterialInstanceAttribute::Vector4 { location })
        }
        _ => None,
    }
}
