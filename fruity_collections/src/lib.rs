#![warn(missing_docs)]

//! Trait Vec
//!
//! Implements a collection that can store multiple object of the same type but without knowing the type
//! This object should implement a specific interface to encode/decode it to the desired trait.
//!

/// Trait for an encodable object that can be stored as an array of u8
pub mod encodable;

/// A vector of encodable object, the memory is compacted to improve iteration performance
pub mod encodable_vec;

/// Functions to compute slices
pub mod slice;
