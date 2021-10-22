use crate::component::component::Component;
use crate::entity::entity_guard::EntityReadGuard;
use crate::entity::entity_guard::EntityWriteGuard;
use std::fmt::Debug;
use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::Arc;
use std::sync::RwLock;

/// RAII structure used to release the shared read access of a lock when dropped.
///
/// This structure is created by the [`read`] methods on [`ComponentRwLock`].
///
/// [`read`]: ComponentRwLock::read
///
#[derive(Clone)]
pub struct ComponentReadGuard<'s> {
    guard: Arc<RwLock<EntityReadGuard<'s>>>,
    component_index: usize,
}

impl<'s> ComponentReadGuard<'s> {
    /// Returns an RwLockReadGuard which is unlocked.
    ///
    /// # Arguments
    /// * `inner_guard` - The typed [`RwLockReadGuard`]
    ///
    pub(crate) fn new(
        guard: Arc<RwLock<EntityReadGuard<'s>>>,
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
        let component = self.guard.read().unwrap();
        let component = component.get(self.component_index).unwrap();

        let component = unsafe { &*(component as *const _) } as &dyn Component;
        component
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
#[derive(Clone)]
pub struct ComponentWriteGuard<'s> {
    guard: Arc<RwLock<EntityWriteGuard<'s>>>,
    component_index: usize,
}

impl<'s> ComponentWriteGuard<'s> {
    /// Returns an ComponentWriteGuard which is unlocked.
    ///
    /// # Arguments
    /// * `inner_guard` - The typed [`RwLockWriteGuard`]
    ///
    pub(crate) fn new(
        guard: Arc<RwLock<EntityWriteGuard<'s>>>,
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
        let component = self.guard.read().unwrap();
        let component = component.get(self.component_index).unwrap();

        let component = unsafe { &*(component as *const _) } as &dyn Component;
        component
    }
}

impl<'s> DerefMut for ComponentWriteGuard<'s> {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        let mut component = self.guard.write().unwrap();
        let component = component.get_mut(self.component_index).unwrap();

        let component = unsafe { &mut *(component as *mut _) } as &mut dyn Component;
        component
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
