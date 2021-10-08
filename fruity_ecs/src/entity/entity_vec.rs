use crate::entity::entity_rwlock::EntityRwLock;
use std::fmt::Debug;
use std::ops::Range;

struct EntityVecEntryInfo {
    buffer_index: usize,
    size: usize,
}

/// A collection that can store multiple object of the same type but without knowing the type
pub struct EntityVec {
    entry_infos: Vec<EntityVecEntryInfo>,
    buffer: Vec<u8>,
}

impl EntityVec {
    /// Returns a EntityVec
    pub fn new() -> EntityVec {
        EntityVec {
            entry_infos: Vec::new(),
            buffer: Vec::new(),
        }
    }

    /// Get an entry from the collection
    ///
    /// # Arguments
    /// * `index` - The entry index
    ///
    pub fn get(&self, index: usize) -> Option<&EntityRwLock> {
        let object_info = match self.entry_infos.get(index) {
            Some(object_info) => object_info,
            None => return None,
        };

        let object_buffer = &self.buffer[object_info.buffer_index..];

        Some(EntityRwLock::decode(object_buffer))
    }

    /// Iterate over the entries of the collection
    pub fn iter(&self) -> Iter<'_> {
        Iter {
            vec: self,
            current_index: 0,
        }
    }

    /// Return the entry count stored in the collection
    pub fn len(&self) -> usize {
        self.entry_infos.len()
    }

    /// Add an entity into the collection
    ///
    /// # Arguments
    /// * `new_object` - The object that will be stored
    ///
    pub fn push(&mut self, new_object: EntityRwLock) {
        // Encode the object to the buffer
        let encode_size = new_object.encode_size();
        let object_buffer_start = self.buffer.len();
        let object_buffer_end = self.buffer.len() + encode_size;

        self.buffer.resize(self.buffer.len() + encode_size, 0);
        let object_buffer = &mut self.buffer[object_buffer_start..object_buffer_end];

        new_object.encode(object_buffer);

        // Store informations about where the object is stored
        self.entry_infos.push(EntityVecEntryInfo {
            buffer_index: self.buffer.len(),
            size: encode_size,
        });
    }

    /// Remove an entry of the collection
    ///
    /// # Arguments
    /// * `index` - The entry index
    ///
    pub fn remove(&mut self, index: usize) {
        // Remove old stored id
        let object_info = self.entry_infos.remove(index);

        // Remove associated binary datas
        let start_index = object_info.buffer_index;
        let end_index = object_info.buffer_index + object_info.size;
        self.buffer.drain(start_index..end_index);

        // Gap all existing indexes
        self.entry_infos.iter_mut().for_each(|object_info| {
            if object_info.buffer_index > start_index {
                object_info.buffer_index -= object_info.size;
            }
        });
    }

    /// Remove many entries of the collection
    ///
    /// # Arguments
    /// * `index` - The entry index
    ///
    pub fn drain(&mut self, range: Range<usize>) {
        let start = range.start;
        for _ in range {
            self.remove(start);
        }
    }
}

/// Iterator over entries of a EntityVec
pub struct Iter<'s> {
    vec: &'s EntityVec,
    current_index: usize,
}

impl<'s> Iterator for Iter<'s> {
    type Item = &'s EntityRwLock;

    fn next(&mut self) -> Option<&'s EntityRwLock> {
        let result = self.vec.get(self.current_index);
        self.current_index += 1;

        result
    }
}

impl Debug for EntityVec {
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
