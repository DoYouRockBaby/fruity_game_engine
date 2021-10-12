use fruity_serialize::serialized::Serialized;
use rusty_v8 as v8;

pub fn deserialize_v8<'a>(
    scope: &mut v8::HandleScope<'a>,
    v8_value: v8::Local<v8::Value>,
) -> Option<Serialized> {
    if v8_value.is_int32() {
        return Some(Serialized::I32(v8_value.int32_value(scope).unwrap()));
    }

    if v8_value.is_uint32() {
        return Some(Serialized::U32(v8_value.uint32_value(scope).unwrap()));
    }

    if v8_value.is_big_int() {
        return Some(Serialized::I64(v8_value.integer_value(scope).unwrap()));
    }

    if v8_value.is_number() {
        return Some(Serialized::F64(v8_value.number_value(scope).unwrap()));
    }

    if v8_value.is_boolean() {
        return Some(Serialized::Bool(v8_value.boolean_value(scope)));
    }

    if v8_value.is_string() {
        return Some(Serialized::String(v8_value.to_rust_string_lossy(scope)));
    }

    None
}
