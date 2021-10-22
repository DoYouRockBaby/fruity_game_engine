use crate::encodable::Decoder;
use crate::encodable::DecoderMut;
use crate::encodable::Encodable;
use std::fmt::Debug;
use std::ops::Range;

struct EncodableVecEntryInfo {
    buffer_start: usize,
    buffer_end: usize,
    decoder: Decoder,
    decoder_mut: DecoderMut,
}

/// A collection that can store multiple object that implements the encodable trait
/// The stored information is organized to be compacted in memory to improve the iteration performances
pub struct EncodableVec {
    entry_infos: Vec<EncodableVecEntryInfo>,
    buffer: Vec<u8>,
}

impl EncodableVec {
    /// Returns an EncodableVec
    pub fn new() -> EncodableVec {
        EncodableVec {
            entry_infos: Vec::new(),
            buffer: Vec::new(),
        }
    }

    /// Get an entry from the collection
    ///
    /// # Arguments
    /// * `index` - The entry index
    ///
    pub fn get(&self, index: usize) -> Option<&dyn Encodable> {
        let object_info = match self.entry_infos.get(index) {
            Some(object_info) => object_info,
            None => return None,
        };

        let object_buffer = &self.buffer[object_info.buffer_start..object_info.buffer_end];
        Some((object_info.decoder)(object_buffer))
    }

    /// Get a mutable entry from the collection
    ///
    /// # Arguments
    /// * `index` - The entry index
    ///
    pub fn get_mut(&mut self, index: usize) -> Option<&mut dyn Encodable> {
        let object_info = match self.entry_infos.get(index) {
            Some(object_info) => object_info,
            None => return None,
        };

        let object_buffer = &mut self.buffer[object_info.buffer_start..object_info.buffer_end];
        Some((object_info.decoder_mut)(object_buffer))
    }

    /// Iterate over the entries of the collection
    pub fn iter(&self) -> Iter<'_> {
        Iter {
            vec: self,
            current_index: 0,
        }
    }

    /// Iterate over the entries of the collection
    pub fn iter_mut(&mut self) -> IterMut<'_> {
        IterMut {
            vec: self,
            current_index: 0,
        }
    }

    /// Return the entry count stored in the collection
    pub fn len(&self) -> usize {
        self.entry_infos.len()
    }

    /// Add an entry into the collection
    ///
    /// # Arguments
    /// * `new_object` - The object that will be stored
    ///
    pub fn push<T>(&mut self, new_object: T)
    where
        T: Encodable,
    {
        // Store informations about where the object is stored
        let encode_size = new_object.encode_size();
        let object_buffer_start = self.buffer.len();
        let object_buffer_end = self.buffer.len() + encode_size;

        self.entry_infos.push(EncodableVecEntryInfo {
            buffer_start: object_buffer_start,
            buffer_end: object_buffer_end,
            decoder: new_object.get_decoder(),
            decoder_mut: new_object.get_decoder_mut(),
        });

        // Encode the object to the buffer
        self.buffer.resize(object_buffer_end, 0);
        let object_buffer = &mut self.buffer[object_buffer_start..object_buffer_end];

        new_object.encode(object_buffer);
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
        let start_index = object_info.buffer_start;
        let end_index = object_info.buffer_end;
        let size = end_index - start_index;
        self.buffer.drain(start_index..end_index);

        // Gap all existing indexes
        self.entry_infos.iter_mut().for_each(|object_info_2| {
            if object_info_2.buffer_start > start_index {
                object_info_2.buffer_start -= size;
                object_info_2.buffer_end -= size;
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

/// Iterator over entries of an EncodableVec
pub struct Iter<'s> {
    vec: &'s EncodableVec,
    current_index: usize,
}

impl<'s> Iterator for Iter<'s> {
    type Item = &'s dyn Encodable;

    fn next(&mut self) -> Option<&'s dyn Encodable> {
        let result = self.vec.get(self.current_index);
        self.current_index += 1;

        result
    }
}

/// Iterator over entries of an EncodableVec
pub struct IterMut<'s> {
    vec: &'s mut EncodableVec,
    current_index: usize,
}

impl<'s> Iterator for IterMut<'s> {
    type Item = &'s mut dyn Encodable;

    fn next(&mut self) -> Option<&'s mut dyn Encodable> {
        let vec = unsafe { &mut *(self.vec as *mut _) } as &mut EncodableVec;
        let result = vec.get_mut(self.current_index);
        self.current_index += 1;

        result
    }
}

impl Debug for EncodableVec {
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
