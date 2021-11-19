use crate::component::component::AnyComponent;
use crate::component::component::ComponentDecoder;
use crate::component::component::ComponentDecoderMut;
use crate::entity::archetype::encode_entity::encode_entity;
use crate::entity::archetype::rwlock::EntitySharedRwLock;
use crate::entity::entity::EntityId;
use crate::entity::entity::EntityTypeIdentifier;
use crate::entity::entity_service::RemoveEntityError;
use fruity_core::signal::Signal;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

/// This store all the information related to a specific entity, is intended to be used by inner_archetype
/// Extern users are not supposed to have access to that
pub struct EntityCellHead {
    /// The entity id
    pub entity_id: EntityId,

    /// the entity name
    pub name: String,

    /// If false, the entity will be ignored by the systems
    pub enabled: bool,

    /// A marker for an entity that is deleted but that is not yet free into memory
    pub deleted: bool,

    /// A signal that is sent when the a write lock on the entity is released
    pub on_deleted: Signal<()>,

    /// The locker for the entity, used to avoir multithread collisions
    pub(crate) lock: RwLock<()>,
}

impl EntityCellHead {
    /// Returns a EntityCellHead
    pub(crate) fn new(entity_id: EntityId, name: String) -> EntityCellHead {
        EntityCellHead {
            entity_id,
            name,
            enabled: true,
            deleted: false,
            on_deleted: Signal::new(),
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
/// | - enabled: bool     
/// | - ...                          |
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
    removed_entities: Vec<usize>,
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
            removed_entities: Vec::new(),
            buffer: Vec::with_capacity(entity_size),
            components_per_entity,
            entity_size,
        }
    }

    /// Returns the entity type identifier of the archetype
    pub(crate) fn get_type_identifier(&self) -> &EntityTypeIdentifier {
        &self.identifiers
    }

    /// Add an entity into the archetype
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    /// * `entity` - The entity datas
    ///
    pub(crate) fn add(&mut self, entity_id: EntityId, name: String, components: Vec<AnyComponent>) {
        // Use an existing entity cell if possible
        if let Some(free_cell) = self.removed_entities.pop() {
            // Write directly into the entity buffer
            let mut entity_buffer = &mut self.buffer[free_cell..(free_cell + self.entity_size)];
            encode_entity(entity_id, name, &mut entity_buffer, components);
            self.index_map.insert(entity_id, free_cell);
        } else {
            // Create then entity buffer
            let entity_index = self.buffer.len();
            let mut entity_buffer: Vec<u8> = vec![0; self.entity_size];
            encode_entity(entity_id, name, &mut entity_buffer, components);
            // Store the entity
            self.buffer.append(&mut entity_buffer);
            // Store the id of the entity
            self.index_map.insert(entity_id, entity_index);
        }
    }

    /// Get a locked entity
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    ///
    pub(crate) fn get(
        this: Arc<RwLock<InnerArchetype>>,
        entity_id: EntityId,
    ) -> Option<EntitySharedRwLock> {
        let this_reader = this.read().unwrap();
        this_reader
            .index_map
            .get(&entity_id)
            .map(|index| InnerArchetype::get_by_index(this.clone(), *index))
    }

    /// Get a locked entity by first component index
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    ///
    pub(crate) fn get_by_index(
        this: Arc<RwLock<InnerArchetype>>,
        index: usize,
    ) -> EntitySharedRwLock {
        EntitySharedRwLock::new(this, index)
    }

    /// Remove an entity based on its id
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    ///
    pub(crate) fn remove(
        this: Arc<RwLock<InnerArchetype>>,
        entity_id: EntityId,
    ) -> Result<(), RemoveEntityError> {
        let mut this_writer = this.write().unwrap();
        if let Some(entity_index) = this_writer.index_map.remove(&entity_id) {
            std::mem::drop(this_writer);

            // Get the write lock on the entity
            let entity = InnerArchetype::get_by_index(this.clone(), entity_index);
            let mut entity_writer = entity.write();
            entity_writer.on_deleted.notify(());
            entity_writer.deleted = true;
            std::mem::drop(entity_writer);

            // Remember that the old entity cell is now free
            // so we will be able to erase it
            let mut this_writer = this.write().unwrap();
            this_writer.removed_entities.push(entity_index);
            std::mem::drop(this_writer);

            // TODO: Notify all the shared lock that the referenced entity has been removed

            Ok(())
        } else {
            Err(RemoveEntityError::NotFound)
        }
    }
}
