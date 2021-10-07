use crate::entity::entity::Entity;
use std::fmt::Debug;
use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::RwLockReadGuard;
use std::sync::RwLockWriteGuard;

/// RAII structure used to release the shared read access of a lock when dropped.
///
/// This structure is created by the [`read`] methods on [`EntityRwLock`].
///
/// [`read`]: EntityRwLock::read
///
pub struct EntityReadGuard<'s> {
    pub inner_guard: RwLockReadGuard<'s, Entity<'s>>,
}

impl<'s> Deref for EntityReadGuard<'s> {
    type Target = Entity<'s>;

    fn deref(&self) -> &<Self as Deref>::Target {
        self.inner_guard.deref()
    }
}

impl<'s> Debug for EntityReadGuard<'s> {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        self.inner_guard.fmt(formatter)
    }
}

impl<'s> EntityReadGuard<'s> {
    /// Returns an RwLockReadGuard which is unlocked.
    ///
    /// # Arguments
    /// * `inner_guard` - The typed [`RwLockReadGuard`]
    ///
    pub fn new(inner_guard: RwLockReadGuard<'s, Entity>) -> EntityReadGuard {
        EntityReadGuard { inner_guard }
    }
}

/// RAII structure used to release the exclusive write access of a lock when dropped.
///
/// This structure is created by the [`write`] methods on [`EntityRwLock`].
///
/// [`write`]: EntityRwLock::write
///
pub struct EntityWriteGuard<'s> {
    pub inner_guard: RwLockWriteGuard<'s, Entity<'s>>,
}

impl<'s> Deref for EntityWriteGuard<'s> {
    type Target = Entity<'s>;

    fn deref(&self) -> &<Self as Deref>::Target {
        self.inner_guard.deref()
    }
}

impl<'s> DerefMut for EntityWriteGuard<'s> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner_guard.deref_mut()
    }
}

impl<'s> Debug for EntityWriteGuard<'s> {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        self.inner_guard.fmt(formatter)
    }
}

impl<'s> EntityWriteGuard<'s> {
    /// Returns an EntityWriteGuard which is unlocked.
    ///
    /// # Arguments
    /// * `inner_guard` - The typed [`RwLockWriteGuard`]
    ///
    pub fn new(inner_guard: RwLockWriteGuard<'s, Entity>) -> EntityWriteGuard {
        EntityWriteGuard { inner_guard }
    }
}
