use crate::introspect::FieldInfo;
use crate::introspect::IntrospectObject;
use crate::introspect::MethodCaller;
use crate::introspect::MethodInfo;
use crate::resource::resource::Resource;
use crate::serialize::serialized::Serialized;
use fruity_any::*;
use std::sync::Arc;

/// A wrapper for resource that come from scripting languages as serialized
#[derive(Debug, FruityAny)]
pub struct SerializedResource {
    serialized: Serialized,
}

impl SerializedResource {
    /// Returns a SerializedResource
    pub fn new(serialized: Serialized) -> SerializedResource {
        SerializedResource { serialized }
    }
}

impl IntrospectObject for SerializedResource {
    fn get_class_name(&self) -> String {
        if let Serialized::SerializedObject { class_name, .. } = &self.serialized {
            class_name.clone()
        } else {
            "SerializedResource".to_string()
        }
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        let this = self.clone();

        if let Serialized::SerializedObject { fields, .. } = &this.serialized {
            fields
                .iter()
                .filter_map(|(key, value)| match value {
                    Serialized::Callback(callback) => Some((key, callback)),
                    _ => None,
                })
                .map(|(key, callback)| {
                    let callback = callback.clone();

                    MethodInfo {
                        name: key.clone(),
                        call: MethodCaller::Mut(Arc::new(move |_this, args| {
                            (callback.callback)(args)
                        })),
                    }
                })
                .collect::<Vec<_>>()
        } else {
            vec![]
        }
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        vec![]
    }
}

impl Resource for SerializedResource {}
