//! Any
//!
//! An extended Any trait
//!
//! The difference with the classic Any is that this Any needs to implement converter

pub use fruity_any_derive::FruityAny;
pub use fruity_any_derive::FruityAnySyncSend;
use std::any::Any;
use std::sync::Arc;

/// The any trait
pub trait FruityAny: Any {
    /// Return self as an Any ref
    fn as_any_ref(&self) -> &dyn Any;

    /// Return self as an Any mutable ref
    fn as_any_mut(&mut self) -> &mut dyn Any;

    /// Return self as an Any box
    fn as_any_box(self: Box<Self>) -> Box<dyn Any>;

    /// Return self as an Any arc
    fn as_any_arc(self: Arc<Self>) -> Arc<dyn Any>;
}

/// The any trait with sync send
pub trait FruityAnySendSync: FruityAny + Send + Sync {
    /// Return self as an Any arc
    fn as_any_arc_send_sync(self: Arc<Self>) -> Arc<dyn Any + Send + Sync>;
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

    fn as_any_arc(self: Arc<Self>) -> Arc<dyn Any> {
        self
    }
}
