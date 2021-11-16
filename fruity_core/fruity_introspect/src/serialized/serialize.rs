use crate::serialized::ObjectFactory;
use crate::Serialized;
use crate::SetterCaller;
use std::collections::HashMap;

/// A trait that implements a function to serialize an object to a [’Serialized’]
pub trait Serialize {
    /// A function to serialize an object to a [’Serialized’]
    fn serialize(&self) -> Option<Serialized>;
}

/// A trait that implements a function to deserialize an object from a [’Serialized’]
pub trait Deserialize {
    /// The deserialize ouput type
    type Output;

    /// A function to deserialize an object from a [’Serialized’]
    fn deserialize(serialized: &Serialized, object_factory: &ObjectFactory)
        -> Option<Self::Output>;
}

impl Serialized {
    /// This returns an other serialized value, the difference between both is that the output converts
    /// the [’Serialized::NativeObject] that into [’Serialized::SerializedObject’]
    ///
    /// # Arguments
    /// * `object_factory` - The object factory that will instantiate the objects
    ///
    pub fn serialize_native_objects(&self) -> Serialized {
        if let Serialized::NativeObject(native_object) = self {
            let mut fields = HashMap::new();

            native_object
                .get_field_infos()
                .into_iter()
                .for_each(|field_info| {
                    if field_info.serializable {
                        let getter = field_info.getter;
                        fields.insert(
                            field_info.name,
                            getter(native_object.as_any_ref()).serialize_native_objects(),
                        );
                    }
                });

            Serialized::SerializedObject {
                class_name: native_object.get_class_name(),
                fields,
            }
        } else if let Serialized::SerializedObject { class_name, fields } = self {
            let mut deserialized_fields = HashMap::<String, Serialized>::new();

            for (key, value) in fields.iter() {
                deserialized_fields.insert(key.clone(), value.serialize_native_objects());
            }

            Serialized::SerializedObject {
                class_name: class_name.clone(),
                fields: deserialized_fields,
            }
        } else if let Serialized::Array(serialized_objects) = self {
            Serialized::Array(
                serialized_objects
                    .iter()
                    .map(|serializable_object| serializable_object.serialize_native_objects())
                    .collect::<Vec<_>>(),
            )
        } else {
            self.clone()
        }
    }

    /// This returns an other serialized value, the difference between both is that the output converts
    /// the [’Serialized::SerializedObject’] that can be into [’Serialized::NativeObject]
    ///
    /// # Arguments
    /// * `object_factory` - The object factory that will instantiate the objects
    ///
    pub fn deserialize_native_objects(&self, object_factory: &ObjectFactory) -> Serialized {
        if let Serialized::SerializedObject { class_name, fields } = self {
            let new_object = object_factory.instantiate(class_name, Vec::new());

            if let Some(new_object) = new_object {
                if let Serialized::NativeObject(mut new_object) = new_object {
                    fields.into_iter().for_each(|(key, value)| {
                        let new_object_fields = new_object.get_field_infos();
                        let field_info = new_object_fields
                            .iter()
                            .find(|field_info| field_info.name == *key);

                        if let Some(field_info) = field_info {
                            match &field_info.setter {
                                SetterCaller::Const(call) => {
                                    call(
                                        new_object.as_any_ref(),
                                        value.deserialize_native_objects(object_factory),
                                    );
                                }
                                SetterCaller::Mut(call) => {
                                    call(
                                        new_object.as_any_mut(),
                                        value.deserialize_native_objects(object_factory),
                                    );
                                }
                                SetterCaller::None => (),
                            }
                        }
                    });

                    Serialized::NativeObject(new_object)
                } else {
                    let mut deserialized_fields = HashMap::<String, Serialized>::new();

                    for (key, value) in fields.iter() {
                        deserialized_fields.insert(
                            key.clone(),
                            value.deserialize_native_objects(object_factory),
                        );
                    }

                    Serialized::SerializedObject {
                        class_name: class_name.clone(),
                        fields: deserialized_fields,
                    }
                }
            } else {
                let mut deserialized_fields = HashMap::<String, Serialized>::new();

                for (key, value) in fields.iter() {
                    deserialized_fields.insert(
                        key.clone(),
                        value.deserialize_native_objects(object_factory),
                    );
                }

                Serialized::SerializedObject {
                    class_name: class_name.clone(),
                    fields: deserialized_fields,
                }
            }
        } else if let Serialized::Array(serialized_objects) = self {
            Serialized::Array(
                serialized_objects
                    .iter()
                    .map(|serializable_object| {
                        serializable_object.deserialize_native_objects(object_factory)
                    })
                    .collect::<Vec<_>>(),
            )
        } else {
            self.clone()
        }
    }
}
