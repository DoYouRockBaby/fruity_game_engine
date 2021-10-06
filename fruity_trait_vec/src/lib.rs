//! Trait Vec
//!
//! Implements a collection that can store multiple object of the same type but without knowing the type
//! This object should implement a specific interface to know the size and to serialize/deserialize it
//! as the desired trait.

use std::any::Any;

pub type TraitVecObjectDeserializer = fn(buffer: &[u8]) -> &dyn TraitVecObject;
pub type TraitVecObjectDeserializerMut = fn(buffer: &mut [u8]) -> &mut dyn TraitVecObject;

pub trait TraitVecObject: Any {
    fn get_deserializer(&self) -> TraitVecObjectDeserializer;
    fn get_deserializer_mut(&self) -> TraitVecObjectDeserializerMut;
    fn serialize(&self) -> Vec<u8>;
}

pub struct TraitVecObjectInfo {
    buffer_index: usize,
    deserializer: TraitVecObjectDeserializer,
    deserializer_mut: TraitVecObjectDeserializerMut,
}

pub struct TraitVec {
    object_infos: Vec<TraitVecObjectInfo>,
    buffer: Vec<u8>,
}

impl TraitVec {
    pub fn new() -> TraitVec {
        TraitVec {
            object_infos: Vec::new(),
            buffer: Vec::new(),
        }
    }

    pub fn get<T: TraitVecObject>(&self, index: usize) -> Option<&dyn TraitVecObject> {
        let object_info = match self.object_infos.get(index) {
            Some(object_info) => object_info,
            None => return None,
        };

        let object_buffer = &self.buffer[object_info.buffer_index..];
        Some((object_info.deserializer)(object_buffer))
    }

    pub fn get_mut<T: TraitVecObject>(&mut self, index: usize) -> Option<&dyn TraitVecObject> {
        let object_info = match self.object_infos.get(index) {
            Some(object_info) => object_info,
            None => return None,
        };

        let object_buffer = &mut self.buffer[object_info.buffer_index..];
        Some((object_info.deserializer_mut)(object_buffer))
    }

    pub fn len(&self) -> usize {
        self.object_infos.len()
    }

    pub fn push(&mut self, new_object: impl TraitVecObject) {
        let mut serialized = new_object.serialize();

        self.object_infos.push(TraitVecObjectInfo {
            buffer_index: self.buffer.len(),
            deserializer: new_object.get_deserializer(),
            deserializer_mut: new_object.get_deserializer_mut(),
        });

        self.buffer.append(&mut serialized);
    }

    pub fn remove(&mut self, index: usize) {}
}
