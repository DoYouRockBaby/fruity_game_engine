use crate::component::component::AnyComponent;
use crate::component::component::Component;
use crate::component::component::ComponentDecoder;
use crate::component::component_reference::ComponentReference;
use crate::entity::archetype::component_array::ComponentArray;
use crate::entity::archetype::entity::Entity;
use crate::entity::entity::get_type_identifier_by_any;
use crate::entity::entity::EntityId;
use crate::entity::entity::EntityTypeIdentifier;
use std::collections::HashMap;
use std::default::Default;
use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;

/// This store all the information that are common accross all entities
pub mod entity;

/// An array of component
pub mod component_array;

/// A collection of entities that share the same component structure
/// Stored as a Struct Of Array
pub struct Archetype {
    identifier: EntityTypeIdentifier,
    component_arrays: Arc<HashMap<String, Arc<RwLock<ComponentArray>>>>,
    removed_entities: Mutex<Vec<usize>>,
}

impl Archetype {
    /// Returns an Archetype and inject the first entity inside
    ///
    /// # Arguments
    /// * `entity_id` - The first entity id
    /// * `name` - The first entity name
    /// * `components` - The first entity components
    ///
    pub fn new(entity_id: EntityId, name: &str, mut components: Vec<AnyComponent>) -> Archetype {
        // Inject the common Entity component
        components.push(AnyComponent::new(Entity {
            entity_id,
            name: name.to_string(),
            ..Entity::default()
        }));

        // Deduce the archetype properties from the components
        let identifier = get_type_identifier_by_any(&components);

        // Build the archetype
        let mut component_arrays = HashMap::new();
        for component in components {
            component_arrays.insert(
                component.get_class_name(),
                Arc::new(RwLock::new(ComponentArray::new(component))),
            );
        }

        Archetype {
            identifier: identifier,
            component_arrays: Arc::new(component_arrays),
            removed_entities: Mutex::new(Vec::new()),
        }
    }

    /// Returns the entity type identifier of the archetype
    pub fn get_type_identifier(&self) -> &EntityTypeIdentifier {
        &self.identifier
    }

    /// Get an iterator over all the components of all the entities
    pub fn iter(
        &self,
        component_identifier: EntityTypeIdentifier,
    ) -> impl Iterator<Item = Vec<ComponentReference>> {
        let component_arrays = self.component_arrays.clone();
        let component_arrays_2 = self.component_arrays.clone();

        let entity_array = component_arrays
            .iter()
            .find(|(identifier, _)| *identifier == "Entity")
            .unwrap()
            .1
            .clone();

        (0..self.len())
            .filter(move |index| {
                let entity_array = entity_array.read().unwrap();
                let entity = entity_array.get(index);
                let entity = entity.read();
                let entity = entity.as_any_ref().downcast_ref::<Entity>().unwrap();

                entity.enabled && !entity.deleted
            })
            .map(move |index| {
                let component_arrays = component_arrays_2.clone();
                component_identifier
                    .0
                    .iter()
                    .filter_map(move |identifier| {
                        let component_array = component_arrays.get(identifier)?;
                        let component_array = component_array.read().unwrap();
                        Some(component_array.get(&index))
                    })
                    .collect::<Vec<_>>()
            })
    }

    /// Get an iterator over all the components of all the entities
    pub fn iter_all_components(&self) -> impl Iterator<Item = Vec<ComponentReference>> {
        // TODO: Find a way to remove that
        let this = unsafe { &*(self as *const _) } as &Archetype;
        (0..self.len()).map(move |index| this.get_components(index, this.identifier.clone()))
    }

    /// Get components from an entity by index in the archetype storage
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    /// * `identifier` - The components identifiers
    ///
    pub fn get_components(
        &self,
        index: usize,
        component_identifier: EntityTypeIdentifier,
    ) -> Vec<ComponentReference> {
        let component_arrays = self.component_arrays.clone();
        component_identifier
            .0
            .iter()
            .filter_map(|identifier| component_arrays.get(identifier))
            .map(|component_array| {
                let component_array = component_array.read().unwrap();
                component_array.get(&index)
            })
            .collect::<Vec<_>>()
    }

    /// Get all components from an entity by index in the archetype storage
    ///
    /// # Arguments
    /// * `index` - The entity index
    ///
    pub fn get_full_entity(&self, index: usize) -> Vec<ComponentReference> {
        self.component_arrays
            .iter()
            .map(|(_, component_array)| {
                let component_array = component_array.read().unwrap();
                component_array.get(&index)
            })
            .collect::<Vec<_>>()
    }

    /// Get entity count
    pub fn len(&self) -> usize {
        let entity_array = self.component_arrays.get("Entity");
        if let Some(entity_array) = entity_array {
            let entity_array = entity_array.read().unwrap();
            entity_array.len()
        } else {
            0
        }
    }

    /// Add an entity into the archetype
    ///
    /// # Arguments
    /// * `entity_id` - The first entity id
    /// * `name` - The first entity name
    /// * `components` - The first entity components
    ///
    pub fn add(&self, entity_id: EntityId, name: &str, mut components: Vec<AnyComponent>) {
        // Inject the common Entity component
        components.push(AnyComponent::new(Entity {
            entity_id,
            name: name.to_string(),
            ..Entity::default()
        }));

        for component in components {
            let component_array = self.component_arrays.get(&component.get_class_name());
            if let Some(component_array) = component_array {
                let mut component_array = component_array.write().unwrap();
                component_array.add(component);
            }
        }

        // TODO: Use the remaining spaces to fill the new entity if possible
    }

    /// Remove an entity based on its id
    ///
    /// # Arguments
    /// * `index` - The entity index
    ///
    pub fn remove(&self, index: usize) {
        // Get the entity
        let components = self.get_full_entity(index);

        // TODO: Can probably be more consize with a specific Vec func
        let entity = components
            .iter()
            .find(|component| {
                let component = component.read();
                if let Some(_) = component.deref().as_any_ref().downcast_ref::<Entity>() {
                    true
                } else {
                    false
                }
            })
            .unwrap();

        let other_components = components
            .iter()
            .filter(|component| {
                let component = component.read();
                if let Some(_) = component.deref().as_any_ref().downcast_ref::<Entity>() {
                    false
                } else {
                    true
                }
            })
            .collect::<Vec<_>>();

        // propagate the deleted signal
        {
            let entity = entity.read();
            let entity = entity
                .deref()
                .as_any_ref()
                .downcast_ref::<Entity>()
                .unwrap();

            entity.on_deleted.notify(());
        }

        // Update the entity to set it as deleted
        {
            let _components_lock = other_components
                .iter()
                .map(|component| component.write())
                .collect::<Vec<_>>();

            let mut entity = entity.write();
            let mut entity = entity
                .deref_mut()
                .as_any_mut()
                .downcast_mut::<Entity>()
                .unwrap();

            entity.deleted = true;
        }

        // Remember that the old entity cell is now free
        // so we will be able to erase it
        {
            let mut removed_entities = self.removed_entities.lock().unwrap();
            removed_entities.push(index)
        };

        // TODO: Notify all the shared lock that the referenced entity has been removed
    }
}
