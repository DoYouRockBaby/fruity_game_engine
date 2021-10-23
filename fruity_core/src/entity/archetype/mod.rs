use crate::component::component::AnyComponent;
use crate::component::component::Component;
use crate::component::component::ComponentDecoder;
use crate::component::component::ComponentDecoderMut;
use crate::entity::archetype::rwlock::EntityRwLock;
use crate::entity::entity::EntityId;
use crate::entity::entity::EntityTypeIdentifier;
use crate::entity::entity_manager::RemoveEntityError;
use crate::utils::slice::copy;
use std::collections::HashMap;
use std::fmt::Debug;
use std::mem::size_of;
use std::sync::atomic::AtomicUsize;

/// Provides a threadsafe lock for entities
pub mod rwlock;

struct EntityLockCell {
    // If it's a writer that handle the lock, the reader_count is maximal cause we cannot create more
    // Otherwise, this is incremented by one per reader added
    reader_count: AtomicUsize,
}

impl EntityLockCell {
    pub fn new() -> EntityLockCell {
        EntityLockCell {
            reader_count: AtomicUsize::new(0),
        }
    }
}

struct ComponentDecodingInfos {
    relative_index: usize,
    size: usize,
    decoder: ComponentDecoder,
    decoder_mut: ComponentDecoderMut,
}

/// A collection of entities that share the same component structure
pub struct Archetype {
    identifiers: EntityTypeIdentifier,
    index_map: HashMap<EntityId, usize>,
    pub(crate) buffer: Vec<u8>,
    pub(crate) components_per_entity: usize,
    pub(crate) entity_size: usize,
}

impl Archetype {
    /// Returns an Archetype and inject the first entity inside
    ///
    /// # Arguments
    /// * `entity_id` - The first entity id
    /// * `entity` - The first entity datas
    ///
    /// # Generic Arguments
    /// * `T` - The type of the entities stored into the archetype
    ///
    pub fn new(entity_id: EntityId, components: Vec<AnyComponent>) -> Archetype {
        // Build identifier
        let identifier = components
            .iter()
            .map(|component| component.get_component_type())
            .collect::<Vec<_>>();

        let identifier = EntityTypeIdentifier(identifier);

        // Create the archetype
        let components_size: usize = components
            .iter()
            .map(|component| component.encode_size())
            .sum();

        let mut archetype = Archetype {
            identifiers: identifier,
            index_map: HashMap::new(),
            buffer: Vec::new(),
            components_per_entity: components.len(),
            entity_size: size_of::<EntityLockCell>()
                + components.len() * size_of::<ComponentDecodingInfos>()
                + components_size,
        };

        // Add the first entity
        archetype.add(entity_id, components);
        archetype
    }

    /// Returns the entity type identifier of the archetype
    pub fn get_type_identifier(&self) -> &EntityTypeIdentifier {
        &self.identifiers
    }

    /// Get a locked entity
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    ///
    pub fn get(&self, entity_id: EntityId) -> Option<EntityRwLock> {
        self.index_map
            .get(&entity_id)
            .map(|index| self.get_by_index(*index))
    }

    /// Get a locked entity by first component index
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    ///
    pub fn get_by_index(&self, index: usize) -> EntityRwLock {
        EntityRwLock::new(self, index)
    }

    /// Iterate over all entities of the archetype
    pub fn iter(&self) -> Iter<'_> {
        Iter {
            archetype: self,
            current_index: 0,
        }
    }

    /// Add an entity into the archetype
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    /// * `entity` - The entity datas
    ///
    /// # Generic Arguments
    /// * `T` - The type of the new entity
    ///
    pub fn add(&mut self, entity_id: EntityId, components: Vec<AnyComponent>) {
        // Store informations about where the object is stored
        let mut entity_buffer = Vec::<u8>::with_capacity(self.entity_size);

        // Store the rwlock
        let rwlock = EntityRwLock::new(self, self.buffer.len());
        let encoded_rwlock = unsafe {
            std::slice::from_raw_parts(
                (&*&rwlock as *const EntityRwLock) as *const u8,
                size_of::<Self>(),
            )
        };
        copy(&mut entity_buffer, encoded_rwlock);

        // Store the component decoding infos
        let decoding_infos_buffer_index = size_of::<EntityLockCell>();
        for (index, component) in components.iter().enumerate() {
            let buffer_index =
                decoding_infos_buffer_index + index * size_of::<ComponentDecodingInfos>();
            let buffer_end = buffer_index + self.entity_size;
            let infos_buffer = &mut entity_buffer[buffer_index..buffer_end];

            let decoding_infos = ComponentDecodingInfos {
                relative_index: decoding_infos_buffer_index,
                size: component.encode_size(),
                decoder: component.get_decoder(),
                decoder_mut: component.get_decoder_mut(),
            };

            let encoded_infos = unsafe {
                std::slice::from_raw_parts(
                    (&*&decoding_infos as *const ComponentDecodingInfos) as *const u8,
                    size_of::<Self>(),
                )
            };
            copy(infos_buffer, encoded_infos);
        }

        // Encode every components into the buffer
        let component_buffer_index =
            size_of::<EntityLockCell>() + components.len() * size_of::<ComponentDecodingInfos>();
        for component in components.iter() {
            let buffer_index = component_buffer_index;
            let buffer_end = component_buffer_index + component.encode_size();
            let component_buffer = &mut entity_buffer[buffer_index..buffer_end];
            component.encode(component_buffer);

            component_buffer_index += component.encode_size();
        }
    }

    /// Remove an entity based on its id
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    ///
    pub fn remove(&mut self, entity_id: EntityId) -> Result<EntityRwLock, RemoveEntityError> {
        /*if let Some(entity_rwlock) = self.get(entity_id) {
            let entity_rwlock = entity_rwlock.clone();

            // Find the entity index in the entity array
            let index = match self.index_map.remove(&entity_id) {
                Some(index) => Ok(index),
                None => Err(RemoveEntityError::NotFound),
            }?;

            // Remove old stored entity
            self.entities.remove(index);

            // Gap all existing indexes
            self.index_map.iter_mut().for_each(|index_2| {
                if *index_2.1 > index {
                    *index_2.1 -= 1;
                }
            });

            Ok(entity_rwlock)
        } else {*/
        Err(RemoveEntityError::NotFound)
        //}
    }
}

/// Iterator over entities of an archetype
pub struct Iter<'s> {
    /// The targeted archetype
    archetype: &'s Archetype,

    /// A counter to know the iterator current index
    current_index: usize,
}

impl<'s> Iterator for Iter<'s> {
    type Item = EntityRwLock;

    fn next(&mut self) -> Option<EntityRwLock> {
        if self.current_index < self.archetype.buffer.len() {
            let result = self.archetype.get_by_index(self.current_index);
            self.current_index += self.archetype.entity_size;

            Some(result)
        } else {
            None
        }
    }
}

impl Debug for Archetype {
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

#[cfg(test)]
mod tests {
    #[derive(Debug, Clone, Component, FruityAny)]
    struct Component1 {
        pub field1: f32,
        pub field2: usize,
    }

    #[derive(Debug, Clone, Component, FruityAny)]
    struct Component2 {
        pub field1: String,
        pub field2: usize,
    }

    #[test]
    fn create_() {
        assert_eq!(2 + 2, 4);
    }
}
