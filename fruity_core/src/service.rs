use fruity_any::FruityAny;
use fruity_introspect::IntrospectMethods;
use std::fmt::Debug;

/// A trait that should be implemented by every service
pub trait Service: IntrospectMethods + FruityAny + Send + Sync + Debug {}
