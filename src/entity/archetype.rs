use crate::entity::entity::EntityIdentifier;
use crate::entity::entity_rwlock::EntityRwLock;
use crate::entity::entity::Entity;
use std::fmt::Debug;
use crate::entity::entity_manager::RemoveEntityError;
use crate::entity::archetype_storage::Iter;
use crate::entity::entity::EntityId;
use crate::entity::archetype_storage::ArchetypeStorage;

#[derive(Clone)]
pub struct ArchetypeComponentType {
    pub identifier: String,
}

impl Debug for ArchetypeComponentType {
    fn fmt(&self, formater: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        self.identifier.fmt(formater)
    }
}

#[derive(Debug)]
pub struct Archetype {
    storage: ArchetypeStorage,
}

impl Archetype {
    pub fn new<T: Entity>(entity_id: EntityId, entity: T) -> Archetype {
        let mut archetype = Archetype {
            storage: ArchetypeStorage::new::<T>(),
        };

        archetype.add(entity_id, entity);
        archetype
    }

    pub fn get_identifier(entity: &dyn Entity) -> EntityIdentifier {
        EntityIdentifier (
            entity
                .untyped_iter()
                .map(|component| component.get_component_type().to_string())
                .collect()
        )
    }

    pub fn get(&self, entity_id: EntityId) -> Option<EntityRwLock> {
        self.storage.get(entity_id)
    }

    pub fn iter(&self) -> Iter<'_> {
        self.storage.iter()
    }

    pub fn add<T: Entity>(&mut self, entity_id: EntityId, entity: T) {
        self.storage.add(entity_id, entity)
    }

    pub fn remove(&mut self, entity_id: EntityId) -> Result<(), RemoveEntityError> {
        self.storage.remove(entity_id)
    }
}