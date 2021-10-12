use crate::js_value::object::ComponentMut;
use crate::js_value::object::ComponentRef;
use crate::js_value::value::JsValue;
use crate::serialize::deserialize::deserialize_v8;
use crate::serialize::serialize::serialize_v8;
use rusty_v8 as v8;
use std::any::Any;

pub struct JsProperty {
    pub(crate) implement_setter: bool,
}

impl JsProperty {
    pub fn new(implement_setter: bool) -> JsProperty {
        JsProperty { implement_setter }
    }
}

impl JsValue for JsProperty {
    fn register(&mut self, scope: &mut v8::HandleScope, name: &str, parent: v8::Local<v8::Object>) {
        let key = v8::String::new(scope, name).unwrap();

        if self.implement_setter {
            parent.set_accessor_with_setter(scope, key.into(), component_getter, component_setter);
        } else {
            parent.set_accessor(scope, key.into(), component_getter);
        }
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
    let this = args.this().get_internal_field(scope, 0).unwrap();
    let this = unsafe { v8::Local::<v8::External>::cast(this) };
    let this = this.value() as *mut ComponentRef;
    let this = unsafe { this.as_ref().unwrap() };

    // Extract the current method info
    let field_info = {
        let field_infos = this.0.get_field_infos();
        let name = name.to_string(scope).unwrap().to_rust_string_lossy(scope);

        field_infos
            .iter()
            .find(|method_info| method_info.name == name)
            .unwrap()
            .clone()
    };

    // Call the function
    let result = (field_info.getter)(this.0.as_any_ref());

    // Return the result
    let deserialized = serialize_v8(scope, result);

    if let Some(serialized) = deserialized {
        return_value.set(serialized.into());
    }
}

fn component_setter(
    scope: &mut v8::HandleScope,
    name: v8::Local<v8::Name>,
    value: v8::Local<v8::Value>,
    args: v8::PropertyCallbackArguments,
) {
    // Get this as service methods
    let this = args.this().get_internal_field(scope, 0).unwrap();
    let this = unsafe { v8::Local::<v8::External>::cast(this) };
    let this = this.value() as *mut ComponentMut;
    let this = unsafe { this.as_mut().unwrap() };

    // Extract the current method info
    let field_info = {
        let field_infos = this.0.get_field_infos();
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
    (field_info.setter)(this.0.as_any_mut(), deserialized_arg);
}
