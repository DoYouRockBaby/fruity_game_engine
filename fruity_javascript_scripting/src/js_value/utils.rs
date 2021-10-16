use crate::serialize::serialize::serialize_v8;
use convert_case::Case;
use convert_case::Casing;
use fruity_ecs::serialize::serialized::Serialized;
use rusty_v8 as v8;
use std::any::Any;
use std::convert::TryFrom;

pub fn get_intern_value_from_v8_object<'a, T: Any>(
    scope: &mut v8::HandleScope,
    v8_object: v8::Local<'a, v8::Object>,
) -> Option<&'a T> {
    let this = v8_object.get_internal_field(scope, 0)?;
    let internal_field = unsafe { v8::Local::<v8::External>::cast(this) };
    let internal_object = internal_field.value() as *const T;
    unsafe { internal_object.as_ref() }
}

pub fn get_intern_value_from_v8_object_mut<'a, T: Any>(
    scope: &mut v8::HandleScope,
    v8_object: v8::Local<'a, v8::Object>,
) -> Option<&'a mut T> {
    let this = v8_object.get_internal_field(scope, 0)?;
    let internal_field = unsafe { v8::Local::<v8::External>::cast(this) };
    let internal_object = internal_field.value() as *mut T;
    unsafe { internal_object.as_mut() }
}

pub fn inject_serialized_into_v8_return_value<'a>(
    scope: &mut v8::HandleScope,
    serialized: &Serialized,
    return_value: &mut v8::ReturnValue,
) {
    let serialized = serialize_v8(scope, &serialized);

    if let Some(serialized) = serialized {
        return_value.set(serialized.into());
    }
}

pub fn inject_option_serialized_into_v8_return_value<'a>(
    scope: &mut v8::HandleScope,
    serialized: &Option<Serialized>,
    return_value: &mut v8::ReturnValue,
) {
    if let Some(serialized) = serialized {
        inject_serialized_into_v8_return_value(scope, serialized, return_value);
    }
}

pub fn format_function_name_from_rust_to_js(name: &str) -> String {
    name.to_case(Case::Camel)
}

pub fn check_object_intern_identifier<'a>(
    scope: &mut v8::HandleScope,
    v8_value: v8::Local<'a, v8::Value>,
    identifier: &str,
) -> Option<v8::Local<'a, v8::Object>> {
    if !v8_value.is_object() {
        return None;
    }

    let v8_object = v8::Local::<v8::Object>::try_from(v8_value).ok()?;
    if v8_object.internal_field_count() < 2 {
        return None;
    }

    let intern_identifier = v8_object.get_internal_field(scope, 1)?;
    let intern_identifier = v8::Local::<v8::String>::try_from(intern_identifier).ok()?;
    let intern_identifier = intern_identifier.to_rust_string_lossy(scope);

    if intern_identifier == identifier {
        Some(v8_object)
    } else {
        None
    }
}
