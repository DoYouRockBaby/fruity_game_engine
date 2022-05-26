use crate::convert::FruityInto;
use crate::convert::FruityTryFrom;
use crate::introspect::FieldInfo;
use crate::introspect::IntrospectObject;
use crate::introspect::MethodCaller;
use crate::introspect::MethodInfo;
use crate::introspect::SetterCaller;
use crate::serialize::serialized::SerializableObject;
use crate::serialize::serialized::Serialized;
use crate::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

impl<T: IntrospectObject + ?Sized> FruityTryFrom<Serialized> for RwLock<Box<T>> {
    type Error = String;

    fn fruity_try_from(value: Serialized) -> Result<Self, Self::Error> {
        match value {
            Serialized::NativeObject(value) => {
                match value.as_any_box().downcast::<RwLock<Box<T>>>() {
                    Ok(value) => Ok(*value),
                    _ => Err(format!("Couldn't convert a Serialized to native object")),
                }
            }
            _ => Err(format!("Couldn't convert {:?} to native object", value)),
        }
    }
}

impl<T: IntrospectObject + ?Sized> FruityTryFrom<Serialized> for Arc<T> {
    type Error = String;

    fn fruity_try_from(value: Serialized) -> Result<Self, Self::Error> {
        match value {
            Serialized::NativeObject(value) => match value.as_any_box().downcast::<Arc<T>>() {
                Ok(value) => Ok(*value),
                _ => Err(format!("Couldn't convert a Serialized to native object")),
            },
            _ => Err(format!("Couldn't convert {:?} to native object", value)),
        }
    }
}

impl<T: FruityInto<Serialized>> FruityInto<Serialized> for Vec<T> {
    fn fruity_into(self) -> Serialized {
        Serialized::Array(
            self.into_iter()
                .map(|elem| elem.fruity_into())
                .collect::<Vec<_>>(),
        )
    }
}

impl<T: FruityTryFrom<Serialized, Error = String> + 'static> FruityTryFrom<Serialized>
    for Option<T>
{
    type Error = String;

    fn fruity_try_from(value: Serialized) -> Result<Self, Self::Error> {
        if let Serialized::Null = value {
            Ok(None)
        } else {
            match T::fruity_try_from(value) {
                Ok(value) => Ok(Some(value)),
                Err(err) => Err(err.to_string()),
            }
        }
    }
}

impl<T: FruityInto<Serialized>> FruityInto<Serialized> for Option<T> {
    fn fruity_into(self) -> Serialized {
        match self {
            Some(value) => value.fruity_into(),
            None => Serialized::Null,
        }
    }
}

impl<T: IntrospectObject> IntrospectObject for Option<T> {
    fn get_class_name(&self) -> String {
        if let Some(value) = &self {
            value.get_class_name()
        } else {
            "null".to_string()
        }
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        if let Some(value) = &self {
            value
                .get_field_infos()
                .into_iter()
                .map(|field_info| FieldInfo {
                    name: field_info.name,
                    serializable: false,
                    getter: Arc::new(move |this| {
                        let this = this.downcast_ref::<Option<T>>().unwrap();

                        (field_info.getter)(this.as_ref().unwrap().as_any_ref())
                    }),
                    setter: match field_info.setter {
                        SetterCaller::Const(call) => {
                            SetterCaller::Const(Arc::new(move |this, args| {
                                let this = this.downcast_ref::<Option<T>>().unwrap();

                                call(this.as_ref().unwrap().as_any_ref(), args)
                            }))
                        }
                        SetterCaller::Mut(call) => {
                            SetterCaller::Mut(Arc::new(move |this, args| {
                                let this = this.downcast_mut::<Option<T>>().unwrap();

                                call(this.as_mut().unwrap().as_any_mut(), args)
                            }))
                        }
                        SetterCaller::None => SetterCaller::None,
                    },
                })
                .collect::<Vec<_>>()
        } else {
            vec![]
        }
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        if let Some(value) = &self {
            value
                .get_method_infos()
                .into_iter()
                .map(|method_info| MethodInfo {
                    name: method_info.name,
                    call: match method_info.call {
                        MethodCaller::Const(call) => {
                            MethodCaller::Const(Arc::new(move |this, args| {
                                let this = this.downcast_ref::<Option<T>>().unwrap();

                                call(this.as_ref().unwrap().as_any_ref(), args)
                            }))
                        }
                        MethodCaller::Mut(call) => {
                            MethodCaller::Mut(Arc::new(move |this, args| {
                                let this = this.downcast_mut::<Option<T>>().unwrap();

                                call(this.as_mut().unwrap().as_any_mut(), args)
                            }))
                        }
                    },
                })
                .collect::<Vec<_>>()
        } else {
            vec![]
        }
    }
}

impl<T: Clone + IntrospectObject> SerializableObject for Option<T> {
    fn duplicate(&self) -> Box<dyn SerializableObject> {
        Box::new(self.clone())
    }
}

impl<T: FruityTryFrom<Serialized, Error = String> + 'static> FruityTryFrom<Serialized>
    for HashMap<String, T>
{
    type Error = String;

    fn fruity_try_from(value: Serialized) -> Result<Self, Self::Error> {
        if let Serialized::SerializedObject { fields, .. } = value {
            let mut result = HashMap::<String, T>::new();

            fields.into_iter().for_each(|(key, value)| {
                if let Some(value) = T::fruity_try_from(value).ok() {
                    result.insert(key, value);
                }
            });

            Ok(result)
        } else {
            Err(format!("Couldn't convert {:?} to HashMap", value))
        }
    }
}

impl<T: FruityInto<Serialized>> FruityInto<Serialized> for HashMap<String, T> {
    fn fruity_into(self) -> Serialized {
        let mut fields = HashMap::<String, Serialized>::new();

        self.into_iter().for_each(|(key, value)| {
            fields.insert(key, value.fruity_into());
        });

        Serialized::SerializedObject {
            class_name: "unknown".to_string(),
            fields,
        }
    }
}
