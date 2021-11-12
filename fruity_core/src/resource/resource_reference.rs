use crate::resource::resource::Resource;
use fruity_any::FruityAny;
use fruity_introspect::serializable_object::SerializableObject;
use fruity_introspect::serialized::Serialized;
use fruity_introspect::FieldInfo;
use fruity_introspect::IntrospectObject;
use fruity_introspect::MethodInfo;
use std::convert::TryFrom;
use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::Arc;
use std::sync::RwLock;
use std::sync::RwLockReadGuard;
use std::sync::RwLockWriteGuard;

/// A reference over a resource that is supposed to be used by components
#[derive(Debug)]
pub struct ResourceReference<T: Resource + ?Sized>(pub Arc<RwLock<Box<T>>>);

impl<T: Resource + ?Sized> Clone for ResourceReference<T> {
    fn clone(&self) -> Self {
        ResourceReference(self.0.clone())
    }
}

impl<T: Resource + ?Sized> ResourceReference<T> {
    /// Create a resource reference from a resource
    pub fn new(resource: Arc<RwLock<Box<T>>>) -> Self {
        ResourceReference(resource)
    }

    /// Create a read guard over the resource
    pub fn read(&self) -> ResourceReadGuard<T> {
        ResourceReadGuard::<T> {
            inner_guard: self.0.read().unwrap(),
        }
    }

    /// Create a write guard over the resource
    pub fn write(&self) -> ResourceWriteGuard<T> {
        ResourceWriteGuard::<T> {
            inner_guard: self.0.write().unwrap(),
        }
    }
}

// TODO: Complete that
impl<T: Resource + ?Sized> IntrospectObject for ResourceReference<T> {
    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}

impl<T: Resource + ?Sized> SerializableObject for ResourceReference<T> {
    fn duplicate(&self) -> Box<dyn SerializableObject> {
        Box::new(self.clone())
    }
}

// TODO: Improve the macro to handle the generics
impl<T: Resource + ?Sized> FruityAny for ResourceReference<T> {
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

impl<T: Resource + ?Sized> TryFrom<Serialized> for ResourceReference<T> {
    type Error = String;

    fn try_from(value: Serialized) -> Result<Self, Self::Error> {
        match value {
            Serialized::NativeObject(value) => {
                if let Ok(value) = value
                    .clone()
                    .as_any_box()
                    .downcast::<ResourceReference<T>>()
                {
                    Ok(*value)
                } else if let Ok(value) = value.clone().as_any_box().downcast::<Arc<dyn Resource>>()
                {
                    if let Ok(value) = value.as_any_arc().downcast::<RwLock<Box<T>>>() {
                        Ok(ResourceReference::new(value))
                    } else {
                        Err(format!("Couldn't convert a Serialized to native object"))
                    }
                } else {
                    Err(format!("Couldn't convert a Serialized to native object"))
                }
            }
            _ => Err(format!("Couldn't convert {:?} to native object", value)),
        }
    }
}

impl<T: Resource + ?Sized> Into<Serialized> for ResourceReference<T> {
    fn into(self) -> Serialized {
        Serialized::NativeObject(Box::new(self))
    }
}

/// A read guard for a resource reference
pub struct ResourceReadGuard<'a, T: Resource + ?Sized> {
    inner_guard: RwLockReadGuard<'a, Box<T>>,
}

impl<'a, T: Resource + ?Sized> ResourceReadGuard<'a, T> {
    /// Downcast to the original sized type that implement the resource trait
    pub fn downcast_ref<U: Resource>(&self) -> &U {
        self.deref().as_any_ref().downcast_ref::<U>().unwrap()
    }
}

impl<'a, T: Resource + ?Sized> Deref for ResourceReadGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner_guard.deref()
    }
}

/// A write guard for a resource reference
pub struct ResourceWriteGuard<'a, T: Resource + ?Sized> {
    inner_guard: RwLockWriteGuard<'a, Box<T>>,
}

impl<'a, T: Resource + ?Sized> ResourceWriteGuard<'a, T> {
    /// Downcast to the original sized type that implement the resource trait
    pub fn downcast_ref<U: Resource>(&self) -> &U {
        self.deref().as_any_ref().downcast_ref::<U>().unwrap()
    }
}

impl<'a, T: Resource + ?Sized> ResourceWriteGuard<'a, T> {
    /// Downcast to the original sized type that implement the resource trait
    pub fn downcast_mut<U: Resource>(&mut self) -> &mut U {
        self.deref_mut().as_any_mut().downcast_mut::<U>().unwrap()
    }
}

impl<'a, T: Resource + ?Sized> Deref for ResourceWriteGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner_guard.deref()
    }
}

impl<'a, T: Resource + ?Sized> DerefMut for ResourceWriteGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner_guard.deref_mut()
    }
}

/// An optionnal reference over a resource that is supposed to be used by components
#[derive(Debug)]
pub struct OptionResourceReference<T: Resource + ?Sized>(pub Option<ResourceReference<T>>);

impl<T: Resource + ?Sized> Clone for OptionResourceReference<T> {
    fn clone(&self) -> Self {
        OptionResourceReference(self.0.clone())
    }
}

impl<T: Resource + ?Sized> OptionResourceReference<T> {
    /// Create an empty resource reference
    pub fn empty() -> Self {
        OptionResourceReference(None)
    }

    /// Create a resource reference from a resource
    pub fn new(resource: ResourceReference<T>) -> Self {
        OptionResourceReference(Some(resource))
    }
}

impl<T: Resource + ?Sized> Deref for OptionResourceReference<T> {
    type Target = Option<ResourceReference<T>>;

    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        &self.0
    }
}

// TODO: Complete that
impl<T: Resource + ?Sized> IntrospectObject for OptionResourceReference<T> {
    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}

impl<T: Resource + ?Sized> SerializableObject for OptionResourceReference<T> {
    fn duplicate(&self) -> Box<dyn SerializableObject> {
        Box::new(self.clone())
    }
}

// TODO: Improve the macro to handle the generics
impl<T: Resource + ?Sized> FruityAny for OptionResourceReference<T> {
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

impl<T: Resource + ?Sized> TryFrom<Serialized> for OptionResourceReference<T> {
    type Error = String;

    fn try_from(value: Serialized) -> Result<Self, Self::Error> {
        match ResourceReference::<T>::try_from(value) {
            Ok(value) => Ok(OptionResourceReference::new(value)),
            Err(_) => Ok(OptionResourceReference::empty()),
        }
    }
}

impl<T: Resource + ?Sized> Into<Serialized> for OptionResourceReference<T> {
    fn into(self) -> Serialized {
        match self.0 {
            Some(value) => Serialized::NativeObject(Box::new(value)),
            None => Serialized::Null,
        }
    }
}
