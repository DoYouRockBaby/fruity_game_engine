use crate::slice::copy;
use std::any::TypeId;
use std::fmt::Debug;
use std::sync::Arc;

/// A function to decode an object from byte array to an any reference
pub type Decoder = fn(buffer: &[u8]) -> &dyn Encodable;

/// A function to decode an object from byte array to an any mutable reference
pub type DecoderMut = fn(buffer: &mut [u8]) -> &mut dyn Encodable;

/// An interface that any object stored in the trait vec should implement
pub trait Encodable: Debug {
    /// Get type id
    fn type_id(&self) -> TypeId;

    /// Return the size that is required to encode the object
    fn encode_size(&self) -> usize;

    /// Encode the object to a byte array
    ///
    /// # Arguments
    /// * `buffer` - The buffer where the encoder will write, should match the result of encode_size function
    ///
    fn encode(&self, buffer: &mut [u8]);

    /// Return a function to decode an object from byte array to an any reference
    fn get_decoder(&self) -> Decoder;

    /// Return a function to decode an object from byte array to an any mutable reference
    fn get_decoder_mut(&self) -> DecoderMut;
}

impl<T: Encodable + 'static> Encodable for Arc<T> {
    fn type_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }

    fn encode_size(&self) -> usize {
        std::mem::size_of::<Arc<()>>() + self.as_ref().encode_size()
    }

    fn encode(&self, buffer: &mut [u8]) {
        let test1 = (Arc::as_ptr(self) as usize);
        let test2 = (self as *const Arc<T> as usize);
        let test3 = (Arc::as_ptr(self) as usize) - (self as *const Arc<T> as usize);
        let inner_encode_index = (Arc::as_ptr(self) as usize) - (self as *const Arc<T> as usize);

        // We encode the arc
        let encoded_arc = unsafe {
            std::slice::from_raw_parts(
                (&*self as *const Self) as *const u8,
                std::mem::size_of::<Self>(),
            )
        };
        copy(buffer, encoded_arc);

        // We encode the inner object
        self.as_ref().encode(&mut buffer[inner_encode_index..]);
    }

    fn get_decoder(&self) -> Decoder {
        |buffer| {
            // If the size of the provided buffer don't match the one of the object, it means that
            // there is extra storage in the buffer that should be handled by the inner decoder
            if std::mem::size_of::<Arc<T>>() != buffer.len() {
                // We decode the arc first to get the inner decoder
                let (_head, body, _tail) = unsafe { buffer.align_to::<Self>() };
                let arc = &body[0];

                // Get the inner object index in the buffer
                let inner_encode_index =
                    (Arc::as_ptr(arc) as usize) - (arc as *const Arc<T> as usize);

                // We decode the inner object
                let inner_decoder = arc.as_ref().get_decoder();
                inner_decoder(&buffer[inner_encode_index..]);
            }

            // We decode the arc
            let (_head, body, _tail) = unsafe { buffer.align_to::<Self>() };
            &body[0]
        }
    }

    fn get_decoder_mut(&self) -> DecoderMut {
        |buffer| {
            // If the size of the provided buffer don't match the one of the object, it means that
            // there is extra storage in the buffer that should be handled by the inner decoder
            if std::mem::size_of::<Arc<T>>() != buffer.len() {
                // We decode the arc first to get the inner decoder
                let (_head, body, _tail) = unsafe { buffer.align_to::<Self>() };
                let arc = &body[0];

                // Get the inner object index in the buffer
                let inner_encode_index =
                    (Arc::as_ptr(arc) as usize) - (arc as *const Arc<T> as usize);

                // We decode the inner object
                let inner_decoder = arc.as_ref().get_decoder_mut();
                inner_decoder(&mut buffer[inner_encode_index..]);
            }

            // We decode the arc
            let (_head, body, _tail) = unsafe { buffer.align_to_mut::<Self>() };
            &mut body[0]
        }
    }
}
