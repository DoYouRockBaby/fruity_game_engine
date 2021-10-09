use crate::entity::entity::Entity;
use crate::entity::entity::EntityId;
use crate::entity::entity_manager::RemoveEntityError;
use crate::entity::entity_rwlock::EntityRwLock;
use fruity_collections::encodable_vec::EncodableVec;
use std::collections::HashMap;
use std::fmt::Debug;

/// A collection of entities that share the same component structure
pub struct Archetype {
    index_map: HashMap<EntityId, usize>,
    entities: EncodableVec,
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
    pub fn new(entity_id: EntityId, entity: Entity) -> Archetype {
        let mut archetype = Archetype {
            index_map: HashMap::new(),
            entities: EncodableVec::new(),
        };

        archetype.add(entity_id, entity);
        archetype
    }

    /// Get a locked entity
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    ///
    pub fn get(&self, entity_id: EntityId) -> Option<&EntityRwLock> {
        let index = match self.index_map.get(&entity_id) {
            Some(index) => index,
            None => return None,
        };

        self.get_by_index(*index)
    }

    /// Get a locked entity by first component index
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    ///
    pub fn get_by_index(&self, index: usize) -> Option<&EntityRwLock> {
        let entities = unsafe { &*(&self.entities as *const _) } as &EncodableVec;
        let entity = match entities.get(index) {
            Some(entity) => entity,
            None => return None,
        };

        match entity.downcast_ref::<EntityRwLock>() {
            Some(entity) => Some(entity),
            None => None,
        }
    }

    /// Iterate over all entities of the archetype
    pub fn iter(&self) -> Iter<'_> {
        Iter::Normal {
            archetype: self,
            current_index: 0,
        }
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
    pub fn add(&mut self, entity_id: EntityId, entity: Entity) {
        self.index_map.insert(entity_id, self.entities.len());
        self.entities.push(Box::new(EntityRwLock::new(entity)));
    }

    /// Remove an entity based on its id
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    ///
    pub fn remove(&mut self, entity_id: EntityId) -> Result<(), RemoveEntityError> {
        // Find the entity index in the entity array
        let index = match self.index_map.remove(&entity_id) {
            Some(index) => Ok(index),
            None => Err(RemoveEntityError::NotFound),
        }?;

        // Remove old stored entity
        self.entities.remove(index);

        // Gap all existing indexes
        self.index_map.iter_mut().for_each(|index_2| {
            if *index_2.1 > index {
                *index_2.1 -= 1;
            }
        });

        Ok(())
    }
}

/// Iterator over entities of an archetype
pub enum Iter<'s> {
    /// Classic iterator that iterate over all entities in the archetype
    Normal {
        /// The targeted archetype
        archetype: &'s Archetype,

        /// A counter to know the iterator current index
        current_index: usize,
    },
    /// Empty iterator
    Empty,
}

impl<'s> Iterator for Iter<'s> {
    type Item = &'s EntityRwLock;

    fn next(&mut self) -> Option<&'s EntityRwLock> {
        match self {
            Iter::Normal {
                archetype,
                current_index,
            } => {
                let result = archetype.get_by_index(*current_index);
                *current_index += 1;
                result
            }
            Iter::Empty => None,
        }
    }
}

impl Debug for Archetype {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        let fmt_error = self.iter().find_map(|elem| match elem.fmt(formatter) {
            Ok(()) => None,
            Err(err) => Some(err),
        });

        match fmt_error {
            Some(err) => Err(err),
            None => Ok(()),
        }
    }
}
