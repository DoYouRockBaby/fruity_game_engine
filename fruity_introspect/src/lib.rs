#![warn(missing_docs)]

//! Introspect
//!
//! Implements traits and macros to make a structure abe to list it's field and to get/set it with any
//!

use fruity_any::FruityAny;
use std::any::Any;
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Debug, Clone)]
/// Informations about a field of an introspect object
pub enum IntrospectError {
    /// Error that occure when you try to call a function with a name that don't exists
    UnknownMethod(String),
    /// Error that occure when you try to call a function with a parameter with the wrong type
    IncorrectArgument {
        /// The method name
        method: String,
        /// The argument index
        arg_index: usize,
    },
    /// Error that occure when you try to call a function with the wrong number of arguments
    WrongNumberArguments {
        /// The method name
        method: String,
        /// The provided number of arguments
        have: usize,
        /// The expected number of arguments
        expected: usize,
    },
    /// Error that occure when a callback from scripting language is nested with an other one
    NestedCallback,
}

/// Display in log an error related with introspection
pub fn log_introspect_error(err: &IntrospectError) {
    match err {
        IntrospectError::UnknownMethod(method) => {
            log::error!("Failed to call an unknown method named {}", method)
        }
        IntrospectError::IncorrectArgument { method, arg_index } => {
            log::error!(
                "Failed to call method {} cause the argument n°{} have a wrong type",
                method,
                arg_index
            )
        }
        IntrospectError::WrongNumberArguments {
            method,
            have,
            expected,
        } => {
            log::error!(
                "Failed to call method {} cause you provided {} arguments, expected {}",
                method,
                have,
                expected
            )
        }
        IntrospectError::NestedCallback => {
            log::error!("Cannot call a callback from scripting language nested with an other one",)
        }
    }
}

/// Informations about a field of an introspect object
///
/// # Arguments
/// * `T` - The type of the object used to provide parameters and function result
///
#[derive(Clone)]
pub struct FieldInfo<T> {
    /// The name of the field
    pub name: String,

    /// The type of the field
    pub ty: String,

    /// Function to get one of the entry field value as Any
    ///
    /// # Arguments
    /// * `property` - The field name
    ///
    pub getter: Arc<dyn Fn(&dyn Any) -> T>,

    /// Function to set one of the entry field
    ///
    /// # Arguments
    /// * `property` - The field name
    /// * `value` - The new field value as Any
    ///
    pub setter: Arc<dyn Fn(&mut dyn Any, T)>,
}

/// Trait to implement static introspection to an object
///
/// # Arguments
/// * `T` - The type of the object used to provide parameters and function result
///
pub trait IntrospectFields<T> {
    /// Get a list of fields with many informations
    fn get_field_infos(&self) -> Vec<FieldInfo<T>>;
}

/// A method caller
///
/// # Arguments
/// * `T` - The type of the object used to provide parameters and function result
///
#[derive(Clone)]
pub enum MethodCaller<T> {
    /// Without mutability
    Const(Arc<dyn Fn(&dyn Any, Vec<T>) -> Result<Option<T>, IntrospectError>>),

    /// With mutability
    Mut(Arc<dyn Fn(&mut dyn Any, Vec<T>) -> Result<Option<T>, IntrospectError>>),
}

/// Informations about a field of an introspect object
///
/// # Arguments
/// * `T` - The type of the object used to provide parameters and function result
///
#[derive(Clone)]
pub struct MethodInfo<T> {
    /// The name of the method
    pub name: String,

    /// The type of the arguments
    pub args: Vec<String>,

    /// The type of the returned value
    pub return_type: Option<String>,

    /// Call for the method with any field
    pub call: MethodCaller<T>,
}

/// Trait to implement static introspection to an object
///
/// # Arguments
/// * `T` - The type of the object used to provide parameters and function result
///
pub trait IntrospectMethods<T>: FruityAny + Debug {
    /// Get a list of fields with many informations
    fn get_method_infos(&self) -> Vec<MethodInfo<T>>;
}

impl<T, U: IntrospectMethods<T>> IntrospectMethods<T> for Box<U> {
    fn get_method_infos(&self) -> Vec<MethodInfo<T>> {
        self.as_ref().get_method_infos()
    }
}
