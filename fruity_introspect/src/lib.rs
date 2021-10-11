#![warn(missing_docs)]

//! Introspect
//!
//! Implements traits and macros to make a structure abe to list it's field and to get/set it with any
//!

use std::any::Any;

#[derive(Debug, Clone)]
/// Informations about a field of an introspect object
pub enum IntrospectError {
    /// Error that occure when you try to call a function with a name that don't exists
    UnknownMethod(String),
    /// Error that occure when you try to call a function with a parameter with the wrong type
    IncorrectArgument,
    /// Error that occure when you try to call a function with the wrong number of arguments
    WrongNumberArguments {
        /// The provided number of arguments
        have: usize,
        /// The expected number of arguments
        expected: usize,
    },
}

#[derive(Debug, Clone)]
/// Informations about a field of an introspect object
pub struct FieldInfo {
    /// The name of the field
    pub name: String,

    /// The type of the field
    pub ty: String,
}

#[derive(Debug, Clone)]
/// Informations about a field of an introspect object
pub struct MethodInfo {
    /// The name of the method
    pub name: String,

    /// The type of the arguments
    pub args: Vec<String>,

    /// The type of the returned value
    pub return_type: Option<String>,
}

/// Trait to implement static introspection to an object
pub trait Introspect {
    /// Get a list of fields with many informations
    fn get_field_infos(&self) -> Vec<FieldInfo>;

    /// Get one of the entry field value as Any
    ///
    /// # Arguments
    /// * `property` - The field name
    ///
    fn get_any_field(&self, property: &str) -> Option<&dyn Any>;

    /// Set one of the entry field
    ///
    /// # Arguments
    /// * `property` - The field name
    /// * `value` - The new field value as Any
    ///
    fn set_any_field(&mut self, property: &str, value: &dyn Any);

    /// Get a list of fields with many informations
    fn get_method_infos(&self) -> Vec<MethodInfo>;

    /// Call a method by it's name
    fn call_method(
        &self,
        name: &str,
        args: Vec<Box<dyn std::any::Any>>,
    ) -> Result<Box<dyn std::any::Any>, IntrospectError>;

    /// Call a mutable method by it's name
    fn call_method_mut(
        &mut self,
        name: &str,
        args: Vec<Box<dyn std::any::Any>>,
    ) -> Result<Box<dyn std::any::Any>, IntrospectError>;
}
