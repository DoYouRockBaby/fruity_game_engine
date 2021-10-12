use crate::value::ValueDeserializer;
use crate::value::ValueSerializer;
use rusty_v8 as v8;

impl ValueDeserializer for String {
    type Value = String;

    fn deserialize(
        scope: &mut v8::HandleScope,
        v8_value: v8::Local<v8::Value>,
    ) -> Option<Self::Value> {
        match v8_value.to_string(scope) {
            Some(v8_value) => Some(v8_value.to_rust_string_lossy(scope)),
            None => None,
        }
    }
}

impl ValueSerializer for String {
    type Value = String;

    fn serialize<'a>(
        scope: &mut v8::HandleScope<'a>,
        value: &Self::Value,
    ) -> v8::Local<'a, v8::Value> {
        v8::String::new(scope, &value).unwrap().into()
    }
}
