use crate::entity::archetype::rwlock::EntityReadGuard;
use crate::entity::archetype::rwlock::EntityWriteGuard;
use std::fmt::Debug;

/// RAII structure used to release the shared read access of a lock when dropped.
///
/// This structure is created by the [`read`] methods on [`ComponentListRwLock`].
///
/// [`read`]: ComponentListRwLock::read
///
pub struct ComponentListReadGuard<'s> {
    guard: EntityReadGuard<'s>,
    component_indexes: Vec<usize>,
}

impl<'s> ComponentListReadGuard<'s> {
    /// Returns an RwLockReadGuard which is unlocked.
    ///
    /// # Arguments
    /// * `inner_guard` - The typed [`RwLockReadGuard`]
    ///
    pub(crate) fn new(
        guard: EntityReadGuard<'s>,
        component_indexes: Vec<usize>,
    ) -> ComponentListReadGuard<'s> {
        ComponentListReadGuard {
            guard,
            component_indexes,
        }
    }
}

impl<'s> Debug for ComponentListReadGuard<'s> {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        self.guard.fmt(formatter)
    }
}

/// RAII structure used to release the exclusive write access of a lock when dropped.
///
/// This structure is created by the [`write`] methods on [`ComponentListRwLock`].
///
/// [`write`]: ComponentListRwLock::write
///
pub struct ComponentListWriteGuard<'s> {
    guard: EntityWriteGuard<'s>,
    component_indexes: Vec<usize>,
}

impl<'s> ComponentListWriteGuard<'s> {
    /// Returns an ComponentWriteGuard which is unlocked.
    ///
    /// # Arguments
    /// * `inner_guard` - The typed [`RwLockWriteGuard`]
    ///
    pub(crate) fn new(
        guard: EntityWriteGuard<'s>,
        component_indexes: Vec<usize>,
    ) -> ComponentListWriteGuard<'s> {
        ComponentListWriteGuard {
            guard,
            component_indexes,
        }
    }
}

impl<'s> Debug for ComponentListWriteGuard<'s> {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        self.guard.fmt(formatter)
    }
}
