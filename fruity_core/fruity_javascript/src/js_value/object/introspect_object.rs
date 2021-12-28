use crate::js_value::utils::check_object_intern_identifier;
use crate::js_value::utils::format_function_name_from_rust_to_js;
use crate::js_value::utils::get_intern_value_from_v8_object;
use crate::js_value::utils::get_intern_value_from_v8_object_mut;
use crate::js_value::utils::inject_option_serialized_into_v8_return_value;
use crate::js_value::utils::inject_serialized_into_v8_return_value;
use crate::serialize::deserialize::deserialize_v8;
use crate::thread_scope_stack::pop_thread_scope_stack;
use crate::thread_scope_stack::push_thread_scope_stack;
use crate::JsObject;
use fruity_core::introspect::log_introspect_error;
use fruity_core::introspect::MethodCaller;
use fruity_core::introspect::SetterCaller;
use fruity_core::serialize::serialized::SerializableObject;
use fruity_core::serialize::serialized::Serialized;
use rusty_v8 as v8;

impl JsObject {
    pub fn from_introspect_object(
        scope: &mut v8::HandleScope,
        introspect_object: Box<dyn SerializableObject>,
    ) -> JsObject {
        let method_infos = introspect_object.get_method_infos();
        let field_infos = introspect_object.get_field_infos();

        let mut object =
            JsObject::from_intern_value(scope, "SerializableObject", introspect_object);

        for method_info in method_infos {
            let function_name = format_function_name_from_rust_to_js(&method_info.name);
            object.set_func(
                scope,
                &function_name,
                method_callback,
                Some(Serialized::String(function_name.clone())),
            );
        }

        for field_info in field_infos {
            object.add_property(scope, &field_info.name, getter_callback, setter_callback);
        }

        object
    }
}

fn method_callback(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut return_value: v8::ReturnValue,
) {
    // Get this as an introspect object
    let intern_value =
        get_intern_value_from_v8_object_mut::<Box<dyn SerializableObject>>(scope, args.this());

    if let Some(introspect_object) = intern_value {
        // Extract the current method info
        let method_info = {
            let method_infos = introspect_object.get_method_infos().clone();
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

        // Push the scope into the scope stack
        push_thread_scope_stack(scope);

        // Call the function
        let result = match method_info.call {
            MethodCaller::Const(call) => {
                match call(introspect_object.as_any_ref(), deserialized_args) {
                    Ok(result) => result,
                    Err(err) => {
                        log_introspect_error(&err);
                        None
                    }
                }
            }
            MethodCaller::Mut(call) => {
                match call(introspect_object.as_any_mut(), deserialized_args) {
                    Ok(result) => result,
                    Err(err) => {
                        log_introspect_error(&err);
                        None
                    }
                }
            }
        };

        // Remove the scope into the scope stack
        pop_thread_scope_stack();

        // Return the result
        inject_option_serialized_into_v8_return_value(scope, result, &mut return_value);
    }
}

fn getter_callback(
    scope: &mut v8::HandleScope,
    name: v8::Local<v8::Name>,
    args: v8::PropertyCallbackArguments,
    mut return_value: v8::ReturnValue,
) {
    // Get this as an introspect object
    let intern_value =
        get_intern_value_from_v8_object::<Box<dyn SerializableObject>>(scope, args.this());

    if let Some(introspect_object) = intern_value {
        // Extract the current field info
        let field_info = {
            let field_infos = introspect_object.get_field_infos();
            let name = name.to_string(scope).unwrap().to_rust_string_lossy(scope);

            field_infos
                .iter()
                .find(|method_info| format_function_name_from_rust_to_js(&method_info.name) == name)
                .unwrap()
                .clone()
        };

        // Push the scope into the scope stack
        push_thread_scope_stack(scope);

        // Call the function
        let result = (field_info.getter)(introspect_object.as_any_ref());

        // Remove the scope into the scope stack
        pop_thread_scope_stack();

        // Return the result
        inject_serialized_into_v8_return_value(scope, result, &mut return_value);
    }
}

fn setter_callback(
    scope: &mut v8::HandleScope,
    name: v8::Local<v8::Name>,
    value: v8::Local<v8::Value>,
    args: v8::PropertyCallbackArguments,
) {
    // Get this as an introspect object
    let intern_value =
        get_intern_value_from_v8_object_mut::<Box<dyn SerializableObject>>(scope, args.this());

    if let Some(introspect_object) = intern_value {
        // Extract the current field info
        let field_info = {
            let field_infos = introspect_object.get_field_infos();
            let name = name.to_string(scope).unwrap().to_rust_string_lossy(scope);

            field_infos
                .iter()
                .find(|method_info| method_info.name == name)
                .unwrap()
                .clone()
        };

        // Build the arguments
        let deserialized_arg = deserialize_v8(scope, value).unwrap();

        // Push the scope into the scope stack
        push_thread_scope_stack(scope);

        // Call the function
        match field_info.setter {
            SetterCaller::Const(call) => {
                call(introspect_object.as_any_ref(), deserialized_arg);
            }
            SetterCaller::Mut(call) => {
                call(introspect_object.as_any_mut(), deserialized_arg);
            }
            SetterCaller::None => (),
        };

        // Remove the scope into the scope stack
        pop_thread_scope_stack();
    }
}

pub fn deserialize_v8_introspect_object(
    scope: &mut v8::HandleScope,
    v8_value: v8::Local<v8::Value>,
) -> Option<Box<dyn SerializableObject>> {
    let v8_object = check_object_intern_identifier(scope, v8_value, "SerializableObject")?;
    let intern_value =
        get_intern_value_from_v8_object::<Box<dyn SerializableObject>>(scope, v8_object)?;

    Some(intern_value.clone())
}
