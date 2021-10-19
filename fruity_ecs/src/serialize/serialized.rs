use crate::component::component::Component;
use crate::component::component_list_rwlock::ComponentListRwLock;
use crate::component::component_rwlock::ComponentRwLock;
use crate::component::serialized_component::SerializedComponent;
use crate::entity::entity_rwlock::EntityRwLock;
use crate::resource::resource::Resource;
use crate::resource::resources_manager::ResourceLoaderParams;
use crate::service::service::Service;
use crate::ServiceManager;
use fruity_introspect::IntrospectError;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt::Debug;
use std::ops::Deref;
use std::sync::Arc;
use std::sync::RwLock;

/// A callback for the serialized type
pub type Callback = Arc<
    dyn Fn(
            Arc<RwLock<ServiceManager>>,
            Vec<Serialized>,
        ) -> Result<Option<Serialized>, IntrospectError>
        + Sync
        + Send
        + 'static,
>;

/// A list of serialized object fields
pub type ObjectFields = HashMap<String, Serialized>;

/// A reference over a service stored as any
pub type AnyServiceReference = Arc<RwLock<Box<dyn Service>>>;

/// A reference over a service
pub type ServiceReference<T> = Arc<RwLock<Box<T>>>;

/// A reference over a resource
#[derive(Debug)]
pub struct ResourceReference<T: Resource>(Option<Arc<T>>);

impl<T: Resource> Clone for ResourceReference<T> {
    fn clone(&self) -> Self {
        ResourceReference(self.0.clone())
    }
}

impl<T: Resource> ResourceReference<T> {
    pub fn new() -> Self {
        ResourceReference(None)
    }

    pub fn from_resource(resource: Arc<T>) -> Self {
        ResourceReference(Some(resource))
    }
}

impl<T: Resource> Deref for ResourceReference<T> {
    type Target = Option<Arc<T>>;

    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        &self.0
    }
}

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

    /// Resource
    Resource(Arc<dyn Resource>),
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

impl TryFrom<Serialized> for ObjectFields {
    type Error = String;

    fn try_from(value: Serialized) -> Result<Self, Self::Error> {
        match value {
            Serialized::Object { fields, .. } => Ok(fields),
            _ => Err(format!("Couldn't convert {:?} to field hashmap", value)),
        }
    }
}

impl TryFrom<Serialized> for Box<dyn Component> {
    type Error = String;

    fn try_from(value: Serialized) -> Result<Self, Self::Error> {
        match value {
            Serialized::Object { .. } => Ok(Box::new(SerializedComponent::new(value))),
            Serialized::Component(component) => Ok(component.duplicate()),
            _ => Err(format!("Couldn't convert {:?} to field hashmap", value)),
        }
    }
}

impl<T: TryFrom<Serialized>> TryFrom<Serialized> for Vec<T> {
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

impl TryFrom<Serialized> for AnyServiceReference {
    type Error = String;

    fn try_from(value: Serialized) -> Result<Self, Self::Error> {
        match value {
            Serialized::Service(value) => Ok(value.clone()),
            _ => Err(format!("Couldn't convert {:?} to service", value)),
        }
    }
}

// TODO
/*impl<T: Service> TryFrom<Serialized> for ServiceReference<T> {
    type Error = String;

    fn try_from(value: Serialized) -> Result<Self, Self::Error> {
        let service = AnyServiceReference::try_from(value)?;

        service
            .as_any_arc_send_sync()
            .downcast::<RwLock<Box<T>>>()
            .map_err(|_| format!("Couldn't convert {:?} to service", value))
    }
}*/

impl TryFrom<Serialized> for Arc<dyn Resource> {
    type Error = String;

    fn try_from(value: Serialized) -> Result<Self, Self::Error> {
        match value {
            Serialized::Resource(resource) => Ok(resource),
            _ => Err(format!("Couldn't convert {:?} to resource", value)),
        }
    }
}

impl<T: Resource> TryFrom<Serialized> for ResourceReference<T> {
    type Error = String;

    fn try_from(value: Serialized) -> Result<Self, Self::Error> {
        match value.clone() {
            Serialized::Resource(resource) => {
                match resource.as_any_arc_send_sync().downcast::<T>() {
                    Ok(value) => Ok(ResourceReference(Some(value))),
                    Err(_) => Err(format!("Couldn't convert {:?} to resource", value)),
                }
            }
            _ => Err(format!("Couldn't convert {:?} to resource", value)),
        }
    }
}

impl TryFrom<Serialized> for ResourceLoaderParams {
    type Error = String;

    fn try_from(value: Serialized) -> Result<Self, Self::Error> {
        match value {
            Serialized::Object { fields, .. } => Ok(ResourceLoaderParams(fields)),
            _ => Err(format!("Couldn't convert {:?} to callback", value)),
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

impl Debug for Serialized {
    fn fmt(
        &self,
        _formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}
