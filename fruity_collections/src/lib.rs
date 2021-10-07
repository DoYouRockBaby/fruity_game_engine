#![warn(missing_docs)]

//! Trait Vec
//!
//! Implements a collection that can store multiple object of the same type but without knowing the type
//! This object should implement a specific interface to encode/decode it to the desired trait.
//!

mod encodable;
mod runtime_tuple;

/*use std::any::Any;
use std::fmt::Debug;
use std::marker::PhantomData;

struct TraitVecObjectInfo {
    buffer_index: usize,
    size: usize,
    decoder: TraitVecObjectDecoder,
    decoder_mut: TraitVecObjectDecoderMut,
}

/// A collection that can store multiple object of the same type but without knowing the type
pub struct TraitVec<T: TraitVecObject> {
    object_infos: Vec<TraitVecObjectInfo>,
    buffer: Vec<u8>,
    phantom: PhantomData<T>,
}

impl<T: TraitVecObject> TraitVec<T> {
    /// Returns a TraitVec
    pub fn new() -> TraitVec<T> {
        TraitVec::<T> {
            object_infos: Vec::new(),
            buffer: Vec::new(),
            phantom: PhantomData,
        }
    }

    /// Get an entry from the collection
    ///
    /// # Arguments
    /// * `index` - The entry index
    ///
    pub fn get(&self, index: usize) -> Option<&T> {
        let object_info = match self.object_infos.get(index) {
            Some(object_info) => object_info,
            None => return None,
        };

        let object_buffer = &self.buffer[object_info.buffer_index..];
        Some((object_info.decoder)(object_buffer))
            .map(|trait_object| trait_object.downcast_ref::<T>().unwrap())
    }

    /// Get a mutable entry from the collection
    ///
    /// # Arguments
    /// * `index` - The entry index
    ///
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        let object_info = match self.object_infos.get(index) {
            Some(object_info) => object_info,
            None => return None,
        };

        let object_buffer = &mut self.buffer[object_info.buffer_index..];
        Some((object_info.decoder_mut)(object_buffer))
            .map(|trait_object| trait_object.downcast_mut::<T>().unwrap())
    }

    /// Iterate over the entries of the collection
    pub fn iter(&self) -> Iter<'_, T> {
        Iter::<T> {
            vec: self,
            current_index: 0,
        }
    }

    /// Iterate over the entries of the collection with mutability
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            vec: self,
            current_index: 0,
        }
    }

    /// Return the entry count stored in the collection
    pub fn len(&self) -> usize {
        self.object_infos.len()
    }

    /// Add an entity into the collection
    ///
    /// # Arguments
    /// * `new_object` - The object that will be stored
    ///
    pub fn push(&mut self, new_object: impl TraitVecObject) {
        let mut encoded = new_object.encode();

        self.object_infos.push(TraitVecObjectInfo {
            buffer_index: self.buffer.len(),
            size: encoded.len(),
            decoder: new_object.get_decoder(),
            decoder_mut: new_object.get_decoder_mut(),
        });

        self.buffer.append(&mut encoded);
    }

    /// Remove an entry of the collection
    ///
    /// # Arguments
    /// * `index` - The entry index
    ///
    pub fn remove(&mut self, index: usize) {
        // Remove old stored id
        let object_info = self.object_infos.remove(index);

        // Remove associated binary datas
        let start_index = object_info.buffer_index;
        let end_index = object_info.buffer_index + object_info.size;
        self.buffer.drain(start_index..end_index);

        // Gap all existing indexes
        self.object_infos.iter_mut().for_each(|object_info| {
            if object_info.buffer_index > start_index {
                object_info.buffer_index -= object_info.size;
            }
        });
    }
}

/// Iterator over entries of a TraitVec
pub struct Iter<'s, T: TraitVecObject> {
    vec: &'s TraitVec<T>,
    current_index: usize,
}

impl<'s, T: TraitVecObject> Iterator for Iter<'s, T> {
    type Item = &'s T;

    fn next(&mut self) -> Option<&'s T> {
        let result = self.vec.get(self.current_index);
        self.current_index += 1;

        result
    }
}

impl<T: TraitVecObject + Debug> Debug for TraitVec<T> {
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

/// Iterator over entries of a TraitVec with mutability
pub struct IterMut<'s, T: TraitVecObject> {
    vec: &'s mut TraitVec<T>,
    current_index: usize,
}

impl<'s, T: TraitVecObject> Iterator for IterMut<'s, T> {
    type Item = &'s mut T;

    fn next(&mut self) -> Option<&'s mut T> {
        let vec = unsafe { &mut *(self.vec as *mut _) } as &mut TraitVec<T>;
        let result = vec.get_mut(self.current_index);
        self.current_index += 1;

        result
    }
}
*/
