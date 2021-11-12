use crate::js_value::utils::inject_serialized_into_v8_return_value;
use crate::serialize::deserialize::deserialize_v8;
use crate::JsRuntime;
use fruity_core::object_factory::ObjectFactory;
use fruity_core::resource::resource_manager::ResourceManager;
use fruity_core::resource::resource_reference::ResourceReference;
use fruity_introspect::serialized::ObjectFields;
use fruity_introspect::serialized::Serialized;
use rusty_v8 as v8;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::sync::Arc;

pub fn configure_constructors(runtime: &mut JsRuntime, resource_manager: Arc<ResourceManager>) {
    let mut global_object = runtime.global_object();
    let scope = &mut runtime.handle_scope();

    let object_factory = resource_manager.require::<ObjectFactory>("object_factory");
    let object_factory_reader = object_factory.read();

    object_factory_reader.iter().for_each(|(key, ..)| {
        let mut data_fields = HashMap::new();

        data_fields.insert(
            "object_factory".to_string(),
            Serialized::NativeObject(Box::new(object_factory.clone())),
        );

        data_fields.insert(
            "object_identifier".to_string(),
            Serialized::String(key.clone()),
        );

        let data = Serialized::SerializedObject {
            class_name: "unknown".to_string(),
            fields: data_fields,
        };

        global_object.set_func_with_raw_name(
            scope,
            key,
            |scope: &mut v8::HandleScope,
             args: v8::FunctionCallbackArguments,
             mut return_value: v8::ReturnValue| {
                // Get the data
                let data = deserialize_v8(scope, args.data().unwrap()).unwrap();
                let data_fields = ObjectFields::try_from(data).unwrap();

                // Get the object factory
                let test = data_fields.get("object_factory").unwrap().clone();
                let object_factory = ResourceReference::<ObjectFactory>::try_from(test).unwrap();
                let object_factory = object_factory.read();

                // Get the object identifier
                let object_identifier =
                    String::try_from(data_fields.get("object_identifier").unwrap().clone())
                        .unwrap();

                // Build the arguments
                let deserialized_args = (0..args.length())
                    .filter_map(|index| deserialize_v8(scope, args.get(index)))
                    .collect::<Vec<_>>();

                if deserialized_args.len() != 1 {
                    log::error!(
                        "Failed to call method get cause you provided {} arguments, expected 1",
                        args.length(),
                    );
                    return ();
                }

                // Call the function
                let result = object_factory.instantiate(&object_identifier, deserialized_args);

                // Return the result
                if let Some(result) = result {
                    inject_serialized_into_v8_return_value(scope, &result, &mut return_value);
                }
            },
            Some(data),
        );
    });
}
