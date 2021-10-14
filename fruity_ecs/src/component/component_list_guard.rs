use crate::component::component_guard::ComponentReadGuard;
use crate::component::component_guard::ComponentWriteGuard;
use crate::entity::entity::Entity;
use std::fmt::Debug;
use std::sync::Arc;
use std::sync::RwLock;
use std::sync::RwLockReadGuard;
use std::sync::RwLockWriteGuard;

/// RAII structure used to release the shared read access of a lock when dropped.
///
/// This structure is created by the [`read`] methods on [`ComponentListRwLock`].
///
/// [`read`]: ComponentListRwLock::read
///
#[derive(Clone)]
pub struct ComponentListReadGuard<'s> {
    guard: Arc<RwLock<RwLockReadGuard<'s, Entity>>>,
    component_indexes: Vec<usize>,
}

impl<'s> ComponentListReadGuard<'s> {
    /// Returns an RwLockReadGuard which is unlocked.
    ///
    /// # Arguments
    /// * `inner_guard` - The typed [`RwLockReadGuard`]
    ///
    pub(crate) fn new(
        guard: Arc<RwLock<RwLockReadGuard<'s, Entity>>>,
        component_indexes: Vec<usize>,
    ) -> ComponentListReadGuard<'s> {
        ComponentListReadGuard {
            guard,
            component_indexes,
        }
    }

    /// Returns an reader for a specific component
    ///
    /// # Arguments
    /// * `index` - The index of the component in this list
    ///
    pub(crate) fn get(&self, index: usize) -> Option<ComponentReadGuard<'s>> {
        self.component_indexes
            .get(index)
            .map(|index| ComponentReadGuard::new(self.guard.clone(), *index))
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
#[derive(Clone)]
pub struct ComponentListWriteGuard<'s> {
    guard: Arc<RwLock<RwLockWriteGuard<'s, Entity>>>,
    component_indexes: Vec<usize>,
}

impl<'s> ComponentListWriteGuard<'s> {
    /// Returns an ComponentWriteGuard which is unlocked.
    ///
    /// # Arguments
    /// * `inner_guard` - The typed [`RwLockWriteGuard`]
    ///
    pub(crate) fn new(
        guard: Arc<RwLock<RwLockWriteGuard<'s, Entity>>>,
        component_indexes: Vec<usize>,
    ) -> ComponentListWriteGuard<'s> {
        ComponentListWriteGuard {
            guard,
            component_indexes,
        }
    }

    /// Returns an reader for a specific component
    ///
    /// # Arguments
    /// * `index` - The index of the component in this list
    ///
    pub(crate) fn get(&self, index: usize) -> Option<ComponentWriteGuard<'s>> {
        self.component_indexes
            .get(index)
            .map(|index| ComponentWriteGuard::new(self.guard.clone(), *index))
    }

    /// Returns an writer for a specific component
    ///
    /// # Arguments
    /// * `index` - The index of the component in this list
    ///
    pub(crate) fn get_mut(&self, index: usize) -> Option<ComponentWriteGuard<'s>> {
        self.component_indexes
            .get(index)
            .map(|index| ComponentWriteGuard::new(self.guard.clone(), *index))
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
