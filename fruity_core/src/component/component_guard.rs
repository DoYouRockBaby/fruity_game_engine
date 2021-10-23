use crate::component::component::Component;
use crate::entity::archetype::rwlock::EntityReadGuard;
use crate::entity::archetype::rwlock::EntityWriteGuard;
use std::fmt::Debug;
use std::ops::Deref;
use std::ops::DerefMut;

/// RAII structure used to release the shared read access of a lock when dropped.
///
/// This structure is created by the [`read`] methods on [`ComponentRwLock`].
///
/// [`read`]: ComponentRwLock::read
///
pub struct ComponentReadGuard<'s> {
    guard: EntityReadGuard<'s>,
    component_index: usize,
}

impl<'s> ComponentReadGuard<'s> {
    /// Returns an RwLockReadGuard which is unlocked.
    ///
    /// # Arguments
    /// * `inner_guard` - The typed [`RwLockReadGuard`]
    ///
    pub(crate) fn new(
        guard: EntityReadGuard<'s>,
        component_index: usize,
    ) -> ComponentReadGuard<'s> {
        ComponentReadGuard {
            guard,
            component_index,
        }
    }
}

impl<'s> Deref for ComponentReadGuard<'s> {
    type Target = dyn Component;

    fn deref(&self) -> &<Self as Deref>::Target {
        *self.guard.get(self.component_index).unwrap()
    }
}

impl<'s> Debug for ComponentReadGuard<'s> {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        self.guard.fmt(formatter)
    }
}

/// RAII structure used to release the exclusive write access of a lock when dropped.
///
/// This structure is created by the [`write`] methods on [`ComponentRwLock`].
///
/// [`write`]: ComponentRwLock::write
///
pub struct ComponentWriteGuard<'s> {
    guard: EntityWriteGuard<'s>,
    component_index: usize,
}

impl<'s> ComponentWriteGuard<'s> {
    /// Returns an ComponentWriteGuard which is unlocked.
    ///
    /// # Arguments
    /// * `inner_guard` - The typed [`RwLockWriteGuard`]
    ///
    pub(crate) fn new(
        guard: EntityWriteGuard<'s>,
        component_index: usize,
    ) -> ComponentWriteGuard<'s> {
        ComponentWriteGuard {
            guard,
            component_index,
        }
    }
}

impl<'s> Deref for ComponentWriteGuard<'s> {
    type Target = dyn Component;

    fn deref(&self) -> &<Self as Deref>::Target {
        *self.guard.get(self.component_index).unwrap()
    }
}

impl<'s> DerefMut for ComponentWriteGuard<'s> {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        *self.guard.get_mut(self.component_index).unwrap()
    }
}

impl<'s> Debug for ComponentWriteGuard<'s> {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        self.guard.fmt(formatter)
    }
}
