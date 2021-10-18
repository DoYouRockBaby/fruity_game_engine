use fruity_any::FruityAnySendSync;
use std::fmt::Debug;

/// A trait that should be implemented by every resources
pub trait Resource: Debug + FruityAnySendSync {}
