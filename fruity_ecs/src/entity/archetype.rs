use crate::entity::internal::archetype_storage::InternalRawArchetypeStorage;
use crate::entity::internal::archetype_storage::InternalArchetypeStorage;
use crate::entity::entity_rwlock::EntityRwLock;
use crate::entity::entity::Entity;
use std::fmt::Debug;
use crate::entity::entity_manager::RemoveEntityError;
use crate::entity::entity::EntityId;

/// A collection of entities that share the same component structure
#[derive(Debug)]
pub struct Archetype {
    internal_storage: Box<dyn InternalArchetypeStorage>,
}

impl Archetype {
    /// Returns an Archetype and inject the first entity inside
    ///
    /// # Arguments
    /// * `entity_id` - The first entity id
    /// * `entity` - The first entity datas
    ///
    /// # Generic Arguments
    /// * `T` - The type of the entities stored into the archetype
    ///
    pub fn new<T: Entity>(entity_id: EntityId, entity: T) -> Archetype {
        let mut archetype = Archetype {
            internal_storage: Box::new(InternalRawArchetypeStorage::<T>::new()),
        };

        archetype.add(entity_id, entity);
        archetype
    }

    /// Get a locked entity
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    ///
    pub fn get(&self, entity_id: EntityId) -> Option<EntityRwLock> {
        self.internal_storage.get(entity_id)
    }

    /// Iterate over all entities of the archetype
    pub fn iter(&self) -> Iter<'_> {
        self.internal_storage.iter()
    }

    /// Add an entity into the archetype
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    /// * `entity` - The entity datas
    ///
    /// # Generic Arguments
    /// * `T` - The type of the new entity
    ///
    pub fn add<T: Entity>(&mut self, entity_id: EntityId, entity: T) {
        self.internal_storage.add(entity_id, Box::new(entity))
    }

    /// Remove an entity based on its id
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    ///
    pub fn remove(&mut self, entity_id: EntityId) -> Result<(), RemoveEntityError> {
        self.internal_storage.remove(entity_id)
    }
}

/// Iterator over entities of an archetype
pub enum Iter<'s> {
    /// Classic iterator that iterate over all entities in the archetype
    Normal {
        /// An internal iterator that contains informations about the type of the entity
        internal_iter: Box<dyn Iterator<Item = EntityRwLock<'s>> + 's + Sync + Send>,
    },
    /// Empty iterator
    Empty,
}

impl<'s> Iterator for Iter<'s> {
    type Item = EntityRwLock<'s>;

    fn next(&mut self) -> Option<EntityRwLock<'s>> {
        match self {
            Iter::Normal { internal_iter } => internal_iter.next(),
            Iter::Empty => None
        }
    }
}