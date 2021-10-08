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
    guard: RwLockReadGuard<'s, Entity>,
}

impl<'s> EntityReadGuard<'s> {
    /// Returns an RwLockReadGuard which is unlocked.
    ///
    /// # Arguments
    /// * `inner_guard` - The typed [`RwLockReadGuard`]
    ///
    pub(crate) fn new(guard: RwLockReadGuard<'s, Entity>) -> EntityReadGuard<'s> {
        EntityReadGuard { guard }
    }
}

impl<'s> Deref for EntityReadGuard<'s> {
    type Target = Entity;

    fn deref(&self) -> &<Self as Deref>::Target {
        self.guard.deref()
    }
}

impl<'s> Debug for EntityReadGuard<'s> {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        self.guard.fmt(formatter)
    }
}

/// RAII structure used to release the exclusive write access of a lock when dropped.
///
/// This structure is created by the [`write`] methods on [`EntityRwLock`].
///
/// [`write`]: EntityRwLock::write
///
pub struct EntityWriteGuard<'s> {
    guard: RwLockWriteGuard<'s, Entity>,
}

impl<'s> EntityWriteGuard<'s> {
    /// Returns an EntityWriteGuard which is unlocked.
    ///
    /// # Arguments
    /// * `inner_guard` - The typed [`RwLockWriteGuard`]
    ///
    pub(crate) fn new(guard: RwLockWriteGuard<'s, Entity>) -> EntityWriteGuard<'s> {
        EntityWriteGuard { guard }
    }
}

impl<'s> Deref for EntityWriteGuard<'s> {
    type Target = Entity;

    fn deref(&self) -> &<Self as Deref>::Target {
        self.guard.deref()
    }
}

impl<'s> DerefMut for EntityWriteGuard<'s> {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        self.guard.deref_mut()
    }
}

impl<'s> Debug for EntityWriteGuard<'s> {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        self.guard.fmt(formatter)
    }
}
