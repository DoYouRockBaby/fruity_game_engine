#![warn(missing_docs)]

//! Fruity core
//!
//! The base of the fruity game engine, it's simply a storage for services, the magic will opere in this services
//! espescialy the services from the ECS crate

/// Provides collection for services
pub mod service_manager;

/// Provides a wrapper for service for a serialized object
pub mod serialized_service;

/// Provides trait for services
pub mod service;

/// Provides RwLock for services
pub mod service_rwlock;

/// Provides guard for services
pub mod service_guard;

/// Functions to simplify the implementation of services, espescialy for introspection
pub mod utils;
