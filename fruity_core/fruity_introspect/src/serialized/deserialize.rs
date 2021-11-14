use crate::serialized::ObjectFactory;
use crate::Serialized;
use crate::SetterCaller;
use std::collections::HashMap;

impl Serialized {
    /// Deserialize a serialized value
    /// This returns an other serialized value, the difference between both is that the output converts
    /// the [’Serialized::SerializedObject’] that can be into [’Serialized::NativeObject]
    ///
    /// # Arguments
    /// * `object_factory` - The object factory that will instantiate the objects
    ///
    pub fn deserialize(&self, object_factory: &ObjectFactory) -> Serialized {
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
                                    call(new_object.as_any_ref(), value.clone());
                                }
                                SetterCaller::Mut(call) => {
                                    call(new_object.as_any_mut(), value.clone());
                                }
                                SetterCaller::None => (),
                            }
                        }
                    });

                    Serialized::NativeObject(new_object)
                } else {
                    let mut deserialized_fields = HashMap::<String, Serialized>::new();

                    for (key, value) in fields.iter() {
                        deserialized_fields.insert(key.clone(), value.deserialize(object_factory));
                    }

                    Serialized::SerializedObject {
                        class_name: class_name.clone(),
                        fields: deserialized_fields,
                    }
                }
            } else {
                let mut deserialized_fields = HashMap::<String, Serialized>::new();

                for (key, value) in fields.iter() {
                    deserialized_fields.insert(key.clone(), value.deserialize(object_factory));
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
                    .map(|serializable_object| serializable_object.deserialize(object_factory))
                    .collect::<Vec<_>>(),
            )
        } else {
            self.clone()
        }
    }
}
