use fruity_introspect::Introspect;
use std::any::Any;

pub trait Service: Introspect + Any + Send + Sync {}
