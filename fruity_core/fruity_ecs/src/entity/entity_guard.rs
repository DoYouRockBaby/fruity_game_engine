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
use std::sync::Arc;
use std::sync::RwLockReadGuard;
use std::sync::RwLockWriteGuard;

/// RAII structure used to release the shared read access of a lock when dropped.
///
/// This structure is created by the [`read`] methods on [`EntityRwLock`].
///
/// [`read`]: EntityRwLock::read
///
#[derive(Clone)]
pub struct EntityReadGuard<'a> {
    pub(crate) _guard: Rc<RwLockReadGuard<'a, ()>>,
    pub(crate) entity_id: usize,
    pub(crate) archetype: Arc<Archetype>,
}

impl<'a> Debug for EntityReadGuard<'a> {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl<'a> EntityReadGuard<'a> {
    /// Get the entity id
    pub fn get_entity_id(&self) -> EntityId {
        let entity_id_array = self.archetype.entity_id_array.read().unwrap();
        *entity_id_array.get(self.entity_id).unwrap()
    }

    /// Get the entity name
    pub fn get_name(&self) -> String {
        let name_array = self.archetype.name_array.read().unwrap();
        name_array.get(self.entity_id).unwrap().clone()
    }

    /// Is the entity enabled
    pub fn is_enabled(&self) -> bool {
        let enabled_array = self.archetype.enabled_array.read().unwrap();
        *enabled_array.get(self.entity_id).unwrap()
    }

    /// Read all components of the entity
    pub fn read_all_components(&self) -> Vec<ComponentReadGuard<'a>> {
        self.archetype
            .component_storages
            .iter()
            .map(|(_, storage)| {
                (0..storage.components_per_entity)
                    .map(|component_index| ComponentReadGuard {
                        _guard: InternalReadGuard::Read(self._guard.clone()),
                        collection: storage.collection.clone(),
                        component_index,
                    })
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect::<Vec<_>>()
    }

    /// Read components with a given type
    ///
    /// # Arguments
    /// * `component_identifier` - The component identifier
    ///
    pub fn read_components<T: Component + StaticComponent>(
        &self,
    ) -> Vec<TypedComponentReadGuard<'a, T>> {
        let component_identifier = T::get_component_name();

        self.read_components_from_type_identifier(&component_identifier)
            .into_iter()
            .map(|guard| guard.try_into())
            .filter_map(|guard| guard.ok())
            .collect::<Vec<_>>()
    }

    /// Read components with a given type
    ///
    /// # Arguments
    /// * `component_identifier` - The component identifier
    ///
    pub fn read_components_from_type_identifier(
        &self,
        component_identifier: &str,
    ) -> Vec<ComponentReadGuard<'a>> {
        match self
            .archetype
            .clone()
            .get_storage_from_type(component_identifier)
        {
            Some(storage) => (0..storage.components_per_entity)
                .map(|component_index| ComponentReadGuard {
                    _guard: InternalReadGuard::Read(self._guard.clone()),
                    collection: storage.collection.clone(),
                    component_index,
                })
                .collect::<Vec<_>>(),
            None => vec![],
        }
    }

    /// Read a single component with a given type
    pub fn read_single_component<T: Component + StaticComponent>(
        &self,
    ) -> Option<TypedComponentReadGuard<'a, T>> {
        let mut components = self.read_components();

        if components.len() > 0 {
            Some(components.remove(0))
        } else {
            None
        }
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
    pub(crate) _guard: Rc<RwLockWriteGuard<'a, ()>>,
    pub(crate) entity_id: usize,
    pub(crate) archetype: Arc<Archetype>,
}

impl<'a> Debug for EntityWriteGuard<'a> {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl<'a> EntityWriteGuard<'a> {
    /// Get the entity id
    pub fn get_entity_id(&self) -> EntityId {
        let entity_id_array = self.archetype.entity_id_array.read().unwrap();
        *entity_id_array.get(self.entity_id).unwrap()
    }

    /// Get the entity name
    pub fn get_name(&self) -> String {
        let name_array = self.archetype.name_array.read().unwrap();
        name_array.get(self.entity_id).unwrap().clone()
    }

    /// Set the entity name
    ///
    /// # Arguments
    /// * `value` - The name value
    ///
    pub fn set_name(&self, value: &str) {
        let mut name_array = self.archetype.name_array.write().unwrap();
        let name = name_array.get_mut(self.entity_id).unwrap();
        *name = value.to_string();
    }

    /// Is the entity enabled
    pub fn is_enabled(&self) -> bool {
        let enabled_array = self.archetype.enabled_array.read().unwrap();
        *enabled_array.get(self.entity_id).unwrap()
    }

    /// Set the entity enabled state
    ///
    /// # Arguments
    /// * `value` - Is the entity enabled
    ///
    pub fn set_enabled(&self, value: bool) {
        let mut enabled_array = self.archetype.enabled_array.write().unwrap();
        let enabled = enabled_array.get_mut(self.entity_id).unwrap();
        *enabled = value;
    }

    /// Read components with a given type
    ///
    /// # Arguments
    /// * `component_identifier` - The component identifier
    ///
    pub fn read_components<T: Component + StaticComponent>(
        &self,
    ) -> Vec<TypedComponentReadGuard<'a, T>> {
        let component_identifier = T::get_component_name();

        self.read_components_from_type_identifier(&component_identifier)
            .into_iter()
            .map(|guard| guard.try_into())
            .filter_map(|guard| guard.ok())
            .collect::<Vec<_>>()
    }

    /// Read components with a given type
    ///
    /// # Arguments
    /// * `component_identifier` - The component identifier
    ///
    pub fn read_components_from_type_identifier(
        &self,
        component_identifier: &str,
    ) -> Vec<ComponentReadGuard<'a>> {
        match self
            .archetype
            .clone()
            .get_storage_from_type(component_identifier)
        {
            Some(storage) => (0..storage.components_per_entity)
                .map(|component_index| ComponentReadGuard {
                    _guard: InternalReadGuard::Write(self._guard.clone()),
                    collection: storage.collection.clone(),
                    component_index,
                })
                .collect::<Vec<_>>(),
            None => vec![],
        }
    }

    /// Read a single component with a given type
    pub fn read_single_component<T: Component + StaticComponent>(
        &self,
    ) -> Option<TypedComponentReadGuard<'a, T>> {
        let mut components = self.read_components();

        if components.len() > 0 {
            Some(components.remove(0))
        } else {
            None
        }
    }

    /// Write components with a given type
    ///
    /// # Arguments
    /// * `component_identifier` - The component identifier
    ///
    pub fn write_components<T: Component + StaticComponent>(
        &self,
    ) -> Vec<TypedComponentWriteGuard<'a, T>> {
        let component_identifier = T::get_component_name();

        self.write_components_from_type_identifier(&component_identifier)
            .into_iter()
            .map(|guard| guard.try_into())
            .filter_map(|guard| guard.ok())
            .collect::<Vec<_>>()
    }

    /// Write components with a given type
    ///
    /// # Arguments
    /// * `component_identifier` - The component identifier
    ///
    pub fn write_components_from_type_identifier(
        &self,
        component_identifier: &str,
    ) -> Vec<ComponentWriteGuard<'a>> {
        match self
            .archetype
            .clone()
            .get_storage_from_type(component_identifier)
        {
            Some(storage) => (0..storage.components_per_entity)
                .map(|component_index| ComponentWriteGuard {
                    _guard: self._guard.clone(),
                    collection: storage.collection.clone(),
                    component_index,
                })
                .collect::<Vec<_>>(),
            None => vec![],
        }
    }

    /// Write a single component with a given type
    pub fn write_single_component<T: Component + StaticComponent>(
        &self,
    ) -> Option<TypedComponentWriteGuard<'a, T>> {
        let mut components = self.write_components();

        if components.len() > 0 {
            Some(components.remove(0))
        } else {
            None
        }
    }
}
