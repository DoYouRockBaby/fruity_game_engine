use crate::js_value::function::JsFunction;
use crate::js_value::property::JsProperty;
use crate::js_value::utils::format_function_name_from_rust_to_js;
use crate::js_value::utils::get_internal_object_from_v8_args;
use crate::js_value::utils::inject_option_serialized_into_v8_return_value;
use crate::js_value::utils::inject_serialized_into_v8_return_value;
use crate::js_value::value::JsValue;
use crate::serialize::deserialize::deserialize_v8;
use crate::serialize::serialize::serialize_v8;
use core::ffi::c_void;
use fruity_ecs::component::component_list_rwlock::ComponentListRwLock;
use fruity_ecs::component::component_rwlock::ComponentRwLock;
use fruity_ecs::entity::entity_rwlock::EntityRwLock;
use fruity_ecs::serialize::serialized::Serialized;
use fruity_ecs::service::service::Service;
use fruity_ecs::service::service_manager::ServiceManager;
use fruity_introspect::log_introspect_error;
use fruity_introspect::MethodCaller;
use rusty_v8 as v8;
use std::any::Any;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt::Debug;
use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::Arc;
use std::sync::RwLock;

#[derive(Clone)]
pub enum JsObjectInternalObject {
    Service(Arc<RwLock<Box<dyn Service>>>),
    Entity(EntityRwLock),
    Component(ComponentRwLock),
    ComponentList(ComponentListRwLock),
    Iterator(Arc<RwLock<dyn Iterator<Item = Serialized> + Send + Sync>>),
    ServiceManager(Arc<RwLock<ServiceManager>>),
}

impl Debug for JsObjectInternalObject {
    fn fmt(
        &self,
        _formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct JsObject {
    pub(crate) fields: HashMap<String, Box<dyn JsValue>>,
    internal_object: Option<JsObjectInternalObject>,
}

impl Drop for JsObject {
    fn drop(&mut self) {
        std::mem::forget(self.internal_object.take());
    }
}

impl JsObject {
    pub fn new() -> JsObject {
        JsObject {
            fields: HashMap::new(),
            internal_object: None,
        }
    }

    pub fn from_internal(internal_object: JsObjectInternalObject) -> JsObject {
        JsObject {
            fields: HashMap::new(),
            internal_object: Some(internal_object),
        }
    }

    pub fn from_iterator(
        iterator: Arc<RwLock<dyn Iterator<Item = Serialized> + Send + Sync>>,
    ) -> JsObject {
        let mut fields: HashMap<String, Box<dyn JsValue>> = HashMap::new();

        fields.insert(
            "next".to_string(),
            Box::new(JsFunction::new(
                |scope: &mut v8::HandleScope,
                 args: v8::FunctionCallbackArguments,
                 mut return_value: v8::ReturnValue| {
                    // Get this an entity
                    let internal_object = get_internal_object_from_v8_args(scope, &args);

                    if let JsObjectInternalObject::Iterator(iterator) = internal_object {
                        // Call the function
                        let mut iterator = iterator.write().unwrap();
                        let result = iterator.next();

                        // Return the result
                        //let serialized = serialize_v8(scope, &serialized);
                        let result = match result {
                            Some(value) => serialize_v8(scope, &value),
                            None => None,
                        };

                        let return_object = match result {
                            Some(value) => {
                                let result = v8::Object::new(scope);

                                // Set done to false
                                let done = v8::Boolean::new(scope, false);
                                let done_string = v8::String::new(scope, "done").unwrap();
                                result.set(scope, done_string.into(), done.into());

                                // Set value
                                let value_string = v8::String::new(scope, "value").unwrap();
                                result.set(scope, value_string.into(), value);

                                result
                            }
                            None => {
                                let result = v8::Object::new(scope);

                                // Set done to true
                                let done = v8::Boolean::new(scope, true);
                                let done_string = v8::String::new(scope, "done").unwrap();
                                result.set(scope, done_string.into(), done.into());

                                result
                            }
                        };

                        return_value.set(return_object.into());
                    }
                },
            )),
        );

        fields.insert(
            "forEach".to_string(),
            Box::new(JsFunction::new(
                |scope: &mut v8::HandleScope,
                 args: v8::FunctionCallbackArguments,
                 mut _return_value: v8::ReturnValue| {
                    let this = args.this();

                    // Get callback
                    let callback = args.get(0);
                    match v8::Local::<v8::Function>::try_from(callback) {
                        Ok(callback) => {
                            // Get next function
                            let next_string = v8::String::new(scope, "next").unwrap();
                            let next = this.get(scope, next_string.into()).unwrap();
                            let next = v8::Local::<v8::Function>::try_from(next).unwrap();

                            // Execute next function
                            let next_value = next.call(scope, this.into(), &[]).unwrap();
                            let mut next_value_object =
                                v8::Local::<v8::Object>::try_from(next_value).unwrap();

                            // Get the next result.done value
                            let done_string = v8::String::new(scope, "done").unwrap();
                            let mut next_done_value = next_value_object
                                .get(scope, done_string.into())
                                .unwrap()
                                .boolean_value(scope);

                            // Iterate for each element
                            while !next_done_value {
                                // Get the current iterator value
                                let value_string = v8::String::new(scope, "value").unwrap();
                                let next_value_value =
                                    next_value_object.get(scope, value_string.into()).unwrap();

                                // Call the callback
                                let undefined = v8::undefined(scope);
                                callback.call(scope, undefined.into(), &[next_value_value]);

                                // Get the next result.done value
                                match next.call(scope, this.into(), &[]) {
                                    Some(next_value) => {
                                        next_value_object =
                                            v8::Local::<v8::Object>::try_from(next_value).unwrap();
                                        next_done_value = next_value_object
                                            .get(scope, done_string.into())
                                            .unwrap()
                                            .boolean_value(scope);
                                    }
                                    None => {
                                        next_done_value = true;
                                    }
                                }
                            }
                        }
                        Err(_) => (),
                    };
                },
            )),
        );

        JsObject {
            fields,
            internal_object: Some(JsObjectInternalObject::Iterator(iterator)),
        }
    }

    pub fn from_service(service: Arc<RwLock<Box<dyn Service>>>) -> JsObject {
        let mut fields: HashMap<String, Box<dyn JsValue>> = HashMap::new();

        let method_infos = {
            let reader = service.read().unwrap();
            reader.get_method_infos()
        };

        for method_info in method_infos {
            fields.insert(
                format_function_name_from_rust_to_js(&method_info.name),
                Box::new(JsFunction::new(service_callback)),
            );
        }

        JsObject {
            fields,
            internal_object: Some(JsObjectInternalObject::Service(service.clone())),
        }
    }

    pub fn from_entity(entity: EntityRwLock) -> JsObject {
        let mut fields: HashMap<String, Box<dyn JsValue>> = HashMap::new();

        fields.insert(
            "lenght".to_string(),
            Box::new(JsFunction::new(
                |scope: &mut v8::HandleScope,
                 args: v8::FunctionCallbackArguments,
                 mut return_value: v8::ReturnValue| {
                    // Get this an entity
                    let internal_object = get_internal_object_from_v8_args(scope, &args);

                    if let JsObjectInternalObject::Entity(entity) = internal_object {
                        // Call the function
                        let entity = entity.read().unwrap();
                        let result = entity.len();

                        // Return the result
                        inject_serialized_into_v8_return_value(
                            scope,
                            &Serialized::USize(result),
                            &mut return_value,
                        );
                    }
                },
            )),
        );

        JsObject {
            fields,
            internal_object: Some(JsObjectInternalObject::Entity(entity)),
        }
    }

    pub fn from_component(component: ComponentRwLock) -> JsObject {
        let mut fields: HashMap<String, Box<dyn JsValue>> = HashMap::new();

        let field_infos = {
            let reader = component.read().unwrap();
            reader.get_field_infos()
        };

        for field_info in field_infos {
            fields.insert(field_info.name, Box::new(JsProperty::new()));
        }

        JsObject {
            fields,
            internal_object: Some(JsObjectInternalObject::Component(component)),
        }
    }

    pub fn from_component_list(component_list: ComponentListRwLock) -> JsObject {
        let mut fields: HashMap<String, Box<dyn JsValue>> = HashMap::new();

        fields.insert(
            "get".to_string(),
            Box::new(JsFunction::new(
                |scope: &mut v8::HandleScope,
                 args: v8::FunctionCallbackArguments,
                 mut return_value: v8::ReturnValue| {
                    // Get this an entity
                    let internal_object = get_internal_object_from_v8_args(scope, &args);

                    if let JsObjectInternalObject::ComponentList(component_list) = internal_object {
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

                        let arg1 = deserialized_args.get(0).unwrap();
                        let arg1 = if let Some(arg) = arg1.as_usize() {
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
                                &Serialized::Component(result),
                                &mut return_value,
                            );
                        }
                    }
                },
            )),
        );

        fields.insert(
            "lenght".to_string(),
            Box::new(JsFunction::new(
                |scope: &mut v8::HandleScope,
                 args: v8::FunctionCallbackArguments,
                 mut return_value: v8::ReturnValue| {
                    // Get this an entity
                    let internal_object = get_internal_object_from_v8_args(scope, &args);

                    if let JsObjectInternalObject::ComponentList(component_list) = internal_object {
                        // Call the function
                        let result = component_list.len();

                        // Return the result
                        inject_serialized_into_v8_return_value(
                            scope,
                            &Serialized::USize(result),
                            &mut return_value,
                        );
                    }
                },
            )),
        );

        JsObject {
            fields,
            internal_object: Some(JsObjectInternalObject::ComponentList(component_list)),
        }
    }

    pub fn add_field<T: JsValue>(&mut self, name: &str, value: T) {
        self.fields
            .insert(format_function_name_from_rust_to_js(name), Box::new(value));
    }

    pub fn set_func(
        &mut self,
        name: &str,
        callback: impl v8::MapFnTo<v8::FunctionCallback>,
    ) -> &mut JsFunction {
        self.fields.insert(
            format_function_name_from_rust_to_js(name),
            Box::new(JsFunction::new(callback)),
        );

        self.fields
            .get_mut(&name.to_string())
            .unwrap()
            .as_mut_any()
            .downcast_mut::<JsFunction>()
            .unwrap()
    }

    pub fn build_v8_object<'a>(
        &mut self,
        scope: &mut v8::HandleScope<'a>,
    ) -> v8::Local<'a, v8::Object> {
        // Create the object
        let object_template = v8::ObjectTemplate::new(scope);
        if let Some(_) = self.internal_object {
            object_template.set_internal_field_count(1);
        }

        let object = object_template.new_instance(scope).unwrap();

        // Add the intern object reference to the js object
        // This will be used to access this on methods
        if let Some(internal_object) = &self.internal_object {
            let internal_object = Box::new(internal_object.clone());
            let ref_value = v8::External::new(
                scope,
                Box::leak(internal_object) as *mut JsObjectInternalObject as *mut c_void,
            );

            object.set_internal_field(0, ref_value.into());
        }

        // Add the fieds
        self.fields
            .iter_mut()
            .for_each(|(name, field)| field.register(scope, name, object));

        object
    }
}

impl JsValue for JsObject {
    fn register(&mut self, scope: &mut v8::HandleScope, name: &str, parent: v8::Local<v8::Object>) {
        // Create the object
        let object = self.build_v8_object(scope);
        let key = v8::String::new(scope, name).unwrap();

        parent.set(scope, key.into(), object.into());
    }

    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self as &mut dyn Any
    }

    fn as_boxed_any(self: Box<Self>) -> Box<dyn Any> {
        self as Box<dyn Any>
    }
}

fn service_callback(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut return_value: v8::ReturnValue,
) {
    // Get this as service
    let this = get_internal_object_from_v8_args(scope, &args);

    if let JsObjectInternalObject::Service(this) = this {
        // Extract the current method info
        let method_info = {
            let reader = this.read().unwrap();
            let this = reader.deref();

            let method_infos = this.get_method_infos().clone();
            let name = args
                .data()
                .unwrap()
                .to_string(scope)
                .unwrap()
                .to_rust_string_lossy(scope);

            method_infos
                .iter()
                .find(|method_info| format_function_name_from_rust_to_js(&method_info.name) == name)
                .unwrap()
                .clone()
        };

        // Build the arguments
        let deserialized_args = (0..args.length())
            .filter_map(|index| deserialize_v8(scope, args.get(index)))
            .collect::<Vec<_>>();

        // Call the function
        let result = match method_info.call {
            MethodCaller::Const(call) => {
                let reader = this.read().unwrap();
                let this = &**reader.deref();
                match call(this.as_any_ref(), deserialized_args) {
                    Ok(result) => result,
                    Err(err) => {
                        log_introspect_error(&err);
                        None
                    }
                }
            }
            MethodCaller::Mut(call) => {
                let mut writer = this.write().unwrap();
                let this = &mut **writer.deref_mut();
                match call(this.as_any_mut(), deserialized_args) {
                    Ok(result) => result,
                    Err(err) => {
                        log_introspect_error(&err);
                        None
                    }
                }
            }
        };

        // Return the result
        inject_option_serialized_into_v8_return_value(scope, &result, &mut return_value);
    }
}
