#![warn(missing_docs)]

//! Fruity core
//!
//! The base of the fruity game engine, it's simply a storage for services, the magic will opere in this services
//! espescialy the services from the ECS crate

/// Provides collection for services
pub mod service_manager;

/// Provides a wrapper for service for a serialized object
pub mod serialized_service;

/// Provides a wrapper to simplify the implementation of a service into a single thread
/// Is used when you want to implement a service with fields that are not Send Sync
pub mod single_thread_service;

/// Provides trait for services
pub mod service;

/// Provides RwLock for services
pub mod service_rwlock;

/// Provides guard for services
pub mod service_guard;

/// Functions to simplify the implementation of services, espescialy for introspection
pub mod utils;
