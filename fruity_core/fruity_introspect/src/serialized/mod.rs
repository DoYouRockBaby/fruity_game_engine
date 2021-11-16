#![warn(missing_docs)]

//! Serialize
//!
//! Provide a structure that will be used all over the application to serialize/deserialize things
//! Will be used to make a bridge between the rust ecosystem and the scripting language and by the
//! data storage

/// Implementation of serialized conversions for primitives
pub mod impl_primitives;

/// Implementation of serialized conversions for containers (like Vec, Box ...)
pub mod impl_containers;

/// Implementation of serialized conversions for tuples
pub mod impl_tuples;

/// Provides a factory for the introspect types
pub mod object_factory;

/// Provides a function to serialize and deserialize
pub mod serialize;

/// Provides functions to serialize and deserialize a serialized value to yaml
pub mod yaml;

use crate::serializable_object::SerializableObject;
use crate::serialized::object_factory::ObjectFactory;
use crate::IntrospectError;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt::Debug;
use std::sync::Arc;
use std::sync::RwLock;

/// A callback for the serialized type
pub type Callback = Arc<
    dyn Fn(Vec<Serialized>) -> Result<Option<Serialized>, IntrospectError> + Sync + Send + 'static,
>;

/// A list of serialized object fields
pub type ObjectFields = HashMap<String, Serialized>;

#[derive(Clone)]
/// A serialized value
pub enum Serialized {
    /// i8 value
    I8(i8),

    /// i16 value
    I16(i16),

    /// i32 value
    I32(i32),

    /// i64 value
    I64(i64),

    /// isize value
    ISize(isize),

    /// u8 value
    U8(u8),

    /// u16 value
    U16(u16),

    /// u32 value
    U32(u32),

    /// u64 value
    U64(u64),

    /// usize value
    USize(usize),

    /// f32 value
    F32(f32),

    /// f64 value
    F64(f64),

    /// bool value
    Bool(bool),

    /// String value
    String(String),

    /// Array of values
    Array(Vec<Serialized>),

    /// A null value, correspond to [’Option::None’]
    Null,

    /// Iterator over values
    Iterator(Arc<RwLock<dyn Iterator<Item = Serialized> + Send + Sync>>),

    /// A callback
    Callback(
        Arc<
            dyn Fn(Vec<Serialized>) -> Result<Option<Serialized>, IntrospectError>
                + Sync
                + Send
                + 'static,
        >,
    ),

    /// An object stored as an hashmap, mostly used to grab objects from the scripting runtime
    SerializedObject {
        /// The object class name
        class_name: String,

        /// The object fields
        fields: HashMap<String, Serialized>,
    },

    /// An object created by rust
    NativeObject(Box<dyn SerializableObject>),
}

impl<T: TryFrom<Serialized> + ?Sized> TryFrom<Serialized> for Vec<T> {
    type Error = String;

    fn try_from(value: Serialized) -> Result<Self, Self::Error> {
        match value {
            Serialized::Array(value) => Ok(value
                .into_iter()
                .filter_map(|elem| T::try_from(elem).ok())
                .collect()),
            _ => Err(format!("Couldn't convert {:?} to array", value)),
        }
    }
}

impl TryFrom<Serialized> for Callback {
    type Error = String;

    fn try_from(value: Serialized) -> Result<Self, Self::Error> {
        match value {
            Serialized::Callback(value) => Ok(value.clone()),
            _ => Err(format!("Couldn't convert {:?} to callback", value)),
        }
    }
}

impl TryFrom<Serialized> for ObjectFields {
    type Error = String;

    fn try_from(value: Serialized) -> Result<Self, Self::Error> {
        match value {
            Serialized::SerializedObject { fields, .. } => Ok(fields),
            _ => Err(format!("Couldn't convert {:?} to field hashmap", value)),
        }
    }
}

impl TryFrom<Serialized> for Box<dyn SerializableObject> {
    type Error = String;

    fn try_from(value: Serialized) -> Result<Self, Self::Error> {
        match value {
            Serialized::NativeObject(value) => Ok(value),
            _ => Err(format!("Couldn't convert {:?} to native object", value)),
        }
    }
}

impl Debug for Serialized {
    fn fmt(
        &self,
        _formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}
