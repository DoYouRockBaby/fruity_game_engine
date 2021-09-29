use std::fmt::Debug;
use std::sync::RwLock;
use crate::entity::entity::Entity;
use crate::entity::entity_guard::EntityReadGuard;
use crate::entity::entity_guard::EntityWriteGuard;

pub struct EntityRwLock<'s> {
    inner_lock: Box<dyn InnerEntityRwLock + 's>,
}

impl<'s> EntityRwLock<'s> {
    pub fn new<T: Entity>(rwlock: &'s RwLock<T>) -> EntityRwLock<'s> {
        EntityRwLock {
            inner_lock: Box::new(InnerRawEntityRwLock::<T> {
                rwlock,
            }),
        }
    }

    pub fn read(&self) -> EntityReadGuard<'_> {
        self.inner_lock.read()
    }

    pub fn write(&self) -> EntityWriteGuard<'_> {
        self.inner_lock.write()
    }
}

impl<'s> Debug for EntityRwLock<'s> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        self.inner_lock.fmt(formatter)
    }
}

trait InnerEntityRwLock: Debug + Send + Sync {
    fn read(&self) -> EntityReadGuard;
    fn write(&self) -> EntityWriteGuard;
}

struct InnerRawEntityRwLock<'s, T: Entity> {
    rwlock: &'s RwLock<T>,
}

impl<'s, T: Entity> InnerEntityRwLock for InnerRawEntityRwLock<'s, T> {
    fn read(&self) -> EntityReadGuard {
        EntityReadGuard::new(self.rwlock.read().unwrap())
    }

    fn write(&self) -> EntityWriteGuard {
        EntityWriteGuard::new(self.rwlock.write().unwrap())
    }
}

impl<'s, T: Entity> Debug for InnerRawEntityRwLock<'s, T> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        self.rwlock.fmt(formatter)
    }
}