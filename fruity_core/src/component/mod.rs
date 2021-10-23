/// Provides an abstraction over a component
pub mod component;

/// Provides a wrapper for service for a serialized object
pub mod serialized_component;

/// Provides RwLock for a component
pub mod component_rwlock;

/// Provides guards for a component
pub mod component_guard;

/// Provides RwLock for components list
pub mod component_list_rwlock;

/// Provides guards for components list
pub mod component_list_guard;

/// Provides a factory for the component types
/// This will be used by the scripting language to expose component creation
pub mod components_factory;
