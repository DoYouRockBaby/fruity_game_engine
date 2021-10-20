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
    pub fn new(mut components: Vec<Box<dyn Component>>) -> Entity {
        let mut entity = Entity {
            entry_infos: Vec::new(),
            buffer: Vec::new(),
        };

        // Sort to store in a more efficient way
        components.sort_by(|a, b| a.get_component_type().cmp(&b.get_component_type()));

        for component in components {
            entity.push(component);
        }

        let entity = entity;

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

        // Forget the component
        std::mem::forget(component);
    }

    /// Returns the entity type identifier of the entity
    pub fn get_type_identifier(&self) -> EntityTypeIdentifier {
        let identifier = self
            .iter()
            .map(|component| component.get_component_type())
            .collect::<Vec<_>>();

        EntityTypeIdentifier(identifier)
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

    /// Get a collection of component indexes
    /// Cause an entity can contain multiple component of the same type, can returns multiple component index list
    /// All components are mapped to the provided component identifiers in the same order
    ///
    /// # Arguments
    /// * `type_identifiers` - The identifier list of the components, components will be returned with the same order
    ///
    pub fn iter_component_indexes(
        &self,
        target_identifier: &EntityTypeIdentifier,
    ) -> impl Iterator<Item = Vec<usize>> {
        let intern_identifier = self.get_type_identifier();
        target_identifier
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
                    .collect::<Vec<_>>()
            })
            .multi_cartesian_product()
            .map(|vec| Vec::from(vec))
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
