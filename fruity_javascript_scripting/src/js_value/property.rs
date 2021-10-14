use crate::js_value::object::JsObjectInternalObject;
use crate::js_value::utils::get_internal_object_from_v8_property_args;
use crate::js_value::value::JsValue;
use crate::serialize::deserialize::deserialize_v8;
use crate::serialize::serialize::serialize_v8;
use rusty_v8 as v8;
use std::any::Any;
use std::fmt::Debug;

pub struct JsProperty {}

impl JsProperty {
    pub fn new() -> JsProperty {
        JsProperty {}
    }
}

impl JsValue for JsProperty {
    fn register(&mut self, scope: &mut v8::HandleScope, name: &str, parent: v8::Local<v8::Object>) {
        let key = v8::String::new(scope, name).unwrap();
        parent.set_accessor_with_setter(scope, key.into(), component_getter, component_setter);
    }

    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self as &mut dyn Any
    }

    fn as_boxed_any(self: Box<Self>) -> Box<dyn Any> {
        self as Box<dyn Any>
    }
}

fn component_getter(
    scope: &mut v8::HandleScope,
    name: v8::Local<v8::Name>,
    args: v8::PropertyCallbackArguments,
    mut return_value: v8::ReturnValue,
) {
    // Get this as service methods
    let internal_object = get_internal_object_from_v8_property_args(scope, &args);

    // Extract the current method info
    if let JsObjectInternalObject::Component(component) = internal_object {
        let field_info = {
            let component = component.read().unwrap();

            let field_infos = component.get_field_infos();
            let name = name.to_string(scope).unwrap().to_rust_string_lossy(scope);

            field_infos
                .iter()
                .find(|method_info| method_info.name == name)
                .unwrap()
                .clone()
        };

        // Call the function
        let component = component.read().unwrap();
        let result = (field_info.getter)(component.as_any_ref());

        // Return the result
        let deserialized = serialize_v8(scope, &result);

        if let Some(serialized) = deserialized {
            return_value.set(serialized.into());
        }
    }
}

fn component_setter(
    scope: &mut v8::HandleScope,
    name: v8::Local<v8::Name>,
    value: v8::Local<v8::Value>,
    args: v8::PropertyCallbackArguments,
) {
    // Get this as service methods
    let internal_object = get_internal_object_from_v8_property_args(scope, &args);

    // Extract the current method info
    if let JsObjectInternalObject::Component(component) = internal_object {
        let field_info = {
            let component = component.read().unwrap();

            let field_infos = component.get_field_infos();
            let name = name.to_string(scope).unwrap().to_rust_string_lossy(scope);

            field_infos
                .iter()
                .find(|method_info| method_info.name == name)
                .unwrap()
                .clone()
        };

        // Build the arguments
        let deserialized_arg = deserialize_v8(scope, value).unwrap();

        // Call the function
        let mut component = component.write().unwrap();
        (field_info.setter)(component.as_any_mut(), deserialized_arg);
    }
}

impl Debug for JsProperty {
    fn fmt(
        &self,
        _formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}
