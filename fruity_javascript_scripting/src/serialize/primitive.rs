use crate::value::ValueDeserializer;
use crate::value::ValueSerializer;
use rusty_v8 as v8;

macro_rules! impl_serialize_deserialize_int {
  { $type:ident } => {
    impl ValueDeserializer for $type {
        type Value = $type;

        fn deserialize(
            scope: &mut v8::HandleScope,
            v8_value: v8::Local<v8::Value>,
        ) -> Option<Self::Value> {
            match v8_value.to_integer(scope) {
                Some(v8_value) => Some(v8_value.value() as $type),
                None => None,
            }
        }
    }
    impl ValueSerializer for $type {
        type Value = $type;

        fn serialize<'a>(
            scope: &mut v8::HandleScope<'a>,
            value: &Self::Value,
        ) -> v8::Local<'a, v8::Value> {
            v8::Integer::new(scope, *value as i32).into()
        }
    }
  };
}

macro_rules! impl_serialize_deserialize_uint {
  { $type:ident } => {
    impl ValueDeserializer for $type {
        type Value = $type;

        fn deserialize(
            scope: &mut v8::HandleScope,
            v8_value: v8::Local<v8::Value>,
        ) -> Option<Self::Value> {
            match v8_value.to_uint32(scope) {
                Some(v8_value) => Some(v8_value.value() as $type),
                None => None,
            }
        }
    }
    impl ValueSerializer for $type {
        type Value = $type;

        fn serialize<'a>(
            scope: &mut v8::HandleScope<'a>,
            value: &Self::Value,
        ) -> v8::Local<'a, v8::Value> {
            v8::Integer::new_from_unsigned(scope, *value as u32).into()
        }
    }
  };
}

macro_rules! impl_serialize_deserialize_float {
  { $type:ident } => {
    impl ValueDeserializer for $type {
        type Value = $type;

        fn deserialize(
            scope: &mut v8::HandleScope,
            v8_value: v8::Local<v8::Value>,
        ) -> Option<Self::Value> {
            match v8_value.to_number(scope) {
                Some(v8_value) => Some(v8_value.value() as $type),
                None => None,
            }
        }
    }
    impl ValueSerializer for $type {
        type Value = $type;

        fn serialize<'a>(
            scope: &mut v8::HandleScope<'a>,
            value: &Self::Value,
        ) -> v8::Local<'a, v8::Value> {
            v8::Number::new(scope, *value as f64).into()
        }
    }
  };
}

impl ValueDeserializer for i64 {
    type Value = i64;

    fn deserialize(
        scope: &mut v8::HandleScope,
        v8_value: v8::Local<v8::Value>,
    ) -> Option<Self::Value> {
        match v8_value.to_big_int(scope) {
            Some(v8_value) => Some(v8_value.i64_value().0),
            None => None,
        }
    }
}

impl ValueSerializer for i64 {
    type Value = i64;

    fn serialize<'a>(
        scope: &mut v8::HandleScope<'a>,
        value: &Self::Value,
    ) -> v8::Local<'a, v8::Value> {
        v8::BigInt::new_from_i64(scope, *value).into()
    }
}

impl ValueDeserializer for u64 {
    type Value = u64;

    fn deserialize(
        scope: &mut v8::HandleScope,
        v8_value: v8::Local<v8::Value>,
    ) -> Option<Self::Value> {
        match v8_value.to_big_int(scope) {
            Some(v8_value) => Some(v8_value.u64_value().0),
            None => None,
        }
    }
}

impl ValueSerializer for u64 {
    type Value = u64;

    fn serialize<'a>(
        scope: &mut v8::HandleScope<'a>,
        value: &Self::Value,
    ) -> v8::Local<'a, v8::Value> {
        v8::BigInt::new_from_u64(scope, *value).into()
    }
}

impl ValueDeserializer for bool {
    type Value = bool;

    fn deserialize(
        scope: &mut v8::HandleScope,
        v8_value: v8::Local<v8::Value>,
    ) -> Option<Self::Value> {
        match v8_value.to_big_int(scope) {
            Some(v8_value) => Some(v8_value.boolean_value(scope)),
            None => None,
        }
    }
}

impl ValueSerializer for bool {
    type Value = bool;

    fn serialize<'a>(
        scope: &mut v8::HandleScope<'a>,
        value: &Self::Value,
    ) -> v8::Local<'a, v8::Value> {
        v8::Boolean::new(scope, *value).into()
    }
}

impl_serialize_deserialize_int! {i8}
impl_serialize_deserialize_int! {i16}
impl_serialize_deserialize_int! {i32}
impl_serialize_deserialize_int! {isize}
impl_serialize_deserialize_uint! {u8}
impl_serialize_deserialize_uint! {u16}
impl_serialize_deserialize_uint! {u32}
impl_serialize_deserialize_uint! {usize}
impl_serialize_deserialize_float! {f32}
impl_serialize_deserialize_float! {f64}
