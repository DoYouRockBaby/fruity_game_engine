use std::fmt::Debug;
use core::hash::Hash;
use crate::entity::entity_manager::RemoveEntityError;
use crate::entity::archetype_storage::Iter;
use crate::entity::entity::EntityId;
use crate::entity::archetype_storage::ArchetypeStorage;
use crate::component::component::Component;
use crate::component::component_rwlock::ComponentRwLock;

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
    pub size: usize,
    pub decoder: fn(datas: &[u8]) -> ComponentRwLock,
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
    pub fn new(entity_id: EntityId, components: &[&dyn Component]) -> Archetype {
        let entity_size = components
            .iter()
            .map(|component| component.get_component_size())
            .sum();

        let component_types: Vec<ArchetypeComponentType> = components
            .iter()
            .map(|component| ArchetypeComponentType {
                identifier: component.get_component_type().to_string(),
                size: component.get_component_size(),
                decoder: component.decoder(),
            })
            .collect();

        let mut archetype = Archetype {
            storage: ArchetypeStorage::new(entity_size, component_types),
        };

        archetype.add(entity_id, components);
        archetype
    }

    pub fn get_identifier(components: &[&dyn Component]) -> ArchetypeIdentifier {
        ArchetypeIdentifier (
            components
                .iter()
                .map(|component| component.get_component_type().to_string())
                .collect()
        )
    }

    pub fn get(&self, entity_id: EntityId) -> Option<Vec<ComponentRwLock>> {
        self.storage.get(entity_id)
    }

    pub fn iter(&self) -> Iter<'_> {
        self.storage.iter()
    }

    pub fn add(&mut self, entity_id: EntityId, components: &[&dyn Component]) {
        self.storage.add(entity_id, components)
    }

    pub fn remove(&mut self, entity_id: EntityId) -> Result<(), RemoveEntityError> {
        self.storage.remove(entity_id)
    }

    pub fn for_each<F: Fn(Vec<ComponentRwLock>) + Send + Sync>(&self, callback: F) {
        self.storage.for_each(callback)
    }
}