use crate::js_value::utils::get_intern_value_from_v8_object;
use crate::js_value::utils::inject_serialized_into_v8_return_value;
use crate::serialize::deserialize::deserialize_v8;
use crate::JsObject;
use fruity_core::component::component_list_rwlock::ComponentListRwLock;
use fruity_core::serialize::serialized::Serialized;
use rusty_v8 as v8;
use std::convert::TryFrom;

impl JsObject {
    pub fn from_component_list_rwlock(
        scope: &mut v8::HandleScope,
        component_list: ComponentListRwLock,
    ) -> JsObject {
        let mut object = JsObject::from_intern_value(scope, "ComponentListRwLock", component_list);
        object.set_func(scope, "get", component_list_get_callback, None);
        object.set_func(scope, "length", component_list_length_callback, None);

        object
    }
}

fn component_list_get_callback(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut return_value: v8::ReturnValue,
) {
    // Get this as a component list
    let intern_value = get_intern_value_from_v8_object::<ComponentListRwLock>(scope, args.this());

    if let Some(component_list) = intern_value {
        // Build the arguments
        let mut deserialized_args = (0..args.length())
            .filter_map(|index| deserialize_v8(scope, args.get(index)))
            .collect::<Vec<_>>();

        if deserialized_args.len() != 1 {
            log::error!(
                "Failed to call method get cause you provided {} arguments, expected 1",
                args.length(),
            );
            return ();
        }

        let arg1 = deserialized_args.remove(0);
        let arg1 = if let Ok(arg) = usize::try_from(arg1) {
            arg
        } else {
            log::error!("Failed to call method get cause the argument n°0 have a wrong type");
            return ();
        };

        // Call the function
        let result = component_list.get(arg1);

        // Return the result
        if let Some(result) = result {
            inject_serialized_into_v8_return_value(
                scope,
                &Serialized::ComponentRwLock(result),
                &mut return_value,
            );
        }
    }
}

fn component_list_length_callback(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut return_value: v8::ReturnValue,
) {
    // Get this as a component list
    let intern_value = get_intern_value_from_v8_object::<ComponentListRwLock>(scope, args.this());

    if let Some(component_list) = intern_value {
        // Call the function
        let result = component_list.len();

        // Return the result
        inject_serialized_into_v8_return_value(
            scope,
            &Serialized::USize(result),
            &mut return_value,
        );
    }
}
