/*use std::ops::DerefMut;
use std::ops::Deref;
use std::sync::RwLockReadGuard;
use std::sync::RwLockWriteGuard;
use crate::component::internal::component_guard::InnerRawComponentWriteGuard;
use crate::component::internal::component_guard::InnerComponentWriteGuard;
use crate::component::internal::component_guard::InnerRawComponentReadGuard;
use crate::component::internal::component_guard::InnerComponentReadGuard;
use crate::component::internal::component_rwlock::InnerRawComponentRwLock;
use crate::component::internal::component_rwlock::InnerComponentRwLock;
use std::sync::RwLock;*/
use std::any::TypeId;
use std::any::type_name;
use std::any::Any;
use std::fmt::Debug;

pub trait Component: Debug + Any + Send + Sync {
    fn get_component_type(&self) -> &str;
    fn get_untyped_field(&self, property: &str) -> Option<&dyn Any>;
    fn set_untyped_field(&mut self, property: &str, value: &dyn Any);
}

impl dyn Component {
    pub fn get_field<T: Any>(&self, property: &str) -> Option<&T> {
        match self.get_untyped_field(property) {
            Some(value) => match value.downcast_ref::<T>() {
                Some(value) => {
                    Some(value)
                }
                None => {
                    log::error!("Try to get a {:?} from property {:?}, got {:?}", type_name::<T>(), property, value);
                    None
                }
            },
            None => None,
        }
    }

    pub fn set_field<T: Any>(&mut self, property: &str, value: T) {
        self.set_untyped_field(property, &value);
    }
    
    pub fn is<T: Component>(&self) -> bool {
        // Get `TypeId` of the type this function is instantiated with.
        let t = TypeId::of::<T>();

        // Get `TypeId` of the type in the trait object (`self`).
        let concrete = self.type_id();

        // Compare both `TypeId`s on equality.
        t == concrete
    }

    pub fn downcast_ref<T: Component>(&self) -> Option<&T> {
        if self.is::<T>() {
            // SAFETY: just checked whether we are pointing to the correct type, and we can rely on
            // that check for memory safety because we have implemented Component for all types; no other
            // impls can exist as they would conflict with our impl.
            unsafe { Some(&*(self as *const dyn Component as *const T)) }
        } else {
            None
        }
    }

    pub fn downcast_mut<T: Component>(&mut self) -> Option<&mut T> {
        if self.is::<T>() {
            // SAFETY: just checked whether we are pointing to the correct type, and we can rely on
            // that check for memory safety because we have implemented Component for all types; no other
            // impls can exist as they would conflict with our impl.
            unsafe { Some(&mut *(self as *mut dyn Component as *mut T)) }
        } else {
            None
        }
    }
}

/*pub struct ComponentRwLock<'s> {
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

pub struct ComponentReadGuard<'s> {
    guard: Box<dyn InnerComponentReadGuard + 's>,
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
}*/