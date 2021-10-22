use crate::service::service::Service;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::RwLockReadGuard;
use std::sync::RwLockWriteGuard;

/// RAII structure used to release the shared read access of a lock when dropped.
///
/// This structure is created by the [`read`] methods on [`ServiceManager`].
///
/// [`read`]: ServiceManager::read
///
pub struct ServiceReadGuard<'s, T: Service + ?Sized> {
    guard: RwLockReadGuard<'s, Box<dyn Service>>,
    _phantom: PhantomData<T>,
}

impl<'s, T: Service> ServiceReadGuard<'s, T> {
    /// Returns an RwLockReadGuard which is unlocked.
    ///
    /// # Arguments
    /// * `inner_guard` - The typed [`RwLockReadGuard`]
    ///
    pub(crate) fn new(guard: RwLockReadGuard<'s, Box<dyn Service>>) -> ServiceReadGuard<'s, T> {
        ServiceReadGuard {
            guard,
            _phantom: PhantomData,
        }
    }
}

impl<'s, T: Service> Deref for ServiceReadGuard<'s, T> {
    type Target = T;

    fn deref(&self) -> &<Self as Deref>::Target {
        self.guard.deref().as_any_ref().downcast_ref::<T>().unwrap()
    }
}

impl<'s, T: Service> Debug for ServiceReadGuard<'s, T> {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        self.guard.fmt(formatter)
    }
}

/// RAII structure used to release the exclusive write access of a lock when dropped.
///
/// This structure is created by the [`write`] methods on [`ServiceManager`].
///
/// [`write`]: ServiceManager::write
///
pub struct ServiceWriteGuard<'s, T: Service + ?Sized> {
    guard: RwLockWriteGuard<'s, Box<dyn Service>>,
    _phantom: PhantomData<T>,
}

impl<'s, T: Service> ServiceWriteGuard<'s, T> {
    /// Returns an ServiceWriteGuard which is unlocked.
    ///
    /// # Arguments
    /// * `inner_guard` - The typed [`RwLockWriteGuard`]
    ///
    pub(crate) fn new(guard: RwLockWriteGuard<'s, Box<dyn Service>>) -> ServiceWriteGuard<'s, T> {
        ServiceWriteGuard {
            guard,
            _phantom: PhantomData,
        }
    }
}

impl<'s, T: Service> Deref for ServiceWriteGuard<'s, T> {
    type Target = T;

    fn deref(&self) -> &<Self as Deref>::Target {
        self.guard.deref().as_any_ref().downcast_ref::<T>().unwrap()
    }
}

impl<'s, T: Service> DerefMut for ServiceWriteGuard<'s, T> {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        self.guard
            .deref_mut()
            .as_any_mut()
            .downcast_mut::<T>()
            .unwrap()
    }
}

impl<'s, T: Service> Debug for ServiceWriteGuard<'s, T> {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        self.guard.fmt(formatter)
    }
}
