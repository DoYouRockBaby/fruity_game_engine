use crate::serialize::deserialize::deserialize_v8;
use fruity_ecs::serialize::deserialize::deserialize_any;
use fruity_ecs::serialize::deserialize::Deserialize;
use rusty_v8 as v8;

#[derive(Debug)]
pub struct JsResult<'a> {
    scope: v8::HandleScope<'a>,
    v8_value: Option<v8::Local<'a, v8::Value>>,
}

impl<'a> JsResult<'a> {
    pub fn new<'b>(
        scope: v8::HandleScope<'b>,
        v8_value: Option<v8::Local<'b, v8::Value>>,
    ) -> JsResult<'b> {
        JsResult::<'b> { scope, v8_value }
    }

    pub fn deserialize<T: Deserialize<Value = T> + 'static>(mut self) -> Option<T> {
        let v8_value = match self.v8_value {
            Some(v8_value) => v8_value,
            None => return None,
        };

        let value = match deserialize_v8(&mut self.scope, v8_value) {
            Some(value) => value,
            None => return None,
        };

        let value = match deserialize_any(value) {
            Some(value) => value,
            None => return None,
        };

        match value.downcast::<T>() {
            Ok(value) => Some(*value),
            Err(_) => None,
        }
    }
}
