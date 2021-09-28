use crate::entity::entity_rwlock::EntityRwLock;
use crate::entity::entity::Entity;
use std::fmt::Debug;
use core::hash::Hash;
use crate::entity::entity_manager::RemoveEntityError;
use crate::entity::archetype_storage::Iter;
use crate::entity::entity::EntityId;
use crate::entity::archetype_storage::ArchetypeStorage;

#[derive(Debug)]
pub struct ArchetypeIdentifier(pub Vec<String>);

impl PartialEq for ArchetypeIdentifier {
    fn eq(&self, other: &ArchetypeIdentifier) -> bool {
        let matching = self.0
            .iter()
            .zip(other.0.iter())
            .filter(|&(a, b)| a == b).count();
        
        matching == self.0.len() && matching == other.0.len()
    }
}

impl Eq for ArchetypeIdentifier { }

impl Hash for ArchetypeIdentifier {
    fn hash<H>(&self, state: &mut H) where H: std::hash::Hasher {
        self.0.hash(state)
    }
}

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

    pub fn get_identifier(entity: &dyn Entity) -> ArchetypeIdentifier {
        ArchetypeIdentifier (
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

    /*pub fn for_each<F: Fn(Vec<ComponentRwLock>) + Send + Sync>(&self, callback: F) {
        self.storage.for_each(callback)
    }*/
}