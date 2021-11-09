use crate::service::service::Service;
use fruity_any::*;
use fruity_introspect::serialized::Serialized;
use fruity_introspect::FieldInfo;
use fruity_introspect::IntrospectObject;
use fruity_introspect::MethodCaller;
use fruity_introspect::MethodInfo;
use std::sync::Arc;

/// A wrapper for services that come from scripting languages as serialized
#[derive(Debug, FruityAny)]
pub struct SerializedService {
    serialized: Serialized,
}

impl SerializedService {
    /// Returns a SerializedService
    pub fn new(serialized: Serialized) -> SerializedService {
        SerializedService { serialized }
    }
}

impl IntrospectObject for SerializedService {
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

impl Service for SerializedService {}
