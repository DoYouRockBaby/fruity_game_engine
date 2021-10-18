use crate::serialize::serialized::Serialized;
use crate::service::service::Service;
use crate::service::utils::cast_service_mut;
use crate::ServiceManager;
use fruity_any_derive::*;
use fruity_introspect::IntrospectMethods;
use fruity_introspect::MethodCaller;
use fruity_introspect::MethodInfo;
use std::sync::Arc;
use std::sync::RwLock;

/// A wrapper for services that come from scripting languages as serialized
#[derive(Debug, FruityAny)]
pub struct SerializedService {
    service_manager: Arc<RwLock<ServiceManager>>,
    serialized: Serialized,
}

impl SerializedService {
    /// Returns a SerializedService
    pub fn new(
        service_manager: Arc<RwLock<ServiceManager>>,
        serialized: Serialized,
    ) -> SerializedService {
        SerializedService {
            service_manager,
            serialized,
        }
    }
}

impl IntrospectMethods<Serialized> for SerializedService {
    fn get_method_infos(&self) -> Vec<MethodInfo<Serialized>> {
        let this = self.clone();

        if let Serialized::Object { fields, .. } = &this.serialized {
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
                        call: MethodCaller::Mut(Arc::new(move |this, args| {
                            let this = cast_service_mut::<SerializedService>(this);
                            callback(this.service_manager.clone(), args)
                        })),
                    }
                })
                .collect::<Vec<_>>()
        } else {
            vec![]
        }
    }
}

impl Service for SerializedService {}
