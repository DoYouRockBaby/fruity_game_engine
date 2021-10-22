use fruity_introspect::IntrospectObject;
use std::fmt::Debug;

/// A trait that should be implemented by every resources
pub trait Resource: IntrospectObject + Debug {}
