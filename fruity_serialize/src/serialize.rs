use crate::serialized::Serialized;
use std::any::Any;

/// Trait that should be implemented to serialize the value
pub trait Serialize {
    /// Serialize a value
    fn serialize(&self) -> Serialized;
}

macro_rules! serialize_type {
    ($value:expr, $type:ident) => {
        match $value.downcast_ref::<$type>() {
            Some(value) => return Some($type::serialize(value)),
            _ => (),
        };
    };
}

/// Serialize an Any value
///
/// # Arguments
/// * `value` - The Any value to serialize
///
pub fn serialize_any<'a>(value: &dyn Any) -> Option<Serialized> {
    serialize_type!(value, i8);
    serialize_type!(value, i16);
    serialize_type!(value, i32);
    serialize_type!(value, i64);
    serialize_type!(value, isize);
    serialize_type!(value, u8);
    serialize_type!(value, u16);
    serialize_type!(value, u32);
    serialize_type!(value, u64);
    serialize_type!(value, usize);
    serialize_type!(value, f32);
    serialize_type!(value, f64);
    serialize_type!(value, bool);
    serialize_type!(value, String);

    None
}
