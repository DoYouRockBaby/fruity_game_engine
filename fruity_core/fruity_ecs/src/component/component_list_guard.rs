use crate::component::component::Component;
use crate::entity::archetype::rwlock::EntityReadGuard;
use crate::entity::archetype::rwlock::EntityWriteGuard;
use crate::entity::archetype::EntityCellHead;
use std::fmt::Debug;
use std::ops::Deref;
use std::ops::DerefMut;

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

    /// Returns a reference array over the components
    pub fn get_components(&self) -> Vec<&dyn Component> {
        let components = self.guard.get_components();

        self.component_indexes
            .iter()
            .map(move |index| {
                let result = components.get(*index).unwrap();
                *result
            })
            .collect::<Vec<_>>()
    }
}

impl<'a> Deref for ComponentListReadGuard<'a> {
    type Target = EntityCellHead;

    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        self.guard.deref()
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

    /// Returns a mutable reference array over the components
    pub fn get_components_mut(&mut self) -> Vec<&mut dyn Component> {
        let components = self.guard.get_components_mut();

        self.component_indexes
            .iter()
            .map(move |index| {
                let result = components.get_mut(*index).unwrap();
                let result = unsafe { &mut *(*result as *mut _) } as &mut dyn Component;

                result
            })
            .collect::<Vec<_>>()
    }
}

impl<'a> Deref for ComponentListWriteGuard<'a> {
    type Target = EntityCellHead;

    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        self.guard.deref()
    }
}

impl<'a> DerefMut for ComponentListWriteGuard<'a> {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        self.guard.deref_mut()
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
