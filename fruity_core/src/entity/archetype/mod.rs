use crate::component::component::AnyComponent;
use crate::component::component::Component;
use crate::component::component::ComponentDecoder;
use crate::component::component::ComponentDecoderMut;
use crate::entity::archetype::rwlock::EntityRwLock;
use crate::entity::archetype::rwlock::EntityRwLockWeak;
use crate::entity::entity::get_type_identifier;
use crate::entity::entity::EntityId;
use crate::entity::entity::EntityTypeIdentifier;
use crate::entity::entity_manager::RemoveEntityError;
use crate::utils::slice::copy;
use std::collections::HashMap;
use std::fmt::Debug;
use std::mem::size_of;

/// Provides a threadsafe lock for entities
pub mod rwlock;

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
        let identifier = get_type_identifier(&components);

        // Create the archetype
        let all_components_size: usize = components
            .iter()
            .map(|component| {
                let reader = component.read().unwrap();
                reader.encode_size()
            })
            .sum();

        let mut archetype = Archetype {
            identifiers: identifier,
            index_map: HashMap::new(),
            buffer: Vec::new(),
            components_per_entity: components.len(),
            entity_size: size_of::<EntityRwLock>()
                + components.len() * size_of::<ComponentDecodingInfos>()
                + all_components_size,
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
    pub fn get(&self, entity_id: EntityId) -> Option<EntityRwLockWeak> {
        self.index_map
            .get(&entity_id)
            .map(|index| self.get_by_index(*index))
    }

    /// Get a locked entity by first component index
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    ///
    pub fn get_by_index(&self, index: usize) -> EntityRwLockWeak {
        let buffer_end = index + size_of::<EntityRwLock>();
        let entity_lock_buffer = &self.buffer[index..buffer_end];
        let (_head, body, _tail) = unsafe { entity_lock_buffer.align_to::<EntityRwLock>() };
        body[0].create_new_weak()
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
        let entity_index = self.entity_size;
        let mut entity_buffer: Vec<u8> = vec![0; self.entity_size];

        // Store the rwlock
        let rwlock = EntityRwLock::new(self);
        let encoded_rwlock = unsafe {
            std::slice::from_raw_parts(
                (&*&rwlock as *const EntityRwLock) as *const u8,
                size_of::<EntityRwLock>(),
            )
        };
        copy(&mut entity_buffer, encoded_rwlock);

        // Store the component decoding infos
        let mut relative_index = 0;
        let decoding_infos_buffer_index = size_of::<EntityRwLock>();
        for (index, component) in components.iter().enumerate() {
            let reader = component.read().unwrap();

            let buffer_index =
                decoding_infos_buffer_index + index * size_of::<ComponentDecodingInfos>();
            let buffer_end = buffer_index + size_of::<ComponentDecodingInfos>();
            let infos_buffer = &mut entity_buffer[buffer_index..buffer_end];

            let decoding_infos = ComponentDecodingInfos {
                relative_index,
                size: reader.encode_size(),
                decoder: reader.get_decoder(),
                decoder_mut: reader.get_decoder_mut(),
            };

            relative_index += reader.encode_size();

            let encoded_infos = unsafe {
                std::slice::from_raw_parts(
                    (&*&decoding_infos as *const ComponentDecodingInfos) as *const u8,
                    size_of::<Self>(),
                )
            };
            copy(infos_buffer, encoded_infos);
        }

        // Encode every components into the buffer
        let mut component_buffer_index =
            size_of::<EntityRwLock>() + components.len() * size_of::<ComponentDecodingInfos>();
        for component in components.iter() {
            let reader = component.read().unwrap();

            let buffer_index = component_buffer_index;
            let buffer_end = component_buffer_index + reader.encode_size();
            let component_buffer = &mut entity_buffer[buffer_index..buffer_end];
            reader.encode(component_buffer);

            component_buffer_index += reader.encode_size();
        }

        // Store the entity
        self.buffer.append(&mut entity_buffer);

        // Store the id of the entity
        self.index_map.insert(entity_id, entity_index);
    }

    /// Remove an entity based on its id
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    ///
    pub fn remove(&mut self, _entity_id: EntityId) -> Result<EntityRwLock, RemoveEntityError> {
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
    type Item = EntityRwLockWeak;

    fn next(&mut self) -> Option<EntityRwLockWeak> {
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
