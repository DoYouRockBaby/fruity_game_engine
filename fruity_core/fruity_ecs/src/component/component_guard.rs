use crate::component::component::Component;
use crate::entity::entity_guard::EntityReadGuard;
use crate::entity::entity_guard::EntityWriteGuard;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::Deref;
use std::ops::DerefMut;

/// RAII structure used to release the shared read access of a lock when dropped.
///
/// This structure is created by the [`read`] methods on [`ComponentReference`].
///
/// [`read`]: ComponentReference::read
///
pub struct ComponentReadGuard<'a> {
    pub(crate) entity_reader: EntityReadGuard<'a>,
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
        self.entity_reader
            .read_components(&self.component_identifier)
            .remove(self.component_index)
    }
}

/// RAII structure used to release the shared write access of a lock when dropped.
///
/// This structure is created by the [`write`] methods on [`ComponentReference`].
///
/// [`write`]: ComponentReference::write
///
pub struct ComponentWriteGuard<'a> {
    pub(crate) entity_writer: EntityWriteGuard<'a>,
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
        self.entity_writer
            .read_components(&self.component_identifier)
            .remove(self.component_index)
    }
}

impl<'a> DerefMut for ComponentWriteGuard<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.entity_writer
            .write_components(&self.component_identifier)
            .remove(self.component_index)
    }
}

/// RAII structure used to release the shared read access of a lock when dropped.
///
/// This structure is created by the [`read`] methods on [`ComponentReference`].
///
/// [`read`]: ComponentReference::read
///
pub struct TypedComponentReadGuard<'a, T: Component> {
    pub(crate) entity_reader: EntityReadGuard<'a>,
    pub(crate) component_identifier: String,
    pub(crate) component_index: usize,
    pub(crate) phantom: PhantomData<T>,
}

impl<'a, T: Component> Debug for TypedComponentReadGuard<'a, T> {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl<'a, T: Component> Deref for TypedComponentReadGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let component = self
            .entity_reader
            .read_components(&self.component_identifier)
            .remove(self.component_index);

        component.as_any_ref().downcast_ref::<T>().unwrap()
    }
}

/// RAII structure used to release the shared write access of a lock when dropped.
///
/// This structure is created by the [`write`] methods on [`ComponentReference`].
///
/// [`write`]: ComponentReference::write
///
pub struct TypedComponentWriteGuard<'a, T: Component> {
    pub(crate) entity_writer: EntityWriteGuard<'a>,
    pub(crate) component_identifier: String,
    pub(crate) component_index: usize,
    pub(crate) phantom: PhantomData<T>,
}

impl<'a, T: Component> Debug for TypedComponentWriteGuard<'a, T> {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl<'a, T: Component> Deref for TypedComponentWriteGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let component = self
            .entity_writer
            .read_components(&self.component_identifier)
            .remove(self.component_index);

        component.as_any_ref().downcast_ref::<T>().unwrap()
    }
}

impl<'a, T: Component> DerefMut for TypedComponentWriteGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let component = self
            .entity_writer
            .write_components(&self.component_identifier)
            .remove(self.component_index);

        component.as_any_mut().downcast_mut::<T>().unwrap()
    }
}
