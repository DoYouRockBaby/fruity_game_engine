use crate::serialize::serialized::Serialized;
use crate::settings::build_settings_serialized_from_yaml;
use crate::settings::ResourceIdentifier;
use crate::ResourcesManager;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use yaml_rust::Yaml;
use yaml_rust::YamlLoader;

/// A resource loader that load other resources from a resource settings file
pub fn resources_loader(
    resources_manager: &mut ResourcesManager,
    _identifier: ResourceIdentifier,
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
    let root = &docs[0];

    // Load each resources
    if let Yaml::Array(resources) = root {
        resources.iter().for_each(|elem| {
            load_resource_from_settings(resources_manager, elem);
        });
    }
}

fn load_resource_from_settings(
    resources_manager: &mut ResourcesManager,
    yaml: &Yaml,
) -> Option<()> {
    // Parse settings
    let resource_settings = build_settings_serialized_from_yaml(yaml)?;
    let fields = if let Serialized::Object { fields, .. } = resource_settings {
        fields
    } else {
        return None;
    };

    // Get the resource path
    let path = fields.get("path")?;
    let path = if let Serialized::String(path) = path {
        path
    } else {
        return None;
    };

    // Deduce informations about the resource from the path
    let resource_type = Path::new(path).extension()?;
    let resource_type = resource_type.to_str()?;
    let resource_identifier = ResourceIdentifier(path.clone());
    let mut resource_file = File::open(path).ok()?;

    // Load the resource
    resources_manager
        .load_resource(resource_identifier, resource_type, &mut resource_file)
        .ok()?;

    Some(())
}