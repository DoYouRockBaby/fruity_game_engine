use std::any::TypeId;
use std::fmt::Debug;

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

impl dyn Encodable {
    /// Returns `true` if the boxed type is the same as `T`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fruity_collection::encodable::Encodable;
    ///
    /// fn is_string(s: &dyn Encodable) {
    ///     if s.is::<String>() {
    ///         println!("It's a string!");
    ///     } else {
    ///         println!("Not a string...");
    ///     }
    /// }
    ///
    /// is_string(&0);
    /// is_string(&"cookie monster".to_string());
    /// ```
    ///
    pub fn is<T: Encodable + 'static>(&self) -> bool {
        // Get `TypeId` of the type this function is instantiated with.
        let t = TypeId::of::<T>();

        // Get `TypeId` of the type in the trait object (`self`).
        let concrete = self.type_id();

        // Compare both `TypeId`s on equality.
        t == concrete
    }

    /// Returns some reference to the boxed value if it is of type `T`, or
    /// `None` if it isn't.
    ///
    /// # Examples
    ///
    /// ```
    /// use fruity_collection::encodable::Encodable;
    ///
    /// fn print_if_string(s: &dyn Encodable) {
    ///     if let Some(string) = s.downcast_ref::<String>() {
    ///         println!("It's a string({}): '{}'", string.len(), string);
    ///     } else {
    ///         println!("Not a string...");
    ///     }
    /// }
    ///
    /// print_if_string(&0);
    /// print_if_string(&"cookie monster".to_string());
    /// ```
    ///
    pub fn downcast_ref<T: Encodable + 'static>(&self) -> Option<&T> {
        if self.is::<T>() {
            // SAFETY: just checked whether we are pointing to the correct type, and we can rely on
            // that check for memory safety because we have implemented Encodable for all types; no other
            // impls can exist as they would conflict with our impl.
            unsafe { Some(&*(self as *const dyn Encodable as *const T)) }
        } else {
            None
        }
    }

    /// Returns some mutable reference to the boxed value if it is of type `T`, or
    /// `None` if it isn't.
    ///
    /// # Examples
    ///
    /// ```
    /// use fruity_collection::encodable::Encodable;
    ///
    /// fn modify_if_u32(s: &mut dyn use Encodable) {
    ///     if let Some(num) = s.downcast_mut::<u32>() {
    ///         *num = 42;
    ///     }
    /// }
    ///
    /// let mut x = 10u32;
    /// let mut s = "starlord".to_string();
    ///
    /// modify_if_u32(&mut x);
    /// modify_if_u32(&mut s);
    ///
    /// assert_eq!(x, 42);
    /// assert_eq!(&s, "starlord");
    /// ```
    ///
    pub fn downcast_mut<T: Encodable + 'static>(&mut self) -> Option<&mut T> {
        if self.is::<T>() {
            // SAFETY: just checked whether we are pointing to the correct type, and we can rely on
            // that check for memory safety because we have implemented Encodable for all types; no other
            // impls can exist as they would conflict with our impl.
            unsafe { Some(&mut *(self as *mut dyn Encodable as *mut T)) }
        } else {
            None
        }
    }
}
