use crate::IntrospectError;
use crate::IntrospectObject;
use fruity_any::FruityAny;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt::Debug;
use std::sync::Arc;
use std::sync::RwLock;

/// A callback for the serialized type
pub type Callback = Arc<
    dyn Fn(Arc<dyn FruityAny>, Vec<Serialized>) -> Result<Option<Serialized>, IntrospectError>
        + Sync
        + Send
        + 'static,
>;

/// A list of serialized object fields
pub type ObjectFields = HashMap<String, Serialized>;

/// A serialized value
#[derive(Clone)]
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

    /// Iterator over values
    Iterator(Arc<RwLock<dyn Iterator<Item = Serialized> + Send + Sync>>),

    /// Service reference value
    Callback(
        Arc<
            dyn Fn(
                    Arc<dyn FruityAny>,
                    Vec<Serialized>,
                ) -> Result<Option<Serialized>, IntrospectError>
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
    NativeObject(Arc<dyn IntrospectObject>),
}

macro_rules! impl_numeric_from_serialized {
    ( $type:ident ) => {
        impl TryFrom<Serialized> for $type {
            type Error = String;

            fn try_from(value: Serialized) -> Result<Self, Self::Error> {
                match value {
                    Serialized::I8(value) => Ok(value as $type),
                    Serialized::I16(value) => Ok(value as $type),
                    Serialized::I32(value) => Ok(value as $type),
                    Serialized::I64(value) => Ok(value as $type),
                    Serialized::ISize(value) => Ok(value as $type),
                    Serialized::U8(value) => Ok(value as $type),
                    Serialized::U16(value) => Ok(value as $type),
                    Serialized::U32(value) => Ok(value as $type),
                    Serialized::U64(value) => Ok(value as $type),
                    Serialized::USize(value) => Ok(value as $type),
                    Serialized::F32(value) => Ok(value as $type),
                    Serialized::F64(value) => Ok(value as $type),
                    _ => Err(format!("Couldn't convert {:?} to {}", value, "$type")),
                }
            }
        }
    };
}

impl_numeric_from_serialized!(i8);
impl_numeric_from_serialized!(i16);
impl_numeric_from_serialized!(i32);
impl_numeric_from_serialized!(i64);
impl_numeric_from_serialized!(isize);
impl_numeric_from_serialized!(u8);
impl_numeric_from_serialized!(u16);
impl_numeric_from_serialized!(u32);
impl_numeric_from_serialized!(u64);
impl_numeric_from_serialized!(usize);
impl_numeric_from_serialized!(f32);
impl_numeric_from_serialized!(f64);

impl TryFrom<Serialized> for bool {
    type Error = String;

    fn try_from(value: Serialized) -> Result<Self, Self::Error> {
        match value {
            Serialized::Bool(value) => Ok(value),
            _ => Err(format!("Couldn't convert {:?} to bool", value)),
        }
    }
}

impl TryFrom<Serialized> for String {
    type Error = String;

    fn try_from(value: Serialized) -> Result<Self, Self::Error> {
        match value {
            Serialized::String(value) => Ok(value),
            _ => Err(format!("Couldn't convert {:?} to bool", value)),
        }
    }
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

impl TryFrom<Serialized> for Arc<dyn IntrospectObject> {
    type Error = String;

    fn try_from(value: Serialized) -> Result<Self, Self::Error> {
        match value {
            Serialized::NativeObject(value) => Ok(value),
            _ => Err(format!("Couldn't convert {:?} to native object", value)),
        }
    }
}

impl<T: IntrospectObject> TryFrom<Serialized> for Option<Arc<T>> {
    type Error = String;

    fn try_from(value: Serialized) -> Result<Self, Self::Error> {
        let introspect_object = Arc::<dyn IntrospectObject>::try_from(value.clone())?;

        match introspect_object.as_any_arc().downcast::<T>() {
            Ok(value) => Ok(Some(value)),
            Err(_) => Err(format!("Couldn't convert {:?} to native object", value)),
        }
    }
}

impl<T: IntrospectObject + ?Sized> TryFrom<Serialized> for Arc<Box<T>> {
    type Error = String;

    fn try_from(value: Serialized) -> Result<Self, Self::Error> {
        let introspect_object = Arc::<dyn IntrospectObject>::try_from(value.clone())?;

        match introspect_object.as_any_arc().downcast::<Box<T>>() {
            Ok(value) => Ok(value),
            Err(_) => Err(format!("Couldn't convert {:?} to native object", value)),
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
