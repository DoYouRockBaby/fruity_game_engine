use crate::component::component::Component;
use crate::component::component::ComponentDecoder;
use crate::component::component::ComponentDecoderMut;
use std::fmt::Debug;
use std::hash::Hash;

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

pub(crate) struct EntityComponentInfo {
    buffer_index: usize,
    decoder: ComponentDecoder,
    decoder_mut: ComponentDecoderMut,
}

pub struct Entity {
    pub(crate) entry_infos: Vec<EntityComponentInfo>,
    pub(crate) buffer: Vec<u8>,
}

impl Entity {
    /// Returns a Entity
    pub fn new(components: &[&dyn Component]) -> Entity {
        let mut entry_infos = Vec::new();
        let mut buffer = Vec::new();

        for component in components {
            let mut encoded = component.encode();

            entry_infos.push(EntityComponentInfo {
                buffer_index: buffer.len(),
                decoder: component.get_decoder(),
                decoder_mut: component.get_decoder_mut(),
            });

            buffer.append(&mut encoded);
        }

        Entity {
            entry_infos: entry_infos,
            buffer: buffer,
        }
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

        let entry_buffer = &self.buffer[entry_info.buffer_index..];
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

        let entry_buffer = &mut self.buffer[entry_info.buffer_index..];
        Some((entry_info.decoder_mut)(entry_buffer))
    }

    /// Iterate over the components of the entity
    pub fn iter(&self) -> Iter<'_> {
        Iter {
            entity: self,
            current_index: 0,
        }
    }

    /// Iterate over the components of the entity with mutability
    pub fn iter_mut(&mut self) -> IterMut<'_> {
        IterMut {
            entity: self,
            current_index: 0,
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
    current_index: usize,
}

impl<'s> Iterator for Iter<'s> {
    type Item = &'s dyn Component;

    fn next(&mut self) -> Option<&'s dyn Component> {
        let result = self.entity.get(self.current_index);
        self.current_index += 1;

        result
    }
}

/// Iterator over components of an Entity with mutability
pub struct IterMut<'s> {
    entity: &'s mut Entity,
    current_index: usize,
}

impl<'s> Iterator for IterMut<'s> {
    type Item = &'s mut dyn Component;

    fn next(&mut self) -> Option<&'s mut dyn Component> {
        let entity = unsafe { &mut *(self.entity as *mut _) } as &mut Entity;
        let result = entity.get_mut(self.current_index);
        self.current_index += 1;

        result
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
