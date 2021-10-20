use crate::js_value::utils::check_object_intern_identifier;
use crate::js_value::utils::get_intern_value_from_v8_object;
use crate::JsObject;
use fruity_core::resource::resource::Resource;
use rusty_v8 as v8;
use std::sync::Arc;

impl JsObject {
    pub fn from_resource(scope: &mut v8::HandleScope, resource: Arc<dyn Resource>) -> JsObject {
        JsObject::from_intern_value(scope, "Resource", resource.clone())
    }
}

pub fn deserialize_v8_resource(
    scope: &mut v8::HandleScope,
    v8_value: v8::Local<v8::Value>,
) -> Option<Arc<dyn Resource>> {
    let v8_object = check_object_intern_identifier(scope, v8_value, "Resource")?;
    let intern_value = get_intern_value_from_v8_object::<Arc<dyn Resource>>(scope, v8_object)?;

    Some(intern_value.clone())
}
