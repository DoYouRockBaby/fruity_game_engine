use crate::convert::FruityInto;
use crate::convert::FruityTryFrom;
use crate::introspect::Constructor;
use crate::introspect::FieldInfo;
use crate::introspect::InstantiableObject;
use crate::introspect::IntrospectObject;
use crate::introspect::MethodCaller;
use crate::introspect::MethodInfo;
use crate::introspect::SetterCaller;
use crate::resource::resource::Resource;
use crate::serialize::serialized::SerializableObject;
use crate::serialize::serialized::Serialized;
use crate::utils::introspect::ArgumentCaster;
use crate::ResourceContainer;
use crate::RwLock;
use crate::RwLockReadGuard;
use crate::RwLockWriteGuard;
use fruity_any::FruityAny;
use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::Arc;

/// A reference over an any resource that is supposed to be used by components
#[derive(Debug, Clone, FruityAny)]
pub struct AnyResourceReference {
    /// The name of the resource
    pub name: String,

    /// The resource reference
    pub resource: Arc<dyn Resource>,

    /// The resource container reference
    pub resource_container: Arc<ResourceContainer>,
}

impl AnyResourceReference {
    /// Create a resource reference from a resource
    pub fn new(
        name: &str,
        resource: Arc<dyn Resource>,
        resource_container: Arc<ResourceContainer>,
    ) -> Self {
        AnyResourceReference {
            name: name.to_string(),
            resource,
            resource_container,
        }
    }

    /// Get the name of the referenced resource
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    /// Get the name of the referenced resource
    pub fn downcast<T: Resource + ?Sized>(&self) -> Option<ResourceReference<T>> {
        self.resource
            .clone()
            .as_any_arc()
            .downcast::<RwLock<Box<T>>>()
            .ok()
            .map(|resource| {
                ResourceReference::new(&self.name, resource, self.resource_container.clone())
            })
    }
}

impl InstantiableObject for AnyResourceReference {
    fn get_constructor() -> Constructor {
        Arc::new(
            |resource_container: Arc<ResourceContainer>, args: Vec<Serialized>| {
                let mut caster = ArgumentCaster::new("ResourceReference", args);
                let arg1 = caster.next()?;

                if let Serialized::SerializedObject { fields, .. } = arg1 {
                    if let Some(Serialized::String(resource_name)) = fields.get("resource_name") {
                        if let Some(resource) = resource_container.get_untyped(resource_name) {
                            Ok(Serialized::NativeObject(Box::new(resource)))
                        } else {
                            Ok(Serialized::Null)
                        }
                    } else {
                        Ok(Serialized::Null)
                    }
                } else {
                    Ok(Serialized::Null)
                }
            },
        )
    }
}

impl IntrospectObject for AnyResourceReference {
    fn get_class_name(&self) -> String {
        "ResourceReference".to_string()
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        let mut fields_infos = self
            .resource
            .get_field_infos()
            .into_iter()
            .map(|field_info| FieldInfo {
                name: field_info.name,
                serializable: false,
                getter: Arc::new(move |this| {
                    let this = this.downcast_ref::<AnyResourceReference>().unwrap();

                    (field_info.getter)(this.resource.as_any_ref())
                }),
                setter: match field_info.setter {
                    SetterCaller::Const(call) => {
                        SetterCaller::Const(Arc::new(move |this, args| {
                            let this = this.downcast_ref::<AnyResourceReference>().unwrap();

                            call(this.resource.as_any_ref(), args)
                        }))
                    }
                    SetterCaller::Mut(call) => SetterCaller::Mut(Arc::new(move |this, args| {
                        let this = this.downcast_mut::<AnyResourceReference>().unwrap();

                        call(this.resource.as_any_mut(), args)
                    })),
                    SetterCaller::None => SetterCaller::None,
                },
            })
            .collect::<Vec<_>>();

        fields_infos.append(&mut vec![FieldInfo {
            name: "resource_name".to_string(),
            serializable: true,
            getter: Arc::new(move |this| {
                let this = this.downcast_ref::<AnyResourceReference>().unwrap();
                Serialized::String(this.name.clone())
            }),
            setter: SetterCaller::Mut(Arc::new(move |this, value| {
                let this = this.downcast_mut::<AnyResourceReference>().unwrap();

                match String::fruity_try_from(value) {
                    Ok(value) => this.name = value,
                    Err(_) => {
                        log::error!("Expected a String for property resource_name",);
                    }
                }
            })),
        }]);

        fields_infos
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        self.resource
            .get_method_infos()
            .into_iter()
            .map(|method_info| MethodInfo {
                name: method_info.name,
                call: match method_info.call {
                    MethodCaller::Const(call) => {
                        MethodCaller::Const(Arc::new(move |this, args| {
                            let this = this.downcast_ref::<AnyResourceReference>().unwrap();

                            call(this.resource.as_any_ref(), args)
                        }))
                    }
                    MethodCaller::Mut(call) => MethodCaller::Mut(Arc::new(move |this, args| {
                        let this = this.downcast_mut::<AnyResourceReference>().unwrap();

                        call(this.resource.as_any_mut(), args)
                    })),
                },
            })
            .collect::<Vec<_>>()
    }
}

impl SerializableObject for AnyResourceReference {
    fn duplicate(&self) -> Box<dyn SerializableObject> {
        Box::new(self.clone())
    }
}

impl FruityInto<Serialized> for AnyResourceReference {
    fn fruity_into(self) -> Serialized {
        Serialized::NativeObject(Box::new(self))
    }
}

/// A reference over a resource that is supposed to be used by components
#[derive(Debug, FruityAny)]
pub struct ResourceReference<T: Resource + ?Sized> {
    /// The name of the resource
    pub name: String,

    /// The resource reference
    pub resource: Arc<RwLock<Box<T>>>,

    /// The resource container reference
    pub resource_container: Arc<ResourceContainer>,
}

impl<T: Resource + ?Sized> ResourceReference<T> {
    /// Create a resource reference from a resource
    pub fn new(
        name: &str,
        resource: Arc<RwLock<Box<T>>>,
        resource_container: Arc<ResourceContainer>,
    ) -> Self {
        ResourceReference {
            name: name.to_string(),
            resource,
            resource_container,
        }
    }

    /// Get the name of the referenced resource
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    /// Create a read guard over the resource
    pub fn read(&self) -> ResourceReadGuard<T> {
        let inner_guard = self.resource.read();

        // Safe cause the resource guard contains an arc to the referenced resource so it will
        // not be released until the guard is released
        let inner_guard = unsafe {
            std::mem::transmute::<RwLockReadGuard<Box<T>>, RwLockReadGuard<'static, Box<T>>>(
                inner_guard,
            )
        };

        ResourceReadGuard::<T> {
            _referenced: self.resource.clone(),
            inner_guard,
        }
    }

    /// Create a write guard over the resource
    pub fn write(&self) -> ResourceWriteGuard<T> {
        let inner_guard = self.resource.write();

        // Safe cause the resource guard contains an arc to the referenced resource so it will
        // not be released until the guard is released
        let inner_guard = unsafe {
            std::mem::transmute::<RwLockWriteGuard<Box<T>>, RwLockWriteGuard<'static, Box<T>>>(
                inner_guard,
            )
        };

        ResourceWriteGuard::<T> {
            _referenced: self.resource.clone(),
            inner_guard,
        }
    }
}

impl<T: Resource + ?Sized> Clone for ResourceReference<T> {
    fn clone(&self) -> Self {
        ResourceReference::<T>::new(
            &self.name,
            self.resource.clone(),
            self.resource_container.clone(),
        )
    }
}

impl<T: Resource + ?Sized> IntrospectObject for ResourceReference<T> {
    fn get_class_name(&self) -> String {
        "ResourceReference".to_string()
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        let mut fields_infos = self
            .resource
            .get_field_infos()
            .into_iter()
            .map(|field_info| FieldInfo {
                name: field_info.name,
                serializable: false,
                getter: Arc::new(move |this| {
                    let this = this.downcast_ref::<ResourceReference<T>>().unwrap();

                    (field_info.getter)(this.resource.as_any_ref())
                }),
                setter: match field_info.setter {
                    SetterCaller::Const(call) => {
                        SetterCaller::Const(Arc::new(move |this, args| {
                            let this = this.downcast_ref::<ResourceReference<T>>().unwrap();

                            call(this.resource.as_any_ref(), args)
                        }))
                    }
                    SetterCaller::Mut(call) => SetterCaller::Mut(Arc::new(move |this, args| {
                        let this = this.downcast_mut::<ResourceReference<T>>().unwrap();

                        call(this.resource.as_any_mut(), args)
                    })),
                    SetterCaller::None => SetterCaller::None,
                },
            })
            .collect::<Vec<_>>();

        fields_infos.append(&mut vec![FieldInfo {
            name: "resource_name".to_string(),
            serializable: true,
            getter: Arc::new(move |this| {
                let this = this.downcast_ref::<ResourceReference<T>>().unwrap();
                Serialized::String(this.name.clone())
            }),
            setter: SetterCaller::Mut(Arc::new(move |this, value| {
                let this = this.downcast_mut::<ResourceReference<T>>().unwrap();

                match String::fruity_try_from(value) {
                    Ok(value) => this.name = value,
                    Err(_) => {
                        log::error!("Expected a String for property resource_name",);
                    }
                }
            })),
        }]);

        fields_infos
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        self.resource
            .get_method_infos()
            .into_iter()
            .map(|method_info| MethodInfo {
                name: method_info.name,
                call: match method_info.call {
                    MethodCaller::Const(call) => {
                        MethodCaller::Const(Arc::new(move |this, args| {
                            let this = this.downcast_ref::<ResourceReference<T>>().unwrap();

                            call(this.resource.as_any_ref(), args)
                        }))
                    }
                    MethodCaller::Mut(call) => MethodCaller::Mut(Arc::new(move |this, args| {
                        let this = this.downcast_mut::<ResourceReference<T>>().unwrap();

                        call(this.resource.as_any_mut(), args)
                    })),
                },
            })
            .collect::<Vec<_>>()
    }
}

impl<T: Resource + ?Sized> SerializableObject for ResourceReference<T> {
    fn duplicate(&self) -> Box<dyn SerializableObject> {
        Box::new(self.clone())
    }
}

impl<T: Resource + ?Sized> FruityTryFrom<Serialized> for ResourceReference<T> {
    type Error = String;

    fn fruity_try_from(value: Serialized) -> Result<Self, Self::Error> {
        match value {
            Serialized::NativeObject(value) => {
                if let Ok(value) = value
                    .clone()
                    .as_any_box()
                    .downcast::<ResourceReference<T>>()
                {
                    Ok(*value)
                } else if let Ok(resource_reference) = value
                    .clone()
                    .as_any_box()
                    .downcast::<AnyResourceReference>()
                {
                    if let Ok(resource) = resource_reference
                        .resource
                        .as_any_arc()
                        .downcast::<RwLock<Box<T>>>()
                    {
                        Ok(ResourceReference::new(
                            &resource_reference.name,
                            resource,
                            resource_reference.resource_container.clone(),
                        ))
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

impl<T: Resource + ?Sized> FruityInto<Serialized> for ResourceReference<T> {
    fn fruity_into(self) -> Serialized {
        Serialized::NativeObject(Box::new(self))
    }
}

/// A read guard for a resource reference
pub struct ResourceReadGuard<T: Resource + ?Sized> {
    _referenced: Arc<RwLock<Box<T>>>,
    inner_guard: RwLockReadGuard<'static, Box<T>>,
}

impl<'a, T: Resource + ?Sized> ResourceReadGuard<T> {
    /// Downcast to the original sized type that implement the resource trait
    pub fn downcast_ref<U: Resource>(&self) -> &U {
        self.deref().as_any_ref().downcast_ref::<U>().unwrap()
    }
}

impl<'a, T: Resource + ?Sized> Deref for ResourceReadGuard<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner_guard.deref()
    }
}

/// A write guard for a resource reference
pub struct ResourceWriteGuard<T: Resource + ?Sized> {
    _referenced: Arc<RwLock<Box<T>>>,
    inner_guard: RwLockWriteGuard<'static, Box<T>>,
}

impl<T: Resource + ?Sized> ResourceWriteGuard<T> {
    /// Downcast to the original sized type that implement the resource trait
    pub fn downcast_ref<U: Resource>(&self) -> &U {
        self.deref().as_any_ref().downcast_ref::<U>().unwrap()
    }
}

impl<T: Resource + ?Sized> ResourceWriteGuard<T> {
    /// Downcast to the original sized type that implement the resource trait
    pub fn downcast_mut<U: Resource>(&mut self) -> &mut U {
        self.deref_mut().as_any_mut().downcast_mut::<U>().unwrap()
    }
}

impl<T: Resource + ?Sized> Deref for ResourceWriteGuard<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner_guard.deref()
    }
}

impl<T: Resource + ?Sized> DerefMut for ResourceWriteGuard<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner_guard.deref_mut()
    }
}
