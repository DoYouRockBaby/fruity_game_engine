use crate::js_value::value::JsValue;
use rusty_v8 as v8;
use std::any::Any;
use std::fmt::Debug;

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
    _scope: &mut v8::HandleScope,
    _name: v8::Local<v8::Name>,
    _args: v8::PropertyCallbackArguments,
    mut _return_value: v8::ReturnValue,
) {
}

fn component_setter(
    _scope: &mut v8::HandleScope,
    _name: v8::Local<v8::Name>,
    _value: v8::Local<v8::Value>,
    _args: v8::PropertyCallbackArguments,
) {
}

impl Debug for JsProperty {
    fn fmt(
        &self,
        _formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}
