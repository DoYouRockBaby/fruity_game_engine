use crate::resource::resource::Resource;
use fruity_any::FruityAny;
use fruity_introspect::serializable_object::SerializableObject;
use fruity_introspect::serialize::serialized::Serialized;
use fruity_introspect::FieldInfo;
use fruity_introspect::IntrospectObject;
use fruity_introspect::MethodInfo;
use std::convert::TryFrom;
use std::ops::Deref;
use std::sync::Arc;

/// A reference over a resource that is supposed to be used by components
#[derive(Debug)]
pub struct ResourceReference<T: Resource>(pub Option<Arc<T>>);

impl<T: Resource> Clone for ResourceReference<T> {
    fn clone(&self) -> Self {
        ResourceReference(self.0.clone())
    }
}

impl<T: Resource> ResourceReference<T> {
    /// Create an empty resource reference
    pub fn new() -> Self {
        ResourceReference(None)
    }

    /// Create a resource reference from a resource
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

impl<T: Resource> IntrospectObject for ResourceReference<T> {
    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}

impl<T: Resource> SerializableObject for ResourceReference<T> {
    fn duplicate(&self) -> Box<dyn SerializableObject> {
        Box::new(self.clone())
    }
}

impl<T: Resource> FruityAny for ResourceReference<T> {
    fn as_any_ref(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn as_any_box(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
    }

    fn as_any_arc(self: std::sync::Arc<Self>) -> std::sync::Arc<dyn std::any::Any + Send + Sync> {
        self
    }
}

impl<T: Resource> TryFrom<Serialized> for ResourceReference<T> {
    type Error = String;

    fn try_from(value: Serialized) -> Result<Self, Self::Error> {
        match value {
            Serialized::NativeObject(value) => {
                match value.clone().as_any_box().downcast::<Arc<dyn Resource>>() {
                    Ok(value) => match value.as_any_arc().downcast::<T>() {
                        Ok(value) => Ok(ResourceReference::from_resource(value)),
                        _ => Err(format!("Couldn't convert a Serialized to native object")),
                    },
                    _ => Err(format!("Couldn't convert a Serialized to native object")),
                }
            }
            _ => Err(format!("Couldn't convert {:?} to native object", value)),
        }
    }
}
