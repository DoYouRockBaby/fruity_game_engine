use crate::js_value::value::JsValue;
use rusty_v8 as v8;
use std::any::Any;
use std::fmt::Debug;

pub struct JsString {
    v8_value: v8::Global<v8::String>,
}

unsafe impl Send for JsString {}
unsafe impl Sync for JsString {}

impl JsString {
    pub fn new(scope: &mut v8::HandleScope, string: &str) -> JsString {
        // Create the value
        let v8_value = v8::String::new(scope, string).unwrap();
        let v8_value = v8::Global::new(scope, v8_value);

        JsString { v8_value }
    }
}

impl JsValue for JsString {
    fn as_v8<'a>(&mut self, scope: &mut v8::HandleScope<'a>) -> v8::Local<'a, v8::Value> {
        // Return the value
        let v8_value = v8::Local::new(scope, &self.v8_value);
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
