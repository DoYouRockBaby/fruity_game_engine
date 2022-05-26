use crate::component::component::Component;
use crate::component::component::StaticComponent;
use crate::component::component_guard::ComponentReadGuard;
use crate::component::component_guard::ComponentWriteGuard;
use crate::component::component_guard::InternalReadGuard;
use crate::component::component_guard::TypedComponentReadGuard;
use crate::component::component_guard::TypedComponentWriteGuard;
use crate::entity::archetype::Archetype;
use crate::entity::entity::EntityId;
use std::fmt::Debug;
use std::rc::Rc;
use fruity_core::RwLockReadGuard;
use fruity_core::RwLockWriteGuard;

/// RAII structure used to release the shared read access of a lock when dropped.
///
/// This structure is created by the [`read`] methods on [`EntityRwLock`].
///
/// [`read`]: EntityRwLock::read
///
#[derive(Clone)]
pub struct EntityReadGuard<'a> {
    pub(crate) _guard: Rc<RwLockReadGuard<'a, ()>>,
    pub(crate) archetype_reader: Rc<RwLockReadGuard<'a, Archetype>>,
    pub(crate) entity_id: usize,
}

impl<'a> Debug for EntityReadGuard<'a> {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl<'a> EntityReadGuard<'a> {
    /// Get the entity id
    pub fn get_entity_id(&self) -> EntityId {
        *self
            .archetype_reader
            .entity_id_array
            .get(self.entity_id)
            .unwrap()
    }

    /// Get the entity name
    pub fn get_name(&self) -> String {
        self.archetype_reader
            .name_array
            .get(self.entity_id)
            .map(|name| name.clone())
            .unwrap()
    }

    /// Is the entity enabled
    pub fn is_enabled(&self) -> bool {
        *self
            .archetype_reader
            .enabled_array
            .get(self.entity_id)
            .unwrap()
    }

    /// Read all components of the entity
    pub fn read_all_components(&self) -> impl Iterator<Item = ComponentReadGuard<'_>> {
        self.archetype_reader
            .component_storages
            .iter()
            .map(|(component_identifier, storage)| {
                let start_index = self.entity_id * storage.components_per_entity;
                let end_index = start_index + storage.components_per_entity;

                (start_index..end_index).map(|component_index| ComponentReadGuard {
                    _guard: InternalReadGuard::Read(self._guard.clone()),
                    archetype_reader: self.archetype_reader.clone(),
                    component_identifier: component_identifier.clone(),
                    component_index,
                })
            })
            .flatten()
    }

    /// Read components with a given type
    ///
    /// # Arguments
    /// * `component_identifier` - The component identifier
    ///
    pub fn iter_components<T: Component + StaticComponent>(
        &self,
    ) -> impl Iterator<Item = TypedComponentReadGuard<'_, T>> {
        self.iter_components_from_type_identifier(T::get_component_name())
            .into_iter()
            .map(|guard| guard.try_into())
            .filter_map(|guard| guard.ok())
    }

    /// Read components with a given type
    ///
    /// # Arguments
    /// * `component_identifier` - The component identifier
    ///
    pub fn iter_components_from_type_identifier(
        &self,
        component_identifier: &str,
    ) -> Box<dyn Iterator<Item = ComponentReadGuard<'_>> + '_> {
        match self
            .archetype_reader
            .get_storage_from_type(component_identifier)
        {
            Some(storage) => {
                let start_index = self.entity_id * storage.components_per_entity;
                let end_index = start_index + storage.components_per_entity;
                let component_identifier = component_identifier.to_string();

                Box::new(
                    (start_index..end_index).map(move |component_index| ComponentReadGuard {
                        _guard: InternalReadGuard::Read(self._guard.clone()),
                        archetype_reader: self.archetype_reader.clone(),
                        component_identifier: component_identifier.clone(),
                        component_index,
                    }),
                )
            }
            None => Box::new(std::iter::empty()),
        }
    }

    /// Read a single component with a given type
    pub fn read_single_component<T: Component + StaticComponent>(
        &self,
    ) -> Option<TypedComponentReadGuard<'_, T>> {
        self.iter_components().next()
    }
}

/// RAII structure used to release the exclusive write access of a lock when dropped.
///
/// This structure is created by the [`write`] methods on [`EntityRwLock`].
///
/// [`write`]: EntityRwLock::write
///
#[derive(Clone)]
pub struct EntityWriteGuard<'a> {
    pub(crate) entity_id: usize,
    pub(crate) _guard: Rc<RwLockWriteGuard<'a, ()>>,
    pub(crate) archetype_reader: Rc<RwLockReadGuard<'a, Archetype>>,
}

impl<'a> Debug for EntityWriteGuard<'a> {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl<'a> EntityWriteGuard<'a> {
    /// Get the entity id
    pub fn get_entity_id(&self) -> EntityId {
        *self
            .archetype_reader
            .entity_id_array
            .get(self.entity_id)
            .unwrap()
    }

    /// Get the entity name
    pub fn get_name(&self) -> String {
        self.archetype_reader
            .name_array
            .get(self.entity_id)
            .map(|name| name.clone())
            .unwrap()
    }

    /// Set the entity name
    ///
    /// # Arguments
    /// * `value` - The name value
    ///
    pub fn set_name(&self, value: &str) {
        let name = self
            .archetype_reader
            .name_array
            .get(self.entity_id)
            .unwrap();

        // Safe cause it is protected by self._guard
        #[allow(mutable_transmutes)]
        let name = unsafe { std::mem::transmute::<&String, &mut String>(name) };

        *name = value.to_string();
    }

    /// Is the entity enabled
    pub fn is_enabled(&self) -> bool {
        *self
            .archetype_reader
            .enabled_array
            .get(self.entity_id)
            .unwrap()
    }

    /// Set the entity enabled state
    ///
    /// # Arguments
    /// * `value` - Is the entity enabled
    ///
    pub fn set_enabled(&self, value: bool) {
        let enabled = self
            .archetype_reader
            .enabled_array
            .get(self.entity_id)
            .unwrap();

        // Safe cause it is protected by self._guard
        #[allow(mutable_transmutes)]
        let enabled = unsafe { std::mem::transmute::<&bool, &mut bool>(enabled) };

        *enabled = value;
    }

    /// Read components with a given type
    ///
    /// # Arguments
    /// * `component_identifier` - The component identifier
    ///
    pub fn iter_components<T: Component + StaticComponent>(
        &self,
    ) -> impl Iterator<Item = TypedComponentReadGuard<'_, T>> {
        self.iter_components_from_type_identifier(T::get_component_name())
            .into_iter()
            .map(|guard| guard.try_into())
            .filter_map(|guard| guard.ok())
    }

    /// Read components with a given type
    ///
    /// # Arguments
    /// * `component_identifier` - The component identifier
    ///
    pub fn iter_components_from_type_identifier(
        &self,
        component_identifier: &str,
    ) -> Box<dyn Iterator<Item = ComponentReadGuard<'_>> + '_> {
        match self
            .archetype_reader
            .get_storage_from_type(component_identifier)
        {
            Some(storage) => {
                let start_index = self.entity_id * storage.components_per_entity;
                let end_index = start_index + storage.components_per_entity;
                let component_identifier = component_identifier.to_string();

                Box::new(
                    (start_index..end_index).map(move |component_index| ComponentReadGuard {
                        _guard: InternalReadGuard::Write(self._guard.clone()),
                        archetype_reader: self.archetype_reader.clone(),
                        component_identifier: component_identifier.clone(),
                        component_index,
                    }),
                )
            }
            None => Box::new(std::iter::empty()),
        }
    }

    /// Read a single component with a given type
    pub fn read_single_component<T: Component + StaticComponent>(
        &self,
    ) -> Option<TypedComponentReadGuard<'_, T>> {
        self.iter_components().next()
    }

    /// Write components with a given type
    ///
    /// # Arguments
    /// * `component_identifier` - The component identifier
    ///
    pub fn iter_components_mut<T: Component + StaticComponent>(
        &self,
    ) -> impl Iterator<Item = TypedComponentWriteGuard<'_, T>> {
        self.iter_components_from_type_identifier_mut(T::get_component_name())
            .into_iter()
            .map(|guard| guard.try_into())
            .filter_map(|guard| guard.ok())
    }

    /// Write components with a given type
    ///
    /// # Arguments
    /// * `component_identifier` - The component identifier
    ///
    pub fn iter_components_from_type_identifier_mut(
        &self,
        component_identifier: &str,
    ) -> Box<dyn Iterator<Item = ComponentWriteGuard<'_>> + '_> {
        match self
            .archetype_reader
            .get_storage_from_type(component_identifier)
        {
            Some(storage) => {
                let start_index = self.entity_id * storage.components_per_entity;
                let end_index = start_index + storage.components_per_entity;
                let component_identifier = component_identifier.to_string();

                Box::new(
                    (start_index..end_index).map(move |component_index| ComponentWriteGuard {
                        _guard: self._guard.clone(),
                        archetype_reader: self.archetype_reader.clone(),
                        component_identifier: component_identifier.clone(),
                        component_index,
                    }),
                )
            }
            None => Box::new(std::iter::empty()),
        }
    }

    /// Write a single component with a given type
    pub fn write_single_component<T: Component + StaticComponent>(
        &self,
    ) -> Option<TypedComponentWriteGuard<'_, T>> {
        self.iter_components_mut().next()
    }
}
