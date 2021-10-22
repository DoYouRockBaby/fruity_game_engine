use crate::component::component_guard::ComponentReadGuard;
use crate::component::component_guard::ComponentWriteGuard;
use crate::entity::entity::Entity;
use crate::entity::entity_rwlock::EntityRwLock;
use fruity_any::*;
use fruity_introspect::FieldInfo;
use fruity_introspect::IntrospectObject;
use fruity_introspect::MethodInfo;
use fruity_introspect::SetterCaller;
use std::any::Any;
use std::sync::Arc;
use std::sync::PoisonError;
use std::sync::RwLock;
use std::sync::RwLockReadGuard;
use std::sync::RwLockWriteGuard;

/// A read write locker for a component instance
#[derive(Debug, Clone, FruityAny)]
pub struct ComponentRwLock {
    entity: Arc<EntityRwLock>,
    component_index: usize,
}

impl ComponentRwLock {
    /// Returns an RwLockReadGuard which is unlocked.
    ///
    /// # Arguments
    /// * `inner_guard` - The typed [`RwLockReadGuard`]
    ///
    pub fn new(entity: Arc<EntityRwLock>, component_index: usize) -> ComponentRwLock {
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
    pub fn read(&self) -> Result<ComponentReadGuard, PoisonError<RwLockReadGuard<Entity>>> {
        let guard = self.entity.read()?;

        Ok(ComponentReadGuard::new(
            Arc::new(RwLock::new(guard)),
            self.component_index,
        ))
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
    pub fn write(&self) -> Result<ComponentWriteGuard, PoisonError<RwLockWriteGuard<Entity>>> {
        let guard = self.entity.write()?;

        Ok(ComponentWriteGuard::new(
            Arc::new(RwLock::new(guard)),
            self.component_index,
        ))
    }
}

impl IntrospectObject for ComponentRwLock {
    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        let component = self.read().unwrap();
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
                        let reader = this.read().unwrap();

                        getter(reader.as_any_ref())
                    }),
                    setter: match setter {
                        SetterCaller::Const(call) => {
                            SetterCaller::Const(Arc::new(move |this, args| {
                                let this = unsafe { &*(this as *const _) } as &dyn Any;
                                let this = this.downcast_ref::<ComponentRwLock>().unwrap();
                                let reader = this.read().unwrap();

                                call(reader.as_any_ref(), args)
                            }))
                        }
                        SetterCaller::Mut(call) => {
                            SetterCaller::Const(Arc::new(move |this, args| {
                                let this = unsafe { &*(this as *const _) } as &dyn Any;
                                let this = this.downcast_ref::<ComponentRwLock>().unwrap();
                                let mut writer = this.write().unwrap();

                                call(writer.as_any_mut(), args)
                            }))
                        }
                    },
                }
            })
            .collect::<Vec<_>>()
    }

    fn as_introspect_arc(self: Arc<Self>) -> Arc<dyn IntrospectObject> {
        self
    }
}
