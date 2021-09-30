use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::RwLockWriteGuard;
use std::sync::RwLockReadGuard;
use crate::entity::entity::Entity;

pub trait InnerEntityReadGuard<'s>: Deref<Target = dyn Entity> {
}

pub struct InnerRawEntityReadGuard<'s, T: Entity> {
    pub inner_guard: RwLockReadGuard<'s, T>
}

impl<'s, T: Entity> Deref for InnerRawEntityReadGuard<'s, T> {
    type Target = dyn Entity;

    fn deref(&self) -> &<Self as Deref>::Target {
        self.inner_guard.deref()
    }
}

impl<'s, T: Entity> InnerEntityReadGuard<'s> for InnerRawEntityReadGuard<'s, T> {
}

pub trait InnerEntityWriteGuard<'s>: Deref<Target = dyn Entity> + DerefMut<Target = dyn Entity> {
}

pub struct InnerRawEntityWriteGuard<'s, T: Entity> {
    pub inner_guard: RwLockWriteGuard<'s, T>
}

impl<'s, T: Entity> Deref for InnerRawEntityWriteGuard<'s, T> {
    type Target = dyn Entity;

    fn deref(&self) -> &<Self as Deref>::Target {
        self.inner_guard.deref()
    }
}

impl<'s, T: Entity> DerefMut for InnerRawEntityWriteGuard<'s, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner_guard.deref_mut()
    }
}

impl<'s, T: Entity> InnerEntityWriteGuard<'s> for InnerRawEntityWriteGuard<'s, T> {
}