use fruity_any::FruityAny;
use std::any::TypeId;
use std::fmt::Debug;

/// A function to decode an object from byte array to an any reference
pub type Decoder = fn(buffer: &[u8]) -> &dyn Encodable;

/// A function to decode an object from byte array to an any mutable reference
pub type DecoderMut = fn(buffer: &mut [u8]) -> &mut dyn Encodable;

/// An interface that any object stored in the trait vec should implement
pub trait Encodable: FruityAny + Debug {
    /// Get type id
    fn type_id(&self) -> TypeId;

    /// Return the size that is required to encode the object
    fn encode_size(&self) -> usize;

    /// Encode the object to a byte array
    ///
    /// # Arguments
    /// * `buffer` - The buffer where the encoder will write, should match the result of encode_size function
    ///
    fn encode(self: Box<Self>, buffer: &mut [u8]);

    /// Return a function to decode an object from byte array to an any reference
    fn get_decoder(&self) -> Decoder;

    /// Return a function to decode an object from byte array to an any mutable reference
    fn get_decoder_mut(&self) -> DecoderMut;
}
