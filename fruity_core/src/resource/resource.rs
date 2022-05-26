use crate::introspect::IntrospectObject;
use crate::RwLock;
use std::fmt::Debug;

/// A trait that should be implemented by every resources
pub trait Resource: IntrospectObject + Debug {}

impl<T: Resource + ?Sized> Resource for RwLock<Box<T>> {}
