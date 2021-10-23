use crate::component::component_guard::ComponentReadGuard;
use crate::component::component_guard::ComponentWriteGuard;
use crate::entity::archetype::rwlock::EntityRwLock;
use fruity_introspect::FieldInfo;
use fruity_introspect::IntrospectObject;
use fruity_introspect::MethodInfo;
use fruity_introspect::SetterCaller;
use std::any::Any;
use std::sync::Arc;

/// A read write locker for a component instance
#[derive(Debug)]
pub struct ComponentRwLock<'s> {
    entity: &'s EntityRwLock,
    component_index: usize,
}

impl<'s> ComponentRwLock<'s> {
    /// Returns an RwLockReadGuard which is unlocked.
    ///
    /// # Arguments
    /// * `inner_guard` - The typed [`RwLockReadGuard`]
    ///
    pub fn new(entity: &EntityRwLock, component_index: usize) -> ComponentRwLock {
        ComponentRwLock {
            entity,
            component_index,
        }
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
        ComponentReadGuard::new(self.entity.read(), self.component_index)
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
        ComponentWriteGuard::new(self.entity.write(), self.component_index)
    }
}

impl fruity_any::FruityAny for ComponentRwLock<'static> {
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

impl IntrospectObject for ComponentRwLock<'static> {
    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        let component = self.read();
        component
            .get_field_infos()
            .into_iter()
            .map(|field_info| {
                let getter = field_info.getter.clone();
                let setter = field_info.setter.clone();

                FieldInfo {
                    name: field_info.name,
                    getter: Arc::new(move |this| {
                        let this = unsafe { &*(this as *const _) } as &dyn Any;
                        let this = this.downcast_ref::<ComponentRwLock>().unwrap();
                        let reader = this.read();

                        getter(reader.as_any_ref())
                    }),
                    setter: match setter {
                        SetterCaller::Const(call) => {
                            SetterCaller::Const(Arc::new(move |this, args| {
                                let this = unsafe { &*(this as *const _) } as &dyn Any;
                                let this = this.downcast_ref::<ComponentRwLock>().unwrap();
                                let reader = this.read();

                                call(reader.as_any_ref(), args)
                            }))
                        }
                        SetterCaller::Mut(call) => {
                            SetterCaller::Const(Arc::new(move |this, args| {
                                let this = unsafe { &*(this as *const _) } as &dyn Any;
                                let this = this.downcast_ref::<ComponentRwLock>().unwrap();
                                let mut writer = this.write();

                                call(writer.as_any_mut(), args)
                            }))
                        }
                    },
                }
            })
            .collect::<Vec<_>>()
    }
}
