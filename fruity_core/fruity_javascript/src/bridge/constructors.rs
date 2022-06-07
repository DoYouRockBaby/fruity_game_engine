use crate::js_value::utils::inject_serialized_into_v8_return_value;
use crate::serialize::deserialize::deserialize_v8;
use crate::JsRuntime;
use fruity_core::convert::FruityTryFrom;
use fruity_core::object_factory_service::ObjectFactoryService;
use fruity_core::resource::resource_container::ResourceContainer;
use fruity_core::resource::resource_reference::ResourceReference;
use fruity_core::serialize::serialized::ObjectFields;
use fruity_core::serialize::serialized::Serialized;
use rusty_v8 as v8;
use std::collections::HashMap;
use std::sync::Arc;

pub fn configure_constructors(runtime: &mut JsRuntime, resource_container: Arc<ResourceContainer>) {
    let mut global_object = runtime.global_object();
    let scope = &mut runtime.handle_scope();

    let object_factory_service = resource_container.require::<ObjectFactoryService>();
    let object_factory_service_reader = object_factory_service.read();

    object_factory_service_reader.iter().for_each(|(key, ..)| {
        let mut data_fields = HashMap::new();

        data_fields.insert(
            "object_factory_service".to_string(),
            Serialized::NativeObject(Box::new(object_factory_service.clone())),
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
                let data_fields = ObjectFields::fruity_try_from(data).unwrap();

                // Get the object factory
                let object_factory_service =
                    ResourceReference::<ObjectFactoryService>::fruity_try_from(
                        data_fields.get("object_factory_service").unwrap().clone(),
                    )
                    .unwrap();
                let object_factory_service = object_factory_service.read();

                // Get the object identifier
                let object_identifier =
                    String::fruity_try_from(data_fields.get("object_identifier").unwrap().clone())
                        .unwrap();

                // Build the arguments
                let deserialized_args = (0..args.length())
                    .filter_map(|index| deserialize_v8(scope, args.get(index)))
                    .collect::<Vec<_>>();

                // Call the function
                let result =
                    object_factory_service.instantiate(&object_identifier, deserialized_args);

                // Return the result
                if let Some(result) = result {
                    inject_serialized_into_v8_return_value(scope, result, &mut return_value);
                }
            },
            Some(data),
        );
    });
}
