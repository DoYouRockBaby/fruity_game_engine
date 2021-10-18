use crate::resource::resource::Resource;
use crate::resource::resources_manager::ResourceIdentifier;
use crate::serialize::serialized::Serialized;
use crate::ResourcesManager;
use fruity_any::*;
use std::collections::HashMap;
use std::io::Read;
use yaml_rust::Yaml;
use yaml_rust::YamlLoader;

/// Load many resources using a resource settings file
pub mod resources_loader;

/// Settings collection
#[derive(Debug, Clone, FruityAnySyncSend)]
pub struct Settings {
    serialized: Serialized,
}

impl Settings {
    /// Return a Settings
    pub fn new(serialized: Serialized) -> Settings {
        Settings { serialized }
    }
}

impl Resource for Settings {}

/// The loader for settings files
pub fn settings_loader(
    resources_manager: &mut ResourcesManager,
    identifier: ResourceIdentifier,
    reader: &mut dyn Read,
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
    let serialized = build_settings_serialized_from_yaml(&docs[0]);
    if let Some(serialized) = serialized {
        let resource = Settings::new(serialized);
        // Store the resource
        if let Err(_) = resources_manager.add_resource(identifier.clone(), resource) {
            log::error!(
                "Couldn't add a resource cause the identifier \"{}\" already exists",
                &identifier.0
            );
            return;
        }
    }
}

/// Build a Serialized by reading a yaml document
pub fn build_settings_serialized_from_yaml(yaml: &Yaml) -> Option<Serialized> {
    match yaml {
        Yaml::Real(string) => match string.parse::<f32>() {
            Ok(value) => Some(Serialized::F32(value)),
            Err(_) => None,
        },
        Yaml::Integer(value) => Some(Serialized::I32(*value as i32)),
        Yaml::String(value) => Some(Serialized::String(value.clone())),
        Yaml::Boolean(value) => Some(Serialized::Bool(*value as bool)),
        Yaml::Array(array) => {
            let serialized_array = array
                .iter()
                .filter_map(|elem| build_settings_serialized_from_yaml(elem))
                .collect::<Vec<_>>();

            Some(Serialized::Array(serialized_array))
        }
        Yaml::Hash(hashmap) => {
            let mut fields = HashMap::new();

            for (key, value) in hashmap {
                if let Yaml::String(key) = key {
                    if let Some(serialized) = build_settings_serialized_from_yaml(value) {
                        fields.insert(key.clone(), serialized);
                    }
                }
            }

            Some(Serialized::Object {
                class_name: "unknown".to_string(),
                fields,
            })
        }
        Yaml::Alias(_) => None,
        Yaml::Null => None,
        Yaml::BadValue => None,
    }
}
