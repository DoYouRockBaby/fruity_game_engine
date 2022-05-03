use crate::component::component::Component;
use crate::entity::archetype::component_array::ComponentArray;
use crate::entity::archetype::InnerArchetype;
use crate::entity::entity::EntityId;
use std::fmt::Debug;
use std::ops::Deref;
use std::sync::Arc;
use std::sync::RwLockReadGuard;
use std::sync::RwLockWriteGuard;

/// RAII structure used to release the shared read access of a lock when dropped.
///
/// This structure is created by the [`read`] methods on [`EntityRwLock`].
///
/// [`read`]: EntityRwLock::read
///
pub struct EntityReadGuard<'a> {
    pub(crate) _guard: RwLockReadGuard<'a, ()>,
    pub(crate) index: usize,
    pub(crate) inner_archetype: Arc<InnerArchetype>,
}

impl<'a> Debug for EntityReadGuard<'a> {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl<'a> EntityReadGuard<'a> {
    /// Get the entity id
    pub fn get_entity_id(&self) -> EntityId {
        let entity_id_array = self.inner_archetype.entity_id_array.read().unwrap();
        *entity_id_array.get(self.index).unwrap()
    }

    /// Get the entity name
    pub fn get_name(&self) -> String {
        let name_array = self.inner_archetype.name_array.read().unwrap();
        name_array.get(self.index).unwrap().clone()
    }

    /// Is the entity enabled
    pub fn is_enabled(&self) -> bool {
        let enabled_array = self.inner_archetype.enabled_array.read().unwrap();
        *enabled_array.get(self.index).unwrap()
    }

    /// Read components with a given type
    ///
    /// # Arguments
    /// * `component_identifier` - The component identifier
    ///
    pub fn read_components(&self, component_identifier: &str) -> Vec<&dyn Component> {
        let component_array = if let Some(component_array) = self
            .inner_archetype
            .component_arrays
            .get(component_identifier)
        {
            component_array
        } else {
            return vec![];
        };

        let component_array = component_array.read().unwrap();

        component_array
            .get(&self.index)
            .into_iter()
            .map(|component| {
                // TODO: Try to find a way to remove that
                unsafe { std::mem::transmute::<&dyn Component, &dyn Component>(component) }
            })
            .collect::<Vec<_>>()
    }

    /// Read components with a given type
    ///
    /// # Arguments
    /// * `component_identifier` - The component identifier
    ///
    pub fn read_typed_components<T: Component>(&self, component_identifier: &str) -> Vec<&T> {
        self.read_components(component_identifier)
            .into_iter()
            .filter_map(|component| component.as_any_ref().downcast_ref::<T>())
            .collect::<Vec<_>>()
    }

    /// Read a single component with a given type
    ///
    /// # Arguments
    /// * `component_identifier` - The component identifier
    ///
    pub fn read_single_typed_component<T: Component>(
        &self,
        component_identifier: &str,
    ) -> Option<&T> {
        let mut components = self.read_typed_components(component_identifier);

        if components.len() > 0 {
            Some(components.remove(0))
        } else {
            None
        }
    }

    /// Iter over all components
    pub fn iter_all_components(&self) -> impl Iterator<Item = &dyn Component> + '_ {
        self.inner_archetype
            .component_arrays
            .iter()
            .map(|(_, components_array)| {
                let components_array = components_array.read().unwrap();

                // TODO: Find a way to remove it
                let components_array = unsafe {
                    std::mem::transmute::<&ComponentArray, &ComponentArray>(
                        components_array.deref(),
                    )
                };

                components_array.get(&self.index)
            })
            .flatten()
    }
}

/// RAII structure used to release the exclusive write access of a lock when dropped.
///
/// This structure is created by the [`write`] methods on [`EntityRwLock`].
///
/// [`write`]: EntityRwLock::write
///
pub struct EntityWriteGuard<'a> {
    pub(crate) _guard: RwLockWriteGuard<'a, ()>,
    pub(crate) index: usize,
    pub(crate) inner_archetype: Arc<InnerArchetype>,
}

impl<'a> Debug for EntityWriteGuard<'a> {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl<'a> EntityWriteGuard<'a> {
    /// Get the entity id
    pub fn get_entity_id(&self) -> EntityId {
        let entity_id_array = self.inner_archetype.entity_id_array.read().unwrap();
        *entity_id_array.get(self.index).unwrap()
    }

    /// Get the entity name
    pub fn get_name(&self) -> String {
        let name_array = self.inner_archetype.name_array.read().unwrap();
        name_array.get(self.index).unwrap().clone()
    }

    /// Set the entity name
    ///
    /// # Arguments
    /// * `value` - The name value
    ///
    pub fn set_name(&self, value: &str) {
        let mut name_array = self.inner_archetype.name_array.write().unwrap();
        let name = name_array.get_mut(self.index).unwrap();
        *name = value.to_string();
    }

    /// Is the entity enabled
    pub fn is_enabled(&self) -> bool {
        let enabled_array = self.inner_archetype.enabled_array.read().unwrap();
        *enabled_array.get(self.index).unwrap()
    }

    /// Set the entity enabled state
    ///
    /// # Arguments
    /// * `value` - Is the entity enabled
    ///
    pub fn set_enabled(&self, value: bool) {
        let mut enabled_array = self.inner_archetype.enabled_array.write().unwrap();
        let enabled = enabled_array.get_mut(self.index).unwrap();
        *enabled = value;
    }

    /// Read components with a given type
    ///
    /// # Arguments
    /// * `component_identifier` - The component identifier
    ///
    pub fn read_components(&self, component_identifier: &str) -> Vec<&dyn Component> {
        let component_array = if let Some(component_array) = self
            .inner_archetype
            .component_arrays
            .get(component_identifier)
        {
            component_array
        } else {
            return vec![];
        };

        let component_array = component_array.read().unwrap();

        component_array
            .get(&self.index)
            .into_iter()
            .map(|component| {
                // TODO: Try to find a way to remove that
                unsafe { std::mem::transmute::<&dyn Component, &dyn Component>(component) }
            })
            .collect::<Vec<_>>()
    }

    /// Write components with a given type
    ///
    /// # Arguments
    /// * `component_identifier` - The component identifier
    ///
    pub fn write_components(&self, component_identifier: &str) -> Vec<&mut dyn Component> {
        self.read_components(component_identifier)
            .into_iter()
            .map(|component| unsafe {
                &mut *(component as *const dyn Component as *mut dyn Component)
                    as &mut dyn Component
            })
            .collect::<Vec<_>>()
    }

    /// Read components with a given type
    ///
    /// # Arguments
    /// * `component_identifier` - The component identifier
    ///
    pub fn read_typed_components<T: Component>(&self, component_identifier: &str) -> Vec<&T> {
        self.read_components(component_identifier)
            .into_iter()
            .filter_map(|component| component.as_any_ref().downcast_ref::<T>())
            .collect::<Vec<_>>()
    }

    /// Read a single component with a given type
    ///
    /// # Arguments
    /// * `component_identifier` - The component identifier
    ///
    pub fn read_single_typed_component<T: Component>(
        &self,
        component_identifier: &str,
    ) -> Option<&T> {
        let mut components = self.read_typed_components(component_identifier);

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
    pub fn write_typed_components<T: Component>(&self, component_identifier: &str) -> Vec<&mut T> {
        self.write_components(component_identifier)
            .into_iter()
            .filter_map(|component| component.as_any_mut().downcast_mut::<T>())
            .collect::<Vec<_>>()
    }

    /// Write a single component with a given type
    ///
    /// # Arguments
    /// * `component_identifier` - The component identifier
    ///
    pub fn write_single_typed_component<T: Component>(
        &self,
        component_identifier: &str,
    ) -> Option<&mut T> {
        let mut components = self.write_typed_components(component_identifier);

        if components.len() > 0 {
            Some(components.remove(0))
        } else {
            None
        }
    }

    /// Iter over all components
    pub fn iter_all_components(&self) -> impl Iterator<Item = &dyn Component> + '_ {
        self.inner_archetype
            .component_arrays
            .iter()
            .map(|(_, components_array)| {
                let components_array = components_array.read().unwrap();

                // TODO: Find a way to remove it
                let components_array = unsafe {
                    std::mem::transmute::<&ComponentArray, &ComponentArray>(
                        components_array.deref(),
                    )
                };

                components_array.get(&self.index)
            })
            .flatten()
    }
}
