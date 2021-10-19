use crate::js_value::object::component::deserialize_v8_component;
use crate::js_value::object::resource::deserialize_v8_resource;
use crate::js_value::object::service::deserialize_v8_service;
use crate::js_value::utils::store_callback;
use crate::JavascriptEngine;
use fruity_ecs::serialize::serialized::Serialized;
use fruity_ecs::service::service_manager::ServiceManager;
use fruity_introspect::IntrospectError;
use rusty_v8 as v8;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::sync::Arc;
use std::sync::RwLock;

pub fn deserialize_v8<'a>(
    scope: &mut v8::HandleScope<'a>,
    v8_value: v8::Local<v8::Value>,
) -> Option<Serialized> {
    if v8_value.is_int32() {
        return Some(Serialized::I32(v8_value.int32_value(scope).unwrap()));
    }

    if v8_value.is_uint32() {
        return Some(Serialized::U32(v8_value.uint32_value(scope).unwrap()));
    }

    if v8_value.is_big_int() {
        return Some(Serialized::I64(v8_value.integer_value(scope).unwrap()));
    }

    if v8_value.is_number() {
        return Some(Serialized::F64(v8_value.number_value(scope).unwrap()));
    }

    if v8_value.is_boolean() {
        return Some(Serialized::Bool(v8_value.boolean_value(scope)));
    }

    if v8_value.is_string() {
        return Some(Serialized::String(v8_value.to_rust_string_lossy(scope)));
    }

    if v8_value.is_array() {
        let v8_array = v8::Local::<v8::Array>::try_from(v8_value).unwrap();
        let serialized_array = (0..v8_array.length())
            .filter_map(|index| {
                let v8_element = v8_array.get_index(scope, index).unwrap();
                deserialize_v8(scope, v8_element)
            })
            .collect::<Vec<_>>();

        return Some(Serialized::Array(serialized_array));
    }

    if v8_value.is_function() {
        // Store the function into a global object
        let v8_function = v8::Local::<v8::Function>::try_from(v8_value).unwrap();
        let callback_identifier = store_callback(scope, v8_function);

        let callback = move |service_manager: Arc<RwLock<ServiceManager>>,
                             args: Vec<Serialized>|
              -> Result<Option<Serialized>, IntrospectError> {
            // Get scope
            let service_manager = service_manager.read().unwrap();
            let javascript_engine = service_manager.read::<JavascriptEngine>();
            javascript_engine.run_callback(callback_identifier, args);

            Ok(None)
        };

        return Some(Serialized::Callback(Arc::new(callback)));
    }

    if let Some(component) = deserialize_v8_component(scope, v8_value) {
        return Some(Serialized::Component(component));
    }

    if let Some(service) = deserialize_v8_service(scope, v8_value) {
        return Some(Serialized::Service(service));
    }

    if let Some(resource) = deserialize_v8_resource(scope, v8_value) {
        return Some(Serialized::Resource(resource));
    }

    if v8_value.is_object() {
        let v8_object = v8::Local::<v8::Object>::try_from(v8_value).unwrap();

        // Read the class name
        let class_name = {
            let constructor_key = v8::String::new(scope, "constructor").unwrap();
            let constructor_object = v8_object.get(scope, constructor_key.into())?;
            let constructor_object = v8::Local::<v8::Object>::try_from(constructor_object).ok()?;

            let name_key = v8::String::new(scope, "name").unwrap();
            let name_string = constructor_object.get(scope, name_key.into())?;
            let name_string = v8::Local::<v8::String>::try_from(name_string).ok()?;
            name_string.to_rust_string_lossy(scope)
        };

        // Read all value properties recursively
        let property_keys = v8_object.get_own_property_names(scope).unwrap();
        let mut properties = (0..property_keys.length())
            .filter_map(|property_index| {
                let property_key = property_keys.get_index(scope, property_index).unwrap();
                let property_name = property_key.to_rust_string_lossy(scope);
                let property = v8_object.get(scope, property_key).unwrap();

                deserialize_v8(scope, property).map(|serialized| (property_name, serialized))
            })
            .collect::<Vec<_>>();

        // Read all prototype properties recursively
        let prototype = v8_object.get_prototype(scope).unwrap();
        let prototype = v8::Local::<v8::Object>::try_from(prototype).unwrap();
        let property_keys = prototype.get_property_names(scope).unwrap();
        let mut prototype_properties = (0..property_keys.length())
            .filter_map(|property_index| {
                let property_key = property_keys.get_index(scope, property_index).unwrap();
                let property_name = property_key.to_rust_string_lossy(scope);
                let property = prototype.get(scope, property_key).unwrap();

                deserialize_v8(scope, property).map(|serialized| (property_name, serialized))
            })
            .collect::<Vec<_>>();

        properties.append(&mut prototype_properties);

        // Create the serialized object
        let mut fields = HashMap::new();
        properties.iter().for_each(|property| {
            fields.insert(property.0.clone(), property.1.clone());
        });

        return Some(Serialized::Object { class_name, fields });
    }

    None
}
