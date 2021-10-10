use fruity_introspect::Introspect;
use std::any::type_name;
use std::any::Any;
use std::any::TypeId;
use std::fmt::Debug;

/// A function to decode an object from byte array to an any reference
pub type ComponentDecoder = fn(buffer: &[u8]) -> &dyn Component;

/// A function to decode an object from byte array to an any mutable reference
pub type ComponentDecoderMut = fn(buffer: &mut [u8]) -> &mut dyn Component;

/// An abstraction over a component, should be implemented for every component
pub trait Component: Introspect + Debug + Send + Sync + Any {
    /// Return the component type identifier
    fn get_component_type(&self) -> String;

    /// Return the size that is required to encode the object
    fn encode_size(&self) -> usize;

    /// Encode the object to a byte array
    ///
    /// # Arguments
    /// * `buffer` - The buffer where the encoder will write, should match the result of encode_size function
    ///
    fn encode(self: Box<Self>, buffer: &mut [u8]);

    /// Return a function to decode an object from byte array to an any reference
    fn get_decoder(&self) -> ComponentDecoder;

    /// Return a function to decode an object from byte array to an any mutable reference
    fn get_decoder_mut(&self) -> ComponentDecoderMut;
}

impl dyn Component {
    /// Get one of the component field value
    ///
    /// # Arguments
    /// * `property` - The field name
    ///
    /// # Generic Arguments
    /// * `T` - The field type
    ///
    pub fn get_field<T: Any>(&self, property: &str) -> Option<&T> {
        match self.get_any_field(property) {
            Some(value) => match value.downcast_ref::<T>() {
                Some(value) => Some(value),
                None => {
                    log::error!(
                        "Try to get a {:?} from property {:?}, got {:?}",
                        type_name::<T>(),
                        property,
                        value
                    );
                    None
                }
            },
            None => None,
        }
    }

    /// Set one of the component field
    ///
    /// # Arguments
    /// * `property` - The field name
    /// * `value` - The new field value
    ///
    /// # Generic Arguments
    /// * `T` - The field type
    ///
    pub fn set_field<T: Any>(&mut self, property: &str, value: T) {
        self.set_any_field(property, &value);
    }

    /// Returns `true` if the boxed type is the same as `T`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::any::Component;
    ///
    /// fn is_string(s: &dyn Component) {
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
    pub fn is<T: Component>(&self) -> bool {
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
    /// use fruity_ecs::component::component::Component;
    ///
    /// fn print_if_string(s: &dyn Component) {
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
    pub fn downcast_ref<T: Component>(&self) -> Option<&T> {
        if self.is::<T>() {
            // SAFETY: just checked whether we are pointing to the correct type, and we can rely on
            // that check for memory safety because we have implemented Component for all types; no other
            // impls can exist as they would conflict with our impl.
            unsafe { Some(&*(self as *const dyn Component as *const T)) }
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
    /// use fruity_ecs::component::component::Component;
    ///
    /// fn modify_if_u32(s: &mut dyn use fruity_ecs::component::component::Component) {
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
    pub fn downcast_mut<T: Component>(&mut self) -> Option<&mut T> {
        if self.is::<T>() {
            // SAFETY: just checked whether we are pointing to the correct type, and we can rely on
            // that check for memory safety because we have implemented Component for all types; no other
            // impls can exist as they would conflict with our impl.
            unsafe { Some(&mut *(self as *mut dyn Component as *mut T)) }
        } else {
            None
        }
    }
}
