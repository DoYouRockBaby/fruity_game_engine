use crate::serialized::Serialized;
use std::any::Any;

/// Trait that should be implemented to deserialize the value
pub trait Deserialize {
    /// The type of the deserialization target
    type Value;

    /// Deserialize a value
    fn deserialize(serialized: Serialized) -> Option<Self::Value>;
}

macro_rules! deserialize_type {
    ($value:expr, $enum:ident) => {
        match $value {
            Serialized::$enum(value) => return Some(Box::new(value)),
            _ => (),
        };
    };
}

/// Serialize an Any value
///
/// # Arguments
/// * `value` - The Any value to serialize
///
pub fn deserialize_any<'a>(value: Serialized) -> Option<Box<dyn Any>> {
    deserialize_type!(value, I8);
    deserialize_type!(value, I16);
    deserialize_type!(value, I32);
    deserialize_type!(value, I64);
    deserialize_type!(value, ISize);
    deserialize_type!(value, U8);
    deserialize_type!(value, U16);
    deserialize_type!(value, U32);
    deserialize_type!(value, U64);
    deserialize_type!(value, USize);
    deserialize_type!(value, F32);
    deserialize_type!(value, F64);
    deserialize_type!(value, Bool);
    deserialize_type!(value, String);

    None
}
