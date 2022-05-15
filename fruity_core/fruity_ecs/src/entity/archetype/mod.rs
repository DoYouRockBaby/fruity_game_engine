use crate::component::component::AnyComponent;
use crate::component::component::Component;
use crate::component::component_reference::ComponentReference;
use crate::entity::archetype::component_storage::ComponentStorage;
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

/// Provides a collection that can store components by taking care of the number of component per entity
pub mod component_storage;

/// An interface that should be implemented by collection of components used into archetypes
pub mod component_collection;

/// A collection of entities that share the same component structure
/// Stored as a Struct Of Array
pub struct Archetype {
    pub(crate) identifier: EntityTypeIdentifier,

    // Store all the component properties into a index persisting storage
    pub(crate) entity_id_array: RwLock<Vec<EntityId>>,
    pub(crate) name_array: RwLock<Vec<String>>,
    pub(crate) enabled_array: RwLock<Vec<bool>>,
    pub(crate) lock_array: RwLock<Vec<RwLock<()>>>,
    pub(crate) component_storages: BTreeMap<String, ComponentStorage>,
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
        let mut component_storages = BTreeMap::new();
        for (class_name, components) in grouped_components {
            component_storages.insert(class_name, ComponentStorage::new(components));
        }

        Archetype {
            identifier: identifier,
            entity_id_array,
            name_array,
            enabled_array,
            lock_array,
            component_storages,
        }
    }

    /// Returns the entity type identifier of the archetype
    pub fn get_type_identifier(&self) -> &EntityTypeIdentifier {
        &self.identifier
    }

    /// Get a reference to an entity by index
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    ///
    pub fn get(self: Arc<Self>, entity_id: usize) -> EntityReference {
        EntityReference {
            entity_id,
            archetype: self.clone(),
        }
    }

    /// Get components from an entity by index
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    ///
    pub fn get_entity_components(self: Arc<Self>, entity_id: usize) -> Vec<ComponentReference> {
        self.component_storages
            .iter()
            .map(|(_, storage)| {
                storage.get_entity_components(EntityReference {
                    entity_id,
                    archetype: self.clone(),
                })
            })
            .flatten()
            .collect::<Vec<_>>()
    }

    /// Get components of a specified type from an entity by index
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    /// * `component_identifier` - The components type identifier
    ///
    pub fn get_entity_components_from_type(
        self: Arc<Self>,
        entity_id: usize,
        component_type_identifier: &str,
    ) -> Vec<ComponentReference> {
        self.component_storages
            .get(component_type_identifier)
            .map(|storage| {
                storage
                    .get_entity_components(EntityReference {
                        entity_id,
                        archetype: self.clone(),
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default()
    }

    /// Get components of a specified type from an entity by index
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    /// * `component_identifier` - The components type identifier
    ///
    pub(crate) fn get_storage_from_type(
        self: Arc<Self>,
        component_type_identifier: &str,
    ) -> Option<ComponentStorage> {
        self.component_storages
            .get(component_type_identifier)
            .map(|storage| storage.clone())
    }

    /// Get an iterator over all the components of all the entities
    pub fn iter(self: Arc<Self>) -> impl Iterator<Item = EntityReference> {
        let inner = self.clone();
        let inner2 = self.clone();

        (0..self.len())
            .filter(move |entity_id| {
                let enabled_array = inner.enabled_array.read().unwrap();
                *enabled_array.get(*entity_id).unwrap()
            })
            .map(move |entity_id| EntityReference {
                entity_id,
                archetype: inner2.clone(),
            })
    }

    /// Get entity count
    pub fn len(&self) -> usize {
        let entity_id_array = self.entity_id_array.read().unwrap();
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
        let mut entity_id_array = self.entity_id_array.write().unwrap();
        let mut name_array = self.name_array.write().unwrap();
        let mut enabled_array = self.enabled_array.write().unwrap();
        let mut lock_array = self.lock_array.write().unwrap();

        entity_id_array.push(entity_id);
        name_array.push(name.to_string());
        enabled_array.push(enabled);
        lock_array.push(RwLock::new(()));

        // Store all the components
        let grouped_components = Self::group_components_by_type(components);
        for (class_name, components) in grouped_components {
            let component_array = self.component_storages.get(&class_name);
            if let Some(component_array) = component_array {
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
        let mut entity_id_array = self.entity_id_array.write().unwrap();
        let mut name_array = self.name_array.write().unwrap();
        let mut enabled_array = self.enabled_array.write().unwrap();
        let mut lock_array = self.lock_array.write().unwrap();

        let entity_id = entity_id_array.remove(index);
        let name = name_array.remove(index);
        let enabled = enabled_array.remove(index);
        let lock = lock_array.remove(index);
        let _write_guard = lock.write().unwrap();

        // Remove the entity components from the storage
        let components = {
            self.component_storages
                .iter()
                .map(|(_, storage)| storage.remove(index))
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
