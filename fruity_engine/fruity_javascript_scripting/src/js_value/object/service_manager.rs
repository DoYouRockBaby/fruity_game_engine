use crate::js_value::utils::get_intern_value_from_v8_object;
use crate::js_value::utils::inject_option_serialized_into_v8_return_value;
use crate::serialize::deserialize::deserialize_v8;
use crate::JsObject;
use fruity_core::service::serialized_service::SerializedService;
use fruity_core::service::service_manager::ServiceManager;
use fruity_introspect::serialized::Serialized;
use rusty_v8 as v8;
use std::sync::Arc;
use std::sync::RwLock;

impl JsObject {
    pub fn from_service_manager(
        scope: &mut v8::HandleScope,
        service_manager: Arc<RwLock<ServiceManager>>,
    ) -> JsObject {
        let mut object = JsObject::from_intern_value(scope, "ServiceManager", service_manager);
        object.set_func(scope, "register", service_manager_register_callback, None);
        object.set_func(scope, "get", service_manager_get_callback, None);

        object
    }
}

fn service_manager_register_callback(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut _return_value: v8::ReturnValue,
) {
    // Get this as an service_manager
    let intern_value =
        get_intern_value_from_v8_object::<Arc<RwLock<ServiceManager>>>(scope, args.this());

    if let Some(service_manager) = intern_value {
        // Build the arguments
        let name = args
            .get(0)
            .to_string(scope)
            .unwrap()
            .to_rust_string_lossy(scope);

        let object = deserialize_v8(scope, args.get(1)).unwrap();

        // Call the function
        let mut service_manager_writer = service_manager.write().unwrap();
        let service = SerializedService::new(object);
        service_manager_writer.register(&name, service);
    }
}

fn service_manager_get_callback(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut return_value: v8::ReturnValue,
) {
    // Get this as an service_manager
    let intern_value =
        get_intern_value_from_v8_object::<Arc<RwLock<ServiceManager>>>(scope, args.this());

    if let Some(service_manager) = intern_value {
        // Build the arguments
        let name = args
            .get(0)
            .to_string(scope)
            .unwrap()
            .to_rust_string_lossy(scope);

        // Call the function
        let service_manager = service_manager.read().unwrap();
        let result = service_manager
            .get_by_name(&name)
            .map(|result| Serialized::NativeObject(Box::new(result)));

        // Return the result
        inject_option_serialized_into_v8_return_value(scope, &result, &mut return_value);
    }
}