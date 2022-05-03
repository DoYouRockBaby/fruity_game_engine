use crate::component::component::AnyComponent;
use crate::component::component::Component;
use crate::entity::archetype::component_collection::ComponentCollection;
use crate::entity::archetype::entity_properties::EntityProperties;
use crate::entity::entity::get_type_identifier_by_any;
use crate::entity::entity::EntityId;
use crate::entity::entity::EntityTypeIdentifier;
use crate::entity::entity_reference::EntityReference;
use itertools::Itertools;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

/// This store all the information that are common accross all entities
pub mod entity_properties;

/// An array of component
pub mod component_array;

/// An interface that should be implemented by collection of components used into archetypes
pub mod component_collection;

pub(crate) struct InnerArchetype {
    pub(crate) entity_id_array: RwLock<Vec<EntityId>>,
    pub(crate) name_array: RwLock<Vec<String>>,
    pub(crate) enabled_array: RwLock<Vec<bool>>,
    pub(crate) lock_array: RwLock<Vec<RwLock<()>>>,
    pub(crate) component_collections: BTreeMap<String, Arc<RwLock<Box<dyn ComponentCollection>>>>,
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
        let lock_array = RwLock::new(vec![RwLock::new(())]);

        let grouped_components = Self::group_components_by_type(components);
        let mut component_collections = BTreeMap::new();
        for (class_name, components) in grouped_components {
            let first_component = components.get(0).unwrap();
            let mut collection = first_component.get_collection(components.len());
            collection.add(components);

            component_collections.insert(class_name, Arc::new(RwLock::new(collection)));
        }

        Archetype {
            identifier: identifier,
            inner: Arc::new(InnerArchetype {
                entity_id_array,
                name_array,
                enabled_array,
                lock_array,
                component_collections,
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
        let mut lock_array = self.inner.lock_array.write().unwrap();

        entity_id_array.push(entity_id);
        name_array.push(name.to_string());
        enabled_array.push(enabled);
        lock_array.push(RwLock::new(()));

        // Store all the components
        let grouped_components = Self::group_components_by_type(components);
        for (class_name, components) in grouped_components {
            let component_array = self.inner.component_collections.get(&class_name);
            if let Some(component_array) = component_array {
                let mut component_array = component_array.write().unwrap();
                component_array.add(components);
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
        let mut lock_array = self.inner.lock_array.write().unwrap();

        let entity_id = entity_id_array.remove(index);
        let name = name_array.remove(index);
        let enabled = enabled_array.remove(index);
        let lock = lock_array.remove(index);
        let _write_guard = lock.write().unwrap();

        // Remove the entity components from the storage
        let components = {
            self.inner
                .component_collections
                .iter()
                .map(|(_, component_array)| {
                    let mut component_array = component_array.write().unwrap();
                    component_array.remove(index)
                })
                .flatten()
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

    fn group_components_by_type(
        components: Vec<AnyComponent>,
    ) -> HashMap<String, Vec<AnyComponent>> {
        components
            .into_iter()
            .group_by(|component| component.get_class_name())
            .into_iter()
            .map(|(class_name, component)| (class_name, component.collect::<Vec<_>>()))
            .collect::<HashMap<_, _>>()
    }
}
