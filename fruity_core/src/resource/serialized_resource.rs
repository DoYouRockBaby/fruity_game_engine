use crate::resource::resource::Resource;
use fruity_any::*;
use fruity_introspect::serialized::Serialized;
use fruity_introspect::FieldInfo;
use fruity_introspect::IntrospectObject;
use fruity_introspect::MethodCaller;
use fruity_introspect::MethodInfo;
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
                        call: MethodCaller::Mut(Arc::new(move |_this, args| callback(args))),
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
