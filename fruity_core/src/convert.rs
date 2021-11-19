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
