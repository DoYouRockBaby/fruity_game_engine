use crate::component::component::Component;
use fruity_collections::TraitVecObject;
use std::fmt::Debug;
use std::hash::Hash;
use std::sync::RwLock;

/// An identifier to an entity type, is composed be the identifier of the contained components
#[derive(Debug)]
pub struct EntityTypeIdentifier(pub Vec<String>);

impl PartialEq for EntityTypeIdentifier {
    fn eq(&self, other: &EntityTypeIdentifier) -> bool {
        let matching = self
            .0
            .iter()
            .zip(other.0.iter())
            .filter(|&(a, b)| a == b)
            .count();
        matching == self.0.len() && matching == other.0.len()
    }
}

impl Eq for EntityTypeIdentifier {}

impl Hash for EntityTypeIdentifier {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        self.0.hash(state)
    }
}

/// Create an entity type, use it like entity_type!["Component1", "Component2"])
#[macro_export]
macro_rules! entity_type {
    ($e:expr) => {{
        let component_names: Vec<&str> = vec![$e];
        fruity_ecs::entity::entity::EntityTypeIdentifier(
            component_names.iter().map(|e| e.to_string()).collect(),
        )
    }};
}

/// An identifier for an entity
#[derive(Debug, Copy, Clone)]
pub struct EntityId(pub u64);

impl PartialEq for EntityId {
    fn eq(&self, other: &EntityId) -> bool {
        self.0 == other.0
    }
}

impl Eq for EntityId {}

impl Hash for EntityId {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        self.0.hash(state)
    }
}

/// Returns the entity type identifier of the entity
pub fn get_type_identifier(components: Vec<Box<dyn Component>>) -> EntityTypeIdentifier {
    EntityTypeIdentifier(
        components
            .iter()
            .map(|component| component.get_component_type())
            .collect(),
    )
}

pub struct Entity<'s> {
    component_buffer: RwLock<&'s [&'s u8]>,
}

impl<'s> TraitVecObject for Entity<'s> {
    fn get_decoder(&self) -> TraitVecObjectDecoder {}

    fn get_decoder_mut(&self) -> TraitVecObjectDecoderMut {}

    fn encode(&self) -> Vec<u8> {
        self.
    }
}
