use std::fmt::Debug;
use std::sync::RwLock;
use crate::component::component::Component;
use crate::component::component_guard::ComponentReadGuard;
use crate::component::component_guard::ComponentWriteGuard;

pub struct ComponentRwLock<'s> {
    inner_lock: Box<dyn InnerComponentRwLock + 's>,
}

impl<'s> ComponentRwLock<'s> {
    pub fn new<T: Component + 's>(rwlock: &'s RwLock<T>) -> ComponentRwLock<'s> {
        ComponentRwLock {
            inner_lock: Box::new(InnerRawComponentRwLock::<T> {
                rwlock,
            }),
        }
    }

    pub fn read(&self) -> ComponentReadGuard<'_> {
        self.inner_lock.read()
    }

    pub fn write(&self) -> ComponentWriteGuard<'_> {
        self.inner_lock.write()
    }
}

impl<'s> Debug for ComponentRwLock<'s> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        self.inner_lock.fmt(formatter)
    }
}

trait InnerComponentRwLock: Debug + Send + Sync {
    fn read(&self) -> ComponentReadGuard;
    fn write(&self) -> ComponentWriteGuard;
}

struct InnerRawComponentRwLock<'s, T: Component + 's> {
    rwlock: &'s RwLock<T>,
}

impl<'s, T: Component + 's> InnerComponentRwLock for InnerRawComponentRwLock<'s, T> {
    fn read(&self) -> ComponentReadGuard<'_> {
        ComponentReadGuard::new(self.rwlock.read().unwrap())
    }

    fn write(&self) -> ComponentWriteGuard<'_> {
        let rwlock = self.rwlock;
        ComponentWriteGuard::new(rwlock.write().unwrap())
    }
}

impl<'s, T: Component + 's> Debug for InnerRawComponentRwLock<'s, T> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        self.rwlock.fmt(formatter)
    }
}