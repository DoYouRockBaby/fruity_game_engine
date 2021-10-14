use crate::entity::entity::Entity;
use crate::entity::entity::EntityComponentInfo;
use crate::entity::entity_guard::EntityReadGuard;
use crate::entity::entity_guard::EntityWriteGuard;
use fruity_any_derive::*;
use fruity_collections::encodable::Decoder;
use fruity_collections::encodable::DecoderMut;
use fruity_collections::encodable::Encodable;
use fruity_collections::slice::copy;
use std::any::TypeId;
use std::sync::Arc;
use std::sync::PoisonError;
use std::sync::RwLock;
use std::sync::RwLockReadGuard;
use std::sync::RwLockWriteGuard;

/// A read write locker for an entity instance
#[derive(Debug, Clone, FruityAny)]
pub struct EntityRwLock {
    entity: Arc<RwLock<Entity>>,
}

impl EntityRwLock {
    /// Returns an RwLockReadGuard which is unlocked.
    ///
    /// # Arguments
    /// * `inner_guard` - The typed [`RwLockReadGuard`]
    ///
    pub fn new(entity: Entity) -> EntityRwLock {
        EntityRwLock {
            entity: Arc::new(RwLock::new(entity)),
        }
    }

    /// Locks this rwlock with shared read access, blocking the current thread
    /// until it can be acquired.
    ///
    /// # Errors
    ///
    /// This function will return an error if the RwLock is poisoned. An RwLock
    /// is poisoned whenever a writer panics while holding an exclusive lock.
    /// The failure will occur immediately after the lock has been acquired.
    ///
    /// # Panics
    ///
    /// This function might panic when called if the lock is already held by the current thread.
    ///
    //pub fn write(&self) -> Result<EntityWriteGuard<'s>, PoisonError<RwLockWriteGuard<()>>> {
    pub fn read(&self) -> Result<EntityReadGuard, PoisonError<RwLockReadGuard<Entity>>> {
        let guard = self.entity.read()?;
        Ok(EntityReadGuard::new(guard))
    }

    /// Locks this rwlock with exclusive write access, blocking the current
    /// thread until it can be acquired.
    ///
    /// # Errors
    ///
    /// This function will return an error if the RwLock is poisoned. An RwLock
    /// is poisoned whenever a writer panics while holding an exclusive lock.
    /// An error will be returned when the lock is acquired.
    ///
    /// # Panics
    ///
    /// This function might panic when called if the lock is already held by the current thread.
    ///
    pub fn write(&self) -> Result<EntityWriteGuard, PoisonError<RwLockWriteGuard<Entity>>> {
        let guard = self.entity.write()?;
        Ok(EntityWriteGuard::new(guard))
    }
}

impl Encodable for EntityRwLock {
    fn type_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }

    fn encode_size(&self) -> usize {
        let reader = self.read().unwrap();

        std::mem::size_of::<Self>()
            + std::mem::size_of::<EntityComponentInfo>() * reader.entry_infos.len()
            + reader.buffer.len()
    }

    fn encode(self: Box<Self>, buffer: &mut [u8]) {
        {
            let reader = self.read().unwrap();

            // Encode each tuple entry info
            for (index, entry_infos) in reader.entry_infos.iter().enumerate() {
                let encoded_entry_info = unsafe {
                    std::slice::from_raw_parts(
                        (entry_infos as *const EntityComponentInfo) as *const u8,
                        std::mem::size_of::<EntityComponentInfo>(),
                    )
                };

                let entry_info_index = std::mem::size_of::<EntityRwLock>()
                    + index * std::mem::size_of::<EntityComponentInfo>();

                copy(&mut buffer[entry_info_index..], encoded_entry_info);
            }

            // Encode entries buffer
            let buffer_index = std::mem::size_of::<EntityRwLock>()
                + reader.entry_infos.len() * std::mem::size_of::<EntityComponentInfo>();
            copy(&mut buffer[buffer_index..], &reader.buffer);

            // Encode the entity object
            let encoded_entity_object = unsafe {
                std::slice::from_raw_parts(
                    (&*self as *const Self) as *const u8,
                    std::mem::size_of::<Self>(),
                )
            };

            copy(buffer, encoded_entity_object);
        }

        std::mem::forget(self);
    }

    fn get_decoder(&self) -> Decoder {
        |buffer| {
            // Decode the base object
            let buffer_ptr = buffer.as_ptr();
            let buffer = unsafe {
                std::slice::from_raw_parts(buffer_ptr, std::mem::size_of::<EntityRwLock>())
            };

            let (_head, body, _tail) = unsafe { buffer.align_to::<EntityRwLock>() };
            let entity_rwlock = &body[0];

            // Update the vector pointers if the targeted buffer was moved
            let mut writer = entity_rwlock.write().unwrap();

            let entry_info_ptr = ((buffer_ptr as usize) + std::mem::size_of::<EntityRwLock>())
                as *mut EntityComponentInfo;

            if writer.entry_infos.as_ptr() != entry_info_ptr {
                writer.entry_infos = unsafe {
                    Vec::from_raw_parts(
                        entry_info_ptr,
                        writer.entry_infos.len(),
                        writer.entry_infos.len(),
                    )
                };
            }

            let entity_buffer_ptr = ((buffer_ptr as usize)
                + std::mem::size_of::<EntityRwLock>()
                + writer.entry_infos.len() * std::mem::size_of::<EntityComponentInfo>())
                as *mut u8;

            if writer.buffer.as_ptr() != entity_buffer_ptr {
                writer.buffer = unsafe {
                    Vec::from_raw_parts(entity_buffer_ptr, writer.buffer.len(), writer.buffer.len())
                }
            }

            entity_rwlock
        }
    }

    fn get_decoder_mut(&self) -> DecoderMut {
        |buffer| {
            // Decode the base object
            let buffer_ptr = buffer.as_mut_ptr();
            let buffer = unsafe {
                std::slice::from_raw_parts_mut(buffer_ptr, std::mem::size_of::<EntityRwLock>())
            };

            let (_head, body, _tail) = unsafe { buffer.align_to_mut::<EntityRwLock>() };
            let entity_rwlock = &mut body[0];

            // Update the vector pointers if the targeted buffer was moved
            {
                let mut writer = entity_rwlock.write().unwrap();

                let entry_info_ptr = ((buffer_ptr as usize) + std::mem::size_of::<EntityRwLock>())
                    as *mut EntityComponentInfo;

                if writer.entry_infos.as_ptr() != entry_info_ptr {
                    writer.entry_infos = unsafe {
                        Vec::from_raw_parts(
                            entry_info_ptr,
                            writer.entry_infos.len(),
                            writer.entry_infos.len(),
                        )
                    };
                }

                let entity_buffer_ptr = ((buffer_ptr as usize)
                    + std::mem::size_of::<EntityRwLock>()
                    + writer.entry_infos.len() * std::mem::size_of::<EntityComponentInfo>())
                    as *mut u8;

                if writer.buffer.as_ptr() != entity_buffer_ptr {
                    writer.buffer = unsafe {
                        Vec::from_raw_parts(
                            entity_buffer_ptr,
                            writer.buffer.len(),
                            writer.buffer.len(),
                        )
                    }
                }
            }

            entity_rwlock
        }
    }
}
