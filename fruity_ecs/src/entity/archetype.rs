use crate::component::component::Component;
use crate::entity::entity::Entity;
use crate::entity::entity::EntityId;
use crate::entity::entity_manager::RemoveEntityError;
use crate::entity::entity_rwlock::EntityRwLock;
use fruity_collections::TraitVec;
use fruity_collections::TraitVecObject;
use std::fmt::Debug;

/// A collection of entities that share the same component structure
pub struct Archetype {
    entities: TraitVec<dyn Component>,
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
    pub fn new(entity_id: EntityId, entity: Vec<Box<dyn Component>>) -> Archetype {
        let mut archetype = Archetype {
            entities: TraitVec::new(),
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
    pub fn add(&mut self, entity_id: EntityId, components: Vec<Box<dyn Component>>) {
        for component in components {
            self.entities
                .push(component.as_ref() as &dyn TraitVecObject);
        }
    }

    /// Remove an entity based on its id
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    ///
    pub fn remove(&mut self, entity_id: EntityId) -> Result<(), RemoveEntityError> {
        let entity = Entity {};
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
