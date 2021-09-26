use std::collections::HashMap;
use crate::entity::archetype::Archetype;
use crate::entity::archetype::ArchetypeIdentifier;
use crate::entity::archetype_storage::Iter;
use crate::component::component::Component;
use crate::entity::entity::EntityId;

pub enum RemoveEntityError {
    NotFound
}

#[derive(Debug)]
pub struct EntityManager {
    id_incrementer: u64,
    archetypes: HashMap<ArchetypeIdentifier, Archetype>,
}

impl<'s> EntityManager {
    pub fn new() -> EntityManager {
        EntityManager {
            id_incrementer: 0,
            archetypes: HashMap::new(),
        }
    }

    pub fn get(&self, entity_id: EntityId) -> Option<Vec<&dyn Component>> {
        self.archetypes
            .values()
            .find_map(|archetype| archetype.get(entity_id))
    }

    pub fn get_mut(&mut self, entity_id: EntityId) -> Option<Vec<&mut dyn Component>> {
        self.archetypes
            .values_mut()
            .find_map(|archetype| archetype.get_mut(entity_id))
    }

    pub fn iter(&self, archetype_identifier: ArchetypeIdentifier) -> Option<Iter> {
        match self.archetypes.get(&archetype_identifier) {
            Some(archetype) => {
                Some(archetype.iter())
            },
            None => {
                None
            },
        }
    }

    /*pub fn iter_mut_entities(&mut self, archetype_identifier: ArchetypeIdentifier) -> Option<IterMut> {
        match self.archetypes.get_mut(&archetype_identifier) {
            Some(archetype) => {
                Some(archetype.iter_mut())
            },
            None => {
                None
            },
        }
    }*/

    pub fn create(&mut self, components: &[&mut dyn Component]) -> EntityId {
        let archetype_identifier = Archetype::get_identifier(components);
        self.id_incrementer += 1;
        let entity_id = EntityId ( self.id_incrementer );

        match self.archetypes.get_mut(&archetype_identifier) {
            Some(archetype) => {
                archetype.add(entity_id, components);
                entity_id
            },
            None => {
                let archetype = Archetype::new(entity_id, components);
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

    pub fn for_each<F: Fn(&mut [&mut dyn Component]) + Send + Sync>(&mut self, archetype_identifier: ArchetypeIdentifier, callback: F) {
        match self.archetypes.get_mut(&archetype_identifier) {
            Some(archetype) => archetype.for_each(callback),
            None => (),
        }
    }
}