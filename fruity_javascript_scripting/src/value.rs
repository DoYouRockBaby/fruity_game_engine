use rusty_v8 as v8;

pub trait ValueDeserializer {
    type Value;

    fn deserialize(
        scope: &mut v8::HandleScope,
        v8_value: v8::Local<v8::Value>,
    ) -> Option<Self::Value>;
}

pub trait ValueSerializer {
    type Value;

    fn serialize<'a>(
        scope: &mut v8::HandleScope<'a>,
        value: Self::Value,
    ) -> v8::Local<'a, v8::Value>;
}

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

    pub fn deserialize<T: ValueDeserializer<Value = T>>(mut self) -> Option<T> {
        match self.v8_value {
            Some(v8_value) => T::deserialize(&mut self.scope, v8_value),
            None => None,
        }
    }
}
