use crate::component::component::AnyComponent;
use crate::component::component::Component;
use crate::entity::archetype::encode_entity::entity_size;
use crate::entity::archetype::inner_archetype::EntityCellHead;
use crate::entity::archetype::inner_archetype::InnerArchetype;
use crate::entity::archetype::rwlock::EntitySharedRwLock;
use crate::entity::entity::get_type_identifier;
use crate::entity::entity::get_type_identifier_by_any;
use crate::entity::entity::EntityId;
use crate::entity::entity::EntityTypeIdentifier;
use crate::entity::entity_manager::RemoveEntityError;
use std::fmt::Debug;
use std::sync::Arc;
use std::sync::RwLock;

/// Implements an the archetype logic
pub mod inner_archetype;

/// Utils to encode and decode entity
pub mod encode_entity;

/// Provides a threadsafe lock for entities
pub mod rwlock;

/// A collection of entities that share the same component structure
pub struct Archetype {
    identifier: EntityTypeIdentifier,
    inner_archetype: Arc<RwLock<InnerArchetype>>,
}

impl Archetype {
    /// Returns an Archetype and inject the first entity inside
    ///
    /// # Arguments
    /// * `entity_id` - The first entity id
    /// * `components` - The first entity components
    ///
    /// # Generic Arguments
    /// * `T` - The type of the entities stored into the archetype
    ///
    pub fn new(entity_id: EntityId, components: Vec<AnyComponent>) -> Archetype {
        // Deduce the archetype properties from the components
        let identifier = get_type_identifier_by_any(&components);

        let components_per_entity = components.len();

        let entity_size: usize = entity_size(&components);

        // Build the inner archetype that implement all the logic
        let inner_archetype =
            InnerArchetype::new(identifier.clone(), components_per_entity, entity_size);
        let inner_archetype = Arc::new(RwLock::new(inner_archetype));

        // Create the first entity
        let mut writer = inner_archetype.write().unwrap();
        writer.add(entity_id, components);
        std::mem::drop(writer);

        Archetype {
            identifier,
            inner_archetype,
        }
    }

    /// Returns the entity type identifier of the archetype
    pub fn get_type_identifier(&self) -> &EntityTypeIdentifier {
        &self.identifier
    }

    /// Get a locked entity
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    ///
    pub fn get(&self, entity_id: EntityId) -> Option<EntitySharedRwLock> {
        let inner_archetype = self.inner_archetype.read().unwrap();
        inner_archetype.get(self.inner_archetype.clone(), entity_id)
    }

    /// Get a locked entity by first component index
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    ///
    pub fn get_by_index(&self, index: usize) -> EntitySharedRwLock {
        let inner_archetype = self.inner_archetype.read().unwrap();
        inner_archetype.get_by_index(self.inner_archetype.clone(), index)
    }

    /// Iterate over all entities of the archetype
    pub(crate) fn iter(&self) -> Iter {
        Iter {
            inner_archetype: self.inner_archetype.clone(),
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
    pub fn add(&self, entity_id: EntityId, components: Vec<AnyComponent>) {
        let mut writer = self.inner_archetype.write().unwrap();
        writer.add(entity_id, components);
    }

    /// Remove an entity based on its id
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    ///
    pub fn remove(&self, entity_id: EntityId) -> Result<EntityCellHead, RemoveEntityError> {
        let mut inner_archetype = self.inner_archetype.write().unwrap();
        inner_archetype.remove(entity_id)
    }
}

/// Iterator over entities of an archetype
pub struct Iter {
    /// The targeted archetype
    inner_archetype: Arc<RwLock<InnerArchetype>>,

    /// A counter to know the iterator current index
    current_index: usize,
}

impl Iterator for Iter {
    type Item = EntitySharedRwLock;

    fn next(&mut self) -> Option<EntitySharedRwLock> {
        let reader = self.inner_archetype.read().unwrap();
        if self.current_index < reader.buffer.len() {
            let result = reader.get_by_index(self.inner_archetype.clone(), self.current_index);
            self.current_index += reader.entity_size;

            Some(result)
        } else {
            None
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

#[cfg(test)]
mod tests {
    #[derive(Debug, Clone, Component, FruityAny)]
    struct Component1 {
        pub field1: f32,
        pub field2: usize,
    }

    #[derive(Debug, Clone, Component, FruityAny)]
    struct Component2 {
        pub field1: String,
        pub field2: usize,
    }

    #[test]
    fn create_() {
        assert_eq!(2 + 2, 4);
    }
}
