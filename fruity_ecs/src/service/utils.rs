use crate::serialize::serialized::Serialized;
use fruity_introspect::IntrospectError;
use std::any::Any;
use std::iter::Enumerate;
use std::vec::IntoIter as VecIntoIter;

/// Cast an any service
///
/// # Arguments
/// * `any` - The service as an any reference
///
pub fn cast_service<T: Any>(any: &dyn Any) -> &T {
    any.downcast_ref::<T>().unwrap()
}

/// Cast an any service with mutability
///
/// # Arguments
/// * `any` - The service as an any mutable reference
///
pub fn cast_service_mut<T: Any>(any: &mut dyn Any) -> &mut T {
    any.downcast_mut::<T>().unwrap()
}

/// A tool that is used to cast serialized arguments, intended to be used into IntrospectMethod implementations
pub struct ArgumentCaster<'s> {
    method: &'s str,
    args_count: usize,
    iter: Enumerate<VecIntoIter<Serialized>>,
    last_index: usize,
}

impl<'s> ArgumentCaster<'s> {
    /// Return an ArgumentCaster
    pub fn new<'a>(method: &'a str, args: Vec<Serialized>) -> ArgumentCaster<'a> {
        ArgumentCaster::<'a> {
            method,
            args_count: args.len(),
            iter: args.into_iter().enumerate(),
            last_index: 1,
        }
    }

    /// Cast a serialized argument from an argument list
    ///
    /// # Arguments
    /// * `converter` - The converter that will turn the argument to a typed one
    ///
    /// # Generic Arguments
    /// * `T` - The type to cast
    /// * `F` - The function type for the converter
    ///
    pub fn cast_next<T, F: Fn(Serialized) -> Option<T>>(
        &mut self,
        converter: F,
    ) -> Result<T, IntrospectError> {
        match self.iter.next() {
            Some((index, arg)) => {
                self.last_index = index + 1;
                match converter(arg) {
                    Some(arg) => Ok(arg),
                    None => Err(IntrospectError::IncorrectArgument {
                        method: self.method.to_string(),
                        arg_index: index,
                    }),
                }
            }
            None => Err(IntrospectError::WrongNumberArguments {
                method: self.method.to_string(),
                have: self.last_index,
                expected: self.args_count,
            }),
        }
    }
}
