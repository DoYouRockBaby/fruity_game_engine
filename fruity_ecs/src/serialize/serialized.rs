use crate::component::component_list_rwlock::ComponentListRwLock;
use crate::component::component_rwlock::ComponentRwLock;
use crate::entity::entity_rwlock::EntityRwLock;
use crate::service::service::Service;
use crate::ServiceManager;
use fruity_introspect::IntrospectError;
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

    /// Component RwLock
    Component(ComponentRwLock),

    /// Component list RwLock
    ComponentList(ComponentListRwLock),
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
            _ => None,
        }
    };
}

macro_rules! as_floating {
    ( $value:expr, $type:ident ) => {
        match $value {
            Serialized::I8(value) => $type::try_from(value).ok(),
            Serialized::I16(value) => $type::try_from(value).ok(),
            Serialized::U8(value) => $type::try_from(value).ok(),
            Serialized::U16(value) => $type::try_from(value).ok(),
            _ => None,
        }
    };
}

impl Serialized {
    /// Convert as i8
    #[allow(dead_code)]
    pub fn as_i8(&self) -> Option<i8> {
        as_integer!(*self, i8)
    }

    /// Convert as i16
    #[allow(dead_code)]
    pub fn as_i16(&self) -> Option<i16> {
        as_integer!(*self, i16)
    }

    /// Convert as i32
    #[allow(dead_code)]
    pub fn as_i32(&self) -> Option<i32> {
        as_integer!(*self, i32)
    }

    /// Convert as i64
    #[allow(dead_code)]
    pub fn as_i64(&self) -> Option<i64> {
        as_integer!(*self, i64)
    }

    /// Convert as isize
    #[allow(dead_code)]
    pub fn as_isize(&self) -> Option<isize> {
        as_integer!(*self, isize)
    }

    /// Convert as u8
    #[allow(dead_code)]
    pub fn as_u8(&self) -> Option<u8> {
        as_integer!(*self, u8)
    }

    /// Convert as u16
    #[allow(dead_code)]
    pub fn as_u16(&self) -> Option<u16> {
        as_integer!(*self, u16)
    }

    /// Convert as u32
    #[allow(dead_code)]
    pub fn as_u32(&self) -> Option<u32> {
        as_integer!(*self, u32)
    }

    /// Convert as u64
    #[allow(dead_code)]
    pub fn as_u64(&self) -> Option<u64> {
        as_integer!(*self, u64)
    }

    /// Convert as usize
    #[allow(dead_code)]
    pub fn as_usize(&self) -> Option<usize> {
        as_integer!(*self, usize)
    }

    /// Convert as f32
    #[allow(dead_code)]
    pub fn as_f32(&self) -> Option<f32> {
        as_floating!(*self, f32)
    }

    /// Convert as f64
    #[allow(dead_code)]
    pub fn as_f64(&self) -> Option<f64> {
        as_floating!(*self, f64)
    }

    /// Convert as bool
    #[allow(dead_code)]
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Serialized::Bool(value) => Some(*value),
            _ => None,
        }
    }

    /// Convert as String
    #[allow(dead_code)]
    pub fn as_string(&self) -> Option<String> {
        match self {
            Serialized::String(value) => Some(value.clone()),
            _ => None,
        }
    }

    /// Convert as String array
    #[allow(dead_code)]
    pub fn as_string_array(&self) -> Option<Vec<String>> {
        match self {
            Serialized::Array(value) => {
                Some(value.iter().filter_map(|elem| elem.as_string()).collect())
            }
            _ => None,
        }
    }

    /// Convert as a thread shared service
    #[allow(dead_code)]
    pub fn as_service(&self) -> Option<Arc<RwLock<Box<dyn Service>>>> {
        match self {
            Serialized::Service(value) => Some(value.clone()),
            _ => None,
        }
    }

    /// Convert as a callback function
    #[allow(dead_code)]
    pub fn as_callback(
        &self,
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
