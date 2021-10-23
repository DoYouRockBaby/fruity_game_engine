use crate::resource::resource::Resource;
use fruity_any::*;
use fruity_introspect::FieldInfo;
use fruity_introspect::IntrospectObject;
use fruity_introspect::MethodInfo;
use std::collections::HashMap;
use std::convert::TryFrom;
use yaml_rust::Yaml;

/// Settings collection
#[derive(Debug, Clone, FruityAny)]
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
    /// Returns a Settings
    pub fn new() -> Settings {
        Settings::Object(HashMap::new())
    }

    /// Get a field into the params
    ///
    /// # Arguments
    /// * `key` - The field identifier
    /// * `default` - The default value, if not found or couldn't serialize
    ///
    /// # Generic Arguments
    /// * `T` - The type to cast the value
    ///
    pub fn get<T: TryFrom<Settings> + ?Sized>(&self, key: &str, default: T) -> T {
        match self {
            Settings::Object(fields) => match fields.get(key) {
                Some(value) => T::try_from(value.clone()).unwrap_or(default),
                None => default,
            },
            _ => default,
        }
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
            let Settings_array = array
                .iter()
                .filter_map(|elem| build_settings_from_yaml(elem))
                .collect::<Vec<_>>();

            Some(Settings::Array(Settings_array))
        }
        Yaml::Hash(hashmap) => {
            let mut fields = HashMap::new();

            for (key, value) in hashmap {
                if let Yaml::String(key) = key {
                    if let Some(Settings) = build_settings_from_yaml(value) {
                        fields.insert(key.clone(), Settings);
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

macro_rules! impl_numeric_from_settings {
    ( $type:ident ) => {
        impl TryFrom<Settings> for $type {
            type Error = String;

            fn try_from(value: Settings) -> Result<Self, Self::Error> {
                match value {
                    Settings::I64(value) => Ok(value as $type),
                    Settings::F64(value) => Ok(value as $type),
                    _ => Err(format!("Couldn't convert {:?} to {}", value, "$type")),
                }
            }
        }
    };
}

impl_numeric_from_settings!(i8);
impl_numeric_from_settings!(i16);
impl_numeric_from_settings!(i32);
impl_numeric_from_settings!(i64);
impl_numeric_from_settings!(isize);
impl_numeric_from_settings!(u8);
impl_numeric_from_settings!(u16);
impl_numeric_from_settings!(u32);
impl_numeric_from_settings!(u64);
impl_numeric_from_settings!(usize);
impl_numeric_from_settings!(f32);
impl_numeric_from_settings!(f64);

impl TryFrom<Settings> for bool {
    type Error = String;

    fn try_from(value: Settings) -> Result<Self, Self::Error> {
        match value {
            Settings::Bool(value) => Ok(value),
            _ => Err(format!("Couldn't convert {:?} to bool", value)),
        }
    }
}

impl TryFrom<Settings> for String {
    type Error = String;

    fn try_from(value: Settings) -> Result<Self, Self::Error> {
        match value {
            Settings::String(value) => Ok(value),
            _ => Err(format!("Couldn't convert {:?} to bool", value)),
        }
    }
}

impl<T: TryFrom<Settings> + ?Sized> TryFrom<Settings> for Vec<T> {
    type Error = String;

    fn try_from(value: Settings) -> Result<Self, Self::Error> {
        match value {
            Settings::Array(value) => Ok(value
                .into_iter()
                .filter_map(|elem| T::try_from(elem).ok())
                .collect()),
            _ => Err(format!("Couldn't convert {:?} to array", value)),
        }
    }
}
