use crate::component::component::Component;
use crate::component::component::ComponentDecoder;
use crate::component::component::ComponentDecoderMut;
use itertools::Itertools;
use std::collections::VecDeque;
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

pub(crate) struct EntityComponentInfo {
    buffer_start: usize,
    buffer_end: usize,
    decoder: ComponentDecoder,
    decoder_mut: ComponentDecoderMut,
}

/// An entity that contains component, component are stored in a vector of u8
/// to compact datas and improve iteration
pub struct Entity {
    pub(crate) entry_infos: Vec<EntityComponentInfo>,
    pub(crate) buffer: Vec<u8>,
}

impl Entity {
    /// Returns a Entity
    pub fn new(components: Vec<Box<dyn Component>>) -> Entity {
        let mut entity = Entity {
            entry_infos: Vec::new(),
            buffer: Vec::new(),
        };

        for component in components {
            entity.push(component);
        }

        entity
    }

    fn push(&mut self, component: Box<dyn Component>) {
        // Store informations about where the object is stored
        let encode_size = component.encode_size();
        let object_buffer_start = self.buffer.len();
        let object_buffer_end = self.buffer.len() + encode_size;

        self.entry_infos.push(EntityComponentInfo {
            buffer_start: object_buffer_start,
            buffer_end: object_buffer_end,
            decoder: component.get_decoder(),
            decoder_mut: component.get_decoder_mut(),
        });

        // Encode the object to the buffer
        let object_buffer_start = self.buffer.len();
        let object_buffer_end = self.buffer.len() + encode_size;

        self.buffer.resize(self.buffer.len() + encode_size, 0);
        let object_buffer = &mut self.buffer[object_buffer_start..object_buffer_end];

        component.encode(object_buffer);
    }

    /// Returns the entity type identifier of the entity
    pub fn get_type_identifier(&self) -> EntityTypeIdentifier {
        EntityTypeIdentifier(
            self.iter()
                .map(|component| component.get_component_type())
                .collect(),
        )
    }

    /// Get a component from the entity
    ///
    /// # Arguments
    /// * `index` - The component index
    ///
    pub fn get(&self, index: usize) -> Option<&dyn Component> {
        let entry_info = match self.entry_infos.get(index) {
            Some(entry_info) => entry_info,
            None => return None,
        };

        let entry_buffer = &self.buffer[entry_info.buffer_start..entry_info.buffer_end];
        Some((entry_info.decoder)(entry_buffer))
    }

    /// Get a mutable component from the entity
    ///
    /// # Arguments
    /// * `index` - The entry index
    ///
    pub fn get_mut(&mut self, index: usize) -> Option<&mut dyn Component> {
        let entry_info = match self.entry_infos.get(index) {
            Some(entry_info) => entry_info,
            None => return None,
        };

        let entry_buffer = &mut self.buffer[entry_info.buffer_start..entry_info.buffer_end];
        Some((entry_info.decoder_mut)(entry_buffer))
    }

    /// Iterate over the components of the entity
    pub fn iter(&self) -> Iter<'_> {
        let indexes = (0..self.len()).map(usize::from).collect();

        Iter {
            entity: self,
            indexes,
        }
    }

    /// Iterate over the components of the entity with mutability
    pub fn iter_mut(&mut self) -> IterMut<'_> {
        let indexes = (0..self.len()).map(usize::from).collect();

        IterMut {
            entity: self,
            indexes,
        }
    }

    /// Iterate over all the component that share the same type
    pub fn iter_typed<T: Component>(&self) -> impl Iterator<Item = &T> {
        self.iter()
            .filter_map(|component| component.downcast_ref::<T>())
    }

    /// Iterate over all the component that share the same type with mutability
    pub fn iter_typed_mut<T: Component>(&mut self) -> impl Iterator<Item = &mut T> {
        self.iter_mut()
            .filter_map(|component| component.downcast_mut::<T>())
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
                intern_identifier
                    .0
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

    /// Iterate over specified components of the entity
    ///
    /// Cause an entity can contain multiple component of the same type, can returns multiple component list
    ///
    /// Return abstractions of the components as [’Component’]
    ///
    /// # Arguments
    /// * `type_identifiers` - The identifier list of the components, components will be returned with the same order
    ///
    pub fn iter_component_tuple(&self, target_identifier: &EntityTypeIdentifier) -> OverTypesIter {
        let intern_identifier = self.get_type_identifier();
        let types_list = target_identifier
            .clone()
            .0
            .into_iter()
            .map(|type_identifier| {
                intern_identifier
                    .0
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
    pub fn iter_mut_component_tuple(
        &mut self,
        target_identifier: &EntityTypeIdentifier,
    ) -> OverTypesIterMut {
        let intern_identifier = self.get_type_identifier();
        let types_list = target_identifier
            .0
            .clone()
            .into_iter()
            .map(|type_identifier| {
                intern_identifier
                    .0
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

    /// Return the components count stored in the entity
    pub fn len(&self) -> usize {
        self.entry_infos.len()
    }
}

/// Iterator over components of an Entity
pub struct Iter<'s> {
    entity: &'s Entity,
    indexes: VecDeque<usize>,
}

impl<'s> Iterator for Iter<'s> {
    type Item = &'s dyn Component;

    fn next(&mut self) -> Option<&'s dyn Component> {
        match self.indexes.pop_front() {
            Some(index) => self.entity.get(index),
            None => None,
        }
    }
}

/// Iterator over components of an Entity with mutability
pub struct IterMut<'s> {
    entity: &'s mut Entity,
    indexes: VecDeque<usize>,
}

impl<'s> Iterator for IterMut<'s> {
    type Item = &'s mut dyn Component;

    fn next(&mut self) -> Option<&'s mut dyn Component> {
        let entity = unsafe { &mut *(self.entity as *mut _) } as &mut Entity;
        match self.indexes.pop_front() {
            Some(index) => entity.get_mut(index),
            None => None,
        }
    }
}

/// An iterator over all the component of an entity
pub struct OverTypesIter<'s> {
    entity: &'s Entity,
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
    entity: &'s mut Entity,
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

impl Debug for Entity {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        let fmt_error = self.iter().find_map(|elem| match elem.fmt(formatter) {
            Ok(()) => None,
            Err(err) => Some(err),
        });

        match fmt_error {
            Some(err) => Err(err),
            None => Ok(()),
        }
    }
}
