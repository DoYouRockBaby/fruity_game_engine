use crate::component::component::Component;
use std::any::Any;
use std::fmt::Debug;
use std::hash::Hash;
use itertools::Itertools;

#[derive(Debug)]
pub struct EntityIdentifier(pub Vec<String>);

impl PartialEq for EntityIdentifier {
    fn eq(&self, other: &EntityIdentifier) -> bool {
        let matching = self.0
            .iter()
            .zip(other.0.iter())
            .filter(|&(a, b)| a == b).count();
        
        matching == self.0.len() && matching == other.0.len()
    }
}

impl Eq for EntityIdentifier { }

impl Hash for EntityIdentifier {
    fn hash<H>(&self, state: &mut H) where H: std::hash::Hasher {
        self.0.hash(state)
    }
}

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
    fn len(&self) -> usize;
}

impl dyn Entity {
    pub fn get_identifier(&self) -> EntityIdentifier {
        EntityIdentifier (
            self
                .untyped_iter()
                .map(|component| component.get_component_type().to_string())
                .collect()
        )
    }

    pub fn untyped_iter(&self) -> Iter {
        let indexes = vec![0, self.len()];

        Iter {
            entity: self,
            indexes,
        }
    }

    pub fn untyped_iter_mut(&mut self) -> IterMut {
        let indexes = vec![0, self.len()];

        IterMut {
            entity: self,
            indexes,
        }
    }
    
    pub fn untyped_iter_over_types(&self, type_identifiers: Vec<String>) -> OverTypesIter {
        let identifier = self.get_identifier();
        let types_list = type_identifiers
                .into_iter()
                .map(|type_identifier| {
                    identifier.0
                        .iter()
                        .enumerate()
                        .filter_map(|(index, component_type)| {
                            if *component_type == type_identifier {
                                Some(index)
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>()
                })
                .multi_cartesian_product()
                .collect::<Vec<_>>();

        OverTypesIter {
            entity: self,
            types_list,
        }
    }
    
    pub fn untyped_iter_mut_over_types(&mut self, target_identifier: EntityIdentifier) -> OverTypesIterMut {
        let intern_identifier = self.get_identifier();
        let types_list = target_identifier.0
                .into_iter()
                .map(|type_identifier| {
                    intern_identifier.0
                        .iter()
                        .enumerate()
                        .filter_map(|(index, component_type)| {
                            if *component_type == type_identifier {
                                Some(index)
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>()
                })
                .multi_cartesian_product()
                .collect::<Vec<_>>();

        OverTypesIterMut {
            entity: self,
            types_list,
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
    indexes: Vec<usize>,
}

impl<'s> Iterator for Iter<'s> {
    type Item = &'s dyn Component;

    fn next(&mut self) -> Option<&'s dyn Component> {
        match self.indexes.pop() {
            Some(index) => self.entity.get(index),
            None => None
        }
    }
}

pub struct IterMut<'s> {
    entity: &'s mut dyn Entity,
    indexes: Vec<usize>,
}

impl<'s> Iterator for IterMut<'s> {
    type Item = &'s mut dyn Component;

    fn next(&mut self) -> Option<&'s mut dyn Component> {
        match self.indexes.pop() {
            Some(index) => match self.entity.get_mut(index) {
                Some(component) => Some(unsafe { &mut *(component as *mut _) }),
                None => None,
            },
            None => None
        }
    }
}

pub struct OverTypesIter<'s> {
    entity: &'s dyn Entity,
    types_list: Vec<Vec<usize>>,
}

impl<'s> Iterator for OverTypesIter<'s> {
    type Item = Iter<'s>;

    fn next(&mut self) -> Option<Iter<'s>> {
        match self.types_list.pop() {
            Some(type_indexes) => Some(Iter {
                entity: self.entity,
                indexes: type_indexes,
            }),
            None => None,
        }
    }
}

pub struct OverTypesIterMut<'s> {
    entity: &'s mut dyn Entity,
    types_list: Vec<Vec<usize>>,
}

impl<'s> Iterator for OverTypesIterMut<'s> {
    type Item = IterMut<'s>;

    fn next(&mut self) -> Option<IterMut<'s>> {
        match self.types_list.pop() {
            Some(type_indexes) => Some(IterMut {
                entity: unsafe { &mut *(self.entity as *mut _) },
                indexes: type_indexes,
            }),
            None => None,
        }
    }
}