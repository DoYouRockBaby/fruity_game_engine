use crate::js_value::utils::format_function_name_from_rust_to_js;
use crate::js_value::utils::get_intern_value_from_v8_args;
use crate::js_value::utils::inject_option_serialized_into_v8_return_value;
use crate::serialize::deserialize::deserialize_v8;
use crate::JsObject;
use fruity_ecs::service::service::Service;
use fruity_introspect::log_introspect_error;
use fruity_introspect::MethodCaller;
use rusty_v8 as v8;
use std::sync::Arc;
use std::sync::RwLock;

impl JsObject {
    pub fn from_service(
        scope: &mut v8::HandleScope,
        service: Arc<RwLock<Box<dyn Service>>>,
    ) -> JsObject {
        let mut object = JsObject::from_intern_value(scope, service.clone());

        let method_infos = {
            let reader = service.read().unwrap();
            reader.get_method_infos()
        };

        for method_info in method_infos {
            object.set_func(
                scope,
                &format_function_name_from_rust_to_js(&method_info.name),
                service_callback,
            );
        }

        object
    }
}

fn service_callback(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut return_value: v8::ReturnValue,
) {
    // Get this as a service
    let intern_value = get_intern_value_from_v8_args::<Arc<RwLock<Box<dyn Service>>>>(scope, &args);

    if let Some(service) = intern_value {
        // Extract the current method info
        let method_info = {
            let service = service.read().unwrap();

            let method_infos = service.get_method_infos().clone();
            let name = args
                .data()
                .unwrap()
                .to_string(scope)
                .unwrap()
                .to_rust_string_lossy(scope);

            method_infos
                .iter()
                .find(|method_info| format_function_name_from_rust_to_js(&method_info.name) == name)
                .unwrap()
                .clone()
        };

        // Build the arguments
        let deserialized_args = (0..args.length())
            .filter_map(|index| deserialize_v8(scope, args.get(index)))
            .collect::<Vec<_>>();

        // Call the function
        let result = match method_info.call {
            MethodCaller::Const(call) => {
                let service = service.read().unwrap();
                match call(service.as_any_ref(), deserialized_args) {
                    Ok(result) => result,
                    Err(err) => {
                        log_introspect_error(&err);
                        None
                    }
                }
            }
            MethodCaller::Mut(call) => {
                let mut service = service.write().unwrap();
                match call(service.as_any_mut(), deserialized_args) {
                    Ok(result) => result,
                    Err(err) => {
                        log_introspect_error(&err);
                        None
                    }
                }
            }
        };

        // Return the result
        inject_option_serialized_into_v8_return_value(scope, &result, &mut return_value);
    }
}