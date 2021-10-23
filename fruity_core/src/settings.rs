use crate::resource::resource::Resource;
use crate::resource::resources_manager::ResourceIdentifier;
use crate::ResourcesManager;
use crate::ServiceManager;
use fruity_any::*;
use fruity_introspect::FieldInfo;
use fruity_introspect::IntrospectObject;
use fruity_introspect::MethodInfo;
use std::collections::HashMap;
use std::io::Read;
use std::sync::Arc;
use std::sync::RwLock;
use yaml_rust::Yaml;
use yaml_rust::YamlLoader;

/// Settings collection
#[derive(Debug, FruityAny)]
pub enum Settings {
    /// i64 value
    I64(i64),

    /// f64 value
    F64(f64),

    /// bool value
    Bool(bool),

    /// String value
    String(String),

    /// Array of values
    Array(Vec<Settings>),

    /// An object stored as an hashmap, mostly used to grab objects from the scripting runtime
    Object(HashMap<String, Settings>),
}

impl Settings {
    pub fn new() -> Settings {
        Settings::Object(HashMap::new())
    }
}

impl Resource for Settings {}

impl IntrospectObject for Settings {
    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}

/// The loader for settings files
pub fn settings_loader(
    resources_manager: &mut ResourcesManager,
    identifier: ResourceIdentifier,
    reader: &mut dyn Read,
    _settings: Settings,
    _service_manager: Arc<RwLock<ServiceManager>>,
) {
    // read the whole file
    let mut buffer = String::new();
    if let Err(err) = reader.read_to_string(&mut buffer) {
        log::error!("{}", err.to_string());
        return;
    }

    // Load the yaml
    let docs = YamlLoader::load_from_str(&buffer).unwrap();

    // Build the settings
    let settings = build_settings_from_yaml(&docs[0]);
    if let Some(settings) = settings {
        // Store the resource
        if let Err(_) = resources_manager.add_resource(identifier.clone(), settings) {
            log::error!(
                "Couldn't add a resource cause the identifier \"{}\" already exists",
                &identifier.0
            );
            return;
        }
    }
}

/// Build a Settings by reading a yaml document
pub fn build_settings_from_yaml(yaml: &Yaml) -> Option<Settings> {
    match yaml {
        Yaml::Real(string) => match string.parse::<f64>() {
            Ok(value) => Some(Settings::F64(value)),
            Err(_) => None,
        },
        Yaml::Integer(value) => Some(Settings::I64(*value)),
        Yaml::String(value) => Some(Settings::String(value.clone())),
        Yaml::Boolean(value) => Some(Settings::Bool(*value)),
        Yaml::Array(array) => {
            let serialized_array = array
                .iter()
                .filter_map(|elem| build_settings_from_yaml(elem))
                .collect::<Vec<_>>();

            Some(Settings::Array(serialized_array))
        }
        Yaml::Hash(hashmap) => {
            let mut fields = HashMap::new();

            for (key, value) in hashmap {
                if let Yaml::String(key) = key {
                    if let Some(serialized) = build_settings_from_yaml(value) {
                        fields.insert(key.clone(), serialized);
                    }
                }
            }

            Some(Settings::Object(fields))
        }
        Yaml::Alias(_) => None,
        Yaml::Null => None,
        Yaml::BadValue => None,
    }
}
