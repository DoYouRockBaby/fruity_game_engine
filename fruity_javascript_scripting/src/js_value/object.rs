use crate::js_value::function::JsFunction;
// use crate::js_value::function::JsFunctionCallback;
use crate::js_value::value::JsValue;
use rusty_v8 as v8;
use std::any::Any;
use std::collections::HashMap;

pub struct JsObject {
    pub(crate) fields: HashMap<String, Box<dyn JsValue>>,
}

impl JsObject {
    pub fn new() -> JsObject {
        JsObject {
            fields: HashMap::new(),
        }
    }

    pub fn add_object(&mut self, name: &str) -> &mut JsObject {
        self.fields
            .insert(name.to_string(), Box::new(JsObject::new()));

        self.fields
            .get_mut(&name.to_string())
            .unwrap()
            .as_mut_any()
            .downcast_mut::<JsObject>()
            .unwrap()
    }

    pub fn set_func(
        &mut self,
        name: &str,
        callback: impl v8::MapFnTo<v8::FunctionCallback>,
    ) -> &mut JsFunction {
        self.fields
            .insert(name.to_string(), Box::new(JsFunction::new(callback)));

        self.fields
            .get_mut(&name.to_string())
            .unwrap()
            .as_mut_any()
            .downcast_mut::<JsFunction>()
            .unwrap()
    }
}

impl JsValue for JsObject {
    fn register(&mut self, scope: &mut v8::HandleScope, name: &str, parent: v8::Local<v8::Object>) {
        let object = v8::Object::new(scope);
        let key = v8::String::new(scope, name).unwrap();
        parent.set(scope, key.into(), object.into());

        self.fields
            .iter_mut()
            .for_each(|(name, field)| field.register(scope, name, object));
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
