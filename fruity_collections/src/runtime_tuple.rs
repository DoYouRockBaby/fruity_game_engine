use crate::encodable::Decoder;
use crate::encodable::Component;
use crate::runtime_tuple_reader::RuntimeTupleReader;
use crate::runtime_tuple_writer::RuntimeTupleWriter;
use std::any::TypeId;
use std::fmt::Debug;
use std::mem::size_of;

pub(crate) struct TupleEntryInfo {
    buffer_index: usize,
    size: usize,
    decoder: Decoder,
}

pub struct RuntimeTuple {
    entry_infos: Vec<TupleEntryInfo>,
    buffer: Vec<u8>,
}

impl RuntimeTuple {
    /// Returns a RuntimeTuple
    pub fn new(fields: &[&dyn Component]) -> RuntimeTuple {
        let mut entry_infos = Vec::new();
        let mut buffer = Vec::new();

        for field in fields {
            let mut encoded = field.encode();

            entry_infos.push(TupleEntryInfo {
                buffer_index: buffer.len(),
                size: encoded.len(),
                decoder: field.get_decoder(),
            });

            buffer.append(&mut encoded);
        }

        RuntimeTuple {
            entry_infos: entry_infos,
            buffer: buffer,
        }
    }

    /// Get an entry from the collection
    ///
    /// # Arguments
    /// * `index` - The entry index
    ///
    pub fn get(&self, index: usize) -> Option<RuntimeTupleReader> {
        let entry_info = match self.entry_infos.get(index) {
            Some(entry_info) => entry_info,
            None => return None,
        };

        let entry_buffer = &self.buffer[entry_info.buffer_index..];
        Some((entry_info.decoder)(entry_buffer))
    }

    /// Get an entry from the collection with mutability
    ///
    /// # Arguments
    /// * `index` - The entry index
    ///
    pub fn get_mut(&mut self, index: usize) -> Option<RuntimeTupleWriter> {
        let entry_info = match self.entry_infos.get(index) {
            Some(entry_info) => entry_info,
            None => return None,
        };

        let entry_buffer = &mut self.buffer[entry_info.buffer_index..];
        Some((entry_info.decoder)(entry_buffer))
    }

    /// Iterate over the entries of the collection
    pub fn iter(&mut self) -> Iter<'_> {
        Iter {
            tuple: self,
            current_index: 0,
        }
    }

    /// Return the entry count stored in the collection
    pub fn len(&self) -> usize {
        self.entry_infos.len()
    }
}

/// Iterator over entries of a RuntimeTuple
pub struct Iter<'s> {
    tuple: &'s mut RuntimeTuple,
    current_index: usize,
}

impl<'s> Iterator for Iter<'s> {
    type Item = RuntimeTupleReader<'s>;

    fn next(&mut self) -> Option<RuntimeTupleReader<'s>> {
        let tuple = unsafe { &mut *(self.tuple as *mut _) } as &mut RuntimeTuple;
        let result = tuple.get(self.current_index);
        self.current_index += 1;

        result
    }
}

/// Iterator over entries of a RuntimeTuple with mutability
pub struct IterMut<'s> {
    tuple: &'s mut RuntimeTuple,
    current_index: usize,
}

impl<'s> Iterator for IterMut<'s> {
    type Item = RuntimeTupleWriter<'s>;

    fn next(&mut self) -> Option<RuntimeTupleWriter<'s>> {
        let tuple = unsafe { &mut *(self.tuple as *mut _) } as &mut RuntimeTuple;
        let result = tuple.get_mut(self.current_index);
        self.current_index += 1;

        result
    }
}

impl Debug for RuntimeTuple {
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

impl<'a> Component for RuntimeTuple {
    fn type_id(&self) -> std::any::TypeId {
        TypeId::of::<RuntimeTuple>()
    }

    fn encode(&self) -> Vec<u8> {
        let mut result = Vec::new();
        /*let encoded_runtime_tuple = EncodedRuntimeTuple {
            entry_infos_len: self.entry_infos.len(),
        };

        // Append the base informations for the decoder
        result.append(&mut unsafe {
            std::slice::from_raw_parts(
                (&encoded_runtime_tuple as *const EncodedRuntimeTuple) as *const u8,
                std::mem::size_of::<EncodedRuntimeTuple>(),
            )
            .to_vec()
        });*/

        // Append each tuple entry info
        for entry_infos in &self.entry_infos {
            result.append(&mut unsafe {
                std::slice::from_raw_parts(
                    (entry_infos as *const TupleEntryInfo) as *const u8,
                    std::mem::size_of::<TupleEntryInfo>(),
                )
                .to_vec()
            });
        }

        // Append each tuple entry encoded buffer
        result.append(&mut self.buffer.to_vec());

        result
    }

    fn get_decoder(&self) -> Decoder {
        |data| {
            // Get the base informations for the decoding operation
            /*let (_head, body, tail) = unsafe { data.align_to::<EncodedRuntimeTuple>() };
            let mut remaining_buffer = tail;
            let encoded_runtime_tuple = &body[0];*/

            // Deserialize each tuple entry info
            let (_head, body, tail) = unsafe { data.align_to::<TupleEntryInfo>() };

            let entry_info_size = body.len() / size_of::<TupleEntryInfo>();
            let entry_infos = unsafe { std::slice::from_raw_parts(body.as_ptr(), entry_info_size) };

            // Deserialize tuple buffer
            let buffer_size = tail.len();
            let buffer = unsafe { std::slice::from_raw_parts(tail.as_ptr(), buffer_size) };

            // Create a tuple that target the datas in the buffer
            Box::new(RuntimeTupleReader {
                entry_infos,
                buffer,
            })
        }
    }
}
