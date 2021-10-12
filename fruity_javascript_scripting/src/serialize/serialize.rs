use crate::value::ValueSerializer;
use rusty_v8 as v8;
use std::any::Any;

macro_rules! serialize_type {
    ($scope:expr, $value:expr, $type:ident) => {
        match $value.downcast_ref::<$type>() {
            Some(value) => return Some($type::serialize($scope, value)),
            _ => (),
        };
    };
}

pub fn serialize<'a>(
    scope: &mut v8::HandleScope<'a>,
    value: &dyn Any,
) -> Option<v8::Local<'a, v8::Value>> {
    serialize_type!(scope, value, i8);
    serialize_type!(scope, value, i16);
    serialize_type!(scope, value, i32);
    serialize_type!(scope, value, i64);
    serialize_type!(scope, value, isize);
    serialize_type!(scope, value, u8);
    serialize_type!(scope, value, u16);
    serialize_type!(scope, value, u32);
    serialize_type!(scope, value, u64);
    serialize_type!(scope, value, usize);
    serialize_type!(scope, value, f32);
    serialize_type!(scope, value, f64);
    serialize_type!(scope, value, bool);
    serialize_type!(scope, value, String);

    None
}
