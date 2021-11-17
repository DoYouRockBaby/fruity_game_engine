use crate::component::component_list_guard::ComponentListReadGuard;
use crate::component::component_list_guard::ComponentListWriteGuard;
use crate::component::component_rwlock::ComponentRwLock;
use crate::entity::archetype::rwlock::EntitySharedRwLock;
use crate::entity::entity::EntityId;
use fruity_any::*;
use fruity_core::introspect::FieldInfo;
use fruity_core::introspect::IntrospectObject;
use fruity_core::introspect::MethodCaller;
use fruity_core::introspect::MethodInfo;
use fruity_core::serialize::serialized::SerializableObject;
use fruity_core::serialize::serialized::Serialized;
use fruity_core::utils::introspect::cast_introspect_ref;
use fruity_core::utils::introspect::ArgumentCaster;
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

    /// Returns the associated entity id
    pub fn entity_id(&self) -> EntityId {
        let entity = self.entity.read();
        entity.entity_id
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
    fn get_class_name(&self) -> String {
        "ComponentListRwLock".to_string()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![
            MethodInfo {
                name: "get".to_string(),
                call: MethodCaller::Const(Arc::new(move |this, args| {
                    let this = cast_introspect_ref::<ComponentListRwLock>(this);

                    let mut caster = ArgumentCaster::new("get", args);
                    let arg1 = caster.cast_next::<usize>()?;

                    Ok(this
                        .get(arg1)
                        .map(|result| Serialized::NativeObject(Box::new(result))))
                })),
            },
            MethodInfo {
                name: "entity_id".to_string(),
                call: MethodCaller::Const(Arc::new(move |this, _args| {
                    let this = cast_introspect_ref::<ComponentListRwLock>(this);
                    Ok(Some(this.entity_id().into()))
                })),
            },
        ]
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
