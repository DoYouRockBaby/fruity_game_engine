use fruity_introspect::IntrospectObject;
use std::fmt::Debug;

/// A trait that should be implemented by every service
pub trait Service: IntrospectObject + Debug {}
