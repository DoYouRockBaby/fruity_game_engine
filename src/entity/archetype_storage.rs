use crate::entity::internal::archetype_storage::InternalRawArchetypeStorage;
use crate::entity::internal::archetype_storage::InternalArchetypeStorage;
use crate::entity::entity_rwlock::EntityRwLock;
use crate::entity::entity::Entity;
use std::fmt::Debug;
use crate::entity::entity_manager::RemoveEntityError;
use crate::entity::entity::EntityId;

#[derive(Debug)]
pub struct ArchetypeStorage {
    internal_storage: Box<dyn InternalArchetypeStorage>,
}

impl ArchetypeStorage {
    pub fn new<T: Entity>() -> ArchetypeStorage {
        ArchetypeStorage {
            internal_storage: Box::new(InternalRawArchetypeStorage::<T>::new()),
        }
    }

    pub fn get(&self, entity_id: EntityId) -> Option<EntityRwLock> {
        self.internal_storage.get(entity_id)
    }

    pub fn iter(&self) -> Iter<'_> {
        self.internal_storage.iter()
    }

    pub fn add<T: Entity>(&mut self, entity_id: EntityId, entity: T) {
        self.internal_storage.add(entity_id, Box::new(entity))
    }

    pub fn remove(&mut self, entity_id: EntityId) -> Result<(), RemoveEntityError> {
        self.internal_storage.remove(entity_id)
    }
}

pub enum Iter<'s> {
    Normal {
        internal_iter: Box<dyn Iterator<Item = EntityRwLock<'s>> + 's + Sync + Send>,
    },
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