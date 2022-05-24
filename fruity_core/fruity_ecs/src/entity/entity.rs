use crate::component::component::AnyComponent;
use crate::component::component::Component;
use std::fmt::Debug;
use std::hash::Hash;

/// An identifier to an entity type, is composed be the identifier of the contained components
#[derive(Debug, Clone)]
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

impl EntityTypeIdentifier {
    /// Check if an entity identifier contains an other one
    /// For example ["c1", "c2", "c3"] contains ["c3", "c2"]
    pub fn contains(&self, other: &String) -> bool {
        self.0.contains(other)
    }
}

/// An identifier for an entity
pub type EntityId = u64;

/// Get the entity type identifier from a list of components
pub fn get_type_identifier_by_any(components: &[AnyComponent]) -> EntityTypeIdentifier {
    let identifier = components
        .iter()
        .map(|component| component.get_class_name())
        .collect::<Vec<_>>();

    EntityTypeIdentifier(identifier)
}

/// Get the entity type identifier from a list of components
pub fn get_type_identifier(components: &[&dyn Component]) -> EntityTypeIdentifier {
    let identifier = components
        .iter()
        .map(|component| component.get_class_name())
        .collect::<Vec<_>>();

    EntityTypeIdentifier(identifier)
}
