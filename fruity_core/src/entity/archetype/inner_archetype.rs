use crate::component::component::AnyComponent;
use crate::component::component::ComponentDecoder;
use crate::component::component::ComponentDecoderMut;
use crate::entity::archetype::encode_entity::encode_entity;
use crate::entity::archetype::rwlock::EntitySharedRwLock;
use crate::entity::entity::EntityId;
use crate::entity::entity::EntityTypeIdentifier;
use crate::entity::entity_manager::RemoveEntityError;
use crate::signal::Signal;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

/// This store all the information related to a specific entity, is intended to be used by inner_archetype
/// Extern users are not supposed to have access to that
pub struct EntityCellHead {
    pub(crate) entity_id: EntityId,
    pub enabled: bool,
    pub(crate) deleted: bool,
    pub(crate) on_updated: Signal<()>,
    pub(crate) lock: RwLock<()>,
}

impl EntityCellHead {
    /// Returns a EntityCellHead
    pub(crate) fn new(entity_id: EntityId) -> EntityCellHead {
        EntityCellHead {
            entity_id,
            enabled: true,
            deleted: false,
            on_updated: Signal::new(),
            lock: RwLock::new(()),
        }
    }
}

pub(crate) struct ComponentDecodingInfos {
    pub(crate) relative_index: usize,
    pub(crate) size: usize,
    pub(crate) decoder: ComponentDecoder,
    pub(crate) decoder_mut: ComponentDecoderMut,
}

/// A collection of entities that share the same component structure
///
/// An archetype is basicaly an array of u8 that contain datas that are transmuted into real objects
/// The structure is complex, we will call the datas related to an entire entity a cell
/// Each cell is organized in this way :
///
/// |-----------------------------------------------|
/// | head: [’EntityHead’]                          | Contains the main properties of the entity
/// | - entity_id: [’EntityId’]                     |
/// | - deleted: bool                               |
/// | - enabled: bool                               |
/// |-----------------------------------------------|
/// | component 1 infos: [’ComponentDecodingInfos’] | Theses structures store informations about how to
/// | component 2 infos: [’ComponentDecodingInfos’] | encode/decode components
/// | component 3 infos: [’ComponentDecodingInfos’] |
/// |-----------------------------------------------|
/// | component1 datas: [u8]                        | The components datas
/// | component2 datas: [u8]                        |
/// | component3 datas: [u8]                        |
/// |-----------------------------------------------|
///
/// When an entity is deleted, it's marked as removed at the top of the entity cell, and the next insertion
/// will use this space to allocate
///
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
    pub(crate) fn new(
        identifier: EntityTypeIdentifier,
        components_per_entity: usize,
        entity_size: usize,
    ) -> InnerArchetype {
        InnerArchetype {
            identifiers: identifier,
            index_map: HashMap::new(),
            buffer: Vec::with_capacity(entity_size),
            components_per_entity,
            entity_size,
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
    pub(crate) fn get(
        &self,
        this: Arc<RwLock<InnerArchetype>>,
        entity_id: EntityId,
    ) -> Option<EntitySharedRwLock> {
        self.index_map
            .get(&entity_id)
            .map(|index| self.get_by_index(this, *index))
    }

    /// Get a locked entity by first component index
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    ///
    pub(crate) fn get_by_index(
        &self,
        this: Arc<RwLock<InnerArchetype>>,
        index: usize,
    ) -> EntitySharedRwLock {
        EntitySharedRwLock::new(this, index)
    }

    /// Add an entity into the archetype
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    /// * `entity` - The entity datas
    ///
    pub(crate) fn add(&mut self, entity_id: EntityId, components: Vec<AnyComponent>) {
        // Create then entity buffer
        let entity_index = self.buffer.len();
        let mut entity_buffer: Vec<u8> = vec![0; self.entity_size];
        encode_entity(entity_id, &mut entity_buffer, components);

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
    pub(crate) fn remove(
        &mut self,
        _entity_id: EntityId,
    ) -> Result<EntityCellHead, RemoveEntityError> {
        /*if let Some(entity_head) = self.get(entity_id) {
            let entity_head = entity_head.clone();

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

            Ok(entity_head)
        } else {*/
        Err(RemoveEntityError::NotFound)
        //}
    }
}
