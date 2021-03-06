use crate::js_value::value::JsValue;
use crate::JsObject;
use fruity_core::serialize::serialized::Serialized;
use rusty_v8 as v8;

pub fn serialize_v8<'a>(
    scope: &mut v8::HandleScope<'a>,
    value: Serialized,
) -> Option<v8::Local<'a, v8::Value>> {
    match value {
        Serialized::I8(value) => Some(v8::Integer::new(scope, value as i32).into()),
        Serialized::I16(value) => Some(v8::Integer::new(scope, value as i32).into()),
        Serialized::I32(value) => Some(v8::Integer::new(scope, value).into()),
        Serialized::I64(value) => Some(v8::BigInt::new_from_i64(scope, value).into()),
        Serialized::ISize(value) => Some(v8::Integer::new(scope, value as i32).into()),
        Serialized::U8(value) => Some(v8::Integer::new_from_unsigned(scope, value as u32).into()),
        Serialized::U16(value) => Some(v8::Integer::new_from_unsigned(scope, value as u32).into()),
        Serialized::U32(value) => Some(v8::Integer::new_from_unsigned(scope, value).into()),
        Serialized::U64(value) => Some(v8::BigInt::new_from_u64(scope, value).into()),
        Serialized::USize(value) => {
            Some(v8::Integer::new_from_unsigned(scope, value as u32).into())
        }
        Serialized::F32(value) => Some(v8::Number::new(scope, value as f64).into()),
        Serialized::F64(value) => Some(v8::Number::new(scope, value).into()),
        Serialized::Bool(value) => Some(v8::Boolean::new(scope, value).into()),
        Serialized::String(value) => Some(v8::String::new(scope, &value).unwrap().into()),
        Serialized::Array(value) => {
            let elements = value
                .into_iter()
                .filter_map(|elem| serialize_v8(scope, elem))
                .collect::<Vec<_>>();

            Some(v8::Array::new_with_elements(scope, &elements).into())
        }
        Serialized::Null => Some(v8::null(scope).into()),
        Serialized::Iterator(value) => {
            let mut object = JsObject::from_iterator(scope, value.clone());
            Some(object.as_v8(scope).into())
        }
        Serialized::Callback(_) => None,
        Serialized::SerializedObject { fields, .. } => {
            let v8_object = v8::Object::new(scope);

            fields.into_iter().for_each(|(key, value)| {
                let key = v8::String::new(scope, &key).unwrap();
                if let Some(serialized) = serialize_v8(scope, value) {
                    v8_object.set(scope, key.into(), serialized);
                }
            });

            Some(v8_object.into())
        }
        Serialized::NativeObject(value) => {
            let mut object = JsObject::from_introspect_object(scope, value);
            Some(object.as_v8(scope).into())
        }
    }
}
