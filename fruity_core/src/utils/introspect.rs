use crate::introspect::IntrospectError;
use crate::serialize::serialized::Serialized;
use std::any::Any;
use std::convert::TryFrom;
use std::iter::Enumerate;
use std::vec::IntoIter as VecIntoIter;

/// Cast an any introspect object
///
/// # Arguments
/// * `any` - The introspect object as an any reference
///
pub fn cast_introspect_ref<T: Any>(any: &dyn Any) -> &T {
    any.downcast_ref::<T>().unwrap()
}

/// Cast an any introspect object with mutability
///
/// # Arguments
/// * `any` - The introspect object as an any mutable reference
///
pub fn cast_introspect_mut<T: Any>(any: &mut dyn Any) -> &mut T {
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

    /// Get a serialized argument from an argument list
    pub fn next(&mut self) -> Result<Serialized, IntrospectError> {
        match self.iter.next() {
            Some((index, arg)) => {
                self.last_index = index + 1;
                Ok(arg)
            }
            None => Err(IntrospectError::WrongNumberArguments {
                method: self.method.to_string(),
                have: self.last_index,
                expected: self.args_count,
            }),
        }
    }

    /// Get all the remaining serialized arguments from an argument list
    pub fn rest(&mut self) -> Vec<Serialized> {
        let mut result = Vec::new();
        while let Some(elem) = self.iter.next() {
            result.push(elem.1);
        }

        result
    }

    /// Cast a serialized argument from an argument list
    ///
    /// # Generic Arguments
    /// * `T` - The type to cast
    ///
    pub fn cast_next<T: TryFrom<Serialized> + ?Sized>(&mut self) -> Result<T, IntrospectError> {
        match self.iter.next() {
            Some((index, arg)) => {
                self.last_index = index + 1;
                T::try_from(arg).map_err(|_| IntrospectError::IncorrectArgument {
                    method: self.method.to_string(),
                    arg_index: index,
                })
            }
            None => Err(IntrospectError::WrongNumberArguments {
                method: self.method.to_string(),
                have: self.last_index,
                expected: self.args_count,
            }),
        }
    }

    /// Cast a serialized optional argument from an argument list
    ///
    /// # Generic Arguments
    /// * `T` - The type to cast
    ///
    pub fn cast_next_optional<T: TryFrom<Serialized> + ?Sized>(&mut self) -> Option<T> {
        match self.iter.next() {
            Some((index, arg)) => {
                self.last_index = index + 1;
                T::try_from(arg)
                    .map_err(|_| IntrospectError::IncorrectArgument {
                        method: self.method.to_string(),
                        arg_index: index,
                    })
                    .ok()
            }
            None => None,
        }
    }
}
