use std::collections::VecDeque;
use crate::component::component::Component;
use std::any::Any;
use std::fmt::Debug;
use std::hash::Hash;
use itertools::Itertools;

/// An identifier to an entity type, is composed be the identifier of the contained components
#[derive(Debug)]
pub struct EntityTypeIdentifier(pub Vec<String>);

impl PartialEq for EntityTypeIdentifier {
    fn eq(&self, other: &EntityTypeIdentifier) -> bool {
        let matching = self.0
            .iter()
            .zip(other.0.iter())
            .filter(|&(a, b)| a == b).count();
        
        matching == self.0.len() && matching == other.0.len()
    }
}

impl Eq for EntityTypeIdentifier { }

impl Hash for EntityTypeIdentifier {
    fn hash<H>(&self, state: &mut H) where H: std::hash::Hasher {
        self.0.hash(state)
    }
}

/// Create an entity type, use it like entity_type!["Component1", "Component2"])
#[macro_export]
macro_rules! entity_type {
    ($e:expr) => {{
        let component_names: Vec<&str> = vec![$e];
        fruity_ecs::entity::entity::EntityTypeIdentifier(component_names
            .iter()
            .map(|e| e.to_string())
            .collect())
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

impl Eq for EntityId { }

impl Hash for EntityId {
    fn hash<H>(&self, state: &mut H) where H: std::hash::Hasher {
        self.0.hash(state)
    }
}

/// An abstraction over an entity, should be implemented for every entity
pub trait Entity: Debug + Any + Send + Sync {
    /// Get a specific component by it's index
    /// 
    /// Return an abstraction of the component as [’Component’]
    ///
    /// # Arguments
    /// * `index` - The index of the component
    ///
    fn get(&self, index: usize) -> Option<&dyn Component>;

    /// Get a specific component by it's index with mutability
    /// 
    /// Return an abstraction of the component as [’Component’]
    ///
    /// # Arguments
    /// * `index` - The index of the component
    ///
    fn get_mut(&mut self, index: usize) -> Option<&mut dyn Component>;

    /// Get the number of components stored by the entity
    fn len(&self) -> usize;
}

impl dyn Entity {
    /// Returns the entity type identifier of the entity
    pub fn get_type_identifier(&self) -> EntityTypeIdentifier {
        EntityTypeIdentifier (
            self
                .untyped_iter()
                .map(|component| component.get_component_type())
                .collect()
        )
    }

    /// Iterate over all the component of the entity
    /// Return abstractions of the components as [’Component’]
    pub fn untyped_iter(&self) -> Iter {
        let indexes = (0..self.len()).map(usize::from).collect();

        Iter {
            entity: self,
            indexes,
        }
    }

    /// Iterate over all the component of the entity with mutability
    /// Return abstractions of the components as [’Component’]
    pub fn untyped_iter_mut(&mut self) -> IterMut {
        let indexes = (0..self.len()).map(usize::from).collect();

        IterMut {
            entity: self,
            indexes,
        }
    }
    
    /// Iterate over specified components of the entity
    /// 
    /// Cause an entity can contain multiple component of the same type, can returns multiple component list
    /// 
    /// Return abstractions of the components as [’Component’]
    ///
    /// # Arguments
    /// * `type_identifiers` - The identifier list of the components, components will be returned with the same order
    ///
    pub fn untyped_iter_over_types(&self, target_identifier: Vec<String>) -> OverTypesIter {
        let intern_identifier = self.get_type_identifier();
        let types_list = target_identifier
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
                        .collect::<VecDeque<_>>()
                })
                .multi_cartesian_product()
                .map(|vec| VecDeque::from(vec))
                .collect::<VecDeque<_>>();

        OverTypesIter {
            entity: self,
            types_list,
        }
    }
    
    /// Iterate over specified components of the entity with mutability
    /// 
    /// Cause an entity can contain multiple component of the same type, can returns multiple component list
    /// 
    /// Return abstractions of the components as [’Component’]
    ///
    /// # Arguments
    /// * `type_identifiers` - The identifier list of the components, components will be returned with the same order
    ///
    pub fn untyped_iter_mut_over_types(&mut self, target_identifier: EntityTypeIdentifier) -> OverTypesIterMut {
        let intern_identifier = self.get_type_identifier();
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
                        .collect::<VecDeque<_>>()
                })
                .multi_cartesian_product()
                .map(|vec| VecDeque::from(vec))
                .collect::<VecDeque<_>>();

        OverTypesIterMut {
            entity: self,
            types_list,
        }
    }
    
    /// Iterate over all the component that share the same type
    /// Return the typed component
    pub fn iter<T: Component>(&self) -> impl Iterator<Item = &T> {
        self
            .untyped_iter()
            .filter_map(|component| component.downcast_ref::<T>())
    }

    /// Iterate over all the component that share the same type with mutability
    /// Return the typed component
    pub fn iter_mut<T: Component>(&mut self) -> impl Iterator<Item = &mut T> {
        self
            .untyped_iter_mut()
            .filter_map(|component| component.downcast_mut::<T>())
    }
}

/// An iterator over all the component of an entity
pub struct Iter<'s> {
    entity: &'s dyn Entity,
    indexes: VecDeque<usize>,
}

impl<'s> Iterator for Iter<'s> {
    type Item = &'s dyn Component;

    fn next(&mut self) -> Option<&'s dyn Component> {
        match self.indexes.pop_front() {
            Some(index) => self.entity.get(index),
            None => None
        }
    }
}

/// An iterator over all the component of an entity with mutability
pub struct IterMut<'s> {
    entity: &'s mut dyn Entity,
    indexes: VecDeque<usize>,
}

impl<'s> Iterator for IterMut<'s> {
    type Item = &'s mut dyn Component;

    fn next(&mut self) -> Option<&'s mut dyn Component> {
        match self.indexes.pop_front() {
            Some(index) => match self.entity.get_mut(index) {
                Some(component) => Some(unsafe { &mut *(component as *mut _) }),
                None => None,
            },
            None => None
        }
    }
}

/// An iterator over all the component of an entity
pub struct OverTypesIter<'s> {
    entity: &'s dyn Entity,
    types_list: VecDeque<VecDeque<usize>>,
}

/// An iterator over specified components of the entity
/// 
/// Cause an entity can contain multiple component of the same type, can returns multiple component list
/// 
/// Return abstractions of the components as [’Component’]
/// 
impl<'s> Iterator for OverTypesIter<'s> {
    type Item = Iter<'s>;

    fn next(&mut self) -> Option<Iter<'s>> {
        match self.types_list.pop_front() {
            Some(type_indexes) => Some(Iter {
                entity: self.entity,
                indexes: type_indexes,
            }),
            None => None,
        }
    }
}

/// An iterator over specified components of the entity with mutability
/// 
/// Cause an entity can contain multiple component of the same type, can returns multiple component list
/// 
/// Return abstractions of the components as [’Component’]
/// 
pub struct OverTypesIterMut<'s> {
    entity: &'s mut dyn Entity,
    types_list: VecDeque<VecDeque<usize>>,
}

impl<'s> Iterator for OverTypesIterMut<'s> {
    type Item = IterMut<'s>;

    fn next(&mut self) -> Option<IterMut<'s>> {
        match self.types_list.pop_front() {
            Some(type_indexes) => Some(IterMut {
                entity: unsafe { &mut *(self.entity as *mut _) },
                indexes: type_indexes,
            }),
            None => None,
        }
    }
}