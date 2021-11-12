use crate::IntrospectObject;
use crate::Serialized;
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
