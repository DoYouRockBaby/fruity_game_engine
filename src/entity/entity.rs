use crate::component::component::Component;
use std::any::Any;
use std::fmt::Debug;
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

pub trait Entity: Debug + Any + Send + Sync {
    fn get(&self, index: usize) -> Option<&dyn Component>;
    fn get_mut(&mut self, index: usize) -> Option<&mut dyn Component>;
}

impl dyn Entity {
    pub fn untyped_iter(&self) -> Iter {
        Iter {
            entity: self,
            index: 0,
        }
    }
    pub fn untyped_iter_mut(&mut self) -> IterMut {
        IterMut {
            entity: self,
            index: 0,
        }
    }
    
    pub fn iter<T: Component>(&self) -> impl Iterator<Item = &T> {
        self
            .untyped_iter()
            .filter_map(|component| component.downcast_ref::<T>())
    }

    pub fn iter_mut<T: Component>(&mut self) -> impl Iterator<Item = &mut T> {
        self
            .untyped_iter_mut()
            .filter_map(|component| component.downcast_mut::<T>())
    }
}

pub struct Iter<'s> {
    entity: &'s dyn Entity,
    index: usize,
}

impl<'s> Iterator for Iter<'s> {
    type Item = &'s dyn Component;

    fn next(&mut self) -> Option<&'s dyn Component> {
        let component = self.entity.get(self.index);
        self.index += 1;
        component
    }
}

pub struct IterMut<'s> {
    entity: &'s mut dyn Entity,
    index: usize,
}

impl<'s> Iterator for IterMut<'s> {
    type Item = &'s mut dyn Component;

    fn next(&mut self) -> Option<&'s mut dyn Component> {
        let component = self.entity.get_mut(self.index);
        self.index += 1;
        component
    }
}