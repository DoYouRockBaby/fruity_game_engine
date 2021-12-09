use crate::graphic_service::GraphicService;
use crate::resources::shader_resource::ShaderResource;
use crate::resources::texture_resource::TextureResource;
use fruity_any::*;
use fruity_core::introspect::FieldInfo;
use fruity_core::introspect::IntrospectObject;
use fruity_core::introspect::MethodInfo;
use fruity_core::resource::resource::Resource;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::resource::resource_reference::ResourceReference;
use fruity_core::settings::build_settings_from_yaml;
use fruity_core::settings::Settings;
use fruity_ecs::*;
use std::collections::HashMap;
use std::io::Read;
use std::sync::Arc;
use yaml_rust::YamlLoader;

pub trait MaterialResource: Resource {
    fn get_shader(&self) -> Option<ResourceReference<dyn ShaderResource>>;
}

#[derive(Debug, Clone, FruityAny, SerializableObject)]
pub struct MaterialResourceSettings {
    pub shader: Option<ResourceReference<dyn ShaderResource>>,
    pub bindings: Vec<MaterialSettingsBinding>,
    pub instance_attributes: HashMap<String, MaterialSettingsInstanceAttribute>,
}

// TODO: Complete that
impl IntrospectObject for MaterialResourceSettings {
    fn get_class_name(&self) -> String {
        "MaterialResourceSettings".to_string()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}

#[derive(Debug, Clone, SerializableObject, FruityAny)]
pub enum MaterialSettingsBinding {
    Texture {
        value: ResourceReference<dyn TextureResource>,
        bind_group: u32,
    },
    Camera {
        bind_group: u32,
    },
}

// TODO: Complete that
impl IntrospectObject for MaterialSettingsBinding {
    fn get_class_name(&self) -> String {
        "MaterialSettingsBinding".to_string()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}

#[derive(Debug, Clone, FruityAny, SerializableObject)]
pub enum MaterialSettingsInstanceAttribute {
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
impl IntrospectObject for MaterialSettingsInstanceAttribute {
    fn get_class_name(&self) -> String {
        "MaterialSettingsInstanceAttribute".to_string()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}

pub fn load_material(
    identifier: &str,
    reader: &mut dyn Read,
    _settings: Settings,
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

    let docs = YamlLoader::load_from_str(&buffer).unwrap();
    let root = &docs[0];
    let settings = if let Some(settings) = build_settings_from_yaml(root) {
        settings
    } else {
        return;
    };

    // Parse settings
    let settings = read_material_settings(&settings, resource_container.clone());

    // Build the resource
    let result = graphic_service.create_material_resource(identifier, settings);

    // Store the resource
    match result {
        Ok(resource) => {
            resource_container.add::<dyn MaterialResource>(identifier, resource);
        }
        Err(err) => {
            log::error!("{}", err);
        }
    }
}

pub fn read_material_settings(
    settings: &Settings,
    resource_container: Arc<ResourceContainer>,
) -> MaterialResourceSettings {
    let shader_identifier = settings.get::<String>("shader", String::default());
    let shader = resource_container.get::<dyn ShaderResource>(&shader_identifier);

    let bindings_settings = settings.get::<Vec<Settings>>("bindings", Vec::new());
    let bindings = bindings_settings
        .iter()
        .filter_map(|params| build_material_binding(params, resource_container.clone()))
        .collect::<Vec<_>>();

    let instance_attributes_settings =
        settings.get::<Vec<Settings>>("instance_attributes", Vec::new());
    let mut instance_attributes = HashMap::<String, MaterialSettingsInstanceAttribute>::new();
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

    MaterialResourceSettings {
        shader,
        bindings,
        instance_attributes,
    }
}

fn build_material_binding(
    settings: &Settings,
    resource_container: Arc<ResourceContainer>,
) -> Option<MaterialSettingsBinding> {
    match &settings.get::<String>("type", String::default()) as &str {
        "texture" => {
            let value = settings.get::<String>("value", String::default());
            let value = resource_container.get::<dyn TextureResource>(&value);
            let bind_group = settings.get::<u32>("bind_group", u32::default());

            if let Some(value) = value {
                Some(MaterialSettingsBinding::Texture { value, bind_group })
            } else {
                None
            }
        }
        "camera" => {
            let bind_group = settings.get::<u32>("bind_group", u32::default());
            Some(MaterialSettingsBinding::Camera { bind_group })
        }
        _ => None,
    }
}

fn build_material_instance_attribute(
    settings: &Settings,
    _resource_container: Arc<ResourceContainer>,
) -> Option<MaterialSettingsInstanceAttribute> {
    match &settings.get::<String>("type", String::default()) as &str {
        "matrix4" => {
            let vec0_location = settings.get::<u32>("vec0_location", u32::default());
            let vec1_location = settings.get::<u32>("vec1_location", u32::default());
            let vec2_location = settings.get::<u32>("vec2_location", u32::default());
            let vec3_location = settings.get::<u32>("vec3_location", u32::default());

            Some(MaterialSettingsInstanceAttribute::Matrix4 {
                vec0_location,
                vec1_location,
                vec2_location,
                vec3_location,
            })
        }
        "rect" => {
            let vec0_location = settings.get::<u32>("vec0_location", u32::default());
            let vec1_location = settings.get::<u32>("vec1_location", u32::default());

            Some(MaterialSettingsInstanceAttribute::Rect {
                vec0_location,
                vec1_location,
            })
        }
        "vec4" => {
            let location = settings.get::<u32>("location", u32::default());

            Some(MaterialSettingsInstanceAttribute::Vector4 { location })
        }
        _ => None,
    }
}
