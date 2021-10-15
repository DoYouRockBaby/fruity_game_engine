use crate::js_value::value::JsValue;
use crate::runtime::JsRuntimeHandles;
use rusty_v8 as v8;
use std::any::Any;
use std::fmt::Debug;
use std::sync::Arc;
use std::sync::Mutex;

pub struct JsString {
    v8_value: v8::Global<v8::String>,
}

unsafe impl Send for JsString {}
unsafe impl Sync for JsString {}

impl JsString {
    pub fn new(handles: Arc<Mutex<JsRuntimeHandles>>, string: &str) -> JsString {
        // Get scope
        let handles_lock = handles.lock().unwrap();
        let scope = handles_lock.handle_scope();

        // Create the value
        let v8_value = v8::String::new(&mut scope, string).unwrap();
        let v8_value = v8::Global::new(&mut scope, v8_value);

        JsString { v8_value }
    }
}

impl JsValue for JsString {
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

impl Debug for JsString {
    fn fmt(
        &self,
        _formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}
