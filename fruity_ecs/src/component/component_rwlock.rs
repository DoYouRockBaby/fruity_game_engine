use crate::component::component_guard::ComponentReadGuard;
use crate::component::component_guard::ComponentWriteGuard;
use crate::entity::entity::Entity;
use fruity_any::*;
use std::sync::Arc;
use std::sync::PoisonError;
use std::sync::RwLock;
use std::sync::RwLockReadGuard;
use std::sync::RwLockWriteGuard;

/// A read write locker for a component instance
#[derive(Debug, Clone, FruityAny)]
pub struct ComponentRwLock {
    entity: Arc<RwLock<Entity>>,
    component_index: usize,
}

impl ComponentRwLock {
    /// Returns an RwLockReadGuard which is unlocked.
    ///
    /// # Arguments
    /// * `inner_guard` - The typed [`RwLockReadGuard`]
    ///
    pub fn new(entity: Arc<RwLock<Entity>>, component_index: usize) -> ComponentRwLock {
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
