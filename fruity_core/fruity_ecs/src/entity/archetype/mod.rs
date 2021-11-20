use crate::component::component::AnyComponent;
use crate::component::component::Component;
use crate::component::component::ComponentDecoder;
use crate::component::component::ComponentDecoderMut;
use crate::entity::archetype::encode_entity::encode_entity;
use crate::entity::archetype::encode_entity::entity_size;
use crate::entity::archetype::rwlock::EntitySharedRwLock;
use crate::entity::entity::get_type_identifier;
use crate::entity::entity::get_type_identifier_by_any;
use crate::entity::entity::EntityId;
use crate::entity::entity::EntityTypeIdentifier;
use crate::entity::entity_service::RemoveEntityError;
use fruity_core::signal::Signal;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;
use std::sync::RwLock;

/// Utils to encode and decode entity
pub mod encode_entity;

/// Provides a threadsafe lock for entities
pub mod rwlock;

/// A collection of entities that share the same component structure
pub struct Archetype {
    identifier: EntityTypeIdentifier,
    pub(crate) components_per_entity: usize,
    pub(crate) entity_size: usize,
    index_map: RwLock<HashMap<EntityId, usize>>,
    removed_entities: RwLock<Vec<usize>>,
    pub(crate) buffer: Arc<RwLock<Vec<u8>>>,
}

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

pub(crate) struct ComponentDecodingInfos {
    pub(crate) relative_index: usize,
    pub(crate) size: usize,
    pub(crate) decoder: ComponentDecoder,
    pub(crate) decoder_mut: ComponentDecoderMut,
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

impl Archetype {
    /// Returns an Archetype and inject the first entity inside
    ///
    /// # Arguments
    /// * `entity_id` - The first entity id
    /// * `components` - The first entity components
    ///
    /// # Generic Arguments
    /// * `T` - The type of the entities stored into the archetype
    ///
    pub fn new(entity_id: EntityId, name: String, components: Vec<AnyComponent>) -> Archetype {
        // Deduce the archetype properties from the components
        let identifier = get_type_identifier_by_any(&components);

        let components_per_entity = components.len();

        let entity_size: usize = entity_size(&components);

        // Build the archetype
        let archetype = Archetype {
            identifier,
            index_map: RwLock::new(HashMap::new()),
            removed_entities: RwLock::new(Vec::new()),
            buffer: Arc::new(RwLock::new(Vec::with_capacity(entity_size))),
            components_per_entity,
            entity_size,
        };

        // Create the first entity
        archetype.add(entity_id, name, components);

        archetype
    }

    /// Returns the entity type identifier of the archetype
    pub fn get_type_identifier(&self) -> &EntityTypeIdentifier {
        &self.identifier
    }

    /// Get a locked entity
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    ///
    pub fn get(&self, entity_id: EntityId) -> Option<EntitySharedRwLock> {
        let buffer_index = {
            let index_map_reader = self.index_map.read().unwrap();
            index_map_reader.get(&entity_id).map(|elem| *elem)
        };

        if let Some(buffer_index) = buffer_index {
            self.get_by_index(buffer_index)
        } else {
            None
        }
    }

    /// Get a locked entity by first component index
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    ///
    pub fn get_by_index(&self, index: usize) -> Option<EntitySharedRwLock> {
        let buffer_len = {
            let buffer = self.buffer.read().unwrap();
            buffer.len()
        };

        if f32::floor(index as f32 / self.entity_size as f32)
            < f32::floor(buffer_len as f32 / self.entity_size as f32)
        {
            Some(EntitySharedRwLock::new(
                self.buffer.clone(),
                index,
                self.identifier.clone(),
                self.components_per_entity,
                self.entity_size,
            ))
        } else {
            None
        }
    }

    /// Iterate over all entities of the archetype
    pub(crate) fn iter(&self) -> Iter<'_> {
        Iter {
            archetype: self,
            current_index: 0,
            entity_size: self.entity_size,
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
    pub fn add(&self, entity_id: EntityId, name: String, components: Vec<AnyComponent>) {
        let free_cell = {
            let mut removed_entities = self.removed_entities.write().unwrap();
            removed_entities.pop()
        };

        // Use an existing entity cell if possible
        if let Some(free_cell) = free_cell {
            // Write directly into the entity buffer
            {
                let mut buffer = self.buffer.write().unwrap();
                let mut entity_buffer = &mut buffer[free_cell..(free_cell + self.entity_size)];
                encode_entity(entity_id, name, &mut entity_buffer, components);
            }

            let mut index_map = self.index_map.write().unwrap();
            index_map.insert(entity_id, free_cell);
        } else {
            // Create the entity buffer
            let entity_index = {
                let buffer = self.buffer.read().unwrap();
                buffer.len()
            };

            let mut entity_buffer: Vec<u8> = vec![0; self.entity_size];
            encode_entity(entity_id, name, &mut entity_buffer, components);

            // Store the entity
            {
                let mut buffer = self.buffer.write().unwrap();
                buffer.append(&mut entity_buffer);
            }

            // Store the id of the entity
            let mut index_map = self.index_map.write().unwrap();
            index_map.insert(entity_id, entity_index);
        }
    }

    /// Remove an entity based on its id
    ///
    /// # Arguments
    /// * `entity_id` - The entity id
    ///
    pub fn remove(&self, entity_id: EntityId) -> Result<(), RemoveEntityError> {
        let entity_index = {
            let mut index_map = self.index_map.write().unwrap();
            index_map.remove(&entity_id)
        };

        // Get the entity
        if let Some(entity_index) = entity_index {
            if let Some(entity) = self.get_by_index(entity_index) {
                // propagate the deleted signal
                {
                    let entity_reader = entity.read();
                    entity_reader.on_deleted.notify(());
                }

                // Update the entity
                {
                    let mut entity_writer = entity.write();
                    entity_writer.deleted = true;
                }

                // Remember that the old entity cell is now free
                // so we will be able to erase it
                {
                    let mut removed_entities = self.removed_entities.write().unwrap();
                    removed_entities.push(entity_index)
                };

                // TODO: Notify all the shared lock that the referenced entity has been removed

                Ok(())
            } else {
                Err(RemoveEntityError::NotFound)
            }
        } else {
            Err(RemoveEntityError::NotFound)
        }
    }
}

/// Iterator over entities of an archetype
pub struct Iter<'a> {
    /// The targeted archetype
    archetype: &'a Archetype,

    /// The memory size of a unique entity of the archetype
    entity_size: usize,

    /// A counter to know the iterator current index
    current_index: usize,
}

impl<'a> Iterator for Iter<'a> {
    type Item = EntitySharedRwLock;

    fn next(&mut self) -> Option<EntitySharedRwLock> {
        let entity = self.archetype.get_by_index(self.current_index);
        self.current_index += self.entity_size;

        if let Some(entity) = entity {
            // Skip if removed
            let entity_reader = entity.read();
            if entity_reader.deleted {
                std::mem::drop(entity_reader);
                self.next()
            } else {
                std::mem::drop(entity_reader);
                Some(entity)
            }
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
