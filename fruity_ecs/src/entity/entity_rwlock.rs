use crate::entity::entity::Entity;
use crate::entity::entity::EntityComponentInfo;
use crate::entity::entity_guard::EntityReadGuard;
use crate::entity::entity_guard::EntityWriteGuard;
use fruity_collections::slice::copy;
use std::sync::PoisonError;
use std::sync::RwLock;
use std::sync::RwLockReadGuard;
use std::sync::RwLockWriteGuard;

/// A read write locker for an entity instance
#[derive(Debug)]
pub struct EntityRwLock {
    entity: RwLock<Entity>,
}

impl EntityRwLock {
    /// Returns an RwLockReadGuard which is unlocked.
    ///
    /// # Arguments
    /// * `inner_guard` - The typed [`RwLockReadGuard`]
    ///
    pub fn new(entity: Entity) -> EntityRwLock {
        EntityRwLock {
            entity: RwLock::new(entity),
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

    pub fn encode_size(&self) -> usize {
        let reader = self.read().unwrap();

        let mut result = std::mem::size_of::<Self>();
        result += std::mem::size_of::<EntityComponentInfo>() * reader.entry_infos.len();
        result += reader.buffer.len();

        result
    }

    pub fn encode(&self, buffer: &mut [u8]) {
        // Encode the base object
        let mut result = unsafe {
            std::slice::from_raw_parts(
                (self as *const Self) as *const u8,
                std::mem::size_of::<Self>(),
            )
            .to_vec()
        };

        let reader = self.read().unwrap();
        // Append each tuple entry info
        for entry_infos in &reader.entry_infos {
            result.append(&mut unsafe {
                std::slice::from_raw_parts(
                    (entry_infos as *const EntityComponentInfo) as *const u8,
                    std::mem::size_of::<EntityComponentInfo>(),
                )
                .to_vec()
            });
        }

        // Append each tuple entry encoded buffer
        result.append(&mut reader.buffer.to_vec());

        // Modify the object to point vec references on the desired buffer
        {
            // Decode the base object
            let (_head, body, tail) = unsafe { buffer.align_to_mut::<EntityRwLock>() };
            let entity_rwlock = &body[0];

            // Decode each component info
            let (_head, body, tail) = unsafe { tail.align_to_mut::<EntityComponentInfo>() };
            let entry_info_size = body.len() / std::mem::size_of::<EntityComponentInfo>();
            let entry_infos =
                unsafe { Vec::from_raw_parts(body.as_mut_ptr(), entry_info_size, entry_info_size) };

            // Decode storage buffer
            let buffer_size = tail.len();
            let buffer =
                unsafe { Vec::from_raw_parts(tail.as_mut_ptr(), buffer_size, buffer_size) };

            // Make our entity point to the desired data pointers
            let mut entity_writer = entity_rwlock.write().unwrap();
            entity_writer.entry_infos = entry_infos;
            entity_writer.buffer = buffer;
        }

        // Copy everything into the buffer
        copy(buffer, &result);
    }

    pub fn decode(buffer: &[u8]) -> &EntityRwLock {
        // Decode the base object
        let (_head, body, tail) = unsafe { buffer.align_to::<EntityRwLock>() };
        &body[0]
    }

    pub fn decode_mut(buffer: &mut [u8]) -> &mut EntityRwLock {
        // Decode the base object
        let (_head, body, tail) = unsafe { buffer.align_to_mut::<EntityRwLock>() };
        &mut body[0]
    }
}
