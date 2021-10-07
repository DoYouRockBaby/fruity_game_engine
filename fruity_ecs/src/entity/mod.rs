/// Provides an abstraction over an entity and collections to store entities
pub mod entity;

/// Provides guards for entities, this is intended to work with EntityRwLock
pub mod entity_guard;

/// Provides a collections to store archetypes
pub mod entity_manager;

/// Provides a threadsafe lock for entities
pub mod component_rwlock;

/// Provides a collections to store entities
pub mod archetype;
