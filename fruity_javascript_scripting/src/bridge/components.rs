use crate::js_value::utils::inject_serialized_into_v8_return_value;
use crate::serialize::deserialize::deserialize_v8;
use crate::JsRuntime;
use fruity_core::component::components_factory::ComponentsFactory;
use fruity_core::service::service::Service;
use fruity_core::service::service_manager::ServiceManager;
use fruity_introspect::serialized::ObjectFields;
use fruity_introspect::serialized::Serialized;
use rusty_v8 as v8;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::sync::Arc;
use std::sync::RwLock;

pub fn configure_components(runtime: &mut JsRuntime, service_manager: Arc<RwLock<ServiceManager>>) {
    let mut global_object = runtime.global_object();
    let scope = &mut runtime.handle_scope();

    let service_manager = service_manager.read().unwrap();
    let components_factory = service_manager.get::<ComponentsFactory>().unwrap();
    let components_factory_reader = components_factory.read().unwrap();

    components_factory_reader.iter().for_each(|(key, ..)| {
        let mut data_fields = HashMap::new();

        let components_factory = components_factory.inner_arc();

        data_fields.insert(
            "components_factory".to_string(),
            Serialized::NativeObject(Box::new(components_factory)),
        );

        data_fields.insert(
            "component_identifier".to_string(),
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

                // Get the components factory
                let components_factory = Arc::<RwLock<Box<dyn Service>>>::try_from(
                    data_fields.get("components_factory").unwrap().clone(),
                )
                .unwrap();

                let components_factory = components_factory.read().unwrap();
                let components_factory = components_factory
                    .as_any_ref()
                    .downcast_ref::<ComponentsFactory>()
                    .unwrap();

                // Get the components identifier
                let component_identifier =
                    String::try_from(data_fields.get("component_identifier").unwrap().clone())
                        .unwrap();

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

                // Call the function
                let result = components_factory.instantiate(&component_identifier, arg1);

                // Return the result
                if let Some(result) = result {
                    inject_serialized_into_v8_return_value(
                        scope,
                        &Serialized::NativeObject(Box::new(result)),
                        &mut return_value,
                    );
                }
            },
            Some(data),
        );
    });
}
