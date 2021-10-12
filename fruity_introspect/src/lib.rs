#![warn(missing_docs)]

//! Introspect
//!
//! Implements traits and macros to make a structure abe to list it's field and to get/set it with any
//!

use fruity_any::FruityAny;
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

#[derive(Clone)]
/// Informations about a field of an introspect object
pub struct FieldInfo {
    /// The name of the field
    pub name: String,

    /// The type of the field
    pub ty: String,

    /// Function to get one of the entry field value as Any
    ///
    /// # Arguments
    /// * `property` - The field name
    ///
    pub getter: fn(this: &dyn Any) -> &dyn Any,

    /// Function to set one of the entry field
    ///
    /// # Arguments
    /// * `property` - The field name
    /// * `value` - The new field value as Any
    ///
    pub setter: fn(this: &mut dyn Any, value: &dyn Any),
}

/// Trait to implement static introspection to an object
pub trait IntrospectFields {
    /// Get a list of fields with many informations
    fn get_field_infos(&self) -> Vec<FieldInfo>;
}

/// A method caller
#[derive(Clone)]
pub enum MethodCaller {
    /// Without mutability
    Const(
        fn(
            this: &dyn IntrospectMethods,
            args: Vec<Box<dyn Any>>,
        ) -> Result<Option<Box<dyn Any>>, IntrospectError>,
    ),

    /// With mutability
    Mut(
        fn(
            this: &mut dyn IntrospectMethods,
            args: Vec<Box<dyn Any>>,
        ) -> Result<Option<Box<dyn Any>>, IntrospectError>,
    ),
}

/// Informations about a field of an introspect object
#[derive(Clone)]
pub struct MethodInfo {
    /// The name of the method
    pub name: String,

    /// The type of the arguments
    pub args: Vec<String>,

    /// The type of the returned value
    pub return_type: Option<String>,

    /// Call for the method with any field
    pub call: MethodCaller,
}

/// Trait to implement static introspection to an object
pub trait IntrospectMethods: FruityAny {
    /// Get a list of fields with many informations
    fn get_method_infos(&self) -> Vec<MethodInfo>;
}
