use crate::component::component::Component;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::RwLockReadGuard;
use std::sync::RwLockWriteGuard;

/// RAII structure used to release the shared read access of a lock when dropped.
///
/// This structure is created by the [`read`] methods on [`ComponentRwLock`].
///
/// [`read`]: ComponentRwLock::read
///
pub struct ComponentReadGuard<'a> {
    pub(crate) guard: RwLockReadGuard<'a, ()>,
    pub(crate) component: &'a dyn Component,
}

impl<'a> ComponentReadGuard<'a> {
    /// Returns an RwLockReadGuard which is unlocked.
    ///
    /// # Arguments
    /// * `inner_guard` - The typed [`RwLockReadGuard`]
    ///
    pub(crate) fn new(
        guard: RwLockReadGuard<'a, ()>,
        component: &'a dyn Component,
    ) -> ComponentReadGuard<'a> {
        ComponentReadGuard { guard, component }
    }

    /// Downcast to a typed read guard
    pub fn downcast<T: Component>(self) -> Option<TypedComponentReadGuard<'a, T>> {
        if let Some(_) = self.deref().as_any_ref().downcast_ref::<T>() {
            Some(TypedComponentReadGuard::<'a, T> {
                guard: self,
                _phantom: PhantomData::default(),
            })
        } else {
            None
        }
    }
}

impl<'a> Deref for ComponentReadGuard<'a> {
    type Target = dyn Component;

    fn deref(&self) -> &<Self as Deref>::Target {
        self.component
    }
}

impl<'a> Debug for ComponentReadGuard<'a> {
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
pub struct ComponentWriteGuard<'a> {
    pub(crate) guard: RwLockWriteGuard<'a, ()>,
    pub(crate) component: &'a mut dyn Component,
}

impl<'a> ComponentWriteGuard<'a> {
    /// Returns an ComponentWriteGuard which is unlocked.
    ///
    /// # Arguments
    /// * `inner_guard` - The typed [`RwLockWriteGuard`]
    ///
    pub(crate) fn new(
        guard: RwLockWriteGuard<'a, ()>,
        component: &'a dyn Component,
    ) -> ComponentWriteGuard<'a> {
        let component = unsafe {
            &mut *(component as *const dyn Component as *mut dyn Component) as &mut dyn Component
        };
        ComponentWriteGuard { guard, component }
    }

    /// Downcast to a typed write guard
    pub fn downcast<T: Component>(self) -> Option<TypedComponentWriteGuard<'a, T>> {
        if let Some(_) = self.deref().as_any_ref().downcast_ref::<T>() {
            Some(TypedComponentWriteGuard::<'a, T> {
                guard: self,
                _phantom: PhantomData::default(),
            })
        } else {
            None
        }
    }
}

impl<'a> Deref for ComponentWriteGuard<'a> {
    type Target = dyn Component;

    fn deref(&self) -> &<Self as Deref>::Target {
        self.component
    }
}

impl<'a> DerefMut for ComponentWriteGuard<'a> {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        self.component
    }
}

impl<'a> Debug for ComponentWriteGuard<'a> {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        self.guard.fmt(formatter)
    }
}

/// RAII structure used to release the shared read access of a lock when dropped.
///
/// This structure is created by the [`read`] methods on [`ComponentRwLock`].
///
/// [`read`]: ComponentRwLock::read
///
pub struct TypedComponentReadGuard<'a, T: Component> {
    pub(crate) guard: ComponentReadGuard<'a>,
    _phantom: PhantomData<T>,
}

impl<'a, T: Component> Deref for TypedComponentReadGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &<Self as Deref>::Target {
        self.guard.deref().as_any_ref().downcast_ref::<T>().unwrap()
    }
}

impl<'a, T: Component> Debug for TypedComponentReadGuard<'a, T> {
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
pub struct TypedComponentWriteGuard<'a, T: Component> {
    pub(crate) guard: ComponentWriteGuard<'a>,
    _phantom: PhantomData<T>,
}

impl<'a, T: Component> Deref for TypedComponentWriteGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &<Self as Deref>::Target {
        self.guard.deref().as_any_ref().downcast_ref::<T>().unwrap()
    }
}

impl<'a, T: Component> DerefMut for TypedComponentWriteGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
        self.guard
            .deref_mut()
            .as_any_mut()
            .downcast_mut::<T>()
            .unwrap()
    }
}

impl<'a, T: Component> Debug for TypedComponentWriteGuard<'a, T> {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        self.guard.fmt(formatter)
    }
}
