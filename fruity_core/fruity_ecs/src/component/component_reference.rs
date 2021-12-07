use crate::component::component::Component;
use crate::component::component_guard::ComponentReadGuard;
use crate::component::component_guard::ComponentWriteGuard;
use fruity_any::*;
use fruity_core::introspect::FieldInfo;
use fruity_core::introspect::IntrospectObject;
use fruity_core::introspect::MethodInfo;
use fruity_core::introspect::SetterCaller;
use fruity_core::serialize::serialized::SerializableObject;
use fruity_core::serialize::serialized::Serialized;
use fruity_core::serialize::Serialize;
use std::sync::Arc;
use std::sync::RwLock;

/// A reference over a component stored into an Archetype
#[derive(Debug, Clone, FruityAny)]
pub struct ComponentReference {
    rwlock: &'static RwLock<()>,
    component: &'static dyn Component,
}

impl ComponentReference {
    /// Returns a [’ComponentReference’]
    ///
    /// # Arguments
    /// * `rwlock` - The rwlock reference from the archetype storage
    /// * `component` - The components references from the archetype storage
    ///
    pub(crate) fn new(rwlock: &RwLock<()>, component: &dyn Component) -> Self {
        // TODO: Find a way to know when the targeted datas are deleted
        let rwlock = unsafe { &*(rwlock as *const _) } as &RwLock<()>;
        let component = unsafe { &*(component as *const _) } as &dyn Component;

        Self { rwlock, component }
    }

    /// Locks this rwlock with shared read access, blocking the current thread
    /// until it can be acquired.
    ///
    /// # Errors
    ///
    /// This function will return an error if the RwLock is poisoned. An RwLock
    /// is poisoned whenever a writer panics while holding an exclusive lock.
    /// The failure will occur immediately after the lock has been acquired.
    ///
    /// # Panics
    ///
    /// This function might panic when called if the lock is already held by the current thread.
    ///
    pub fn read(&self) -> ComponentReadGuard {
        ComponentReadGuard::new(self.rwlock.read().unwrap(), self.component)
    }

    /// Locks this rwlock with exclusive write access, blocking the current
    /// thread until it can be acquired.
    ///
    /// # Errors
    ///
    /// This function will return an error if the RwLock is poisoned. An RwLock
    /// is poisoned whenever a writer panics while holding an exclusive lock.
    /// An error will be returned when the lock is acquired.
    ///
    /// # Panics
    ///
    /// This function might panic when called if the lock is already held by the current thread.
    ///
    pub fn write(&self) -> ComponentWriteGuard {
        ComponentWriteGuard::new(self.rwlock.write().unwrap(), self.component)
    }
}

impl Serialize for ComponentReference {
    fn serialize(&self) -> Option<Serialized> {
        let native_serialized = Serialized::NativeObject(Box::new(self.clone()));
        let serialized = native_serialized.serialize_native_objects();
        Some(serialized)
    }
}

impl IntrospectObject for ComponentReference {
    fn get_class_name(&self) -> String {
        let component = self.read();
        component.get_class_name()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        let component = self.read();
        component
            .get_field_infos()
            .into_iter()
            .map(|field_info| FieldInfo {
                name: field_info.name,
                serializable: field_info.serializable,
                getter: Arc::new(move |this| {
                    let this = this.downcast_ref::<ComponentReference>().unwrap();
                    let reader = this.read();

                    (field_info.getter)(reader.as_any_ref())
                }),
                setter: match field_info.setter {
                    SetterCaller::Const(call) => {
                        SetterCaller::Const(Arc::new(move |this, args| {
                            let this = this.downcast_ref::<ComponentReference>().unwrap();
                            let reader = this.read();

                            call(reader.as_any_ref(), args)
                        }))
                    }
                    SetterCaller::Mut(call) => SetterCaller::Const(Arc::new(move |this, args| {
                        let this = this.downcast_ref::<ComponentReference>().unwrap();
                        let mut writer = this.write();

                        call(writer.as_any_mut(), args)
                    })),
                    SetterCaller::None => SetterCaller::None,
                },
            })
            .collect::<Vec<_>>()
    }
}

impl SerializableObject for ComponentReference {
    fn duplicate(&self) -> Box<dyn SerializableObject> {
        Box::new(self.clone())
    }
}
