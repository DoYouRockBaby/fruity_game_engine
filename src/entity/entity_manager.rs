use crate::Component;
use crate::entity::entity_rwlock::EntityRwLock;
use crate::entity::entity::Entity;
use std::collections::HashMap;
use crate::entity::archetype::Archetype;
use crate::entity::archetype::ArchetypeIdentifier;
use crate::entity::archetype_storage::Iter as ArchetypeIter;
use crate::entity::entity::Iter as EntityIter;
use crate::entity::entity::EntityId;
use rayon::prelude::*;

pub enum RemoveEntityError {
    NotFound
}

#[derive(Debug)]
pub struct EntityManager {
    id_incrementer: u64,
    archetypes: HashMap<ArchetypeIdentifier, Archetype>,
}

impl EntityManager {
    pub fn new() -> EntityManager {
        EntityManager {
            id_incrementer: 0,
            archetypes: HashMap::new(),
        }
    }

    pub fn get(&self, entity_id: EntityId) -> Option<EntityRwLock> {
        self.archetypes
            .values()
            .find_map(|archetype| archetype.get(entity_id))
    }

    pub fn for_each<T: Fn(&[&dyn Component]) + Send + Sync>(&self, archetype_identifier: ArchetypeIdentifier, callback: T) {
        match self.archetypes.get(&archetype_identifier) {
            Some(archetype) => {
                archetype
                    .iter()
                    .par_bridge()
                    .for_each(|entity| {
                        let components = entity
                            .read()
                            .untyped_iter()
                            .collect::<Vec<_>>();
                        
                        callback(&components[..]);
                    });
            },
            None => (),
        }
    }

    pub fn for_each_mut<T: Fn(&[&mut dyn Component]) + Send + Sync>(&self, archetype_identifier: ArchetypeIdentifier, callback: T) {
        match self.archetypes.get(&archetype_identifier) {
            Some(archetype) => {
                archetype
                    .iter()
                    .par_bridge()
                    .for_each(|entity| {
                        let components = entity
                            .read()
                            .untyped_iter_mut()
                            .collect::<Vec<_>>();
                        
                        callback(&components[..]);
                    });
            },
            None => (),
        }
    }

    /*pub fn iter(&self, archetype_identifier: ArchetypeIdentifier) -> impl Iterator<Item = Vec<&dyn Component>> {
        match self.archetypes.get(&archetype_identifier) {
            Some(archetype) => {
                let test = archetype
                    .iter()
                    .map(|entity| {
                        entity
                            .read()
                            .untyped_iter()
                            .collect::<Vec<_>>()
                    });
                
                test
            },
            None => {
                //Iter::Empty
                panic!("Need to be removed");
            },
        }
    }*/

    pub fn create<T: Entity>(&mut self, entity: T) -> EntityId {
        let archetype_identifier = Archetype::get_identifier(&entity);
        self.id_incrementer += 1;
        let entity_id = EntityId ( self.id_incrementer );

        match self.archetypes.get_mut(&archetype_identifier) {
            Some(archetype) => {
                archetype.add(entity_id, entity);
                entity_id
            },
            None => {
                let archetype = Archetype::new(entity_id, entity);
                self.archetypes.insert(archetype_identifier, archetype);
                entity_id
            },
        }
    }

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

    /*pub fn for_each<F: Fn(Vec<ComponentRwLock>) + Send + Sync>(&self, archetype_identifier: ArchetypeIdentifier, callback: F) {
        match self.archetypes.get(&archetype_identifier) {
            Some(archetype) => archetype.for_each(callback),
            None => (),
        }
    }*/
}