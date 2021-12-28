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

    /// Read a specific component
    ///
    /// # Arguments
    /// * `component_identifier` - The component identifier
    ///
    pub fn read_component(&self, component_identifier: &str) -> Option<&dyn Component> {
        let component_array = self
            .inner_archetype
            .component_arrays
            .get(component_identifier)?;
        let component_array = component_array.read().unwrap();

        // TODO: Try to find a way to remove that
        let component = unsafe {
            std::mem::transmute::<&dyn Component, &dyn Component>(component_array.get(&self.index))
        };

        Some(component)
    }

    /// Read a specific component
    ///
    /// # Arguments
    /// * `component_identifier` - The component identifier
    ///
    pub fn read_typed_component<T: Component>(&self, component_identifier: &str) -> Option<&T> {
        let component = self.read_component(component_identifier)?;
        component.as_any_ref().downcast_ref::<T>()
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

    /// Read a specific component
    ///
    /// # Arguments
    /// * `component_identifier` - The component identifier
    ///
    pub fn read_component(&self, component_identifier: &str) -> Option<&dyn Component> {
        let component_array = self
            .inner_archetype
            .component_arrays
            .get(component_identifier)?;
        let component_array = component_array.read().unwrap();

        // TODO: Try to find a way to remove that
        let component = unsafe {
            std::mem::transmute::<&dyn Component, &dyn Component>(component_array.get(&self.index))
        };

        Some(component)
    }

    /// Write a specific component
    ///
    /// # Arguments
    /// * `component_identifier` - The component identifier
    ///
    pub fn write_component(&self, component_identifier: &str) -> Option<&mut dyn Component> {
        let component = self.read_component(component_identifier)?;
        let component = unsafe {
            &mut *(component as *const dyn Component as *mut dyn Component) as &mut dyn Component
        };
        Some(component)
    }

    /// Read a specific component
    ///
    /// # Arguments
    /// * `component_identifier` - The component identifier
    ///
    pub fn read_typed_component<T: Component>(&self, component_identifier: &str) -> Option<&T> {
        let component = self.read_component(component_identifier)?;
        component.as_any_ref().downcast_ref::<T>()
    }

    /// Write a specific component
    ///
    /// # Arguments
    /// * `component_identifier` - The component identifier
    ///
    pub fn write_typed_component<T: Component>(
        &self,
        component_identifier: &str,
    ) -> Option<&mut T> {
        let component = self.write_component(component_identifier)?;
        component.as_any_mut().downcast_mut::<T>()
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
    }
}
