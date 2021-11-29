use crate::serialize::serialized::Serialized;

/// A value-to-value conversion that consumes the input value.
pub trait FruityInto<T>: Sized {
    /// Performs the conversion.
    fn fruity_into(self) -> T;
}

/// Simple and safe type conversions that may fail in a controlled
/// way under some circumstances.
pub trait FruityTryFrom<T>: Sized {
    /// The type returned in the event of a conversion error.
    type Error;

    /// Performs the conversion.
    fn fruity_try_from(value: T) -> Result<Self, Self::Error>;
}

impl<T: FruityTryFrom<Serialized, Error = String>> FruityTryFrom<Option<&Serialized>> for T {
    type Error = T::Error;

    fn fruity_try_from(value: Option<&Serialized>) -> Result<Self, Self::Error> {
        if let Some(value) = value {
            T::fruity_try_from(value.clone())
        } else {
            Err(format!("Couldn't convert {:?} to object", value))
        }
    }
}
