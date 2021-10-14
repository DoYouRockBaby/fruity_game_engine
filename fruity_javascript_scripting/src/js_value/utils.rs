use crate::js_value::object::JsObjectInternalObject;
use crate::serialize::serialize::serialize_v8;
use convert_case::Case;
use convert_case::Casing;
use fruity_ecs::serialize::serialized::Serialized;
use rusty_v8 as v8;

pub fn get_internal_object_from_v8_args<'a>(
    scope: &mut v8::HandleScope,
    args: &v8::FunctionCallbackArguments,
) -> &'a JsObjectInternalObject {
    let this = args.this().get_internal_field(scope, 0).unwrap();
    let this = unsafe { v8::Local::<v8::External>::cast(this) };
    let internal_object = this.value() as *const JsObjectInternalObject;
    unsafe { internal_object.as_ref().unwrap() }
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
