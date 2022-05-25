use crate::component::component::Component;
use crate::component::component::StaticComponent;
use crate::entity::archetype::Archetype;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::marker::PhantomData;
use std::ops::Deref;
use std::ops::DerefMut;
use std::rc::Rc;
use std::sync::RwLockReadGuard;
use std::sync::RwLockWriteGuard;

#[derive(Clone)]
pub(crate) enum InternalReadGuard<'a> {
    Read(Rc<RwLockReadGuard<'a, ()>>),
    Write(Rc<RwLockWriteGuard<'a, ()>>),
}

/// RAII structure used to release the shared read access of a lock when dropped.
///
/// This structure is created by the [`read`] methods on [`ComponentReference`].
///
/// [`read`]: ComponentReference::read
///
#[derive(Clone)]
pub struct ComponentReadGuard<'a> {
    pub(crate) _guard: InternalReadGuard<'a>,
    pub(crate) archetype_reader: Rc<RwLockReadGuard<'a, Archetype>>,
    pub(crate) component_identifier: String,
    pub(crate) component_index: usize,
}

impl<'a> Debug for ComponentReadGuard<'a> {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl<'a> Deref for ComponentReadGuard<'a> {
    type Target = dyn Component;

    fn deref(&self) -> &Self::Target {
        let storage = self
            .archetype_reader
            .component_storages
            .get(&self.component_identifier)
            .unwrap();

        storage.collection.get(&self.component_index).unwrap()
    }
}

impl<'a, T: Component + StaticComponent> TryInto<TypedComponentReadGuard<'a, T>>
    for ComponentReadGuard<'a>
{
    type Error = String;

    fn try_into(self) -> Result<TypedComponentReadGuard<'a, T>, Self::Error> {
        match self.as_any_ref().downcast_ref::<T>() {
            Some(_result) => Ok(TypedComponentReadGuard {
                component_reader: self,
                phantom: PhantomData::<T> {},
            }),
            None => Err(format!("Couldn't convert {:?} to typed component", self)),
        }
    }
}

/// RAII structure used to release the shared write access of a lock when dropped.
///
/// This structure is created by the [`write`] methods on [`ComponentReference`].
///
/// [`write`]: ComponentReference::write
///
#[derive(Clone)]
pub struct ComponentWriteGuard<'a> {
    pub(crate) _guard: Rc<RwLockWriteGuard<'a, ()>>,
    pub(crate) archetype_reader: Rc<RwLockReadGuard<'a, Archetype>>,
    pub(crate) component_identifier: String,
    pub(crate) component_index: usize,
}

impl<'a> Debug for ComponentWriteGuard<'a> {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl<'a> Deref for ComponentWriteGuard<'a> {
    type Target = dyn Component;

    fn deref(&self) -> &Self::Target {
        let storage = self
            .archetype_reader
            .component_storages
            .get(&self.component_identifier)
            .unwrap();

        storage.collection.get(&self.component_index).unwrap()
    }
}

impl<'a> DerefMut for ComponentWriteGuard<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let storage = self
            .archetype_reader
            .component_storages
            .get(&self.component_identifier)
            .unwrap();

        let component = storage.collection.get(&self.component_index).unwrap();

        // Safe cause it is protected by self._guard
        #[allow(mutable_transmutes)]
        unsafe {
            std::mem::transmute::<&dyn Component, &mut dyn Component>(component)
        }
    }
}

impl<'a, T: Component + StaticComponent> TryInto<TypedComponentWriteGuard<'a, T>>
    for ComponentWriteGuard<'a>
{
    type Error = String;

    fn try_into(self) -> Result<TypedComponentWriteGuard<'a, T>, Self::Error> {
        match self.as_any_ref().downcast_ref::<T>() {
            Some(_result) => Ok(TypedComponentWriteGuard {
                component_writer: self,
                phantom: PhantomData::<T> {},
            }),
            None => Err(format!("Couldn't convert {:?} to typed component", self)),
        }
    }
}

/// RAII structure used to release the shared read access of a lock when dropped.
///
/// This structure is created by the [`read`] methods on [`ComponentReference`].
///
/// [`read`]: ComponentReference::read
///
pub struct TypedComponentReadGuard<'a, T: Component + StaticComponent> {
    pub(crate) component_reader: ComponentReadGuard<'a>,
    pub(crate) phantom: PhantomData<T>,
}

impl<'a, T: Component + StaticComponent> Clone for TypedComponentReadGuard<'a, T> {
    fn clone(&self) -> Self {
        Self {
            component_reader: self.component_reader.clone(),
            phantom: PhantomData {},
        }
    }
}

impl<'a, T: Component + StaticComponent> Debug for TypedComponentReadGuard<'a, T> {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), std::fmt::Error> {
        self.deref().fmt(formatter)
    }
}

impl<'a, T: Component + StaticComponent> Deref for TypedComponentReadGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.component_reader
            .as_any_ref()
            .downcast_ref::<T>()
            .unwrap()
    }
}

/// RAII structure used to release the shared write access of a lock when dropped.
///
/// This structure is created by the [`write`] methods on [`ComponentReference`].
///
/// [`write`]: ComponentReference::write
///
pub struct TypedComponentWriteGuard<'a, T: Component + StaticComponent> {
    pub(crate) component_writer: ComponentWriteGuard<'a>,
    pub(crate) phantom: PhantomData<T>,
}

impl<'a, T: Component + StaticComponent> Clone for TypedComponentWriteGuard<'a, T> {
    fn clone(&self) -> Self {
        Self {
            component_writer: self.component_writer.clone(),
            phantom: PhantomData {},
        }
    }
}

impl<'a, T: Component + StaticComponent> Debug for TypedComponentWriteGuard<'a, T> {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), std::fmt::Error> {
        self.deref().fmt(formatter)
    }
}

impl<'a, T: Component + StaticComponent> Deref for TypedComponentWriteGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.component_writer
            .as_any_ref()
            .downcast_ref::<T>()
            .unwrap()
    }
}

impl<'a, T: Component + StaticComponent> DerefMut for TypedComponentWriteGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.component_writer
            .as_any_mut()
            .downcast_mut::<T>()
            .unwrap()
    }
}
