use crate::entity::entity_rwlock::EntityRwLock;
use std::sync::RwLock;

pub struct RawInternalIter<'s, T: Entity> {
    pub entities_iterator: std::slice::Iter<'s, RwLock<T>>,
}

impl<'s, T: Entity> Iterator for RawInternalIter<'s, T> {
    type Item = EntityRwLock<'s>;

    fn next(&mut self) -> Option<EntityRwLock<'s>> {
        match self.entities_iterator.next() {
            Some(entity) => Some(EntityRwLock::new(entity)),
            None => None,
        }
    }
}
