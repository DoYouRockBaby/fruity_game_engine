use crate::serialize::serialized::Serialized;
use fruity_any::FruityAny;
use fruity_introspect::IntrospectMethods;
use std::fmt::Debug;

/// A trait that should be implemented by every service
pub trait Service: IntrospectMethods<Serialized> + FruityAny + Send + Sync + Debug {}
