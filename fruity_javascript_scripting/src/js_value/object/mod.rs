use crate::js_value::function::JsFunction;
use crate::js_value::utils::format_function_name_from_rust_to_js;
use crate::js_value::value::JsValue;
use core::ffi::c_void;
use fruity_any_derive::*;
use rusty_v8 as v8;
use std::any::Any;
use std::fmt::Debug;

pub mod component;
pub mod component_list_rwlock;
pub mod component_rwlock;
pub mod entity;
pub mod iterator;
pub mod service;
pub mod service_manager;

#[derive(Debug, FruityAny)]
pub struct JsObject {
    v8_value: v8::Global<v8::Object>,
}

impl JsObject {
    pub fn new(scope: &mut v8::HandleScope) -> JsObject {
        let v8_value = {
            let v8_value = v8::Object::new(scope);
            v8::Global::new(scope, v8_value)
        };

        // Create the object
        JsObject { v8_value }
    }

    pub fn from_v8(v8_value: v8::Global<v8::Object>) -> JsObject {
        JsObject { v8_value }
    }

    pub fn from_intern_value<T: Any>(
        scope: &mut v8::HandleScope,
        identifier: &str,
        intern_value: T,
    ) -> JsObject {
        // Set the intern value
        let object_template = v8::ObjectTemplate::new(scope);
        object_template.set_internal_field_count(2);

        let object = object_template.new_instance(scope).unwrap();

        let intern_value = Box::new(intern_value);
        let ref_value = v8::External::new(scope, Box::leak(intern_value) as *mut T as *mut c_void);
        let identifier = v8::String::new(scope, identifier).unwrap();

        object.set_internal_field(0, ref_value.into());
        object.set_internal_field(1, identifier.into());

        // Create the object
        JsObject {
            v8_value: v8::Global::new(scope, object),
        }
    }

    pub fn add_field<T: JsValue>(&mut self, scope: &mut v8::HandleScope, name: &str, mut value: T) {
        // Add the value into the object field
        let key = v8::String::new(scope, &format_function_name_from_rust_to_js(name)).unwrap();
        let v8_value = v8::Local::new(scope, &self.v8_value);

        let field_value = value.as_v8(scope);
        v8_value.set(scope, key.into(), field_value.into());
        // TODO: try to remove
        self.v8_value = v8::Global::new(scope, v8_value);
    }

    pub fn add_property(
        &mut self,
        scope: &mut v8::HandleScope,
        name: &str,
        getter: impl for<'s> v8::MapFnTo<v8::AccessorNameGetterCallback<'s>>,
        setter: impl for<'s> v8::MapFnTo<v8::AccessorNameSetterCallback<'s>>,
    ) {
        // Add the value into the object field
        let key = v8::String::new(scope, &format_function_name_from_rust_to_js(name)).unwrap();
        let v8_value = v8::Local::new(scope, &self.v8_value);
        v8_value.set_accessor_with_setter(scope, key.into(), getter, setter);
        // TODO: try to remove
        self.v8_value = v8::Global::new(scope, v8_value);
    }

    pub fn set_func(
        &mut self,
        scope: &mut v8::HandleScope,
        name: &str,
        callback: impl v8::MapFnTo<v8::FunctionCallback>,
    ) {
        let function = JsFunction::new(scope, name, callback);
        self.add_field(scope, name, function);
    }
}

impl JsValue for JsObject {
    fn as_v8<'a>(&mut self, scope: &mut v8::HandleScope<'a>) -> v8::Local<'a, v8::Value> {
        // Return the value
        let v8_value = v8::Local::new(scope, &self.v8_value);
        v8_value.into()
    }
}
