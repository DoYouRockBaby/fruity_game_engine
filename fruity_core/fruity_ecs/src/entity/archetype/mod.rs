use crate::component::component::AnyComponent;
use crate::component::component::Component;
use crate::component::component_reference::ComponentReference;
use crate::entity::archetype::component_storage::ComponentStorage;
use crate::entity::archetype::entity_properties::EntityProperties;
use crate::entity::entity::get_type_identifier_by_any;
use crate::entity::entity::EntityId;
use crate::entity::entity::EntityTypeIdentifier;
use crate::entity::entity_reference::EntityReference;
use crate::ExtensionComponentService;
use fruity_core::resource::resource_reference::ResourceReference;
use fruity_core::RwLock;
use itertools::Itertools;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;

/// This store all the information that are common accross all entities
pub mod entity_properties;

/// An array of component
pub mod component_array;

/// Provides a collection that can store components by taking care of the number of component per entity
pub mod component_storage;

/// An interface that should be implemented by collection of components used into archetypes
pub mod component_collection;

#[derive(Clone)]
pub(crate) struct ArchetypeArcRwLock(Arc<RwLock<Archetype>>);

/// A collection of entities that share the same component structure
/// Stored as a Struct Of Array
pub struct Archetype {
    extension_component_service: ResourceReference<ExtensionComponentService>,
    pub(crate) identifier: EntityTypeIdentifier,

    // Indexes with dead memory
    pub(crate) erased_indexes: RwLock<Vec<usize>>,

    // Store all the component properties into a index persisting storage
    pub(crate) entity_id_array: Vec<EntityId>,
    pub(crate) name_array: Vec<String>,
    pub(crate) enabled_array: Vec<bool>,
    pub(crate) lock_array: Vec<RwLock<()>>,
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
        extension_component_service: ResourceReference<ExtensionComponentService>,
        entity_id: EntityId,
        name: &str,
        enabled: bool,
        mut components: Vec<AnyComponent>,
    ) -> Archetype {
        // Deduce the archetype properties from the first components
        let identifier = get_type_identifier_by_any(&components);

        // Inject the extensions
        let mut extensions_component = {
            let extension_component_service = extension_component_service.read();

            components
                .iter()
                .map(|component| {
                    extension_component_service
                        .get_component_extension(component.deref())
                        .into_iter()
                })
                .flatten()
                .collect::<Vec<_>>()
        };
        components.append(&mut extensions_component);

        // Build the archetype component containers
        let grouped_components = Self::group_components_by_type(components);
        let mut component_storages = BTreeMap::new();
        for (class_name, components) in grouped_components {
            component_storages.insert(class_name, ComponentStorage::new(components));
        }

        Archetype {
            extension_component_service,
            identifier: identifier,
            erased_indexes: RwLock::new(vec![]),
            entity_id_array: vec![entity_id],
            name_array: vec![name.to_string()],
            enabled_array: vec![enabled],
            lock_array: vec![RwLock::new(())],
            component_storages,
        }
    }

    /// Returns the entity type identifier of the archetype
    pub fn get_type_identifier(&self) -> &EntityTypeIdentifier {
        &self.identifier
    }

    /// Get components of a specified type from an entity by index
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    /// * `component_identifier` - The components type identifier
    ///
    pub(crate) fn get_storage_from_type(
        &self,
        component_type_identifier: &str,
    ) -> Option<&ComponentStorage> {
        self.component_storages.get(component_type_identifier)
    }

    /// Get entity count
    pub fn len(&self) -> usize {
        self.entity_id_array.len()
    }

    /// Add an entity into the archetype
    ///
    /// # Arguments
    /// * `entity_id` - The first entity id
    /// * `name` - The first entity name
    /// * `components` - The first entity components
    ///
    pub fn add(
        &mut self,
        entity_id: EntityId,
        name: &str,
        enabled: bool,
        mut components: Vec<AnyComponent>,
    ) {
        // TODO: Use previous deleted cells

        // Store the entity properties
        self.entity_id_array.push(entity_id);
        self.name_array.push(name.to_string());
        self.enabled_array.push(enabled);
        self.lock_array.push(RwLock::new(()));

        // Inject the extensions
        let mut extensions_component = {
            let extension_component_service = self.extension_component_service.read();

            components
                .iter()
                .map(|component| {
                    extension_component_service
                        .get_component_extension(component.deref())
                        .into_iter()
                })
                .flatten()
                .collect::<Vec<_>>()
        };
        components.append(&mut extensions_component);

        // Store all the components
        let grouped_components = Self::group_components_by_type(components);
        for (class_name, components) in grouped_components {
            let component_array = self.component_storages.get_mut(&class_name);
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
        // Get the write lock over the entity
        // TODO: Decide to keep it or not
        // let lock = self.lock_array.get(index);
        // let _write_guard = lock.write();

        // Mark the index as deleted
        {
            let mut erased_indexes_writer = self.erased_indexes.write();
            erased_indexes_writer.push(index);
        }

        // Get the entity properties from the storage
        let entity_id = *self.entity_id_array.get(index).unwrap();
        let name = self.name_array.get(index).unwrap().clone();
        let enabled = *self.enabled_array.get(index).unwrap();

        // Get the entity components from the storage
        let components = {
            self.component_storages
                .iter()
                .map(|(_, storage)| storage.get(index))
                .flatten()
                .map(|component| AnyComponent::from_box(component.duplicate()))
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

impl ArchetypeArcRwLock {
    /// Returns an ArchetypeArcRwLock
    pub fn new(archetype: Archetype) -> Self {
        Self(Arc::new(RwLock::new(archetype)))
    }

    /// Get a reference to an entity by index
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    ///
    pub fn get(&self, entity_id: usize) -> EntityReference {
        EntityReference {
            entity_id,
            archetype: self.clone(),
        }
    }

    /// Get components of a specified type from an entity by index
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    /// * `component_identifier` - The components type identifier
    ///
    pub fn get_entity_components_from_type(
        &self,
        entity_id: usize,
        component_identifier: &str,
    ) -> Vec<ComponentReference> {
        let archetype_reader = self.0.read();

        archetype_reader
            .component_storages
            .get(component_identifier)
            .map(|storage| {
                let start_index = entity_id * storage.components_per_entity;
                let end_index = start_index + storage.components_per_entity;

                (start_index..end_index)
                    .into_iter()
                    .map(move |index| ComponentReference {
                        entity_reference: EntityReference {
                            entity_id,
                            archetype: self.clone(),
                        },
                        component_identifier: component_identifier.to_string(),
                        component_index: index,
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default()
    }

    /// Get an iterator over all the components of all the entities
    pub fn iter(&self, ignore_enabled: bool) -> impl Iterator<Item = EntityReference> + '_ {
        let archetype_len = self.0.read().len();

        (0..archetype_len)
            .filter(move |entity_id| {
                let is_deleted = {
                    let archetype_reader = self.0.read();
                    let erased_indexes_reader = archetype_reader.erased_indexes.read();
                    erased_indexes_reader.contains(entity_id)
                };

                if is_deleted {
                    return false;
                }

                if !ignore_enabled {
                    // TODO: Try yo move it outside
                    let archetype_reader = self.0.read();
                    *archetype_reader.enabled_array.get(*entity_id).unwrap()
                } else {
                    true
                }
            })
            .map(move |entity_id| EntityReference {
                entity_id,
                archetype: self.clone(),
            })
    }

    /// Get components from an entity by index
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    ///
    pub fn get_entity_components(&self, entity_id: usize) -> Vec<ComponentReference> {
        let archetype_reader = self.0.read();

        archetype_reader
            .component_storages
            .iter()
            .map(|(component_identifier, storage)| {
                let start_index = entity_id * storage.components_per_entity;
                let end_index = start_index + storage.components_per_entity;

                (start_index..end_index)
                    .into_iter()
                    .map(move |index| ComponentReference {
                        entity_reference: EntityReference {
                            entity_id,
                            archetype: self.clone(),
                        },
                        component_identifier: component_identifier.clone(),
                        component_index: index,
                    })
            })
            .flatten()
            .collect::<Vec<_>>()
    }
}

impl Deref for ArchetypeArcRwLock {
    type Target = Arc<RwLock<Archetype>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
