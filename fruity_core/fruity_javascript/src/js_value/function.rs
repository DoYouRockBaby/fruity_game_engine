use crate::js_value::value::JsValue;
use crate::serialize::serialize::serialize_v8;
use fruity_core::serialize::serialized::Serialized;
use rusty_v8 as v8;
use std::fmt::Debug;

pub struct JsFunction {
    v8_value: v8::Global<v8::Function>,
}

impl JsFunction {
    pub fn new(
        scope: &mut v8::HandleScope,
        data: Option<Serialized>,
        callback: impl v8::MapFnTo<v8::FunctionCallback>,
    ) -> JsFunction {
        // Create the function
        let mut builder = v8::Function::builder(callback);

        if let Some(data) = data {
            if let Some(data) = serialize_v8(scope, &data) {
                builder = builder.data(data);
            }
        }

        let function = builder.build(scope).unwrap();

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
