use crate::component::component_list_guard::ComponentListReadGuard;
use crate::component::component_list_guard::ComponentListWriteGuard;
use crate::component::component_rwlock::ComponentRwLock;
use crate::entity::entity::Entity;
use fruity_any_derive::*;
use std::sync::Arc;
use std::sync::PoisonError;
use std::sync::RwLock;
use std::sync::RwLockReadGuard;
use std::sync::RwLockWriteGuard;

/// A read write locker for a component list instance
#[derive(Debug, Clone, FruityAny)]
pub struct ComponentListRwLock {
    entity: Arc<RwLock<Entity>>,
    component_indexes: Vec<usize>,
}

impl ComponentListRwLock {
    /// Returns an RwLockReadGuard which is unlocked.
    ///
    /// # Arguments
    /// * `inner_guard` - The typed [`RwLockReadGuard`]
    ///
    pub fn new(entity: Arc<RwLock<Entity>>, component_indexes: Vec<usize>) -> ComponentListRwLock {
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
    pub fn read(&self) -> Result<ComponentListReadGuard, PoisonError<RwLockReadGuard<Entity>>> {
        let guard = self.entity.read()?;

        Ok(ComponentListReadGuard::new(
            Arc::new(RwLock::new(guard)),
            self.component_indexes.clone(),
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
    pub fn write(&self) -> Result<ComponentListWriteGuard, PoisonError<RwLockWriteGuard<Entity>>> {
        let guard = self.entity.write()?;

        Ok(ComponentListWriteGuard::new(
            Arc::new(RwLock::new(guard)),
            self.component_indexes.clone(),
        ))
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
