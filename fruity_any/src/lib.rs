//! Any
//!
//! An extended Any trait
//!
//! The difference with the classic Any is that this Any needs to implement converter

use std::any::Any;

/// The any trait
pub trait FruityAny: Any {
    /// Return self as an Any ref
    fn as_any_ref(&self) -> &dyn Any;

    /// Return self as an Any mutable ref
    fn as_any_mut(&mut self) -> &mut dyn Any;

    /// Return self as an Any box
    fn as_any_box(self: Box<Self>) -> Box<dyn Any>;
}

impl<T: FruityAny> FruityAny for Box<T> {
    fn as_any_ref(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn as_any_box(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}
