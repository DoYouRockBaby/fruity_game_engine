use crate::serialize::serialized::Serialized;
use std::convert::TryFrom;

macro_rules! impl_numeric_from_serialized {
    ( $type:ident ) => {
        impl TryFrom<Serialized> for $type {
            type Error = String;

            fn try_from(value: Serialized) -> Result<Self, Self::Error> {
                match value {
                    Serialized::I8(value) => Ok(value as $type),
                    Serialized::I16(value) => Ok(value as $type),
                    Serialized::I32(value) => Ok(value as $type),
                    Serialized::I64(value) => Ok(value as $type),
                    Serialized::ISize(value) => Ok(value as $type),
                    Serialized::U8(value) => Ok(value as $type),
                    Serialized::U16(value) => Ok(value as $type),
                    Serialized::U32(value) => Ok(value as $type),
                    Serialized::U64(value) => Ok(value as $type),
                    Serialized::USize(value) => Ok(value as $type),
                    Serialized::F32(value) => Ok(value as $type),
                    Serialized::F64(value) => Ok(value as $type),
                    _ => Err(format!("Couldn't convert {:?} to {}", value, "$type")),
                }
            }
        }
    };
}

impl_numeric_from_serialized!(i8);
impl_numeric_from_serialized!(i16);
impl_numeric_from_serialized!(i32);
impl_numeric_from_serialized!(i64);
impl_numeric_from_serialized!(isize);
impl_numeric_from_serialized!(u8);
impl_numeric_from_serialized!(u16);
impl_numeric_from_serialized!(u32);
impl_numeric_from_serialized!(u64);
impl_numeric_from_serialized!(usize);
impl_numeric_from_serialized!(f32);
impl_numeric_from_serialized!(f64);

impl TryFrom<Serialized> for bool {
    type Error = String;

    fn try_from(value: Serialized) -> Result<Self, Self::Error> {
        match value {
            Serialized::Bool(value) => Ok(value),
            _ => Err(format!("Couldn't convert {:?} to bool", value)),
        }
    }
}

impl TryFrom<Serialized> for String {
    type Error = String;

    fn try_from(value: Serialized) -> Result<Self, Self::Error> {
        match value {
            Serialized::String(value) => Ok(value),
            _ => Err(format!("Couldn't convert {:?} to bool", value)),
        }
    }
}

impl Into<Serialized> for i8 {
    fn into(self) -> Serialized {
        Serialized::I8(self)
    }
}

impl Into<Serialized> for i16 {
    fn into(self) -> Serialized {
        Serialized::I16(self)
    }
}

impl Into<Serialized> for i32 {
    fn into(self) -> Serialized {
        Serialized::I32(self)
    }
}

impl Into<Serialized> for i64 {
    fn into(self) -> Serialized {
        Serialized::I64(self)
    }
}

impl Into<Serialized> for isize {
    fn into(self) -> Serialized {
        Serialized::ISize(self)
    }
}

impl Into<Serialized> for u8 {
    fn into(self) -> Serialized {
        Serialized::U8(self)
    }
}

impl Into<Serialized> for u16 {
    fn into(self) -> Serialized {
        Serialized::U16(self)
    }
}

impl Into<Serialized> for u32 {
    fn into(self) -> Serialized {
        Serialized::U32(self)
    }
}

impl Into<Serialized> for u64 {
    fn into(self) -> Serialized {
        Serialized::U64(self)
    }
}

impl Into<Serialized> for usize {
    fn into(self) -> Serialized {
        Serialized::USize(self)
    }
}

impl Into<Serialized> for f32 {
    fn into(self) -> Serialized {
        Serialized::F32(self)
    }
}

impl Into<Serialized> for f64 {
    fn into(self) -> Serialized {
        Serialized::F64(self)
    }
}

impl Into<Serialized> for bool {
    fn into(self) -> Serialized {
        Serialized::Bool(self)
    }
}

impl Into<Serialized> for String {
    fn into(self) -> Serialized {
        Serialized::String(self)
    }
}
