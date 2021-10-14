use crate::serialize::serialize::serialize_v8;
use crate::JsRuntime;
use fruity_ecs::serialize::serialized::Serialized;
use fruity_ecs::service::service_manager::ServiceManager;
use fruity_introspect::IntrospectError;
use rusty_v8 as v8;
use std::convert::TryFrom;
use std::sync::Arc;
use std::sync::RwLock;

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

    if v8_value.is_array() {
        let v8_array = v8::Local::<v8::Array>::try_from(v8_value).unwrap();
        let serialized_array = (0..v8_array.length())
            .map(|index| {
                Serialized::String(
                    v8_array
                        .get_index(scope, index)
                        .unwrap()
                        .to_rust_string_lossy(scope),
                )
            })
            .collect::<Vec<_>>();

        return Some(Serialized::Array(serialized_array));
    }

    if v8_value.is_function() {
        let js_function = JsFunctionWrapper::from_value(scope, v8_value);

        let callback = move |service_manager: Arc<RwLock<ServiceManager>>,
                             args: Vec<Serialized>|
              -> Result<Option<Serialized>, IntrospectError> {
            // Get scope
            let js_runtime = {
                let service_manager = service_manager.read().unwrap();
                service_manager.get::<JsRuntime>().unwrap()
            };

            let js_runtime = js_runtime.read().unwrap();
            let lock = js_runtime.datas.try_lock();
            match lock {
                Ok(mut datas) => {
                    let mut scope = datas.handle_scope();
                    let context = v8::Context::new(&mut scope);

                    // Instantiate parameters and return handle
                    let args = args
                        .iter()
                        .filter_map(|arg| serialize_v8(&mut scope, arg))
                        .collect::<Vec<_>>();

                    let global = context.global(&mut scope);
                    let recv: v8::Local<v8::Value> = global.into();

                    // Call function
                    let result = js_function.call(&mut scope, recv, &args);

                    // Return result
                    if let Some(result) = result {
                        let result = deserialize_v8(&mut scope, result);
                        Ok(result)
                    } else {
                        Ok(None)
                    }
                }
                Err(_) => Err(IntrospectError::NestedCallback),
            }
        };

        return Some(Serialized::Callback(Arc::new(callback)));
    }

    None
}

struct JsFunctionWrapper {
    inner: rusty_v8::Global<rusty_v8::Function>,
}

unsafe impl Send for JsFunctionWrapper {}
unsafe impl Sync for JsFunctionWrapper {}

impl JsFunctionWrapper {
    fn from_value(scope: &mut v8::HandleScope, value: v8::Local<v8::Value>) -> JsFunctionWrapper {
        let js_function = v8::Local::<v8::Function>::try_from(value).unwrap();
        let js_function = v8::Global::new(scope, js_function);

        JsFunctionWrapper { inner: js_function }
    }

    pub fn call<'s>(
        &self,
        scope: &mut v8::HandleScope<'s>,
        recv: v8::Local<v8::Value>,
        args: &[v8::Local<v8::Value>],
    ) -> Option<v8::Local<'s, v8::Value>> {
        let js_function = v8::Local::<v8::Function>::new(scope, self.inner.clone());
        js_function.call(scope, recv, &args)
    }
}
