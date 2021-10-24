use crate::IntrospectObject;
use std::sync::Arc;

/// Provides trait to implement a self duplication for an introspect object that can be stored in serialized
pub trait SerializableObject: IntrospectObject {
    /// Create a copy of self
    fn duplicate(&self) -> Box<dyn SerializableObject>;
}

impl<T: IntrospectObject + ?Sized> SerializableObject for Arc<T> {
    fn duplicate(&self) -> Box<dyn SerializableObject> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn SerializableObject> {
    fn clone(&self) -> Self {
        self.duplicate()
    }
}
