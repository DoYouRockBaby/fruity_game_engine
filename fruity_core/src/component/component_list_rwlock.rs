use crate::component::component_list_guard::ComponentListReadGuard;
use crate::component::component_list_guard::ComponentListWriteGuard;
use crate::component::component_rwlock::ComponentRwLock;
use crate::entity::archetype::rwlock::EntitySharedRwLock;
use crate::service::utils::cast_service;
use crate::service::utils::ArgumentCaster;
use fruity_any::*;
use fruity_introspect::serializable_object::SerializableObject;
use fruity_introspect::serialized::Serialized;
use fruity_introspect::FieldInfo;
use fruity_introspect::IntrospectObject;
use fruity_introspect::MethodCaller;
use fruity_introspect::MethodInfo;
use std::sync::Arc;

/// A read write locker for a component list instance
#[derive(Debug, Clone, FruityAny)]
pub struct ComponentListRwLock {
    entity: EntitySharedRwLock,
    component_indexes: Vec<usize>,
}

impl ComponentListRwLock {
    /// Returns an RwLockReadGuard which is unlocked.
    ///
    /// # Arguments
    /// * `inner_guard` - The typed [`RwLockReadGuard`]
    ///
    pub fn new(entity: EntitySharedRwLock, component_indexes: Vec<usize>) -> ComponentListRwLock {
        ComponentListRwLock {
            entity,
            component_indexes,
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
    pub fn read(&self) -> ComponentListReadGuard {
        ComponentListReadGuard::new(self.entity.read(), self.component_indexes.clone())
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
    pub fn write(&self) -> ComponentListWriteGuard {
        ComponentListWriteGuard::new(self.entity.write(), self.component_indexes.clone())
    }

    /// Returns a RwLock for a specific component
    ///
    /// # Arguments
    /// * `index` - The index of the component in this list
    ///
    pub fn get(&self, index: usize) -> Option<ComponentRwLock> {
        self.component_indexes
            .get(index)
            .map(|index| ComponentRwLock::new(self.entity.clone(), *index))
    }

    /// Returns the component count
    pub fn len(&self) -> usize {
        self.component_indexes.len()
    }
}

impl IntrospectObject for ComponentListRwLock {
    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![MethodInfo {
            name: "get".to_string(),
            call: MethodCaller::Const(Arc::new(move |this, args| {
                let this = cast_service::<ComponentListRwLock>(this);

                let mut caster = ArgumentCaster::new("get", args);
                let arg1 = caster.cast_next::<usize>()?;

                Ok(this
                    .get(arg1)
                    .map(|result| Serialized::NativeObject(Box::new(result))))
            })),
        }]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}

impl SerializableObject for ComponentListRwLock {
    fn duplicate(&self) -> Box<dyn SerializableObject> {
        Box::new(self.clone())
    }
}
