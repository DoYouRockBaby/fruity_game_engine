use crate::js_value::utils::check_object_intern_identifier;
use crate::js_value::utils::get_intern_value_from_v8_object;
use crate::js_value::utils::get_intern_value_from_v8_object_mut;
use crate::js_value::utils::inject_serialized_into_v8_return_value;
use crate::serialize::deserialize::deserialize_v8;
use crate::JsObject;
use fruity_core::component::component::Component;
use rusty_v8 as v8;

impl JsObject {
    pub fn from_component(scope: &mut v8::HandleScope, component: Box<dyn Component>) -> JsObject {
        let mut object = JsObject::from_intern_value(scope, "Component", component.clone());

        let field_infos = component.get_field_infos();

        for field_info in field_infos {
            object.add_property(scope, &field_info.name, component_getter, component_setter);
        }

        object
    }
}

fn component_getter(
    scope: &mut v8::HandleScope,
    name: v8::Local<v8::Name>,
    args: v8::PropertyCallbackArguments,
    mut return_value: v8::ReturnValue,
) {
    // Get this as a component
    let intern_value = get_intern_value_from_v8_object::<Box<dyn Component>>(scope, args.this());

    if let Some(component) = intern_value {
        // Extract the current field info
        let field_info = {
            let field_infos = component.get_field_infos();
            let name = name.to_string(scope).unwrap().to_rust_string_lossy(scope);

            field_infos
                .iter()
                .find(|method_info| method_info.name == name)
                .unwrap()
                .clone()
        };

        // Call the function
        let result = (field_info.getter)(component.as_any_ref());

        // Return the result
        inject_serialized_into_v8_return_value(scope, &result, &mut return_value);
    }
}

fn component_setter(
    scope: &mut v8::HandleScope,
    name: v8::Local<v8::Name>,
    value: v8::Local<v8::Value>,
    args: v8::PropertyCallbackArguments,
) {
    // Get this as a component
    let intern_value =
        get_intern_value_from_v8_object_mut::<Box<dyn Component>>(scope, args.this());

    if let Some(component) = intern_value {
        // Extract the current field info
        let field_info = {
            let field_infos = component.get_field_infos();
            let name = name.to_string(scope).unwrap().to_rust_string_lossy(scope);

            field_infos
                .iter()
                .find(|method_info| method_info.name == name)
                .unwrap()
                .clone()
        };

        // Build the arguments
        let deserialized_arg = deserialize_v8(scope, value).unwrap();

        // Call the function
        (field_info.setter)(component.as_any_mut(), deserialized_arg);
    }
}

pub fn deserialize_v8_component(
    scope: &mut v8::HandleScope,
    v8_value: v8::Local<v8::Value>,
) -> Option<Box<dyn Component>> {
    let v8_object = check_object_intern_identifier(scope, v8_value, "Component")?;
    let intern_value = get_intern_value_from_v8_object::<Box<dyn Component>>(scope, v8_object)?;

    Some(intern_value.clone())
}
