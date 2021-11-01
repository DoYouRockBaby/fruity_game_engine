use crate::js_value::utils::inject_serialized_into_v8_return_value;
use crate::serialize::deserialize::deserialize_v8;
use crate::JsRuntime;
use fruity_core::object_factory::ObjectFactory;
use fruity_core::service::service::Service;
use fruity_core::service::service_manager::ServiceManager;
use fruity_introspect::serialized::ObjectFields;
use fruity_introspect::serialized::Serialized;
use rusty_v8 as v8;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::sync::Arc;
use std::sync::RwLock;

pub fn configure_constructors(
    runtime: &mut JsRuntime,
    service_manager: Arc<RwLock<ServiceManager>>,
) {
    let mut global_object = runtime.global_object();
    let scope = &mut runtime.handle_scope();

    let service_manager = service_manager.read().unwrap();
    let object_factory = service_manager.get::<ObjectFactory>().unwrap();
    let object_factory_reader = object_factory.read().unwrap();

    object_factory_reader.iter().for_each(|(key, ..)| {
        let mut data_fields = HashMap::new();

        let object_factory = object_factory.inner_arc();

        data_fields.insert(
            "object_factory".to_string(),
            Serialized::NativeObject(Box::new(object_factory)),
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
                let object_factory = Arc::<RwLock<Box<dyn Service>>>::try_from(
                    data_fields.get("object_factory").unwrap().clone(),
                )
                .unwrap();

                let object_factory = object_factory.read().unwrap();
                let object_factory = object_factory
                    .as_any_ref()
                    .downcast_ref::<ObjectFactory>()
                    .unwrap();

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
                    inject_serialized_into_v8_return_value(
                        scope,
                        &Serialized::NativeObject(result),
                        &mut return_value,
                    );
                }
            },
            Some(data),
        );
    });
}
