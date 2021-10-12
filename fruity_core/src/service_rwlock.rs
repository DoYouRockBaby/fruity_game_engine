use crate::service::Service;
use crate::service_guard::ServiceReadGuard;
use crate::service_guard::ServiceWriteGuard;
use std::marker::PhantomData;
use std::sync::Arc;
use std::sync::PoisonError;
use std::sync::RwLock;
use std::sync::RwLockReadGuard;
use std::sync::RwLockWriteGuard;

/// A read write locker for an service instance
#[derive(Debug)]
pub struct ServiceRwLock<T: Service> {
    service: Arc<RwLock<Box<dyn Service>>>,
    _phantom: PhantomData<T>,
}

impl<T: Service> ServiceRwLock<T> {
    /// Returns an RwLockReadGuard which is unlocked.
    ///
    /// # Arguments
    /// * `inner_guard` - The typed [`RwLockReadGuard`]
    ///
    pub fn new(service: Arc<RwLock<Box<dyn Service>>>) -> ServiceRwLock<T> {
        ServiceRwLock {
            service: service,
            _phantom: PhantomData,
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
    //pub fn write(&self) -> Result<ServiceWriteGuard<'s>, PoisonError<RwLockWriteGuard<()>>> {
    pub fn read(
        &self,
    ) -> Result<ServiceReadGuard<T>, PoisonError<RwLockReadGuard<Box<dyn Service>>>> {
        let guard = self.service.read()?;
        Ok(ServiceReadGuard::new(guard))
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
    pub fn write(
        &self,
    ) -> Result<ServiceWriteGuard<T>, PoisonError<RwLockWriteGuard<Box<dyn Service>>>> {
        let guard = self.service.write()?;
        Ok(ServiceWriteGuard::new(guard))
    }
}
