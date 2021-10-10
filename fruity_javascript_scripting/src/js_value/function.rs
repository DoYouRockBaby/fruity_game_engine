use crate::js_value::value::JsValue;
use rusty_v8 as v8;
use std::any::Any;

pub struct JsFunction {
    pub(crate) function_builder: Option<v8::FunctionBuilder<'static, v8::Function>>,
}

impl JsFunction {
    pub fn new(callback: impl v8::MapFnTo<v8::FunctionCallback>) -> JsFunction {
        JsFunction {
            function_builder: Some(v8::Function::builder(callback)),
        }
    }
}

impl JsValue for JsFunction {
    fn register(&mut self, scope: &mut v8::HandleScope, name: &str, parent: v8::Local<v8::Object>) {
        let function = self.function_builder.take().unwrap().build(scope).unwrap();
        let key = v8::String::new(scope, name).unwrap();
        parent.set(scope, key.into(), function.into());
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
