use fruity_any::FruityAny;
use fruity_introspect::Introspect;
use std::fmt::Debug;

/// A trait that should be implemented by every service
pub trait Service: Introspect + FruityAny + Send + Sync + Debug {}
