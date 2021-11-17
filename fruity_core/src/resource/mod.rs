/// Errors related with ResourceContainer
pub mod error;

/// A trait that should be implemented by every resources
pub mod resource;

/// A reference over a resource that is supposed to be used by components
pub mod resource_reference;

/// The resource manager
pub mod resource_container;

/// A wrapper for resource that come from scripting languages as serialized
pub mod serialized_resource;
