use crate::serialize::Serialized;
use linked_hash_map::LinkedHashMap;
use std::collections::HashMap;
use std::io::Read;
use std::io::Write;
use yaml_rust::Yaml;
use yaml_rust::YamlEmitter;
use yaml_rust::YamlLoader;

/// Serialize a [’Serialized’] as a yaml file
///
/// # Arguments
/// * `reader` - The read io stream
///
pub fn serialize_yaml(
    writer: &mut dyn Write,
    serialized: &Serialized,
) -> Result<(), std::io::Error> {
    let yaml = intern_serialize_yaml(serialized);

    let mut write_buf = String::new();
    let mut emitter = YamlEmitter::new(&mut write_buf);
    emitter.dump(&yaml).unwrap();

    writer.write_all(write_buf.as_bytes())?;
    Ok(())
}

/// Deserialize a [’Serialized’] from a yaml file
///
/// # Arguments
/// * `reader` - The read io stream
///
pub fn deserialize_yaml(reader: &mut dyn Read) -> Option<Serialized> {
    let mut buffer = String::new();
    if let Err(err) = reader.read_to_string(&mut buffer) {
        log::error!("{}", err.to_string());
        return None;
    }

    let docs = YamlLoader::load_from_str(&buffer).unwrap();
    let yaml = &docs[0];

    intern_deserialize_yaml(yaml)
}

fn intern_serialize_yaml(serialized: &Serialized) -> Yaml {
    match serialized {
        Serialized::I8(value) => Yaml::Integer(*value as i64),
        Serialized::I16(value) => Yaml::Integer(*value as i64),
        Serialized::I32(value) => Yaml::Integer(*value as i64),
        Serialized::I64(value) => Yaml::Integer(*value as i64),
        Serialized::ISize(value) => Yaml::Integer(*value as i64),
        Serialized::U8(value) => Yaml::Integer(*value as i64),
        Serialized::U16(value) => Yaml::Integer(*value as i64),
        Serialized::U32(value) => Yaml::Integer(*value as i64),
        Serialized::U64(value) => Yaml::Integer(*value as i64),
        Serialized::USize(value) => Yaml::Integer(*value as i64),
        Serialized::F32(value) => Yaml::Real(value.to_string()),
        Serialized::F64(value) => Yaml::Real(value.to_string()),
        Serialized::Bool(value) => Yaml::Boolean(*value),
        Serialized::String(value) => Yaml::String(value.to_string()),
        Serialized::Array(value) => {
            let elements = value
                .iter()
                .map(|elem| intern_serialize_yaml(elem))
                .collect::<Vec<_>>();

            Yaml::Array(elements)
        }
        Serialized::Null => Yaml::Null,
        Serialized::Iterator(_) => Yaml::BadValue,
        Serialized::Callback(_) => Yaml::BadValue,
        Serialized::SerializedObject { class_name, fields } => {
            let mut hashmap = LinkedHashMap::<Yaml, Yaml>::new();
            let field_hashmap = {
                let mut hashmap = LinkedHashMap::<Yaml, Yaml>::new();

                fields.iter().for_each(|(key, value)| {
                    let key = Yaml::String(key.clone());
                    hashmap.insert(key, intern_serialize_yaml(value));
                });

                hashmap
            };

            hashmap.insert(
                Yaml::String("class_name".to_string()),
                Yaml::String(class_name.clone()),
            );

            hashmap.insert(
                Yaml::String("fields".to_string()),
                Yaml::Hash(field_hashmap),
            );

            Yaml::Hash(hashmap)
        }
        Serialized::NativeObject(_) => Yaml::BadValue,
    }
}

fn intern_deserialize_yaml(yaml: &Yaml) -> Option<Serialized> {
    match yaml {
        Yaml::Real(string) => match string.parse::<f64>() {
            Ok(value) => Some(Serialized::F64(value)),
            Err(_) => None,
        },
        Yaml::Integer(value) => Some(Serialized::I64(*value)),
        Yaml::String(value) => Some(Serialized::String(value.clone())),
        Yaml::Boolean(value) => Some(Serialized::Bool(*value)),
        Yaml::Array(array) => {
            let settings_array = array
                .iter()
                .filter_map(|elem| intern_deserialize_yaml(elem))
                .collect::<Vec<_>>();

            Some(Serialized::Array(settings_array))
        }
        Yaml::Hash(hashmap) => {
            let class_name_key = Yaml::String("class_name".to_string());
            let class_name = hashmap.get(&class_name_key)?;

            let fields_key = Yaml::String("fields".to_string());
            let fields = hashmap.get(&fields_key)?;

            if let Yaml::String(class_name) = class_name {
                if let Yaml::Hash(hashmap) = fields {
                    let mut fields = HashMap::new();

                    for (key, value) in hashmap {
                        if let Yaml::String(key) = key {
                            if let Some(settings) = intern_deserialize_yaml(value) {
                                fields.insert(key.clone(), settings);
                            }
                        }
                    }

                    Some(Serialized::SerializedObject {
                        class_name: class_name.to_string(),
                        fields,
                    })
                } else {
                    return None;
                }
            } else {
                return None;
            }
        }
        Yaml::Alias(_) => None,
        Yaml::Null => None,
        Yaml::BadValue => None,
    }
}
