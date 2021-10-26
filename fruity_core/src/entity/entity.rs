use crate::component::component::AnyComponent;
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
    pub fn contains(&self, other: &EntityTypeIdentifier) -> bool {
        let matching = other
            .0
            .iter()
            .filter(|component_identifier| self.0.contains(component_identifier))
            .count();

        matching == other.0.len()
    }
}

/// An identifier for an entity
pub type EntityId = u64;

/// Get the entity type identifier from a list of components
pub fn get_type_identifier(components: &[AnyComponent]) -> EntityTypeIdentifier {
    let identifier = components
        .iter()
        .map(|component| component.get_component_type())
        .collect::<Vec<_>>();

    EntityTypeIdentifier(identifier)
}
