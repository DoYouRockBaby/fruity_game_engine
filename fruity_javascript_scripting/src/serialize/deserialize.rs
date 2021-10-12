use rusty_v8 as v8;
use std::any::Any;

pub fn deserialize<'a>(
    scope: &mut v8::HandleScope<'a>,
    v8_value: v8::Local<v8::Value>,
) -> Option<Box<dyn Any>> {
    if v8_value.is_int32() {
        return Some(Box::new(v8_value.int32_value(scope).unwrap()));
    }

    if v8_value.is_uint32() {
        return Some(Box::new(v8_value.uint32_value(scope).unwrap()));
    }

    if v8_value.is_big_int() {
        return Some(Box::new(v8_value.integer_value(scope).unwrap()));
    }

    if v8_value.is_number() {
        return Some(Box::new(v8_value.number_value(scope).unwrap()));
    }

    if v8_value.is_boolean() {
        return Some(Box::new(v8_value.boolean_value(scope)));
    }

    if v8_value.is_string() {
        return Some(Box::new(v8_value.to_rust_string_lossy(scope)));
    }

    None
}
