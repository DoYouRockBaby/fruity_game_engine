use crate::component::component::Component;
use crate::component::component_guard::ComponentReadGuard;
use crate::component::component_guard::ComponentWriteGuard;
use std::fmt::Debug;
use std::sync::RwLock;

/// A reader-writer lock
///
/// This type of lock allows a number of readers or at most one writer at any
/// point in time. The write portion of this lock typically allows modification
/// of the underlying data (exclusive access) and the read portion of this lock
/// typically allows for read-only access (shared access).
///
/// <details><summary>Potential deadlock example</summary>
///
/// ```text
/// // Thread 1             |  // Thread 2
/// let _rg = lock.read();  |
///                         |  // will block
///                         |  let _wg = lock.write();
/// // may deadlock         |
/// let _rg = lock.read();  |
/// ```
/// </details>
///
/// # Poisoning
///
/// An `RwLock`, like [`Mutex`], will become poisoned on a panic. Note, however,
/// that an `RwLock` may only be poisoned if a panic occurs while it is locked
/// exclusively (write mode). If a panic occurs in any reader, then the lock
/// will not be poisoned.
///
pub struct ComponentRwLock<'s> {
    rwlock: &'s RwLock<Component<'s>>,
}

impl<'s> ComponentRwLock<'s> {
    /// Returns an ComponentRwLock which is unlocked.
    ///
    /// # Arguments
    /// * `rwlock` - The typed [`RwLock`]
    ///
    pub fn new(rwlock: &'s RwLock<Component>) -> ComponentRwLock<'s> {
        ComponentRwLock { rwlock }
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
    pub fn read(&self) -> ComponentReadGuard<'_> {
        ComponentReadGuard::new(self.rwlock.read().unwrap())
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
    pub fn write(&self) -> ComponentWriteGuard<'_> {
        ComponentWriteGuard::new(self.rwlock.write().unwrap())
    }
}

impl<'s> Debug for ComponentRwLock<'s> {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        self.rwlock.fmt(formatter)
    }
}
