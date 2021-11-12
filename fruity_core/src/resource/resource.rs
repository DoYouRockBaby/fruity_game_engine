use fruity_introspect::IntrospectObject;
use std::fmt::Debug;
use std::sync::RwLock;

/// A trait that should be implemented by every resources
pub trait Resource: IntrospectObject + Debug {}

impl<T: Resource + ?Sized> Resource for RwLock<Box<T>> {}
