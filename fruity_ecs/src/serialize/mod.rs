#![warn(missing_docs)]

//! Serialize
//!
//! Provide a structure that will be used all over the application to serialize/deserialize things
//! Will be used to make a bridge between the rust ecosystem and the scripting language and by the
//! data storage

/// Traits and function to deserialize
pub mod deserialize;

/// Traits and function to serialize
pub mod serialize;

/// Enum used to serialize objects
pub mod serialized;

/// Implementation of serialize and deserialize for primitives
pub mod primitive;
