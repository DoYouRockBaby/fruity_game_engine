#![warn(missing_docs)]

//! Trait Vec
//!
//! Implements a collection that can store multiple object of the same type but without knowing the type
//! This object should implement a specific interface to encode/decode it to the desired trait.
//!

// mod array;
pub mod encodable;
pub mod slice;
