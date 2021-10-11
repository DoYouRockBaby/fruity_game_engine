//! Any
//!
//! An extended Any trait
//!
//! The difference with the classic Any is that this Any needs to implement converter

use std::any::Any;
use std::any::TypeId;

/// The any trait
pub trait FruityAny: Any {
    /// Return self as an Any ref
    fn as_any_ref(&self) -> &dyn FruityAny;

    /// Return self as an Any mutable ref
    fn as_any_mut(&mut self) -> &mut dyn FruityAny;

    /// Return self as an Any box
    fn as_any_box(self: Box<Self>) -> Box<dyn FruityAny>;
}

impl dyn FruityAny {
    /// Returns `true` if the boxed type is the same as `T`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fruity_any::Any;
    ///
    /// fn is_string(s: &dyn Any) {
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
    pub fn is<T: FruityAny>(&self) -> bool {
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
    /// use fruity_any::Any;
    ///
    /// fn print_if_string(s: &dyn Any) {
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
    pub fn downcast_ref<T: FruityAny>(&self) -> Option<&T> {
        if self.is::<T>() {
            // SAFETY: just checked whether we are pointing to the correct type, and we can rely on
            // that check for memory safety because we have implemented Any for all types; no other
            // impls can exist as they would conflict with our impl.
            unsafe { Some(&*(self as *const dyn FruityAny as *const T)) }
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
    /// use fruity_any::Any;
    ///
    /// fn modify_if_u32(s: &mut dyn use Any) {
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
    pub fn downcast_mut<T: FruityAny>(&mut self) -> Option<&mut T> {
        if self.is::<T>() {
            // SAFETY: just checked whether we are pointing to the correct type, and we can rely on
            // that check for memory safety because we have implemented Any for all types; no other
            // impls can exist as they would conflict with our impl.
            unsafe { Some(&mut *(self as *mut dyn FruityAny as *mut T)) }
        } else {
            None
        }
    }
}
