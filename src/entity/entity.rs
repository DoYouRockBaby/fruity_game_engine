use std::hash::Hash;

#[derive(Debug, Copy, Clone)]
pub struct EntityId(pub u64);

impl PartialEq for EntityId {
    fn eq(&self, other: &EntityId) -> bool {
        self.0 == other.0
    }
}

impl Eq for EntityId { }

impl Hash for EntityId {
    fn hash<H>(&self, state: &mut H) where H: std::hash::Hasher {
        self.0.hash(state)
    }
}
