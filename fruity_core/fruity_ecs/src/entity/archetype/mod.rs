use crate::component::component::AnyComponent;
use crate::component::component::Component;
use crate::component::component::ComponentDecoder;
use crate::entity::archetype::component_array::ComponentArray;
use crate::entity::archetype::entity_properties::EntityProperties;
use crate::entity::entity::get_type_identifier_by_any;
use crate::entity::entity::EntityId;
use crate::entity::entity::EntityTypeIdentifier;
use crate::entity::entity_reference::EntityReference;
use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::RwLock;

/// This store all the information that are common accross all entities
pub mod entity_properties;

/// An array of component
pub mod component_array;

pub(crate) struct InnerArchetype {
    pub(crate) entity_id_array: RwLock<Vec<EntityId>>,
    pub(crate) name_array: RwLock<Vec<String>>,
    pub(crate) enabled_array: RwLock<Vec<bool>>,
    pub(crate) component_arrays: BTreeMap<String, Arc<RwLock<ComponentArray>>>,
}

/// A collection of entities that share the same component structure
/// Stored as a Struct Of Array
pub struct Archetype {
    identifier: EntityTypeIdentifier,
    inner: Arc<InnerArchetype>,
}

impl Archetype {
    /// Returns an Archetype and inject the first entity inside
    ///
    /// # Arguments
    /// * `entity_id` - The first entity id
    /// * `name` - The first entity name
    /// * `components` - The first entity components
    ///
    pub fn new(
        entity_id: EntityId,
        name: &str,
        enabled: bool,
        components: Vec<AnyComponent>,
    ) -> Archetype {
        // Deduce the archetype properties from the first components
        let identifier = get_type_identifier_by_any(&components);

        // Build the archetype containers with the first component
        let entity_id_array = RwLock::new(vec![entity_id]);
        let name_array = RwLock::new(vec![name.to_string()]);
        let enabled_array = RwLock::new(vec![enabled]);

        let mut component_arrays = BTreeMap::new();
        for component in components {
            component_arrays.insert(
                component.get_class_name(),
                Arc::new(RwLock::new(ComponentArray::new(component))),
            );
        }

        Archetype {
            identifier: identifier,
            inner: Arc::new(InnerArchetype {
                entity_id_array,
                name_array,
                enabled_array,
                component_arrays,
            }),
        }
    }

    /// Returns the entity type identifier of the archetype
    pub fn get_type_identifier(&self) -> &EntityTypeIdentifier {
        &self.identifier
    }

    /// Get components from an entity by index in the archetype storage
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    /// * `identifier` - The components identifiers
    ///
    pub fn get(&self, index: usize) -> EntityReference {
        EntityReference {
            index,
            inner_archetype: self.inner.clone(),
        }
    }

    /// Get an iterator over all the components of all the entities
    pub fn iter(&self) -> impl Iterator<Item = EntityReference> {
        let inner = self.inner.clone();
        let inner2 = self.inner.clone();

        (0..self.len())
            .filter(move |index| {
                let enabled_array = inner.enabled_array.read().unwrap();
                *enabled_array.get(*index).unwrap()
            })
            .map(move |index| EntityReference {
                index,
                inner_archetype: inner2.clone(),
            })
    }

    /// Get entity count
    pub fn len(&self) -> usize {
        let entity_id_array = self.inner.entity_id_array.read().unwrap();
        entity_id_array.len()
    }

    /// Add an entity into the archetype
    ///
    /// # Arguments
    /// * `entity_id` - The first entity id
    /// * `name` - The first entity name
    /// * `components` - The first entity components
    ///
    pub fn add(
        &self,
        entity_id: EntityId,
        name: &str,
        enabled: bool,
        components: Vec<AnyComponent>,
    ) {
        // Store the entity properties
        let mut entity_id_array = self.inner.entity_id_array.write().unwrap();
        let mut name_array = self.inner.name_array.write().unwrap();
        let mut enabled_array = self.inner.enabled_array.write().unwrap();

        entity_id_array.push(entity_id);
        name_array.push(name.to_string());
        enabled_array.push(enabled);

        // Store all the components
        for component in components {
            let component_array = self.inner.component_arrays.get(&component.get_class_name());
            if let Some(component_array) = component_array {
                let mut component_array = component_array.write().unwrap();
                component_array.add(component);
            }
        }
    }

    /// Remove an entity based on its id
    ///
    /// # Arguments
    /// * `index` - The entity index
    ///
    pub fn remove(&self, index: usize) -> (EntityProperties, Vec<AnyComponent>) {
        // Remove the entity properties from the storage
        let mut entity_id_array = self.inner.entity_id_array.write().unwrap();
        let mut name_array = self.inner.name_array.write().unwrap();
        let mut enabled_array = self.inner.enabled_array.write().unwrap();

        let entity_id = entity_id_array.remove(index);
        let name = name_array.remove(index);
        let enabled = enabled_array.remove(index);

        // Remove the entity components from the storage
        let components = {
            self.inner
                .component_arrays
                .iter()
                .map(|(_, component_array)| {
                    let mut component_array = component_array.write().unwrap();
                    component_array.remove(index)
                })
                .collect::<Vec<_>>()
        };

        // Return the deleted components
        (
            EntityProperties {
                entity_id,
                name,
                enabled,
            },
            components,
        )
    }
}
