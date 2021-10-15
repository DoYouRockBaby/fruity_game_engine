use crate::js_value::value::JsValue;
use crate::runtime::JsRuntimeHandles;
use rusty_v8 as v8;
use std::any::Any;
use std::fmt::Debug;
use std::sync::Arc;
use std::sync::Mutex;

pub struct JsFunction {
    v8_value: v8::Global<v8::Function>,
}

unsafe impl Send for JsFunction {}
unsafe impl Sync for JsFunction {}

impl JsFunction {
    pub fn new(
        handles: Arc<Mutex<JsRuntimeHandles>>,
        callback: impl v8::MapFnTo<v8::FunctionCallback>,
    ) -> JsFunction {
        // Get scope
        let handles_lock = handles.lock().unwrap();
        let scope = handles_lock.handle_scope();

        // Create the function
        let function = v8::Function::builder(callback).build(&mut scope).unwrap();
        let function = v8::Global::new(&mut scope, function);

        JsFunction { v8_value: function }
    }
}

impl JsValue for JsFunction {
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

impl Debug for JsFunction {
    fn fmt(
        &self,
        _formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}
