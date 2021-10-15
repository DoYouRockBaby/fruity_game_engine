use crate::js_value::function::JsFunction;
use crate::js_value::utils::format_function_name_from_rust_to_js;
use crate::js_value::value::JsValue;
use crate::runtime::JsRuntimeHandles;
use core::ffi::c_void;
use rusty_v8 as v8;
use std::any::Any;
use std::fmt::Debug;
use std::sync::Arc;
use std::sync::Mutex;

pub mod component;
pub mod component_list;
pub mod entity;
pub mod iterator;
pub mod service;

#[derive(Debug)]
pub struct JsObject {
    v8_value: v8::Global<v8::Object>,
}

impl JsObject {
    pub fn new(handles: Arc<Mutex<JsRuntimeHandles>>) -> JsObject {
        // Get scope
        let handles_lock = handles.lock().unwrap();
        let scope = handles_lock.handle_scope();

        // Create the object
        JsObject {
            v8_value: v8::Global::new(&mut scope, v8::Object::new(&mut scope)),
        }
    }

    pub fn from_intern_value<T: Any>(
        handles: Arc<Mutex<JsRuntimeHandles>>,
        intern_value: T,
    ) -> JsObject {
        // Get scope
        let handles_lock = handles.lock().unwrap();
        let scope = handles_lock.handle_scope();

        // Set the intern value
        let object_template = v8::ObjectTemplate::new(&mut scope);
        object_template.set_internal_field_count(1);

        let object = object_template.new_instance(&mut scope).unwrap();

        let intern_value = Box::new(intern_value);
        let ref_value =
            v8::External::new(&mut scope, Box::leak(intern_value) as *mut T as *mut c_void);

        object.set_internal_field(0, ref_value.into());

        // Create the object
        JsObject {
            v8_value: v8::Global::new(&mut scope, v8::Object::new(&mut scope)),
        }
    }

    pub fn add_field<T: JsValue>(
        &mut self,
        handles: Arc<Mutex<JsRuntimeHandles>>,
        name: &str,
        value: T,
    ) {
        // Get scope
        let handles_lock = handles.lock().unwrap();
        let scope = handles_lock.handle_scope();

        // Add the value into the object field
        let key = v8::String::new(&mut scope, &format_function_name_from_rust_to_js(name)).unwrap();
        let v8_value = v8::Local::new(&mut scope, self.v8_value);
        v8_value.set(&mut scope, key.into(), value.as_v8(handles));
        // TODO: try to remove
        self.v8_value = v8::Global::new(&mut scope, v8::Object::new(&mut scope));
    }

    pub fn add_property(
        &mut self,
        handles: Arc<Mutex<JsRuntimeHandles>>,
        name: &str,
        getter: impl for<'s> v8::MapFnTo<v8::AccessorNameGetterCallback<'s>>,
        setter: impl for<'s> v8::MapFnTo<v8::AccessorNameSetterCallback<'s>>,
    ) {
        // Get scope
        let handles_lock = handles.lock().unwrap();
        let scope = handles_lock.handle_scope();

        // Add the value into the object field
        let key = v8::String::new(&mut scope, &format_function_name_from_rust_to_js(name)).unwrap();
        let v8_value = v8::Local::new(&mut scope, self.v8_value);
        v8_value.set_accessor_with_setter(&mut scope, key.into(), getter, setter);
        // TODO: try to remove
        self.v8_value = v8::Global::new(&mut scope, v8::Object::new(&mut scope));
    }

    pub fn set_func(
        &mut self,
        handles: Arc<Mutex<JsRuntimeHandles>>,
        name: &str,
        callback: impl v8::MapFnTo<v8::FunctionCallback>,
    ) {
        self.add_field(handles, name, JsFunction::new(handles, callback));
    }
}

impl JsValue for JsObject {
    fn as_v8(&mut self, handles: Arc<Mutex<JsRuntimeHandles>>) -> v8::Local<v8::Value> {
        // Get scope
        let handles = handles.lock().unwrap();
        let scope = handles.handle_scope();

        // Return the value
        let v8_value = v8::Local::new(&mut scope, self.v8_value);
        v8_value.into()
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
