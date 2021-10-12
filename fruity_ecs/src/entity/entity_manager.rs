use crate::entity::archetype::Archetype;
use crate::entity::entity::Entity;
use crate::entity::entity::EntityId;
use crate::entity::entity::EntityTypeIdentifier;
use crate::entity::entity::Iter as EntityIter;
use crate::entity::entity::IterMut as EntityIterMut;
use crate::entity::entity_rwlock::EntityRwLock;
use fruity_any_derive::*;
use fruity_core::service::Service;
use fruity_introspect::IntrospectMethods;
use fruity_introspect::MethodInfo;
use rayon::prelude::*;
use std::sync::Arc;
use std::sync::Mutex;

/// An error over entity deletion
pub enum RemoveEntityError {
    /// The entity don't exists in any archetype storage
    NotFound,
}

/// A storage for every entities, use [’Archetypes’] to store entities of different types
#[derive(Debug, FruityAny)]
pub struct EntityManager {
    id_incrementer: u64,
    archetypes: Vec<Archetype>,
}

impl EntityManager {
    /// Returns an EntityManager
    pub fn new() -> EntityManager {
        EntityManager {
            id_incrementer: 0,
            archetypes: Vec::new(),
        }
    }

    /// Get a locked entity
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    ///
    pub fn get(&self, entity_id: EntityId) -> Option<&EntityRwLock> {
        self.archetypes
            .iter()
            .find_map(|archetype| archetype.get(entity_id))
    }

    /// Iterate over all entities with a specific archetype type
    /// Use every entity that contains the provided entity type
    ///
    /// # Arguments
    /// * `entity_identifier` - The entity type identifier
    ///
    pub fn for_each<F: Fn(EntityIter) + Sync + Send>(
        &self,
        entity_identifier: EntityTypeIdentifier,
        callback: F,
    ) {
        let entity_identifier_1 = Arc::new(Mutex::new(entity_identifier.clone()));
        let entity_identifier_2 = Arc::new(Mutex::new(entity_identifier.clone()));

        self.archetypes
            .iter()
            .filter(move |archetype| {
                archetype
                    .get_type_identifier()
                    .contains(&entity_identifier_1.lock().unwrap())
            })
            .map(|archetype| archetype.iter())
            .flatten()
            .par_bridge()
            .for_each(move |entity| {
                entity
                    .read()
                    .unwrap()
                    .iter_component_tuple(&entity_identifier_2.lock().unwrap())
                    .for_each(|components| callback(components));
            });
    }

    /// Iterate over all entities with a specific archetype type with mutability
    /// Use every entity that contains the provided entity type
    ///
    /// # Arguments
    /// * `entity_identifier` - The entity type identifier
    ///
    pub fn for_each_mut<F: Fn(EntityIterMut) + Sync + Send>(
        &self,
        entity_identifier: EntityTypeIdentifier,
        callback: F,
    ) {
        let entity_identifier_1 = Arc::new(Mutex::new(entity_identifier.clone()));
        let entity_identifier_2 = Arc::new(Mutex::new(entity_identifier.clone()));

        self.archetypes
            .iter()
            .filter(move |archetype| {
                archetype
                    .get_type_identifier()
                    .contains(&entity_identifier_1.lock().unwrap())
            })
            .map(|archetype| archetype.iter())
            .flatten()
            .par_bridge()
            .for_each(move |entity| {
                entity
                    .write()
                    .unwrap()
                    .iter_mut_component_tuple(&entity_identifier_2.lock().unwrap())
                    .for_each(|components| callback(components));
            });
    }

    /// Add a new entity in the storage
    /// Create the archetype if it don't exists
    /// Returns the newly created entity id
    ///
    /// # Arguments
    /// * `entity` - The entity that will be added
    ///
    pub fn create(&mut self, entity: Entity) -> EntityId {
        self.id_incrementer += 1;
        let entity_id = EntityId(self.id_incrementer);
        let entity_identifier = entity.get_type_identifier();

        match self.archetype_mut_by_identifier(entity_identifier) {
            Some(archetype) => {
                archetype.add(entity_id, entity);
                entity_id
            }
            None => {
                let archetype = Archetype::new(entity_id, entity);
                self.archetypes.push(archetype);
                entity_id
            }
        }
    }
    /// Remove an entity based on its id
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    ///
    pub fn remove(&mut self, entity_id: EntityId) {
        if !self
            .archetypes
            .iter_mut()
            .any(|archetype| match archetype.remove(entity_id) {
                Ok(()) => true,
                Err(err) => match err {
                    RemoveEntityError::NotFound => false,
                },
            })
        {
            log::error!(
                "Trying to delete an unregistered entity with entity id {:?}",
                entity_id
            );
        }
    }

    fn archetype_by_identifier(
        &self,
        entity_identifier: EntityTypeIdentifier,
    ) -> Option<&Archetype> {
        self.archetypes
            .iter()
            .find(|archetype| *archetype.get_type_identifier() == entity_identifier)
    }

    fn archetype_mut_by_identifier(
        &mut self,
        entity_identifier: EntityTypeIdentifier,
    ) -> Option<&mut Archetype> {
        self.archetypes
            .iter_mut()
            .find(|archetype| *archetype.get_type_identifier() == entity_identifier)
    }
}

impl IntrospectMethods for EntityManager {
    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }
}

impl Service for EntityManager {}
