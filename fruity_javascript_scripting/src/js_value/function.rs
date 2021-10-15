use crate::js_value::value::JsValue;
use fruity_any_derive::*;
use rusty_v8 as v8;
use std::fmt::Debug;

#[derive(FruityAny)]
pub struct JsFunction {
    v8_value: v8::Global<v8::Function>,
}

unsafe impl Send for JsFunction {}
unsafe impl Sync for JsFunction {}

impl JsFunction {
    pub fn new(
        scope: &mut v8::HandleScope,
        name: &str,
        callback: impl v8::MapFnTo<v8::FunctionCallback>,
    ) -> JsFunction {
        // Create the function
        let name = v8::String::new(scope, name).unwrap();
        let function = v8::Function::builder(callback)
            .data(name.into())
            .build(scope)
            .unwrap();

        let function = v8::Global::new(scope, function);

        JsFunction { v8_value: function }
    }
}

impl JsValue for JsFunction {
    fn as_v8<'a>(&mut self, scope: &mut v8::HandleScope<'a>) -> v8::Local<'a, v8::Value> {
        // Return the value
        let v8_value = v8::Local::new(scope, &self.v8_value);
        v8_value.into()
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
