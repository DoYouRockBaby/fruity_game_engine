use fruity_introspect::IntrospectError;
use fruity_serialize::serialized::Serialized;
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

/// Assert that the argument count given match an expected value
///
/// # Arguments
/// * `expected` - The  expected argument count
/// * `args` - The argument list
///
pub fn assert_argument_count(
    expected: usize,
    args: &Vec<Serialized>,
) -> Result<(), IntrospectError> {
    if args.len() != expected {
        return Err(IntrospectError::WrongNumberArguments {
            have: args.len(),
            expected: expected,
        });
    }

    Ok(())
}

/// Cast a serialized argument from an argument list
///
/// # Arguments
/// * `index` - The index of the argument
/// * `args` - The argument list
/// * `converter` - The converter that will turn the argument to a typed one
///
/// # Generic Arguments
/// * `T` - The type to cast
/// * `F` - The function type for the converter
///
pub fn cast_argument<T, F: Fn(&Serialized) -> Option<T>>(
    index: usize,
    args: &Vec<Serialized>,
    converter: F,
) -> Result<T, IntrospectError> {
    match converter(args.get(index).unwrap()) {
        Some(arg) => Ok(arg),
        None => Err(IntrospectError::IncorrectArgument),
    }
}
