use crate::component::component::AnyComponent;
use crate::component::component::ComponentDecoder;
use crate::component::component::ComponentDecoderMut;
use crate::entity::archetype::rwlock::EntityRwLock;
use crate::entity::archetype::rwlock::EntityRwLockWeak;
use crate::entity::entity::EntityId;
use crate::entity::entity::EntityTypeIdentifier;
use crate::entity::entity_manager::RemoveEntityError;
use crate::utils::slice::copy;
use std::collections::HashMap;
use std::mem::size_of;
use std::sync::Arc;
use std::sync::RwLock;

pub(crate) struct ComponentDecodingInfos {
    pub(crate) relative_index: usize,
    pub(crate) size: usize,
    pub(crate) decoder: ComponentDecoder,
    pub(crate) decoder_mut: ComponentDecoderMut,
}

/// A collection of entities that share the same component structure
pub(crate) struct InnerArchetype {
    identifiers: EntityTypeIdentifier,
    index_map: HashMap<EntityId, usize>,
    pub(crate) buffer: Vec<u8>,
    pub(crate) components_per_entity: usize,
    pub(crate) entity_size: usize,
}

impl InnerArchetype {
    /// Returns an InnerArchetype and inject the first entity inside
    ///
    /// # Arguments
    /// * `identifier` - The archetype identifier
    /// * `components_per_entity` - The number of components stored in one entity
    /// * `all_components_size` - The sum of the size of each components in memory
    ///
    /// # Generic Arguments
    /// * `T` - The type of the entities stored into the archetype
    ///
    pub(crate) fn new(
        identifier: EntityTypeIdentifier,
        components_per_entity: usize,
        all_components_size: usize,
    ) -> InnerArchetype {
        InnerArchetype {
            identifiers: identifier,
            index_map: HashMap::new(),
            buffer: Vec::new(),
            components_per_entity,
            entity_size: size_of::<EntityRwLock>()
                + components_per_entity * size_of::<ComponentDecodingInfos>()
                + all_components_size,
        }
    }

    /// Returns the entity type identifier of the archetype
    pub(crate) fn get_type_identifier(&self) -> &EntityTypeIdentifier {
        &self.identifiers
    }

    /// Get a locked entity
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    ///
    pub(crate) fn get(&self, entity_id: EntityId) -> Option<EntityRwLockWeak> {
        self.index_map
            .get(&entity_id)
            .map(|index| self.get_by_index(*index))
    }

    /// Get a locked entity by first component index
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    ///
    pub(crate) fn get_by_index(&self, index: usize) -> EntityRwLockWeak {
        let buffer_end = index + size_of::<EntityRwLock>();
        let entity_lock_buffer = &self.buffer[index..buffer_end];
        let (_head, body, _tail) = unsafe { entity_lock_buffer.align_to::<EntityRwLock>() };
        body[0].create_new_weak()
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
    pub(crate) fn add(
        this: &Arc<RwLock<Self>>,
        entity_id: EntityId,
        components: Vec<AnyComponent>,
    ) {
        let mut writer = this.write().unwrap();

        // Store informations about where the object is stored
        let entity_index = writer.entity_size;
        let mut entity_buffer: Vec<u8> = vec![0; writer.entity_size];

        // Store the rwlock
        let rwlock = EntityRwLock::new(this.clone());
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
        for component in components.into_iter() {
            let reader = component.read().unwrap();

            let buffer_index = component_buffer_index;
            let buffer_end = component_buffer_index + reader.encode_size();
            let component_buffer = &mut entity_buffer[buffer_index..buffer_end];
            reader.encode(component_buffer);

            component_buffer_index += reader.encode_size();

            // TODO: Remove that one day
            // Exists to prevent the nested values to be droped
            std::mem::drop(reader);
            std::mem::forget(component);
        }

        // Store the entity
        writer.buffer.append(&mut entity_buffer);

        // Store the id of the entity
        writer.index_map.insert(entity_id, entity_index);
    }

    /// Remove an entity based on its id
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    ///
    pub(crate) fn remove(
        &mut self,
        _entity_id: EntityId,
    ) -> Result<EntityRwLock, RemoveEntityError> {
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

#[cfg(test)]
mod tests {
    #[derive(Debug, Clone, Component, FruityAny)]
    struct Component1 {
        pub(crate) field1: f32,
        pub(crate) field2: usize,
    }

    #[derive(Debug, Clone, Component, FruityAny)]
    struct Component2 {
        pub(crate) field1: String,
        pub(crate) field2: usize,
    }

    #[test]
    fn create_() {
        assert_eq!(2 + 2, 4);
    }
}
