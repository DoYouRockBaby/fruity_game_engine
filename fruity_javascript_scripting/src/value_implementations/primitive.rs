use rusty_v8 as v8;

use crate::value::ValueDeserializer;
use crate::value::ValueSerializer;

impl ValueDeserializer for i32 {
    type Value = i32;

    fn deserialize(
        scope: &mut v8::HandleScope,
        v8_value: v8::Local<v8::Value>,
    ) -> Option<Self::Value> {
        match v8_value.to_uint32(scope) {
            Some(v8_value) => Some(v8_value.value() as i32),
            None => None,
        }
    }
}

impl ValueSerializer for i32 {
    type Value = i32;

    fn serialize<'a>(
        scope: &mut v8::HandleScope<'a>,
        value: Self::Value,
    ) -> v8::Local<'a, v8::Value> {
        v8::Local::<'a, v8::Value>::from(v8::Integer::new(scope, value))
    }
}
