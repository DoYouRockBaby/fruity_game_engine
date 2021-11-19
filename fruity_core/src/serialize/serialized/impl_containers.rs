use crate::introspect::IntrospectObject;
use crate::serialize::serialized::Serialized;
use std::convert::TryFrom;
use std::sync::Arc;
use std::sync::RwLock;

impl<T: IntrospectObject + ?Sized> TryFrom<Serialized> for Arc<RwLock<Box<T>>> {
    type Error = String;

    fn try_from(value: Serialized) -> Result<Self, Self::Error> {
        match value {
            Serialized::NativeObject(value) => {
                match value.as_any_box().downcast::<Arc<RwLock<Box<T>>>>() {
                    Ok(value) => Ok(*value),
                    _ => Err(format!("Couldn't convert a Serialized to native object")),
                }
            }
            _ => Err(format!("Couldn't convert {:?} to native object", value)),
        }
    }
}

impl<T: IntrospectObject + ?Sized> TryFrom<Serialized> for Option<Arc<T>> {
    type Error = String;

    fn try_from(value: Serialized) -> Result<Self, Self::Error> {
        match value {
            Serialized::NativeObject(value) => {
                match value.clone().as_any_box().downcast::<Arc<T>>() {
                    Ok(value) => Ok(Some(*value)),
                    _ => match value.as_any_box().downcast::<Option<Arc<T>>>() {
                        Ok(value) => Ok(*value),
                        _ => Err(format!("Couldn't convert a Serialized to native object")),
                    },
                }
            }
            Serialized::Null => Ok(None),
            _ => Err(format!("Couldn't convert {:?} to native object", value)),
        }
    }
}

impl<T: Into<Serialized>> Into<Serialized> for Vec<T> {
    fn into(self) -> Serialized {
        Serialized::Array(self.into_iter().map(|elem| elem.into()).collect::<Vec<_>>())
    }
}

impl<T: TryFrom<Serialized>> TryFrom<Serialized> for Option<T> {
    type Error = String;

    fn try_from(value: Serialized) -> Result<Self, Self::Error> {
        if let Serialized::Null = value {
            Ok(None)
        } else {
            match T::try_from(value) {
                Ok(value) => Ok(Some(value)),
                Err(err) => Err(err.to_string()),
            }
        }
    }
}

impl<T: Into<Serialized>> Into<Serialized> for Option<T> {
    fn into(self) -> Serialized {
        match self {
            Some(value) => value.into(),
            None => Serialized::Null,
        }
    }
}
