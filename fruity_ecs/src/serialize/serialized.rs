use crate::component::component::Component;
use crate::component::component_list_rwlock::ComponentListRwLock;
use crate::component::component_rwlock::ComponentRwLock;
use crate::component::serialized_component::SerializedComponent;
use crate::entity::entity_rwlock::EntityRwLock;
use crate::service::service::Service;
use crate::ServiceManager;
use fruity_introspect::IntrospectError;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt::Debug;
use std::sync::Arc;
use std::sync::RwLock;

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

    /// Array of values
    Object {
        /// The object class name
        class_name: String,

        /// The object fields
        fields: HashMap<String, Serialized>,
    },

    /// Iterator over values
    Iterator(Arc<RwLock<dyn Iterator<Item = Serialized> + Send + Sync>>),

    /// Service reference value
    Callback(
        Arc<
            dyn Fn(
                    Arc<RwLock<ServiceManager>>,
                    Vec<Serialized>,
                ) -> Result<Option<Serialized>, IntrospectError>
                + Sync
                + Send
                + 'static,
        >,
    ),

    /// Service reference value
    Service(Arc<RwLock<Box<dyn Service>>>),

    /// Entity RwLock
    Entity(EntityRwLock),

    /// Component
    Component(Box<dyn Component>),

    /// Component RwLock
    ComponentRwLock(ComponentRwLock),

    /// Component list RwLock
    ComponentListRwLock(ComponentListRwLock),
}

macro_rules! as_integer {
    ( $value:expr, $type:ident ) => {
        match $value {
            Serialized::I8(value) => $type::try_from(value).ok(),
            Serialized::I16(value) => $type::try_from(value).ok(),
            Serialized::I32(value) => $type::try_from(value).ok(),
            Serialized::I64(value) => $type::try_from(value).ok(),
            Serialized::ISize(value) => $type::try_from(value).ok(),
            Serialized::U8(value) => $type::try_from(value).ok(),
            Serialized::U16(value) => $type::try_from(value).ok(),
            Serialized::U32(value) => $type::try_from(value).ok(),
            Serialized::U64(value) => $type::try_from(value).ok(),
            Serialized::USize(value) => $type::try_from(value).ok(),
            Serialized::F32(value) => $type::try_from(value as i64).ok(),
            Serialized::F64(value) => $type::try_from(value as i64).ok(),
            _ => None,
        }
    };
}
macro_rules! as_floating {
    ( $value:expr, $type:ident ) => {
        match $value {
            Serialized::I8(value) => Some(value as $type),
            Serialized::I16(value) => Some(value as $type),
            Serialized::I32(value) => Some(value as $type),
            Serialized::I64(value) => Some(value as $type),
            Serialized::ISize(value) => Some(value as $type),
            Serialized::U8(value) => Some(value as $type),
            Serialized::U16(value) => Some(value as $type),
            Serialized::U32(value) => Some(value as $type),
            Serialized::U64(value) => Some(value as $type),
            Serialized::USize(value) => Some(value as $type),
            Serialized::F32(value) => Some(value as $type),
            Serialized::F64(value) => Some(value as $type),
            _ => None,
        }
    };
}

impl Serialized {
    /// Convert as i8
    pub fn as_i8(self) -> Option<i8> {
        as_integer!(self, i8)
    }

    /// Convert as i16
    pub fn as_i16(self) -> Option<i16> {
        as_integer!(self, i16)
    }

    /// Convert as i32
    pub fn as_i32(self) -> Option<i32> {
        as_integer!(self, i32)
    }

    /// Convert as i64
    pub fn as_i64(self) -> Option<i64> {
        as_integer!(self, i64)
    }

    /// Convert as isize
    pub fn as_isize(self) -> Option<isize> {
        as_integer!(self, isize)
    }

    /// Convert as u8
    pub fn as_u8(self) -> Option<u8> {
        as_integer!(self, u8)
    }

    /// Convert as u16
    pub fn as_u16(self) -> Option<u16> {
        as_integer!(self, u16)
    }

    /// Convert as u32
    pub fn as_u32(self) -> Option<u32> {
        as_integer!(self, u32)
    }

    /// Convert as u64
    pub fn as_u64(self) -> Option<u64> {
        as_integer!(self, u64)
    }

    /// Convert as usize
    pub fn as_usize(self) -> Option<usize> {
        as_integer!(self, usize)
    }

    /// Convert as f32
    pub fn as_f32(self) -> Option<f32> {
        as_floating!(self, f32)
    }

    /// Convert as f64
    pub fn as_f64(self) -> Option<f64> {
        as_floating!(self, f64)
    }

    /// Convert as bool
    pub fn as_bool(self) -> Option<bool> {
        match self {
            Serialized::Bool(value) => Some(value),
            _ => None,
        }
    }

    /// Convert as String
    pub fn as_string(self) -> Option<String> {
        match self {
            Serialized::String(value) => Some(value),
            _ => None,
        }
    }

    /// Convert as Serialized object
    pub fn as_object_fields(self) -> Option<HashMap<String, Serialized>> {
        match self {
            Serialized::Object { fields, .. } => Some(fields),
            _ => None,
        }
    }

    /// Convert as Component
    pub fn as_component(self) -> Option<Box<dyn Component>> {
        match self {
            Serialized::Object { .. } => Some(Box::new(SerializedComponent::new(self))),
            Serialized::Component(component) => Some(component.duplicate()),
            _ => None,
        }
    }

    /// Convert as String array
    pub fn as_string_array(self) -> Option<Vec<String>> {
        match self {
            Serialized::Array(value) => Some(
                value
                    .into_iter()
                    .filter_map(|elem| elem.as_string())
                    .collect(),
            ),
            _ => None,
        }
    }

    /// Convert as component array
    pub fn as_component_array(self) -> Option<Vec<Box<dyn Component>>> {
        match self {
            Serialized::Array(value) => Some(
                value
                    .into_iter()
                    .filter_map(|elem| elem.as_component())
                    .collect(),
            ),
            _ => None,
        }
    }

    /// Convert as a thread shared service
    pub fn as_service(self) -> Option<Arc<RwLock<Box<dyn Service>>>> {
        match self {
            Serialized::Service(value) => Some(value.clone()),
            _ => None,
        }
    }

    /// Convert as a callback function
    pub fn as_callback(
        self,
    ) -> Option<
        Arc<
            dyn Fn(
                    Arc<RwLock<ServiceManager>>,
                    Vec<Serialized>,
                ) -> Result<Option<Serialized>, IntrospectError>
                + Sync
                + Send
                + 'static,
        >,
    > {
        match self {
            Serialized::Callback(value) => Some(value.clone()),
            _ => None,
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
