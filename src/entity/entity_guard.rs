use crate::entity::internal::entity_guard::InnerRawEntityWriteGuard;
use crate::entity::internal::entity_guard::InnerEntityWriteGuard;
use crate::entity::internal::entity_guard::InnerEntityReadGuard;
use crate::entity::internal::entity_guard::InnerRawEntityReadGuard;
use std::fmt::Debug;
use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::RwLockWriteGuard;
use std::sync::RwLockReadGuard;
use crate::entity::entity::Entity;

pub struct EntityReadGuard<'s> {
    guard: Box<dyn InnerEntityReadGuard<'s> + 's>,
}

impl<'s> Deref for EntityReadGuard<'s> {
    type Target = dyn Entity;

    fn deref(&self) -> &<Self as Deref>::Target {
        self.guard.deref()
    }
}

impl<'s> Debug for EntityReadGuard<'s> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        self.guard.fmt(formatter)
    }
}

impl<'s> EntityReadGuard<'s> {
    pub fn new<T: Entity>(inner_guard: RwLockReadGuard<'s, T>) -> EntityReadGuard {
        EntityReadGuard {
            guard: Box::new(InnerRawEntityReadGuard::<'s, T> {
                inner_guard,
            }),
        }
    }
}

pub struct EntityWriteGuard<'s> {
    guard: Box<dyn InnerEntityWriteGuard<'s> + 's>,
}

impl<'s> Deref for EntityWriteGuard<'s> {
    type Target = dyn Entity;

    fn deref(&self) -> &<Self as Deref>::Target {
        self.guard.deref()
    }
}

impl<'s> DerefMut for EntityWriteGuard<'s> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.guard.deref_mut()
    }
}

impl<'s> Debug for EntityWriteGuard<'s> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        self.guard.fmt(formatter)
    }
}

impl<'s> EntityWriteGuard<'s> {
    pub fn new<T: Entity>(inner_guard: RwLockWriteGuard<'s, T>) -> EntityWriteGuard {
        EntityWriteGuard {
            guard: Box::new(InnerRawEntityWriteGuard::<'s, T> {
                inner_guard,
            }),
        }
    }
}