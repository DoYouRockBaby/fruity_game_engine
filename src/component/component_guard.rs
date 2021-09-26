use std::fmt::Debug;
use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::RwLockWriteGuard;
use std::sync::RwLockReadGuard;
use crate::component::component::Component;

pub struct ComponentReadGuard<'s> {
    guard: Box<dyn InnerComponentReadGuard + 's>,
}

impl<'s> Deref for ComponentReadGuard<'s> {
    type Target = dyn Component;

    fn deref(&self) -> &<Self as Deref>::Target {
        self.guard.deref()
    }
}

impl<'s> Debug for ComponentReadGuard<'s> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        self.guard.fmt(formatter)
    }
}

impl<'s> ComponentReadGuard<'s> {
    pub fn new<T: Component>(inner_guard: RwLockReadGuard<'s, T>) -> ComponentReadGuard {
        ComponentReadGuard {
            guard: Box::new(InnerRawComponentReadGuard::<'s, T> {
                inner_guard,
            }),
        }
    }
}

pub trait InnerComponentReadGuard: Deref<Target = dyn Component> {
}

pub struct InnerRawComponentReadGuard<'s, T: Component> {
    inner_guard: RwLockReadGuard<'s, T>
}

impl<'s, T: Component> Deref for InnerRawComponentReadGuard<'s, T> {
    type Target = dyn Component;

    fn deref(&self) -> &<Self as Deref>::Target {
        self.inner_guard.deref()
    }
}

impl<'s, T: Component> InnerComponentReadGuard for InnerRawComponentReadGuard<'s, T> {
}

pub struct ComponentWriteGuard<'s> {
    guard: Box<dyn InnerComponentWriteGuard + 's>,
}

impl<'s> Deref for ComponentWriteGuard<'s> {
    type Target = dyn Component;

    fn deref(&self) -> &<Self as Deref>::Target {
        self.guard.deref()
    }
}

impl<'s> DerefMut for ComponentWriteGuard<'s> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.guard.deref_mut()
    }
}

impl<'s> Debug for ComponentWriteGuard<'s> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        self.guard.fmt(formatter)
    }
}

impl<'s> ComponentWriteGuard<'s> {
    pub fn new<T: Component>(inner_guard: RwLockWriteGuard<'s, T>) -> ComponentWriteGuard {
        ComponentWriteGuard {
            guard: Box::new(InnerRawComponentWriteGuard::<'s, T> {
                inner_guard,
            }),
        }
    }
}

pub trait InnerComponentWriteGuard: Deref<Target = dyn Component> + DerefMut<Target = dyn Component> {
}

pub struct InnerRawComponentWriteGuard<'s, T: Component> {
    inner_guard: RwLockWriteGuard<'s, T>
}

impl<'s, T: Component> Deref for InnerRawComponentWriteGuard<'s, T> {
    type Target = dyn Component;

    fn deref(&self) -> &<Self as Deref>::Target {
        self.inner_guard.deref()
    }
}

impl<'s, T: Component> DerefMut for InnerRawComponentWriteGuard<'s, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner_guard.deref_mut()
    }
}

impl<'s, T: Component> InnerComponentWriteGuard for InnerRawComponentWriteGuard<'s, T> {
}