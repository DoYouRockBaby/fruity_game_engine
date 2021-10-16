use crate::serialize::serialized::Serialized;
use fruity_introspect::IntrospectError;
use std::any::Any;

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

/// Cast a serialized argument from an argument list, take the first one
///
/// # Arguments
/// * `args` - The argument list
/// * `converter` - The converter that will turn the argument to a typed one
///
/// # Generic Arguments
/// * `T` - The type to cast
/// * `F` - The function type for the converter
///
pub fn cast_next_argument<T, F: Fn(Serialized) -> Option<T>>(
    method: &str,
    args: &mut Vec<Serialized>,
    converter: F,
) -> Result<T, IntrospectError> {
    if args.len() == 0 {
        return Err(IntrospectError::WrongNumberArguments {
            method: method.to_string(),
        });
    }

    match converter(args.remove(0)) {
        Some(arg) => Ok(arg),
        None => Err(IntrospectError::IncorrectArgument {
            method: method.to_string(),
        }),
    }
}
