use crate::js_value::value::JsValue;
use rusty_v8 as v8;
use std::any::Any;
use std::fmt::Debug;

pub struct JsString {
    v8_value: v8::Global<v8::String>,
}

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
}

impl Debug for JsString {
    fn fmt(
        &self,
        _formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}
