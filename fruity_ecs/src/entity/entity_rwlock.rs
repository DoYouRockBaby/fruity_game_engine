use std::fmt::Debug;
use std::sync::RwLock;
use crate::entity::entity::Entity;
use crate::entity::entity_guard::EntityReadGuard;
use crate::entity::entity_guard::EntityWriteGuard;

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
pub struct EntityRwLock<'s> {
    inner_lock: Box<dyn InnerEntityRwLock + 's>,
}

impl<'s> EntityRwLock<'s> {
    /// Returns an EntityRwLock which is unlocked.
    ///
    /// # Arguments
    /// * `rwlock` - The typed [`RwLock`]
    ///
    pub fn new<T: Entity>(rwlock: &'s RwLock<T>) -> EntityRwLock<'s> {
        EntityRwLock {
            inner_lock: Box::new(InnerRawEntityRwLock::<T> {
                rwlock,
            }),
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
    pub fn read(&self) -> EntityReadGuard<'_> {
        self.inner_lock.read()
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
    pub fn write(&self) -> EntityWriteGuard<'_> {
        self.inner_lock.write()
    }
}

impl<'s> Debug for EntityRwLock<'s> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        self.inner_lock.fmt(formatter)
    }
}

trait InnerEntityRwLock: Debug + Send + Sync {
    fn read(&self) -> EntityReadGuard;
    fn write(&self) -> EntityWriteGuard;
}

struct InnerRawEntityRwLock<'s, T: Entity> {
    rwlock: &'s RwLock<T>,
}

impl<'s, T: Entity> InnerEntityRwLock for InnerRawEntityRwLock<'s, T> {
    fn read(&self) -> EntityReadGuard {
        EntityReadGuard::new(self.rwlock.read().unwrap())
    }

    fn write(&self) -> EntityWriteGuard {
        EntityWriteGuard::new(self.rwlock.write().unwrap())
    }
}

impl<'s, T: Entity> Debug for InnerRawEntityRwLock<'s, T> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        self.rwlock.fmt(formatter)
    }
}