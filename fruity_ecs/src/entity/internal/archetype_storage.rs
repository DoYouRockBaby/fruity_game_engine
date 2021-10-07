use crate::entity::archetype::Iter;
use crate::entity::entity::EntityId;
use crate::entity::entity_manager::RemoveEntityError;
use crate::entity::entity_rwlock::EntityRwLock;
use crate::entity::internal::archetype_storage_iter::RawInternalIter;
use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::RwLock;

pub trait InternalArchetypeStorage: Debug + Send + Sync {
    fn get(&self, entity_id: EntityId) -> Option<EntityRwLock>;
    fn iter(&self) -> Iter<'_>;
    fn add(&mut self, entity_id: EntityId, entity: Box<dyn Any>);
    fn remove(&mut self, entity_id: EntityId) -> Result<(), RemoveEntityError>;
}

#[derive(Debug)]
pub struct InternalRawArchetypeStorage<T: Entity> {
    entity_indexes: HashMap<EntityId, usize>,
    entities: Vec<RwLock<T>>,
}

impl<T: Entity> InternalRawArchetypeStorage<T> {
    pub fn new() -> InternalRawArchetypeStorage<T> {
        InternalRawArchetypeStorage::<T> {
            entity_indexes: HashMap::new(),
            entities: Vec::new(),
        }
    }
}

impl<T: Entity> InternalArchetypeStorage for InternalRawArchetypeStorage<T> {
    fn get(&self, entity_id: EntityId) -> Option<EntityRwLock> {
        match self.entity_indexes.get(&entity_id) {
            Some(index) => match self.entities.get(*index) {
                Some(entity) => Some(EntityRwLock::new(entity)),
                None => None,
            },
            None => None,
        }
    }

    fn iter(&self) -> Iter {
        Iter::Normal {
            internal_iter: Box::new(RawInternalIter::<T> {
                entities_iterator: self.entities.iter(),
            }),
        }
    }

    fn add(&mut self, entity_id: EntityId, entity: Box<dyn Any>) {
        let entity = match entity.downcast::<T>() {
            Ok(entity) => *entity,
            Err(_) => {
                log::error!(
                    "Failed to insert an entity into its archetype wich is {:#?}",
                    self
                );
                return;
            }
        };

        self.entity_indexes.insert(entity_id, self.entities.len());
        self.entities.push(RwLock::new(entity));
    }

    fn remove(&mut self, entity_id: EntityId) -> Result<(), RemoveEntityError> {
        // Remove old stored id
        match self.entity_indexes.remove(&entity_id) {
            Some(index) => {
                // Remove associated binary datas
                self.entities.remove(index);

                // Gap all existing indexes
                self.entity_indexes.iter_mut().for_each(|(_, index_2)| {
                    if *index_2 > index {
                        *index_2 -= 1;
                    }
                });
                Ok(())
            }
            None => Err(RemoveEntityError::NotFound),
        }
    }
}
