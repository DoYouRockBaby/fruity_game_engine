use crate::serialize::serialize::serialize_v8;
use crate::JsRuntime;
use fruity_ecs::serialize::serialized::Serialized;
use fruity_ecs::service::service_manager::ServiceManager;
use fruity_introspect::IntrospectError;
use rusty_v8 as v8;
use rusty_v8::Handle;
use std::convert::TryFrom;

pub fn deserialize_v8<'a>(
    scope: &mut v8::HandleScope<'a>,
    v8_value: v8::Local<v8::Value>,
) -> Option<Serialized> {
    if v8_value.is_int32() {
        return Some(Serialized::I32(v8_value.int32_value(scope).unwrap()));
    }

    if v8_value.is_uint32() {
        return Some(Serialized::U32(v8_value.uint32_value(scope).unwrap()));
    }

    if v8_value.is_big_int() {
        return Some(Serialized::I64(v8_value.integer_value(scope).unwrap()));
    }

    if v8_value.is_number() {
        return Some(Serialized::F64(v8_value.number_value(scope).unwrap()));
    }

    if v8_value.is_boolean() {
        return Some(Serialized::Bool(v8_value.boolean_value(scope)));
    }

    if v8_value.is_string() {
        return Some(Serialized::String(v8_value.to_rust_string_lossy(scope)));
    }

    if v8_value.is_function() {
        let js_function = v8::Local::<v8::Function>::try_from(v8_value).unwrap();
        let js_function = v8::Global::new(scope, js_function);

        let callback = move |service_manager: &ServiceManager,
                             args: Vec<Serialized>|
              -> Result<Option<Serialized>, IntrospectError> {
            // Get scope
            let js_runtime = service_manager.get::<JsRuntime>().unwrap();
            let js_runtime = js_runtime.write().unwrap();
            let scope = js_runtime.handle_scope();
            let context = v8::Context::new(&mut scope);

            // Instantiate parameters and return handle
            let args = args
                .iter()
                .filter_map(|arg| serialize_v8(&mut scope, arg))
                .collect::<Vec<_>>();

            let global = context.global(&mut scope);
            let recv: v8::Local<v8::Value> = global.into();

            // Call function
            let js_function = v8::Local::<v8::Function>::new(&mut scope, js_function.clone());
            js_function.call(&mut scope, recv, &args);

            // Return result
            let result = deserialize_v8(&mut scope, recv);
            Ok(result)
        };

        return Some(Serialized::Callback(Box::new(callback)));
    }

    None
}

struct FunctionData<'a> {
    js_function: v8::Local<'a, v8::Function>,
    scope: &'a mut v8::HandleScope<'a>,
}
