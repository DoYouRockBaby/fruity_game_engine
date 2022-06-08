use crate::component::component::Component;
use crate::component::component::StaticComponent;
use crate::entity::archetype::Archetype;
use fruity_core::RwLockReadGuard;
use fruity_core::RwLockWriteGuard;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::marker::PhantomData;
use std::ops::Deref;
use std::ops::DerefMut;
use std::rc::Rc;

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

impl<'a> ComponentReadGuard<'a> {
    /// Get an extension component reader
    pub fn get_extension(&self, extension_identifier: &str) -> Option<ComponentReadGuard<'a>> {
        let has_extension = self
            .archetype_reader
            .component_storages
            .contains_key(extension_identifier);

        if has_extension {
            Some(ComponentReadGuard {
                _guard: self._guard.clone(),
                archetype_reader: self.archetype_reader.clone(),
                component_identifier: extension_identifier.to_string(),
                component_index: self.component_index.clone(),
            })
        } else {
            None
        }
    }
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

impl<'a> ComponentWriteGuard<'a> {
    /// Get an extension component reader
    pub fn get_extension(&self, extension_identifier: &str) -> Option<ComponentReadGuard<'a>> {
        let has_extension = self
            .archetype_reader
            .component_storages
            .contains_key(extension_identifier);

        if has_extension {
            Some(ComponentReadGuard {
                _guard: InternalReadGuard::Write(self._guard.clone()),
                archetype_reader: self.archetype_reader.clone(),
                component_identifier: extension_identifier.to_string(),
                component_index: self.component_index.clone(),
            })
        } else {
            None
        }
    }

    /// Get an extension component writer
    pub fn get_extension_mut(&self, extension_identifier: &str) -> Option<ComponentWriteGuard<'a>> {
        let has_extension = self
            .archetype_reader
            .component_storages
            .contains_key(extension_identifier);

        if has_extension {
            Some(ComponentWriteGuard {
                _guard: self._guard.clone(),
                archetype_reader: self.archetype_reader.clone(),
                component_identifier: extension_identifier.to_string(),
                component_index: self.component_index.clone(),
            })
        } else {
            None
        }
    }
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

impl<'a, T: Component + StaticComponent> TypedComponentReadGuard<'a, T> {
    /// Get an extension component reader
    pub fn get_extension<E: Component + StaticComponent>(
        &self,
    ) -> Option<TypedComponentReadGuard<'a, E>> {
        self.component_reader
            .get_extension(E::get_component_name())
            .map(|component_reader| TypedComponentReadGuard::<'a, E> {
                component_reader,
                phantom: PhantomData {},
            })
    }
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

impl<'a, T: Component + StaticComponent> TypedComponentWriteGuard<'a, T> {
    /// Get an extension component reader
    pub fn get_extension<E: Component + StaticComponent>(
        &self,
    ) -> Option<TypedComponentReadGuard<'a, E>> {
        self.component_writer
            .get_extension(E::get_component_name())
            .map(|component_reader| TypedComponentReadGuard::<'a, E> {
                component_reader,
                phantom: PhantomData {},
            })
    }

    /// Get an extension component writer
    pub fn get_extension_mut<E: Component + StaticComponent>(
        &self,
    ) -> Option<TypedComponentWriteGuard<'a, E>> {
        self.component_writer
            .get_extension_mut(E::get_component_name())
            .map(|component_writer| TypedComponentWriteGuard::<'a, E> {
                component_writer,
                phantom: PhantomData {},
            })
    }
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
