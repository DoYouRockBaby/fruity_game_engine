use crate::entity::entity_rwlock::EntityRwLock;
use crate::entity::entity::Entity;
use std::collections::HashMap;
use crate::entity::archetype::Archetype;
use crate::entity::entity::EntityTypeIdentifier;
use crate::entity::archetype::Iter as ArchetypeIter;
use crate::entity::entity::EntityId;

/// An error over entity deletion
pub enum RemoveEntityError {
    /// The entity don't exists in any archetype storage
    NotFound
}

/// A storage for every entities, use [’Archetypes’] to store entities of different types
#[derive(Debug)]
pub struct EntityManager {
    id_incrementer: u64,
    archetypes: HashMap<EntityTypeIdentifier, Archetype>,
}

impl EntityManager {
    /// Returns an EntityManager
    pub fn new() -> EntityManager {
        EntityManager {
            id_incrementer: 0,
            archetypes: HashMap::new(),
        }
    }

    /// Get a locked entity
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    ///
    pub fn get(&self, entity_id: EntityId) -> Option<EntityRwLock> {
        self.archetypes
            .values()
            .find_map(|archetype| archetype.get(entity_id))
    }

    /// Iterate over all entities with a specific archetype type
    /// Use every entity that contains the provided entity type
    ///
    /// # Arguments
    /// * `entity_identifier` - The entity type identifier
    ///
    pub fn iter(&self, entity_identifier: EntityTypeIdentifier) -> ArchetypeIter {
        match self.archetypes.get(&entity_identifier) {
            Some(archetype) => {
                archetype.iter()
            },
            None => {
                ArchetypeIter::Empty
            },
        }
    }

    /// Add a new entity in the storage
    /// Create the archetype if it don't exists
    /// Returns the newly created entity id
    ///
    /// # Arguments
    /// * `entity` - The entity that will be added
    ///
    pub fn create<T: Entity>(&mut self, entity: T) -> EntityId {
        let entity_ref = &entity as &dyn Entity;
        let entity_identifier = entity_ref.get_type_identifier();
        self.id_incrementer += 1;
        let entity_id = EntityId ( self.id_incrementer );

        match self.archetypes.get_mut(&entity_identifier) {
            Some(archetype) => {
                archetype.add(entity_id, entity);
                entity_id
            },
            None => {
                let archetype = Archetype::new(entity_id, entity);
                self.archetypes.insert(entity_identifier, archetype);
                entity_id
            },
        }
    }
    
    /// Remove an entity based on its id
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    ///
    pub fn remove(&mut self, entity_id: EntityId) {
        if !self.archetypes.values_mut().any(|archetype| {
            match archetype.remove(entity_id) {
                Ok(()) => true,
                Err(err) => match err {
                    RemoveEntityError::NotFound => false,
                },
            }
        }) {
            log::error!("Trying to delete an unregistered entity with entity id {:?}", entity_id);
        }
    }
}